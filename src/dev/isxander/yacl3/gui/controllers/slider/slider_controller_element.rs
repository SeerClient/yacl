//! Ported from dev.isxander.yacl3.gui.controllers.slider.SliderControllerElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController;
use crate::dev::isxander::yacl3::gui::utils::key_utils::KeyUtils;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::util::mth;

/// java: public class SliderControllerElement extends ControllerWidget<ISliderController<?>>
pub struct SliderControllerElement {
    /// java: extends ControllerWidget
    pub base: ControllerWidgetBase<compat::Shared<dyn ISliderController<compat::Object>>>,
    /// java: private final double min, max, interval;
    pub min: f64,
    /// java: private final double min, max, interval;
    pub max: f64,
    /// java: private final double min, max, interval;
    pub interval: f64,
    /// java: private float interpolation;
    pub interpolation: f32,
    /// java: private Dimension<Integer> sliderBounds;
    pub slider_bounds: Option<compat::Shared<dyn Dimension<i32>>>,
    /// java: private boolean mouseDown = false;
    pub mouse_down: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for SliderControllerElement {
    type Target = ControllerWidgetBase<compat::Shared<dyn ISliderController<compat::Object>>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SliderControllerElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SliderControllerElement {
    /// java: public SliderControllerElement(ISliderController<?> option, YACLScreen screen, Dimension<Integer> dim, double min, double max, double interval)
    pub fn new(mut option: compat::Shared<dyn ISliderController<compat::Object>>, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>, mut min: f64, mut max: f64, mut interval: f64) -> compat::Shared<Self> {
        let __init_min = min;
        let __init_max = max;
        let __init_interval = interval;
        let __this = compat::shared(Self {
            base: ControllerWidgetBase::<compat::Shared<dyn ISliderController<compat::Object>>>::new(option.clone(), screen.clone(), dim.clone()),
            self_ref: std::sync::Weak::new(),
            min: __init_min,
            max: __init_max,
            interval: __init_interval,
            interpolation: 0.0,
            slider_bounds: None,
            mouse_down: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        ({ let __f = compat::dispatch(&__this, |__t| __t.set_dimension()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn AbstractWidget>), dim.clone()) });
        __this
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        if !(__this.lock().borrow().is_available()) || event.button() != 0 || !(SliderControllerElement::is_hovered_slider_bounds(__this, event.x(), event.y())) {
            return false;
        }
        { let __v = true; compat::write(__this, |__t| __t.mouse_down = __v); }
        SliderControllerElement::set_value_from_mouse(__this, event.x());
        return true;
    }

    /// java: private boolean isHoveredSliderBounds(double mouseX, double mouseY)
    pub fn is_hovered_slider_bounds(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return { let (__a0, __a1,) = (&((mouse_x) as i32), &((mouse_y) as i32),); __this.lock().borrow().slider_bounds.as_ref().unwrap().is_point_inside(__a0, __a1) };
    }

    /// java: @Override public boolean mouseDragged(@NonNull MouseButtonEvent event, double dx, double dy)
    pub fn mouse_dragged(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut dx: f64, mut dy: f64) -> bool {
        if !(__this.lock().borrow().is_available()) || event.button() != 0 || !(compat::read(__this, |__t| __t.mouse_down)) {
            return false;
        }
        SliderControllerElement::set_value_from_mouse(__this, event.x());
        return true;
    }

    /// java: public void incrementValue(double amount)
    pub fn increment_value(__this: &compat::Shared<Self>, mut amount: f64) {
        // TODO(fabric2seer): `.setPendingValue(..)` on a T
        //   java: control.setPendingValue(Mth.clamp(control.pendingValue() + interval * amount, min, max));
        SliderControllerElement::calculate_interpolation(__this);
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal: f64, mut vertical: f64) -> bool {
        if !(__this.lock().borrow().is_available()) || !({ let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), mouse_x, mouse_y) }) || !(KeyUtils::has_shift_down()) && !(KeyUtils::has_control_down()) {
            return false;
        }
        SliderControllerElement::increment_value(__this, vertical);
        return true;
    }

    /// java: @Override public boolean mouseReleased(@NonNull MouseButtonEvent event)
    pub fn mouse_released(__this: &compat::Shared<Self>, event: &MouseButtonEvent) -> bool {
        if __this.lock().borrow().is_available() && compat::read(__this, |__t| __t.mouse_down) {
            ({ let __f = compat::dispatch(__this, |__t| __t.play_down_sound()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) });
        }
        { let __v = false; compat::write(__this, |__t| __t.mouse_down = __v); }
        // TODO(fabric2seer): super.mouseReleased(..)
        //   java: return super.mouseReleased(event);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    pub fn key_pressed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if !(compat::read(__this, |__t| __t.focused)) {
            return false;
        }
        match event.key() {
            263 => {
                SliderControllerElement::increment_value(__this, ((-1) as f64) as f64);
            }
            262 => {
                SliderControllerElement::increment_value(__this, ((1) as f64) as f64);
            }
            _ => {
                return false;
            }
        }
        return true;
    }

    /// java: protected void setValueFromMouse(double mouseX)
    pub fn set_value_from_mouse(__this: &compat::Shared<Self>, mut mouse_x: f64) {
        // TODO(fabric2seer): `.range(..)` on a T
        //   java: double value = (mouseX - sliderBounds.x()) / sliderBounds.width() * control.range();
        let mut value: f64 = todo!("fabric2seer: the initialiser of `value` (TODO above)");
        // TODO(fabric2seer): `.setPendingValue(..)` on a T
        //   java: control.setPendingValue(roundToInterval(value));
        SliderControllerElement::calculate_interpolation(__this);
    }

    /// java: protected double roundToInterval(double value)
    pub fn round_to_interval(__this: &compat::Shared<Self>, mut value: f64) -> f64 {
        return mth::clamp_f64((compat::read(__this, |__t| __t.min) + (compat::read(__this, |__t| __t.interval) * (compat::java_round_f64((value / compat::read(__this, |__t| __t.interval))) as f64))), compat::read(__this, |__t| __t.min), compat::read(__this, |__t| __t.max));
    }

    /// java: protected void calculateInterpolation()
    pub fn calculate_interpolation(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `.pendingValue(..)` on a T
        //   java: interpolation = Mth.clamp((float) ((control.pendingValue() - control.min()) * 1 / control.range()), 0f, 1f);
    }

    /// java: protected int getThumbX()
    pub fn get_thumb_x(__this: &compat::Shared<Self>) -> i32 {
        return (((__this.lock().borrow().slider_bounds.as_ref().unwrap().x() + (__this.lock().borrow().slider_bounds.as_ref().unwrap().width() * compat::read(__this, |__t| __t.interpolation)))) as i32);
    }

    /// java: protected int getThumbWidth()
    pub fn get_thumb_width(__this: &compat::Shared<Self>) -> i32 {
        return 4;
    }

}

impl ControllerWidget<compat::Shared<dyn crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController<compat::Object>>> for SliderControllerElement {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<compat::Shared<dyn crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController<compat::Object>>> {
        &self.base
    }
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<compat::Shared<dyn crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController<compat::Object>>> {
        &mut self.base
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        self.base.extract_render_state(graphics, mouse_x, mouse_y, a);
        SliderControllerElement::calculate_interpolation(&self.self_ref.upgrade().unwrap());
    }

    /// java: @Override protected void extractHoveredControl(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_hovered_control(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.slider_bounds.as_ref().unwrap().lock().borrow().x(), (self.slider_bounds.as_ref().unwrap().lock().borrow().center_y()).wrapping_sub(1), self.slider_bounds.as_ref().unwrap().lock().borrow().x_limit(), self.slider_bounds.as_ref().unwrap().lock().borrow().center_y(), -1);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (self.slider_bounds.as_ref().unwrap().lock().borrow().x()).wrapping_add(1), self.slider_bounds.as_ref().unwrap().lock().borrow().center_y(), (self.slider_bounds.as_ref().unwrap().lock().borrow().x_limit()).wrapping_add(1), (self.slider_bounds.as_ref().unwrap().lock().borrow().center_y()).wrapping_add(1), 0xFF404040u32 as i32);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), ((SliderControllerElement::get_thumb_x(&self.self_ref.upgrade().unwrap())).wrapping_sub((SliderControllerElement::get_thumb_width(&self.self_ref.upgrade().unwrap())).wrapping_div(2))).wrapping_add(1), (self.slider_bounds.as_ref().unwrap().lock().borrow().y()).wrapping_add(1), ((SliderControllerElement::get_thumb_x(&self.self_ref.upgrade().unwrap())).wrapping_add((SliderControllerElement::get_thumb_width(&self.self_ref.upgrade().unwrap())).wrapping_div(2))).wrapping_add(1), (self.slider_bounds.as_ref().unwrap().lock().borrow().y_limit()).wrapping_add(1), 0xFF404040u32 as i32);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (SliderControllerElement::get_thumb_x(&self.self_ref.upgrade().unwrap())).wrapping_sub((SliderControllerElement::get_thumb_width(&self.self_ref.upgrade().unwrap())).wrapping_div(2)), self.slider_bounds.as_ref().unwrap().lock().borrow().y(), (SliderControllerElement::get_thumb_x(&self.self_ref.upgrade().unwrap())).wrapping_add((SliderControllerElement::get_thumb_width(&self.self_ref.upgrade().unwrap())).wrapping_div(2)), self.slider_bounds.as_ref().unwrap().lock().borrow().y_limit(), -1);
        if SliderControllerElement::is_hovered_slider_bounds(&self.self_ref.upgrade().unwrap(), (mouse_x) as f64, (mouse_y) as f64) {
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&(if self.is_available() { RESIZE_EW } else { NOT_ALLOWED }));
        }
    }

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let __pose_1 = gui_graphics_extractor::push_pose();
        if self.is_hovered() {
            gui_graphics_extractor::pose_translate(((-(((self.slider_bounds.as_ref().unwrap().lock().borrow().width()).wrapping_add(6) as f32) + ((SliderControllerElement::get_thumb_width(&self.self_ref.upgrade().unwrap()) as f32) / 2.0f32)))) as f32, (0) as f32);
        }
        self.base.extract_value_text(graphics, mouse_x, mouse_y, a);
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        return (((self.slider_bounds.as_ref().unwrap().lock().borrow().width()).wrapping_add(self.get_unhovered_control_width())).wrapping_add(6)).wrapping_add((SliderControllerElement::get_thumb_width(&self.self_ref.upgrade().unwrap())).wrapping_div(2));
    }

}

impl AbstractWidget for SliderControllerElement {
    fn abstract_widget_base(&self) -> &AbstractWidgetBase {
        &self.base.base
    }
    fn abstract_widget_base_mut(&mut self) -> &mut AbstractWidgetBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY) (dispatch: the body for this class)
    fn is_mouse_over(&self) -> fn(&compat::Shared<dyn AbstractWidget>, f64, f64) -> bool {
        SliderControllerElement::is_mouse_over_dyn
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim) (dispatch: the body for this class)
    fn set_dimension(&self) -> fn(&compat::Shared<dyn AbstractWidget>, compat::Shared<dyn Dimension<i32>>) {
        SliderControllerElement::set_dimension_dyn
    }

    /// java: @Override public boolean canReset() (dispatch: the body for this class)
    fn can_reset(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> bool {
        <dyn ControllerWidget>::can_reset_from::<Self>
    }

    /// java: @Override public NarrationPriority narrationPriority() (dispatch: the body for this class)
    fn narration_priority(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> crate::unresolved::NarrationPriority {
        <dyn ControllerWidget>::narration_priority_from::<Self>
    }

    /// java: @Override public void unfocus() (dispatch: the body for this class)
    fn unfocus(&self) -> fn(&compat::Shared<dyn AbstractWidget>) {
        <dyn ControllerWidget>::unfocus_from::<Self>
    }

    /// java: @Override public boolean matchesSearch(String query) (dispatch: the body for this class)
    fn matches_search(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &str) -> bool {
        <dyn ControllerWidget>::matches_search_from::<Self>
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_narration(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &mut dyn NarrationElementOutput) {
        <dyn ControllerWidget>::update_narration_from::<Self>
    }

}

impl SliderControllerElement {
    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    pub fn is_mouse_over_impl(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return (<dyn AbstractWidget>::is_mouse_over_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), mouse_x, mouse_y) || compat::read(__this, |__t| __t.mouse_down));
    }

    /// SliderControllerElement.isMouseOver as the dispatch of AbstractWidget.isMouseOver
    pub fn is_mouse_over_dyn(__this: &compat::Shared<dyn AbstractWidget>, __p0: f64, __p1: f64) -> bool {
        Self::is_mouse_over_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim)
    pub fn set_dimension_impl(__this: &compat::Shared<Self>, mut dim: compat::Shared<dyn Dimension<i32>>) {
        <dyn AbstractWidget>::set_dimension_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), dim.clone());
        let mut track_width: i32 = (dim.lock().borrow().width()).wrapping_div(3);
        if __this.lock().borrow().option_name_string.is_empty() {
            track_width = (dim.lock().borrow().width()).wrapping_div(2);
        }
        { let __v = Some((dimension::of_int((((dim.lock().borrow().x_limit()).wrapping_sub(__this.lock().borrow().get_x_padding())).wrapping_sub((SliderControllerElement::get_thumb_width(__this)).wrapping_div(2))).wrapping_sub(track_width), (dim.lock().borrow().center_y()).wrapping_sub(5), track_width, 10) as compat::Shared<dyn Dimension<compat::Object>>)); compat::write(__this, |__t| __t.slider_bounds = __v); }
    }

    /// SliderControllerElement.setDimension as the dispatch of AbstractWidget.setDimension
    pub fn set_dimension_dyn(__this: &compat::Shared<dyn AbstractWidget>, __p0: compat::Shared<dyn Dimension<i32>>) {
        Self::set_dimension_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
