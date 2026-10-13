//! Ported from dev.isxander.yacl3.gui.LowProfileButtonWidget.

#[allow(unused_imports)]
use crate::prelude::*;
use compat::{Button, Graphics, GuiEventListener};
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class LowProfileButtonWidget extends Button.Plain
pub struct LowProfileButtonWidget {
    /// java: extends Button.Plain
    pub button: Button,
}

impl LowProfileButtonWidget {
    /// java: public LowProfileButtonWidget(int x, int y, int width, int height, Component message, OnPress onPress)
    pub fn new(x: i32, y: i32, width: i32, height: i32, message: MutableComponent, on_press: impl Fn(&Button) + 'static) -> Self {
        Self { button: Button::new(x, y, width, height, message, on_press) }
    }

    /// java: public LowProfileButtonWidget(int x, int y, int width, int height, Component message, OnPress onPress,
    /// Tooltip tooltip)
    #[allow(clippy::too_many_arguments)]
    pub fn new_7(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        message: MutableComponent,
        on_press: impl Fn(&Button) + 'static,
        tooltip: Option<MutableComponent>,
    ) -> Self {
        let this = Self::new(x, y, width, height, message, on_press);
        this.button.fields.set_tooltip(tooltip);
        this
    }
}

impl GuiEventListener for LowProfileButtonWidget {
    /// `AbstractWidget.extractRenderState` -> `AbstractButton.extractWidgetRenderState` with this class's
    /// `extractContents`.
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        let f = &self.button.fields;
        if !f.visible.get() {
            return;
        }
        f.update_hover(mouse_x, mouse_y);

        // java: @Override protected void extractContents(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
        if f.is_hovered_or_focused() && f.is_active() {
            self.button.extract_default_sprite(graphics);
        }
        self.button.extract_default_label(graphics);

        self.button.handle_cursor(graphics);
        f.refresh_tooltip_for_next_render_pass(graphics, mouse_x, mouse_y);
    }
    fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.button.mouse_clicked(event, double_click)
    }
    fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
        self.button.mouse_released(event)
    }
    fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        self.button.mouse_dragged(event, dx, dy)
    }
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        self.button.key_pressed(event)
    }
    fn char_typed(&self, event: &CharacterEvent) -> bool {
        self.button.char_typed(event)
    }
    fn is_mouse_over(&self, x: f64, y: f64) -> bool {
        self.button.is_mouse_over(x, y)
    }
    fn set_focused(&self, focused: bool) {
        self.button.set_focused(focused)
    }
    fn is_focused(&self) -> bool {
        self.button.is_focused()
    }
}
