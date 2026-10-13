//! Ported from dev.isxander.yacl3.gui.TextScaledButtonWidget.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::tooltip_button_widget::TooltipButtonWidget;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::{Button, Graphics, GuiEventListener};
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::cell::Cell;

/// java: public class TextScaledButtonWidget extends TooltipButtonWidget
pub struct TextScaledButtonWidget {
    /// java: extends TooltipButtonWidget
    pub base: TooltipButtonWidget,
    /// java: public float textScale;
    pub text_scale: Cell<f32>,
}

impl TextScaledButtonWidget {
    /// java: public TextScaledButtonWidget(Screen screen, int x, int y, int width, int height, float textScale,
    /// Component message, Component tooltip, OnPress onPress)
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        screen: &Arc<YACLScreen>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        text_scale: f32,
        message: MutableComponent,
        tooltip: Option<MutableComponent>,
        on_press: impl Fn(&Button) + 'static,
    ) -> Self {
        Self { base: TooltipButtonWidget::new(screen, x, y, width, height, message, tooltip, on_press), text_scale: Cell::new(text_scale) }
    }

    /// java: public TextScaledButtonWidget(Screen screen, int x, int y, int width, int height, float textScale,
    /// Component message, OnPress onPress)
    #[allow(clippy::too_many_arguments)]
    pub fn new_8(
        screen: &Arc<YACLScreen>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        text_scale: f32,
        message: MutableComponent,
        on_press: impl Fn(&Button) + 'static,
    ) -> Self {
        Self::new(screen, x, y, width, height, text_scale, message, None, on_press)
    }

    /// The `Button` half.
    pub fn button(&self) -> &Button {
        &self.base.button
    }
}

impl GuiEventListener for TextScaledButtonWidget {
    /// `extractDefaultLabel` is overridden only to call `super` (YACL's FIXME: the scale is not applied), so the
    /// button draws as a `TooltipButtonWidget`.
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.base.extract_render_state(graphics, mouse_x, mouse_y, a)
    }
    fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.base.mouse_clicked(event, double_click)
    }
    fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
        self.base.mouse_released(event)
    }
    fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        self.base.mouse_dragged(event, dx, dy)
    }
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        self.base.key_pressed(event)
    }
    fn char_typed(&self, event: &CharacterEvent) -> bool {
        self.base.char_typed(event)
    }
    fn is_mouse_over(&self, x: f64, y: f64) -> bool {
        self.base.is_mouse_over(x, y)
    }
    fn set_focused(&self, focused: bool) {
        self.base.set_focused(focused)
    }
    fn is_focused(&self) -> bool {
        self.base.is_focused()
    }
}
