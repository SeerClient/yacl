//! Ported from dev.isxander.yacl3.gui.render.GuiRenderStateSink.
//!
//! The interface `GuiGraphicsExtractorMixin` adds to `GuiGraphicsExtractor`; implemented for the port's type in
//! `mixins::gui_graphics_extractor_mixin` over the extractor's per-frame `GuiRenderState` and scissor stack.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::renderer::state::gui::gui_element_render_state::GuiElementRenderState;

/// java: public interface GuiRenderStateSink
pub trait GuiRenderStateSink {
    /// java: void yacl$submit(GuiElementRenderState renderState);
    fn yacl_submit(&self, render_state: Box<dyn GuiElementRenderState>);

    /// java: ScreenRectangle yacl$peekScissorStack(); (nullable)
    fn yacl_peek_scissor_stack(&self) -> Option<ScreenRectangle>;
}

/// java: static void submit(GuiGraphicsExtractor graphics, GuiElementRenderState renderState)
pub fn submit(graphics: &GuiGraphicsExtractor, render_state: Box<dyn GuiElementRenderState>) {
    graphics.yacl_submit(render_state);
}

/// java: static ScreenRectangle peekScissorStack(GuiGraphicsExtractor graphics)
pub fn peek_scissor_stack(graphics: &GuiGraphicsExtractor) -> Option<ScreenRectangle> {
    graphics.yacl_peek_scissor_stack()
}
