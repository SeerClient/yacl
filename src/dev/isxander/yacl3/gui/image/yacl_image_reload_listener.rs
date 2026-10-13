//! Ported from dev.isxander.yacl3.gui.image.YACLImageReloadListener.
//!
//! `implements PreparableReloadListener`: prepares every `.webp` / `.gif` texture the preloaded factories match (on
//! the background executor), then completes them on the game executor into `PRELOADED_IMAGE_CACHE`.
//!
//! rust: port gap (GAPS_Y2.md G5) - the port has no API for a mod to add a resource reload listener
//! (`ResourceLoader.get(PackType.CLIENT_RESOURCES).registerReloadListener`), so `entry::platform_entrypoint` cannot
//! register this one and images load on demand (`ImageRendererManager.registerOrGetImage`). `reload` is the
//! listener's work run in one call on the render thread, for the reload hook to call once it exists.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::{ImageRendererFactory, ImageSupplier};
use crate::dev::isxander::yacl3::gui::image::image_renderer_manager::{self, ImageRendererManager, PRELOADED_IMAGE_CACHE};
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::server::packs::resources::reloadable_resource_manager::ReloadableResourceManager;

/// java: private record SupplierPreparation(Identifier location, ImageRendererFactory.ImageSupplier supplier)
pub struct SupplierPreparation {
    /// java: Identifier location
    pub location: Identifier,
    /// java: ImageRendererFactory.ImageSupplier supplier
    pub supplier: Box<dyn ImageSupplier>,
}

/// java: public class YACLImageReloadListener implements PreparableReloadListener
#[derive(Default)]
pub struct YACLImageReloadListener;

impl YACLImageReloadListener {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self
    }

    /// java: @Override public CompletableFuture<Void> reload(SharedState sharedState, Executor backgroundExecutor,
    /// PreparationBarrier preparationBarrier, Executor gameExecutor) - `prepare`, the barrier, then `apply`.
    pub fn reload(&self, resource_manager: &ReloadableResourceManager) {
        let suppliers = self.prepare(resource_manager);
        self.apply(suppliers);
    }

    /// java: private CompletableFuture<List<Optional<SupplierPreparation>>> prepare(ResourceManager manager, Executor executor)
    fn prepare(&self, manager: &ReloadableResourceManager) -> Vec<Option<SupplierPreparation>> {
        let factories = image_renderer_manager::preloaded_image_factories();
        let image_resources = manager.list_resources("textures", &|location: &Identifier| {
            factories.iter().any(|factory| (factory.predicate)(location))
        });

        image_resources
            .into_keys()
            .map(|location| {
                let image_factory: Box<dyn ImageRendererFactory> = factories
                    .iter()
                    .filter(|factory| (factory.predicate)(&location))
                    .map(|factory| (factory.factory)(location.clone()))
                    .next()
                    .expect("NoSuchElementException: a listed image matched no factory");
                ImageRendererManager::safely_prepare_factory(&location, &*image_factory)
                    .map(|supplier| SupplierPreparation { location, supplier })
            })
            .collect()
    }

    /// java: private CompletableFuture<Void> apply(List<Optional<SupplierPreparation>> suppliers, Executor executor)
    fn apply(&self, suppliers: Vec<Option<SupplierPreparation>>) {
        for prep in suppliers.into_iter().flatten() {
            let location = prep.location.clone();
            let image_renderer: Arc<dyn ImageRenderer> = match prep.supplier.complete_image() {
                Ok(image_renderer) => image_renderer,
                Err(e) => {
                    LOGGER.error(&format!("Failed to create image '{location}': {e}"));
                    continue;
                }
            };
            PRELOADED_IMAGE_CACHE.with(|cache| cache.borrow_mut().insert(location.clone(), image_renderer));
            LOGGER.info(&format!("Successfully loaded image '{location}'"));
        }
    }

    /// java: public static Identifier getId()
    pub fn get_id() -> Identifier {
        YACLPlatform::rl("image_reload_listener")
    }
}

/// java: public static class CompletableFutureCollector<X, T extends CompletableFuture<X>> implements
/// Collector<T, List<T>, CompletableFuture<List<X>>> - collects futures into one future of their results, in order
/// (`CompletableFuture.allOf(..).thenApply(v -> ls.stream().map(CompletableFuture::join).toList())`).
pub struct CompletableFutureCollector;

impl CompletableFutureCollector {
    /// java: public static <X, T extends CompletableFuture<X>> Collector<T, List<T>, CompletableFuture<List<X>>> allOf()
    /// - the collector applied to `futures`.
    pub fn all_of<X: Clone + 'static>(futures: Vec<compat::LocalFuture<X>>) -> compat::LocalFuture<Vec<X>> {
        let result = compat::LocalFuture::<Vec<X>>::new();
        let remaining = Rc::new(std::cell::Cell::new(futures.len()));
        if futures.is_empty() {
            result.complete(Vec::new());
            return result;
        }
        let futures = Rc::new(futures);
        for future in futures.iter() {
            let (result, remaining, futures) = (result.clone(), remaining.clone(), futures.clone());
            future.then_accept(move |_| {
                remaining.set(remaining.get() - 1);
                if remaining.get() == 0 {
                    result.complete(futures.iter().map(|future| future.join()).collect());
                }
            });
        }
        result
    }
}
