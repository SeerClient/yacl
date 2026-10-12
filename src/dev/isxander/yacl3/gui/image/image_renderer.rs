//! Ported from dev.isxander.yacl3.gui.image.ImageRenderer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;

/// java: public interface ImageRenderer
pub trait ImageRenderer: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: int render(GuiGraphicsExtractor graphics, int x, int y, int renderWidth, float tickDelta); (dispatch: the body for this class)
    fn render(&self) -> fn(&compat::Shared<dyn ImageRenderer>, &GuiGraphicsExtractor, i32, i32, i32, f32) -> i32;

    /// java: void close(); (dispatch: the body for this class)
    fn close(&self) -> fn(&compat::Shared<dyn ImageRenderer>);

    /// java: default void tick() (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn ImageRenderer>) {
        <dyn ImageRenderer>::tick_body
    }

}

impl dyn ImageRenderer {
    /// java: default void tick()
    pub fn tick_body(__this: &compat::Shared<dyn ImageRenderer>) {
    }

}
