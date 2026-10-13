//! Ported from dev.isxander.yacl3.gui.controllers.TickBoxController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::{ControllerWidget, ControllerWidgetBase};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::{CursorKind, Graphics};
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public record TickBoxController(Option<Boolean> option) implements Controller<Boolean>
///
/// This controller renders a tickbox.
pub struct TickBoxController {
    /// java: Option<Boolean> option
    pub option: Arc<dyn YOption<bool>>,
}

impl TickBoxController {
    /// java: public TickBoxController (the canonical record constructor) - constructs a tickbox controller
    pub fn new(option: Arc<dyn YOption<bool>>) -> Self {
        Self { option }
    }
}

impl Controller<bool> for TickBoxController {
    /// java: @Override public Option<Boolean> option()
    fn option(&self) -> Arc<dyn YOption<bool>> {
        self.option.clone()
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        component::empty()
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn abstract_widget::AbstractWidget> {
        Arc::new(TickBoxControllerElement::new(self, screen, widget_dimension))
    }
}

/// java: public static class TickBoxControllerElement extends ControllerWidget<TickBoxController>
pub struct TickBoxControllerElement {
    /// java: extends ControllerWidget<TickBoxController>
    pub cw: ControllerWidgetBase,
    /// java: protected final T control;
    pub control: Arc<TickBoxController>,
}

impl TickBoxControllerElement {
    /// java: public TickBoxControllerElement(TickBoxController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(control: Arc<TickBoxController>, screen: &Arc<YACLScreen>, dim: Arc<dyn Dimension<i32>>) -> Self {
        Self { cw: ControllerWidgetBase::new::<bool, TickBoxController>(&control, screen, dim), control }
    }

    /// java: public void toggleSetting()
    pub fn toggle_setting(&self) {
        let option = self.control.option();
        option.request_set(!option.pending_value());
        ControllerWidget::play_down_sound(self);
    }
}

impl ControllerWidget for TickBoxControllerElement {
    fn controller_widget(&self) -> &ControllerWidgetBase {
        &self.cw
    }

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&self, graphics: &Graphics<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        let outline_size = 10;
        let dim = ControllerWidget::get_dimension(self);
        let outline_x1 = dim.x_limit() - self.get_x_padding() - outline_size;
        let outline_y1 = dim.center_y() - outline_size / 2;
        let outline_x2 = dim.x_limit() - self.get_x_padding();
        let outline_y2 = dim.center_y() + outline_size / 2;

        let color = self.get_value_color();
        let shadow_color = abstract_widget::multiply_color(color, 0.25);

        graphics.outline(outline_x1 + 1, outline_y1 + 1, outline_x2 + 1 - (outline_x1 + 1), outline_y2 + 1 - (outline_y1 + 1), shadow_color);
        graphics.outline(outline_x1, outline_y1, outline_x2 - outline_x1, outline_y2 - outline_y1, color);
        if self.control.option().pending_value() {
            graphics.fill(outline_x1 + 3, outline_y1 + 3, outline_x2 - 1, outline_y2 - 1, shadow_color);
            graphics.fill(outline_x1 + 2, outline_y1 + 2, outline_x2 - 2, outline_y2 - 2, color);
        }

        if self.cw.hovered.get() {
            graphics.request_cursor(if self.is_available() { CursorKind::PointingHand } else { CursorKind::NotAllowed });
        }
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked(&self, event: &MouseButtonEvent, _double_click: bool) -> bool {
        if !ControllerWidget::is_mouse_over(self, event.x(), event.y()) || !self.is_available() {
            return false;
        }

        self.toggle_setting();
        true
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        10
    }

    /// java: @Override protected int getUnhoveredControlWidth()
    fn get_unhovered_control_width(&self) -> i32 {
        10
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        if !self.cw.focused.get() {
            return false;
        }

        if event.key() == input_constants::KEY_RETURN
            || event.key() == input_constants::KEY_SPACE
            || event.key() == input_constants::KEY_NUMPADENTER
        {
            self.toggle_setting();
            return true;
        }

        false
    }
}
