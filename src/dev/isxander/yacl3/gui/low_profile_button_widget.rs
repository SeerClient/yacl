//! Ported from dev.isxander.yacl3.gui.LowProfileButtonWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class LowProfileButtonWidget extends Button.Plain
pub struct LowProfileButtonWidget {
    /// java: extends net.minecraft.client.gui.components.Button$Plain
    pub base: AbstractWidget,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for LowProfileButtonWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LowProfileButtonWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LowProfileButtonWidget {
    /// java: public LowProfileButtonWidget(int x, int y, int width, int height, Component message, OnPress onPress)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut on_press: OnPress) -> compat::Shared<Self> {
        // TODO(fabric2seer): `new Plain(..)`: the port has no `Plain::new`
        //   java: super(x, y, width, height, message, onPress, DEFAULT_NARRATION);
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: public LowProfileButtonWidget(int x, int y, int width, int height, Component message, OnPress onPress, Tooltip tooltip)
    pub fn new_7(mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut on_press: OnPress, mut tooltip: Tooltip) -> compat::Shared<Self> {
        let __this = Self::new(x, y, width, height, message.clone(), on_press);
        { let (__q0,) = (tooltip.clone(),); __this.lock().borrow_mut().set_tooltip(__q0) };
        __this
    }

    /// java: @Override protected void extractContents(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_contents(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if __this.lock().borrow().is_hovered_or_focused() && __this.lock().borrow().is_active() {
            // TODO(fabric2seer): `Plain.extractDefaultSprite` has no Rust method in the port (looked for `extract_default_sprite` in Button and its supertypes)
            // rust: __this.lock().borrow().extract_default_sprite(graphics);
        }
        // TODO(fabric2seer): field `NONE` of a value of unknown type
        //   java: this.extractDefaultLabel(graphics.textRendererForWidget(this, GuiGraphicsExtractor.HoveredTextEffects.NONE));
    }

}

// NOTE(fabric2seer): extractContents override(s) of Plain - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
