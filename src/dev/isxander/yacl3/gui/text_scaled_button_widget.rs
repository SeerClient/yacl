//! Ported from dev.isxander.yacl3.gui.TextScaledButtonWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::tooltip_button_widget::TooltipButtonWidget;
use crate::dev::isxander::yacl3::gui::tooltip_button_widget::TooltipButtonWidgetBase;
use mcrust::net::minecraft::client::gui::active_text_collector::ActiveTextCollector;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class TextScaledButtonWidget extends TooltipButtonWidget
pub struct TextScaledButtonWidget {
    /// java: extends TooltipButtonWidget
    pub base: TooltipButtonWidgetBase,
    /// java: public float textScale;
    pub text_scale: f32,
}

impl std::ops::Deref for TextScaledButtonWidget {
    type Target = TooltipButtonWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TextScaledButtonWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TextScaledButtonWidget {
    /// java: public TextScaledButtonWidget(Screen screen, int x, int y, int width, int height, float textScale, Component message, Component tooltip, OnPress onPress)
    pub fn new(mut screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut text_scale: f32, mut message: MutableComponent, mut tooltip: MutableComponent, mut on_press: OnPress) -> compat::Shared<Self> {
        let __init_text_scale = text_scale;
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: super(screen, x, y, width, height, message, tooltip, onPress);
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: public TextScaledButtonWidget(Screen screen, int x, int y, int width, int height, float textScale, Component message, OnPress onPress)
    pub fn new_8(mut screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut text_scale: f32, mut message: MutableComponent, mut on_press: OnPress) -> compat::Shared<Self> {
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this(screen, x, y, width, height, textScale, message, null, onPress);
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: @Override protected void extractDefaultLabel(ActiveTextCollector output)
    pub fn extract_default_label(__this: &compat::Shared<Self>, output: &dyn ActiveTextCollector) {
        // TODO(fabric2seer): `Plain.extractDefaultLabel` has no Rust method in the port (looked for `extract_default_label` in Button and its supertypes)
        // rust: __this.lock().borrow_mut().base.extract_default_label(output);
    }

}

impl TooltipButtonWidget for TextScaledButtonWidget {
    fn tooltip_button_widget_base(&self) -> &TooltipButtonWidgetBase {
        &self.base
    }
    fn tooltip_button_widget_base_mut(&mut self) -> &mut TooltipButtonWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
