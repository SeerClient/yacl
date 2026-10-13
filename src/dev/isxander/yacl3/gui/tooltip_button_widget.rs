//! Ported from dev.isxander.yacl3.gui.TooltipButtonWidget.
//!
//! `extends Button.Plain`: the YACL screen draws and dispatches its own widgets (PATTERNS_Y2.md P3), so the button
//! half is a `compat::Button` (the game's look, the `OnPress` lambda, the tooltip).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::{Button, Graphics, GuiEventListener};
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::sync::Weak;

/// java: public class TooltipButtonWidget extends Button.Plain
pub struct TooltipButtonWidget {
    /// java: extends Button.Plain
    pub button: Button,
    /// java: protected final Screen screen; (held weakly: the screen holds its widgets)
    pub screen: Weak<YACLScreen>,
}

impl TooltipButtonWidget {
    /// java: public TooltipButtonWidget(Screen screen, int x, int y, int width, int height, Component message,
    /// Component tooltip, OnPress onPress)
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        screen: &Arc<YACLScreen>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        message: MutableComponent,
        tooltip: Option<MutableComponent>,
        on_press: impl Fn(&Button) + 'static,
    ) -> Self {
        let button = Button::new(x, y, width, height, message, on_press);
        if let Some(tooltip) = tooltip {
            // setTooltip(Tooltip.create(tooltip));
            button.fields.set_tooltip(Some(tooltip));
        }
        Self { button, screen: Arc::downgrade(screen) }
    }
}

impl GuiEventListener for TooltipButtonWidget {
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.button.extract_render_state(graphics, mouse_x, mouse_y, a)
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
