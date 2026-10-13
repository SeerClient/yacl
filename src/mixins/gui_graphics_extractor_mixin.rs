//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.GuiGraphicsExtractorMixin.
//! @Mixin(net.minecraft.client.gui.GuiGraphicsExtractor)
//!
//! The mixin makes `GuiGraphicsExtractor` implement `GuiRenderStateSink` over its `@Shadow`ed `guiRenderState` and
//! `scissorStack`. The port's extractor keeps both per frame in `gui_graphics_extractor` (`with_gui_render_state`,
//! `peek_scissor`), so the interface is implemented for the port's type over those.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::render::gui_render_state_sink::GuiRenderStateSink;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::{self, GuiGraphicsExtractor};
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::renderer::state::gui::gui_element_render_state::GuiElementRenderState;

impl GuiRenderStateSink for GuiGraphicsExtractor {
    /// java: @Override public void yacl$submit(GuiElementRenderState renderState)
    fn yacl_submit(&self, render_state: Box<dyn GuiElementRenderState>) {
        // this.guiRenderState.addGuiElement(renderState);
        gui_graphics_extractor::with_gui_render_state(|state| state.add_gui_element(render_state));
    }

    /// java: @Override public ScreenRectangle yacl$peekScissorStack()
    fn yacl_peek_scissor_stack(&self) -> Option<ScreenRectangle> {
        // return this.scissorStack.peek();
        gui_graphics_extractor::peek_scissor()
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this. (An interface mixin registers nothing:
/// the impl above is the mixin.)
pub fn register() {}
