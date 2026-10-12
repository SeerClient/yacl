//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.GuiGraphicsExtractorMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.GuiGraphicsExtractor)

#[allow(unused_imports)]
use crate::prelude::*;

use crate::dev::isxander::yacl3::gui::render::gui_render_state_sink::GuiRenderStateSink;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::renderer::state::gui::gui_element_render_state::GuiElementRenderState;


/// @Override
fn yacl_submit(this: &mut mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor, render_state: &mut mcrust::net::minecraft::client::renderer::state::gui::gui_element_render_state::GuiElementRenderState) {
    // TODO(fabric2seer): `.addGuiElement(..)` on a GuiRenderState
    //   java: this.guiRenderState.addGuiElement(renderState);
}

/// @Override
fn yacl_peek_scissor_stack(this: &mut mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor) -> /* TODO: ScreenRectangle */ () {
    // TODO(fabric2seer): `.peek(..)` on a ScissorStack
    //   java: return this.scissorStack.peek();
}

/// GuiGraphicsExtractorMixin makes every GuiGraphicsExtractor implement GuiRenderStateSink (an interface added by a Mixin): an
/// extension trait of the mod, implemented for the port's type.
impl GuiRenderStateSink for mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn yacl_submit(&self, render_state: &dyn GuiElementRenderState) {
        // TODO(fabric2seer): `.addGuiElement(..)` on a GuiRenderState
        //   java: this.guiRenderState.addGuiElement(renderState);
    }

    fn yacl_peek_scissor_stack(&self) -> ScreenRectangle {
        // TODO(fabric2seer): `.peek(..)` on a ScissorStack
        //   java: return this.scissorStack.peek();
    }

}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
