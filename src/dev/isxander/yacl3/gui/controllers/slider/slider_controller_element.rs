//! Ported from dev.isxander.yacl3.gui.controllers.slider.SliderControllerElement.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::{self, Dimension};
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::{self, ControllerWidget, ControllerWidgetBase};
use crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController;
use crate::dev::isxander::yacl3::gui::utils::key_utils::KeyUtils;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::{CursorKind, Graphics};
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::util::mth;
use std::cell::{Cell, RefCell};

/// `ISliderController<?>` as the element holds it: the controller's members that do not mention `T`.
#[derive(Clone)]
pub struct SliderControl {
    min: Rc<dyn Fn() -> f64>,
    range: Rc<dyn Fn() -> f64>,
    set_pending_value: Rc<dyn Fn(f64)>,
    pending_value: Rc<dyn Fn() -> f64>,
}

impl SliderControl {
    /// The controller seen through the wildcard.
    pub fn of<T: Clone + 'static, C: ISliderController<T> + ?Sized + 'static>(control: &Arc<C>) -> SliderControl {
        let (a, b, c, d) = (control.clone(), control.clone(), control.clone(), control.clone());
        SliderControl {
            min: Rc::new(move || <C as ISliderController<T>>::min(&a)),
            range: Rc::new(move || b.range()),
            set_pending_value: Rc::new(move |value| c.set_pending_value(value)),
            pending_value: Rc::new(move || d.pending_value()),
        }
    }

    /// java: double min();
    pub fn min(&self) -> f64 {
        (self.min)()
    }

    /// java: default double range()
    pub fn range(&self) -> f64 {
        (self.range)()
    }

    /// java: void setPendingValue(double value);
    pub fn set_pending_value(&self, value: f64) {
        (self.set_pending_value)(value)
    }

    /// java: double pendingValue();
    pub fn pending_value(&self) -> f64 {
        (self.pending_value)()
    }
}

/// java: public class SliderControllerElement extends ControllerWidget<ISliderController<?>>
pub struct SliderControllerElement {
    /// java: extends ControllerWidget<ISliderController<?>>
    pub cw: ControllerWidgetBase,
    /// java: protected final T control;
    pub control: SliderControl,
    /// java: private final double min, max, interval;
    min: f64,
    max: f64,
    interval: f64,
    /// java: private float interpolation;
    interpolation: Cell<f32>,
    /// java: private Dimension<Integer> sliderBounds;
    slider_bounds: RefCell<Option<Arc<dyn Dimension<i32>>>>,
    /// java: private boolean mouseDown = false;
    mouse_down: Cell<bool>,
}

impl SliderControllerElement {
    /// java: public SliderControllerElement(ISliderController<?> option, YACLScreen screen, Dimension<Integer> dim,
    /// double min, double max, double interval)
    pub fn new<T: Clone + 'static, C: ISliderController<T> + ?Sized + 'static>(
        option: Arc<C>,
        screen: &Arc<YACLScreen>,
        dim: Arc<dyn Dimension<i32>>,
        min: f64,
        max: f64,
        interval: f64,
    ) -> Self {
        let element = Self {
            cw: ControllerWidgetBase::new::<T, C>(&option, screen, dim.clone()),
            control: SliderControl::of(&option),
            min,
            max,
            interval,
            interpolation: Cell::new(0.0),
            slider_bounds: RefCell::new(None),
            mouse_down: Cell::new(false),
        };
        // java: setDimension(dim);
        ControllerWidget::set_dimension(&element, dim);
        element
    }

    /// `sliderBounds`
    fn slider_bounds(&self) -> Arc<dyn Dimension<i32>> {
        self.slider_bounds.borrow().clone().expect("SliderControllerElement.sliderBounds before setDimension")
    }

    /// java: private boolean isHoveredSliderBounds(double mouseX, double mouseY)
    fn is_hovered_slider_bounds(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.slider_bounds().is_point_inside(mouse_x as i32, mouse_y as i32)
    }

    /// java: public void incrementValue(double amount)
    pub fn increment_value(&self, amount: f64) {
        self.control.set_pending_value(mth::clamp_f64(self.control.pending_value() + self.interval * amount, self.min, self.max));
        self.calculate_interpolation();
    }

    /// java: protected void setValueFromMouse(double mouseX)
    pub fn set_value_from_mouse(&self, mouse_x: f64) {
        let bounds = self.slider_bounds();
        let value = (mouse_x - bounds.x() as f64) / bounds.width() as f64 * self.control.range();
        self.control.set_pending_value(self.round_to_interval(value));
        self.calculate_interpolation();
    }

    /// java: protected double roundToInterval(double value)
    pub fn round_to_interval(&self, value: f64) -> f64 {
        // extremely imprecise, requires clamping
        mth::clamp_f64(self.min + (self.interval * compat::java_round_f64(value / self.interval) as f64), self.min, self.max)
    }

    /// java: protected void calculateInterpolation()
    pub fn calculate_interpolation(&self) {
        self.interpolation.set(mth::clamp_f32(
            ((self.control.pending_value() - self.control.min()) * 1.0 / self.control.range()) as f32,
            0.0,
            1.0,
        ));
    }

    /// java: protected int getThumbX()
    pub fn get_thumb_x(&self) -> i32 {
        let bounds = self.slider_bounds();
        (bounds.x() as f32 + bounds.width() as f32 * self.interpolation.get()) as i32
    }

    /// java: protected int getThumbWidth()
    pub fn get_thumb_width(&self) -> i32 {
        4
    }
}

impl ControllerWidget for SliderControllerElement {
    fn controller_widget(&self) -> &ControllerWidgetBase {
        &self.cw
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        controller_widget::extract_render_state(self, graphics, mouse_x, mouse_y, a);

        self.calculate_interpolation();
    }

    /// java: @Override protected void extractHoveredControl(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_hovered_control(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        let bounds = self.slider_bounds();
        // track
        graphics.fill(bounds.x(), bounds.center_y() - 1, bounds.x_limit(), bounds.center_y(), -1);
        // track shadow
        graphics.fill(bounds.x() + 1, bounds.center_y(), bounds.x_limit() + 1, bounds.center_y() + 1, 0xFF404040u32 as i32);

        let thumb_x = self.get_thumb_x();
        let thumb_width = self.get_thumb_width();
        // thumb shadow
        graphics.fill(thumb_x - thumb_width / 2 + 1, bounds.y() + 1, thumb_x + thumb_width / 2 + 1, bounds.y_limit() + 1, 0xFF404040u32 as i32);
        // thumb
        graphics.fill(thumb_x - thumb_width / 2, bounds.y(), thumb_x + thumb_width / 2, bounds.y_limit(), -1);

        if self.is_hovered_slider_bounds(mouse_x as f64, mouse_y as f64) {
            graphics.request_cursor(if self.is_available() { CursorKind::ResizeEw } else { CursorKind::NotAllowed });
        }
    }

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        graphics.push_pose();
        if self.is_hovered() {
            graphics.translate(-(self.slider_bounds().width() as f32 + 6.0 + self.get_thumb_width() as f32 / 2.0), 0.0);
        }
        controller_widget::extract_value_text(self, graphics, mouse_x, mouse_y, a);
        graphics.pop_pose();
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked(&self, event: &MouseButtonEvent, _double_click: bool) -> bool {
        if !self.is_available() || event.button() != 1 || !self.is_hovered_slider_bounds(event.x(), event.y()) {
            return false;
        }

        self.mouse_down.set(true);

        self.set_value_from_mouse(event.x());
        true
    }

    /// java: @Override public boolean mouseDragged(@NonNull MouseButtonEvent event, double dx, double dy)
    fn mouse_dragged(&self, event: &MouseButtonEvent, _dx: f64, _dy: f64) -> bool {
        if !self.is_available() || event.button() != 1 || !self.mouse_down.get() {
            return false;
        }

        self.set_value_from_mouse(event.x());
        true
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    fn mouse_scrolled(&self, mouse_x: f64, mouse_y: f64, _horizontal: f64, vertical: f64) -> bool {
        if !self.is_available()
            || (!ControllerWidget::is_mouse_over(self, mouse_x, mouse_y))
            || (!KeyUtils::has_shift_down() && !KeyUtils::has_control_down())
        {
            return false;
        }

        self.increment_value(vertical);
        true
    }

    /// java: @Override public boolean mouseReleased(@NonNull MouseButtonEvent event)
    fn mouse_released(&self, _event: &MouseButtonEvent) -> bool {
        if self.is_available() && self.mouse_down.get() {
            ControllerWidget::play_down_sound(self);
        }
        self.mouse_down.set(false);

        // super.mouseReleased(event) - GuiEventListener's `false`
        false
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        if !self.cw.focused.get() {
            return false;
        }

        match event.key() {
            input_constants::KEY_LEFT => self.increment_value(-1.0),
            input_constants::KEY_RIGHT => self.increment_value(1.0),
            _ => return false,
        }

        true
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.cw.base.is_mouse_over(mouse_x, mouse_y) || self.mouse_down.get()
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        self.slider_bounds().width() + self.get_unhovered_control_width() + 6 + self.get_thumb_width() / 2
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim)
    fn set_dimension(&self, dim: Arc<dyn Dimension<i32>>) {
        // super.setDimension(dim);
        self.cw.base.set_dimension(dim.clone());
        let mut track_width = dim.width() / 3;
        if self.cw.option_name_string.is_empty() {
            track_width = dim.width() / 2;
        }

        let bounds: Arc<dyn Dimension<i32>> = dimension::of_int(
            dim.x_limit() - self.get_x_padding() - self.get_thumb_width() / 2 - track_width,
            dim.center_y() - 5,
            track_width,
            10,
        );
        *self.slider_bounds.borrow_mut() = Some(bounds);
    }
}
