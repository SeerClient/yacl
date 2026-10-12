//! Ported from dev.isxander.yacl3.gui.render.GuiRenderStateSink by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::renderer::state::gui::gui_element_render_state::GuiElementRenderState;

/// java: public interface GuiRenderStateSink
pub trait GuiRenderStateSink {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void yacl$submit(GuiElementRenderState renderState);
    fn yacl_submit(&self, render_state: &dyn GuiElementRenderState);

    /// java: ScreenRectangle yacl$peekScissorStack();
    fn yacl_peek_scissor_stack(&self) -> ScreenRectangle;

}

/// java: static void submit(GuiGraphicsExtractor graphics, GuiElementRenderState renderState)
pub fn submit(graphics: &GuiGraphicsExtractor, render_state: &dyn GuiElementRenderState) {
    graphics.yacl_submit(render_state);
}

/// java: static ScreenRectangle peekScissorStack(GuiGraphicsExtractor graphics)
pub fn peek_scissor_stack(graphics: &GuiGraphicsExtractor) -> ScreenRectangle {
    return graphics.yacl_peek_scissor_stack();
}
