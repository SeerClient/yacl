//! Ported from dev.isxander.yacl3.gui.image.ImageRendererManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::ImageRendererFactory;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::ImageSupplier;
use crate::dev::isxander::yacl3::platform::yacl_config;
use mcrust::com::mojang::blaze3d::systems::render_system;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::resources::identifier::Identifier;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler as _;

// TODO(fabric2seer): Executors.newSingleThreadExecutor: use std::thread / the port's executors
//   java: private static final ExecutorService SINGLE_THREAD_EXECUTOR = Executors.newSingleThreadExecutor(task -> new Thread(task, "YACL Image Prep"));
/// java: private static final ExecutorService SINGLE_THREAD_EXECUTOR = Executors.newSingleThreadExecutor(task -> new Thread(task, "YACL Image Prep"));
pub static SINGLE_THREAD_EXECUTOR: compat::MainThread<crate::unresolved::ExecutorService> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `SINGLE_THREAD_EXECUTOR` - see the TODO above"));
/// java: private static final Map<Identifier, CompletableFuture<ImageRenderer>> IMAGE_CACHE = new ConcurrentHashMap<>();
pub static IMAGE_CACHE: compat::MainThread<std::collections::HashMap<Identifier, mcrust::java::util::concurrent::completable_future::CompletableFuture<compat::Shared<dyn ImageRenderer>>>> = compat::MainThread::new(|| std::collections::HashMap::new());
/// java: static final Map<Identifier, ImageRenderer> PRELOADED_IMAGE_CACHE = new ConcurrentHashMap<>();
pub static PRELOADED_IMAGE_CACHE: compat::MainThread<std::collections::HashMap<Identifier, compat::Shared<dyn ImageRenderer>>> = compat::MainThread::new(|| std::collections::HashMap::new());
// TODO(fabric2seer): field `preloadComplexImageFormats` of a T
//   java: static final List<PreloadedImageFactory> PRELOADED_IMAGE_FACTORIES = Stream.of(
//   java:             YACLConfig.HANDLER.instance().preloadComplexImageFormats ? new PreloadedImageFactory(
//   java:                     location -> location.getPath().endsWith(".webp"),
//   java:                     AnimatedDynamicTextureImage::createWEBPFromTexture
//   java:             ) : null,
//   java:             YACLConfig.HANDLER.instance().preloadComplexImageFormats ? new PreloadedImageFactory(
//   java:                     location -> location.getPath().endsWith(".gif"),
//   java:                     AnimatedDynamicTextureImage::createGIFFromTexture
//   java:             ) : null
//   java:     ).filter(Objects::nonNull).toList();
/// java: static final List<PreloadedImageFactory> PRELOADED_IMAGE_FACTORIES = Stream.of( YACLConfig.HANDLER.instance().preloadComplexImageFormats ? new PreloadedImageFactory( location -> location.getPath().end
pub static PRELOADED_IMAGE_FACTORIES: compat::MainThread<Vec<PreloadedImageFactory>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `PRELOADED_IMAGE_FACTORIES` - see the TODO above"));

/// java: public class ImageRendererManager
#[derive(Clone)]
pub struct ImageRendererManager {}

impl ImageRendererManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static <T extends ImageRenderer> Optional<T> getImage(Identifier id)
    pub fn get_image(id: &Identifier) -> Option<compat::Shared<dyn ImageRenderer>> {
        if PRELOADED_IMAGE_CACHE.get_mut().contains_key(id) {
            return PRELOADED_IMAGE_CACHE.get_mut().get(id).cloned();
        }
        if IMAGE_CACHE.get_mut().contains_key(id) {
            return Some(IMAGE_CACHE.get_mut().get(id).cloned().unwrap().get_now(None).unwrap());
        }
        return None;
    }

    /// java: @SuppressWarnings("unchecked") public static <T extends ImageRenderer> CompletableFuture<T> registerOrGetImage(Identifier id, Supplier<ImageRendererFactory> factorySupplier)
    pub fn register_or_get_image(mut id: Identifier, factory_supplier: Arc<dyn Fn() -> compat::Shared<dyn ImageRendererFactory>>) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<compat::Shared<dyn ImageRenderer>> {
        if PRELOADED_IMAGE_CACHE.get_mut().contains_key(&id) {
            return CompletableFuture::completed_future(PRELOADED_IMAGE_CACHE.get_mut().get(&id).cloned());
        }
        if IMAGE_CACHE.get_mut().contains_key(&id) {
            return IMAGE_CACHE.get_mut().get(&id).cloned().unwrap();
        }
        let mut future = CompletableFuture::new();
        IMAGE_CACHE.get_mut().insert(id.clone(), future.clone());
        let mut factory: compat::Shared<dyn ImageRendererFactory> = (factory_supplier)();
        // TODO(fabric2seer): `.submit(..)` on a ExecutorService
        //   java: SINGLE_THREAD_EXECUTOR.submit(() -> {
        //   java:             Supplier<Optional<ImageRendererFactory.ImageSupplier>> supplier =
        //   java:                     factory.requiresOffThreadPreparation()
        //   java:                             ? new CompletedSupplier<>(safelyPrepareFactory(id, factory))
        //   java:                             : () -> safelyPrepareFactory(id, factory);
        //   java:
        //   java:             Minecraft.getInstance().execute(() -> completeImageFactory(id, supplier, future));
        //   java:         });
        return future.clone();
    }

    /// java: @Deprecated public static <T extends ImageRenderer> CompletableFuture<T> registerImage(Identifier id, ImageRendererFactory factory)
    pub fn register_image(id: &Identifier, factory: &compat::Shared<dyn ImageRendererFactory>) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<compat::Shared<dyn ImageRenderer>> {
        return ImageRendererManager::register_or_get_image(id.clone(), Arc::new(move || factory.clone()));
    }

    /// java: private static <T extends ImageRenderer> void completeImageFactory(Identifier id, Supplier<Optional<ImageRendererFactory.ImageSupplier>> supplier, CompletableFuture<ImageRenderer> future)
    pub fn complete_image_factory(id: &Identifier, supplier: Arc<dyn Fn() -> Option<ImageSupplier>>, future: &mcrust::java::util::concurrent::completable_future::CompletableFuture<compat::Shared<dyn ImageRenderer>>) {
        render_system::assert_on_render_thread();
        let Some(mut completable_image) = (supplier)() else {
            return;
        };
        if future.is_done() {
            logger().error(&format!("Image '{}' was already completed", compat::jstring(&id)));
            return;
        }
        let mut image: compat::Shared<dyn ImageRenderer>;
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            image = (completable_image)();
        }));
        if let Err(e) = __result {
            logger().error(&format!("Failed to create image '{}'", compat::jstring(&id)));
            return;
        }
        future.complete(image.clone());
    }

    /// java: public static void closeAll()
    pub fn close_all() {
        // TODO(fabric2seer): `.shutdownNow(..)` on a ExecutorService
        //   java: SINGLE_THREAD_EXECUTOR.shutdownNow();
        IMAGE_CACHE.get_mut().retain(|_, __v| !(|future| {
            if future.is_done() {
                // TODO(fabric2seer): `.close(..)` on a T
                //   java: future.join().close();
            }
            return true;
        })(__v));
    }

    /// java: static Optional<ImageRendererFactory.ImageSupplier> safelyPrepareFactory(Identifier id, ImageRendererFactory factory)
    pub fn safely_prepare_factory(id: &Identifier, factory: &compat::Shared<dyn ImageRendererFactory>) -> Option<ImageSupplier> {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return Some(({ let __f = compat::dispatch(factory, |__t| __t.prepare_image()); __f(factory) }));
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("Failed to prepare image '{}'", compat::jstring(&id)));
                IMAGE_CACHE.get_mut().remove(id);
                return None;
            }
        }
    }

}

/// java: public record PreloadedImageFactory(Predicate<Identifier> predicate, Function<Identifier, ImageRendererFactory> factory)
#[derive(Clone, Debug)]
pub struct PreloadedImageFactory {
    /// java: Predicate<Identifier> predicate
    pub predicate: Arc<dyn Fn(Identifier) -> bool>,
    /// java: Function<Identifier, ImageRendererFactory> factory
    pub factory: Arc<dyn Fn(Identifier) -> compat::Shared<dyn ImageRendererFactory>>,
}

impl PreloadedImageFactory {
    /// java: the canonical record constructor
    pub fn new(mut predicate: Arc<dyn Fn(Identifier) -> bool>, mut factory: Arc<dyn Fn(Identifier) -> compat::Shared<dyn ImageRendererFactory>>) -> Self {
        Self { predicate, factory }
    }

    /// java: the record accessor `predicate()`
    pub fn predicate(&self) -> Arc<dyn Fn(Identifier) -> bool> {
        self.predicate.clone()
    }

    /// java: the record accessor `factory()`
    pub fn factory(&self) -> Arc<dyn Fn(Identifier) -> compat::Shared<dyn ImageRendererFactory>> {
        self.factory.clone()
    }

}

/// java: private record CompletedSupplier<T>(T get) implements Supplier<T>
#[derive(Clone, Debug)]
pub struct CompletedSupplier<T> {
    /// java: T get
    pub get: T,
}

impl<T: Clone + 'static> CompletedSupplier<T> {
    /// java: the canonical record constructor
    pub fn new(mut get: T) -> Self {
        Self { get }
    }

    /// java: the record accessor `get()`
    pub fn get(&self) -> T {
        self.get.clone()
    }

}
