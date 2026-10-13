//! Ported from dev.isxander.yacl3.gui.image.ImageRendererFactory.
//!
//! A factory is prepared on YACL's image thread (`requiresOffThreadPreparation`), so it and the supplier it prepares
//! are `Send`; the supplier's `completeImage` runs on the render thread and builds the (non-`Send`) renderer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;

/// java: public interface ImageRendererFactory
pub trait ImageRendererFactory: Send {
    /// java: ImageSupplier prepareImage() throws Exception; - prepares the image. This can be run off-thread, and
    /// should NOT contain any GL calls whatsoever.
    fn prepare_image(&self) -> Result<Box<dyn ImageSupplier>, String>;

    /// java: default boolean requiresOffThreadPreparation()
    fn requires_off_thread_preparation(&self) -> bool {
        true
    }
}

/// java: interface ImageSupplier - `ImageRenderer completeImage() throws Exception;`
pub trait ImageSupplier: Send {
    /// java: ImageRenderer completeImage() throws Exception;
    fn complete_image(self: Box<Self>) -> Result<Arc<dyn ImageRenderer>, String>;
}

impl<F: FnOnce() -> Result<Arc<dyn ImageRenderer>, String> + Send> ImageSupplier for F {
    fn complete_image(self: Box<Self>) -> Result<Arc<dyn ImageRenderer>, String> {
        (*self)()
    }
}

/// A factory written as a lambda (`() -> supplier`), prepared off-thread.
pub struct FnFactory<F: Fn() -> Result<Box<dyn ImageSupplier>, String> + Send>(pub F);

impl<F: Fn() -> Result<Box<dyn ImageSupplier>, String> + Send> ImageRendererFactory for FnFactory<F> {
    fn prepare_image(&self) -> Result<Box<dyn ImageSupplier>, String> {
        (self.0)()
    }
}

/// java: interface OnThread extends ImageRendererFactory - `requiresOffThreadPreparation()` is `false`.
pub struct OnThread<F: Fn() -> Result<Box<dyn ImageSupplier>, String> + Send>(pub F);

impl<F: Fn() -> Result<Box<dyn ImageSupplier>, String> + Send> ImageRendererFactory for OnThread<F> {
    fn prepare_image(&self) -> Result<Box<dyn ImageSupplier>, String> {
        (self.0)()
    }

    fn requires_off_thread_preparation(&self) -> bool {
        false
    }
}
