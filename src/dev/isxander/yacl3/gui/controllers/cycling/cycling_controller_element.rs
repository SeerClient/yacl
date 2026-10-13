//! Ported from dev.isxander.yacl3.gui.controllers.cycling.CyclingControllerElement.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::{self, ControllerWidget, ControllerWidgetBase};
use crate::dev::isxander::yacl3::gui::controllers::cycling::i_cycling_controller::ICyclingController;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::{CursorKind, Graphics};
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::net::minecraft::client::input::input_with_modifiers::InputWithModifiers as _;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;

/// `ICyclingController<?>` as the element holds it: the controller's members that do not mention `T`.
#[derive(Clone)]
pub struct CyclingControl {
    set_pending_value: Rc<dyn Fn(i32)>,
    get_pending_value: Rc<dyn Fn() -> i32>,
    get_cycle_length: Rc<dyn Fn() -> i32>,
}

impl CyclingControl {
    /// The controller seen through the wildcard.
    pub fn of<T: Clone + 'static, C: ICyclingController<T> + ?Sized + 'static>(control: &Arc<C>) -> CyclingControl {
        let (a, b, c) = (control.clone(), control.clone(), control.clone());
        CyclingControl {
            set_pending_value: Rc::new(move |ordinal| a.set_pending_value(ordinal)),
            get_pending_value: Rc::new(move || b.get_pending_value()),
            get_cycle_length: Rc::new(move || c.get_cycle_length()),
        }
    }

    /// java: void setPendingValue(int ordinal);
    pub fn set_pending_value(&self, ordinal: i32) {
        (self.set_pending_value)(ordinal)
    }

    /// java: int getPendingValue();
    pub fn get_pending_value(&self) -> i32 {
        (self.get_pending_value)()
    }

    /// java: int getCycleLength();
    pub fn get_cycle_length(&self) -> i32 {
        (self.get_cycle_length)()
    }
}

/// java: public class CyclingControllerElement extends ControllerWidget<ICyclingController<?>>
pub struct CyclingControllerElement {
    /// java: extends ControllerWidget<ICyclingController<?>>
    pub cw: ControllerWidgetBase,
    /// java: protected final T control;
    pub control: CyclingControl,
}

impl CyclingControllerElement {
    /// java: public CyclingControllerElement(ICyclingController<?> control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new<T: Clone + 'static, C: ICyclingController<T> + ?Sized + 'static>(
        control: Arc<C>,
        screen: &Arc<YACLScreen>,
        dim: Arc<dyn Dimension<i32>>,
    ) -> Self {
        Self { cw: ControllerWidgetBase::new::<T, C>(&control, screen, dim), control: CyclingControl::of(&control) }
    }

    /// java: public void cycleValue(int increment)
    pub fn cycle_value(&self, increment: i32) {
        let mut target_idx = self.control.get_pending_value() + increment;
        if target_idx >= self.control.get_cycle_length() {
            target_idx -= self.control.get_cycle_length();
        } else if target_idx < 0 {
            target_idx += self.control.get_cycle_length();
        }
        self.control.set_pending_value(target_idx);
    }
}

impl ControllerWidget for CyclingControllerElement {
    fn controller_widget(&self) -> &ControllerWidgetBase {
        &self.cw
    }

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        controller_widget::extract_value_text(self, graphics, mouse_x, mouse_y, a);

        if self.cw.hovered.get() {
            graphics.request_cursor(if self.is_available() { CursorKind::PointingHand } else { CursorKind::NotAllowed });
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked(&self, event: &MouseButtonEvent, _double_click: bool) -> bool {
        if !ControllerWidget::is_mouse_over(self, event.x(), event.y())
            || (event.button() != 0 && event.button() != 1)
            || !self.is_available()
        {
            return false;
        }

        ControllerWidget::play_down_sound(self);
        let info = event.button_info();
        self.cycle_value(if event.button() == 1 || info.has_shift_down() || info.has_control_down() { -1 } else { 1 });

        true
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        if !self.cw.focused.get() {
            return false;
        }

        match event.key() {
            input_constants::KEY_LEFT => self.cycle_value(-1),
            input_constants::KEY_RIGHT => self.cycle_value(1),
            input_constants::KEY_RETURN | input_constants::KEY_SPACE | input_constants::KEY_NUMPADENTER => {
                self.cycle_value(if event.has_control_down() || event.has_shift_down() { -1 } else { 1 })
            }
            _ => return false,
        }

        true
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        self.get_unhovered_control_width()
    }
}
