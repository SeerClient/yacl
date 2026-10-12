//! Ported from dev.isxander.yacl3.gui.image.ImageRendererFactory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;

/// java: public interface ImageRendererFactory
pub trait ImageRendererFactory: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: ImageSupplier prepareImage() throws Exception; (dispatch: the body for this class)
    fn prepare_image(&self) -> fn(&compat::Shared<dyn ImageRendererFactory>) -> ImageSupplier;

    /// java: default boolean requiresOffThreadPreparation() (dispatch: the body for this class)
    fn requires_off_thread_preparation(&self) -> fn(&compat::Shared<dyn ImageRendererFactory>) -> bool {
        <dyn ImageRendererFactory>::requires_off_thread_preparation_body
    }

}

impl dyn ImageRendererFactory {
    /// java: default boolean requiresOffThreadPreparation()
    pub fn requires_off_thread_preparation_body(__this: &compat::Shared<dyn ImageRendererFactory>) -> bool {
        return true;
    }

}

/// java: interface ImageSupplier
/// a functional interface: `completeImage()` - a lambda / method reference is this closure type
pub type ImageSupplier = Arc<dyn Fn() -> compat::Shared<dyn ImageRenderer>>;

/// java: interface OnThread extends ImageRendererFactory
pub trait OnThread: ImageRendererFactory {
}

impl dyn OnThread {
    /// java: @Override default boolean requiresOffThreadPreparation()
    pub fn requires_off_thread_preparation_body(__this: &compat::Shared<dyn OnThread>) -> bool {
        return false;
    }

    /// OnThread.requiresOffThreadPreparation as the dispatch of ImageRendererFactory.requiresOffThreadPreparation for the concrete class `__C`
    pub fn requires_off_thread_preparation_from<__C: OnThread + 'static>(__this: &compat::Shared<dyn ImageRendererFactory>) -> bool {
        <dyn OnThread>::requires_off_thread_preparation_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn OnThread>))
    }

}
