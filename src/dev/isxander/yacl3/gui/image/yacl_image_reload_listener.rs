//! Ported from dev.isxander.yacl3.gui.image.YACLImageReloadListener by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::ImageRendererFactory;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::ImageSupplier;
use crate::dev::isxander::yacl3::gui::image::image_renderer_manager;
use crate::dev::isxander::yacl3::gui::image::image_renderer_manager::ImageRendererManager;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::server::packs::resources::resource::Resource;
use mcrust::net::minecraft::server::packs::resources::resource_manager::ResourceManager;

/// java: public class YACLImageReloadListener implements PreparableReloadListener
#[derive(Clone)]
pub struct YACLImageReloadListener {}

impl YACLImageReloadListener {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static Identifier getId()
    pub fn get_id() -> Identifier {
        return YACLPlatform::rl("image_reload_listener");
    }

    /// java: @Override public @NotNull CompletableFuture<Void> reload( SharedState sharedState, Executor backgroundExecutor, PreparationBarrier preparationBarrier, Executor gameExecutor )
    pub fn reload(&self, shared_state: &crate::unresolved::SharedState, background_executor: &crate::unresolved::Executor, preparation_barrier: &crate::unresolved::PreparationBarrier, game_executor: &crate::unresolved::Executor) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        // TODO(fabric2seer): `.resourceManager(..)` on a SharedState
        //   java: return this.reload0(sharedState.resourceManager(), backgroundExecutor, preparationBarrier, gameExecutor);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private @NotNull CompletableFuture<Void> reload0( ResourceManager resourceManager, Executor backgroundExecutor, PreparationBarrier preparationBarrier, Executor gameExecutor )
    pub fn reload0(&self, resource_manager: &ResourceManager, background_executor: &crate::unresolved::Executor, preparation_barrier: &crate::unresolved::PreparationBarrier, game_executor: &crate::unresolved::Executor) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        // TODO(fabric2seer): `.wait(..)` on a PreparationBarrier
        //   java: return prepare(resourceManager, backgroundExecutor)
        //   java:                 .thenCompose(preparationBarrier::wait)
        //   java:                 .thenCompose(suppliers -> apply(suppliers, gameExecutor));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private CompletableFuture<List<Optional<SupplierPreparation>>> prepare( ResourceManager manager, Executor executor )
    pub fn prepare(&self, manager: &ResourceManager, executor: &crate::unresolved::Executor) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Vec<Option<SupplierPreparation>>> {
        let mut image_resources: std::collections::HashMap<Identifier, Resource> = manager.list_resources("textures", &|location| image_renderer_manager::PRELOADED_IMAGE_FACTORIES.get_mut().iter().cloned().any(|factory| (factory.predicate())(location.clone())));
        // TODO(fabric2seer): collect(..) with a collector that is not `Collectors.*`
        //   java: return imageResources.keySet().stream()
        //   java:                 .map(location -> {
        //   java:                     ImageRendererFactory imageFactory = ImageRendererManager.PRELOADED_IMAGE_FACTORIES
        //   java:                             .stream()
        //   java:                             .filter(factory -> factory.predicate().test(location))
        //   java:                             .map(factory -> factory.factory().apply(location))
        //   java:                             .findAny()
        //   java:                             .orElseThrow();
        //   java:
        //   java:                     return CompletableFuture.supplyAsync(
        //   java:                             () -> ImageRendererManager.safelyPrepareFactory(
        //   java:                                     location, imageFactory
        //   java:                             ).map(supplier -> new SupplierPreparation(location, supplier)),
        //   java:                             executor
        //   java:                     );
        //   java:                 })
        //   java:                 .collect(CompletableFutureCollector.allOf());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private CompletableFuture<Void> apply( List<Optional<SupplierPreparation>> suppliers, Executor executor )
    pub fn apply(&self, suppliers: &Vec<Option<SupplierPreparation>>, executor: &crate::unresolved::Executor) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<()> {
        // TODO(fabric2seer): not translated (unexpected '?' (offset 4205))
        //   java: return CompletableFuture.allOf(suppliers.stream()
        //   java:                 .flatMap(Optional::stream)
        //   java:                 .map(prep -> CompletableFuture.supplyAsync(
        //   java:                         () -> {
        //   java:                             ImageRenderer imageRenderer;
        //   java:                             try {
        //   java:                                 imageRenderer = prep.supplier().completeImage();
        //   java:                             } catch (Exception e) {
        //   java:                                 YACLConstants.LOGGER.error("Failed to create image '{}'", prep.location(), e);
        //   java:                                 return Optional.empty();
        //   java:                             }
        //   java:                             ImageRendererManager.PRELOADED_IMAGE_CACHE.put(prep.location(), imageRenderer);
        //   java:                             YACLConstants.LOGGER.info("Successfully loaded image '{}'", prep.location());
        //   java:                             return Optional.of(imageRenderer);
        //   java:                         },
        //   java:                         executor
        //   java:                 ))
        //   java:                 .toArray(CompletableFuture<?>[]::new));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: private record SupplierPreparation(Identifier location, ImageRendererFactory.ImageSupplier supplier)
#[derive(Clone, Debug)]
pub struct SupplierPreparation {
    /// java: Identifier location
    pub location: Identifier,
    /// java: ImageRendererFactory.ImageSupplier supplier
    pub supplier: ImageSupplier,
}

impl SupplierPreparation {
    /// java: the canonical record constructor
    pub fn new(mut location: Identifier, mut supplier: ImageSupplier) -> Self {
        Self { location, supplier }
    }

    /// java: the record accessor `location()`
    pub fn location(&self) -> Identifier {
        self.location.clone()
    }

    /// java: the record accessor `supplier()`
    pub fn supplier(&self) -> ImageSupplier {
        self.supplier.clone()
    }

}

/// java: public static class CompletableFutureCollector<X, T extends CompletableFuture<X>> implements Collector<T, List<T>, CompletableFuture<List<X>>>
#[derive(Clone)]
pub struct CompletableFutureCollector<X, T> {}

impl<X: Clone + 'static, T: Clone + 'static> CompletableFutureCollector<X, T> {
    /// java: private CompletableFutureCollector()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static <X, T extends CompletableFuture<X>> Collector<T, List<T>, CompletableFuture<List<X>>> allOf()
    pub fn all_of<X: 'static, T: 'static>() -> crate::unresolved::Collector {
        return CompletableFutureCollector::new();
    }

    /// java: @Override public Supplier<List<T>> supplier()
    pub fn supplier(&self) -> Arc<dyn Fn() -> Vec<T>> {
        return Arc::new(move || Vec::new());
    }

    /// java: @Override public BiConsumer<List<T>, T> accumulator()
    pub fn accumulator(&self) -> Arc<dyn Fn(Vec<T>, T)> {
        // TODO(fabric2seer): static `List.add` has no Rust counterpart in the port
        // rust: return Arc::new(move |__a0, __a1| { List::add(__a0.clone(), __a1.clone()); });
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public BinaryOperator<List<T>> combiner()
    pub fn combiner(&self) -> Arc<dyn Fn(Vec<T>, Vec<T>) -> Vec<T>> {
        return Arc::new(move |left, right| {
            left.extend(right.iter().cloned());
            return left;
        });
    }

    /// java: @Override public Function<List<T>, CompletableFuture<List<X>>> finisher()
    pub fn finisher(&self) -> Arc<dyn Fn(Vec<T>) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Vec<X>>> {
        // TODO(fabric2seer): `.join(..)` on a T
        //   java: return ls -> CompletableFuture.allOf(ls.toArray(CompletableFuture[]::new))
        //   java:                     .thenApply(v -> ls
        //   java:                             .stream()
        //   java:                             .map(CompletableFuture::join)
        //   java:                             .toList());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Set<Characteristics> characteristics()
    pub fn characteristics(&self) -> std::collections::HashSet<crate::unresolved::Characteristics> {
        return Default::default();
    }

}
