//! Ported from dev.isxander.yacl3.gui.image.ImageRendererManager.
//!
//! The two caches hold render-thread objects (`ImageRenderer`s, `CompletableFuture`s over them), so they are
//! render-thread statics here; the image thread (`SINGLE_THREAD_EXECUTOR`) hands what it prepared back through a
//! queue drained by a `Minecraft.execute` task, exactly where Java's `Minecraft.getInstance().execute(..)` completes it.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::{ImageRendererFactory, ImageSupplier};
use crate::dev::isxander::yacl3::gui::image::r#impl::animated_dynamic_texture_image::AnimatedDynamicTextureImage;
use crate::dev::isxander::yacl3::platform::yacl_config::HANDLER;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER;
use mcrust::net::minecraft::resources::identifier::Identifier;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, channel};

type Task = Box<dyn FnOnce() + Send>;

/// java: private static final ExecutorService SINGLE_THREAD_EXECUTOR = Executors.newSingleThreadExecutor(task ->
/// new Thread(task, "YACL Image Prep")); - started on first use, `None` once shut down.
static SINGLE_THREAD_EXECUTOR: Mutex<Option<Option<Sender<Task>>>> = Mutex::new(None);

/// `SINGLE_THREAD_EXECUTOR.submit(task)`
fn submit(task: Task) {
    let mut executor = SINGLE_THREAD_EXECUTOR.lock().unwrap_or_else(|p| p.into_inner());
    if executor.is_none() {
        let (sender, receiver) = channel::<Task>();
        let spawned = std::thread::Builder::new().name("YACL Image Prep".to_string()).spawn(move || {
            while let Ok(task) = receiver.recv() {
                task();
            }
        });
        *executor = Some(spawned.ok().map(|_| sender));
    }
    // a shut-down executor rejects the task (`RejectedExecutionException`); the image is never completed
    if let Some(Some(sender)) = executor.as_ref() {
        let _ = sender.send(task);
    }
}

/// What the image thread hands back: a factory to prepare on the render thread (`OnThread`), or what it prepared
/// (`Optional<ImageSupplier>`, empty when preparation failed).
enum Prepared {
    OnThread(Box<dyn ImageRendererFactory>),
    Prepared(Option<Box<dyn ImageSupplier>>),
}

/// The image thread's results, waiting for the render thread.
static COMPLETIONS: Mutex<Vec<(Identifier, Prepared)>> = Mutex::new(Vec::new());

thread_local! {
    /// java: private static final Map<Identifier, CompletableFuture<ImageRenderer>> IMAGE_CACHE
    static IMAGE_CACHE: RefCell<HashMap<Identifier, compat::LocalFuture<Arc<dyn ImageRenderer>>>> = RefCell::new(HashMap::new());
    /// java: static final Map<Identifier, ImageRenderer> PRELOADED_IMAGE_CACHE
    pub(crate) static PRELOADED_IMAGE_CACHE: RefCell<HashMap<Identifier, Arc<dyn ImageRenderer>>> = RefCell::new(HashMap::new());
}

/// java: public record PreloadedImageFactory(Predicate<Identifier> predicate, Function<Identifier, ImageRendererFactory> factory)
#[derive(Clone, Copy)]
pub struct PreloadedImageFactory {
    /// java: Predicate<Identifier> predicate
    pub predicate: fn(&Identifier) -> bool,
    /// java: Function<Identifier, ImageRendererFactory> factory
    pub factory: fn(Identifier) -> Box<dyn ImageRendererFactory>,
}

/// java: static final List<PreloadedImageFactory> PRELOADED_IMAGE_FACTORIES - `.webp` and `.gif` when
/// `preloadComplexImageFormats` is on.
pub fn preloaded_image_factories() -> Vec<PreloadedImageFactory> {
    let preload = compat::read(&HANDLER.get().instance(), |config| config.preload_complex_image_formats);
    let mut factories = Vec::new();
    if preload {
        factories.push(PreloadedImageFactory {
            predicate: |location| location.path().ends_with(".webp"),
            factory: AnimatedDynamicTextureImage::create_webp_from_texture,
        });
        factories.push(PreloadedImageFactory {
            predicate: |location| location.path().ends_with(".gif"),
            factory: AnimatedDynamicTextureImage::create_gif_from_texture,
        });
    }
    factories
}

/// java: public class ImageRendererManager
pub struct ImageRendererManager;

impl ImageRendererManager {
    /// java: public static <T extends ImageRenderer> Optional<T> getImage(Identifier id)
    pub fn get_image(id: &Identifier) -> Option<Arc<dyn ImageRenderer>> {
        if let Some(image) = PRELOADED_IMAGE_CACHE.with(|cache| cache.borrow().get(id).cloned()) {
            return Some(image);
        }
        IMAGE_CACHE.with(|cache| cache.borrow().get(id).cloned()).and_then(|future| {
            if future.is_done() { Some(future.join()) } else { None }
        })
    }

    /// java: public static <T extends ImageRenderer> CompletableFuture<T> registerOrGetImage(Identifier id,
    /// Supplier<ImageRendererFactory> factorySupplier)
    pub fn register_or_get_image(
        id: &Identifier,
        factory_supplier: impl FnOnce() -> Box<dyn ImageRendererFactory>,
    ) -> compat::LocalFuture<Arc<dyn ImageRenderer>> {
        if let Some(image) = PRELOADED_IMAGE_CACHE.with(|cache| cache.borrow().get(id).cloned()) {
            return compat::LocalFuture::completed_future(image);
        }

        if let Some(future) = IMAGE_CACHE.with(|cache| cache.borrow().get(id).cloned()) {
            return future;
        }

        let future = compat::LocalFuture::new();
        IMAGE_CACHE.with(|cache| cache.borrow_mut().insert(id.clone(), future.clone()));

        let factory: Box<dyn ImageRendererFactory> = factory_supplier();
        let id = id.clone();
        submit(Box::new(move || {
            let prepared = if factory.requires_off_thread_preparation() {
                Prepared::Prepared(Self::safely_prepare_factory(&id, &*factory))
            } else {
                Prepared::OnThread(factory)
            };
            COMPLETIONS.lock().unwrap_or_else(|p| p.into_inner()).push((id, prepared));
            // Minecraft.getInstance().execute(() -> completeImageFactory(id, supplier, future));
            mcrust::mcrust::modding::game::execute(Self::drain_completions);
        }));

        future
    }

    /// java: @Deprecated public static <T extends ImageRenderer> CompletableFuture<T> registerImage(Identifier id,
    /// ImageRendererFactory factory)
    pub fn register_image(id: &Identifier, factory: Box<dyn ImageRendererFactory>) -> compat::LocalFuture<Arc<dyn ImageRenderer>> {
        Self::register_or_get_image(id, move || factory)
    }

    /// The `Minecraft.execute` task: every completion the image thread queued.
    fn drain_completions() {
        let completions = std::mem::take(&mut *COMPLETIONS.lock().unwrap_or_else(|p| p.into_inner()));
        for (id, prepared) in completions {
            let supplier = match prepared {
                Prepared::Prepared(supplier) => supplier,
                Prepared::OnThread(factory) => Self::safely_prepare_factory(&id, &*factory),
            };
            match supplier {
                // safelyPrepareFactory's failure: `IMAGE_CACHE.remove(id)`
                None => {
                    IMAGE_CACHE.with(|cache| cache.borrow_mut().remove(&id));
                }
                Some(supplier) => {
                    // closed (closeAll) before it completed: nothing waits for it
                    let Some(future) = IMAGE_CACHE.with(|cache| cache.borrow().get(&id).cloned()) else {
                        continue;
                    };
                    Self::complete_image_factory(&id, supplier, &future);
                }
            }
        }
    }

    /// java: private static <T extends ImageRenderer> void completeImageFactory(Identifier id,
    /// Supplier<Optional<ImageRendererFactory.ImageSupplier>> supplier, CompletableFuture<ImageRenderer> future)
    fn complete_image_factory(id: &Identifier, completable_image: Box<dyn ImageSupplier>, future: &compat::LocalFuture<Arc<dyn ImageRenderer>>) {
        // RenderSystem.assertOnRenderThread(); - a `Minecraft.execute` task
        // sanity check - this should never happen
        if future.is_done() {
            LOGGER.error(&format!("Image '{id}' was already completed"));
            return;
        }

        match completable_image.complete_image() {
            Ok(image) => {
                future.complete(image);
            }
            Err(e) => LOGGER.error(&format!("Failed to create image '{id}': {e}")),
        }
    }

    /// java: public static void closeAll()
    pub fn close_all() {
        *SINGLE_THREAD_EXECUTOR.lock().unwrap_or_else(|p| p.into_inner()) = Some(None);
        let futures: Vec<_> = IMAGE_CACHE.with(|cache| cache.borrow_mut().drain().map(|(_, future)| future).collect());
        for future in futures {
            if future.is_done() {
                future.join().close();
            }
        }
    }

    /// java: static Optional<ImageRendererFactory.ImageSupplier> safelyPrepareFactory(Identifier id,
    /// ImageRendererFactory factory)
    ///
    /// The failure's `IMAGE_CACHE.remove(id)` is made by the render thread when it receives the empty result (the
    /// cache is a render-thread map).
    pub(crate) fn safely_prepare_factory(id: &Identifier, factory: &dyn ImageRendererFactory) -> Option<Box<dyn ImageSupplier>> {
        match factory.prepare_image() {
            Ok(supplier) => Some(supplier),
            Err(e) => {
                LOGGER.error(&format!("Failed to prepare image '{id}': {e}"));
                None
            }
        }
    }
}
