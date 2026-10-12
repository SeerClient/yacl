//! Ported from dev.isxander.yacl3.gui.controllers.ColorPickerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::api::utils::mutable_dimension::MutableDimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::color_controller::ColorController;
use crate::dev::isxander::yacl3::gui::controllers::color_controller::ColorControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::ControllerPopupWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::ControllerPopupWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElement;
use crate::dev::isxander::yacl3::gui::render::color_gradient_render_state::ColorGradientRenderState;
use crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::mth;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public static final Identifier COLOR_PICKER_SPRITE = YACLPlatform.rl("controller/colorpicker");
pub static COLOR_PICKER_SPRITE: compat::MainThread<Identifier> = compat::MainThread::new(|| YACLPlatform::rl("controller/colorpicker"));
/// java: public static final Identifier TRANSPARENT_SPRITE = YACLPlatform.rl("controller/transparent");
pub static TRANSPARENT_SPRITE: compat::MainThread<Identifier> = compat::MainThread::new(|| YACLPlatform::rl("controller/transparent"));

/// java: public class ColorPickerWidget extends ControllerPopupWidget<ColorController>
pub struct ColorPickerWidget {
    /// java: extends ControllerPopupWidget
    pub base: ControllerPopupWidgetBase<ColorController>,
    /// java: private final ColorController controller;
    pub controller: ColorController,
    /// java: private final ColorController.ColorControllerElement entryWidget;
    pub entry_widget: compat::Shared<ColorControllerElement>,
    /// java: protected MutableDimension<Integer> colorPickerDim;
    pub color_picker_dim: Option<compat::Shared<dyn MutableDimension<i32>>>,
    /// java: protected MutableDimension<Integer> previewColorDim;
    pub preview_color_dim: Option<compat::Shared<dyn MutableDimension<i32>>>,
    /// java: protected MutableDimension<Integer> saturationLightDim;
    pub saturation_light_dim: Option<compat::Shared<dyn MutableDimension<i32>>>,
    /// java: protected MutableDimension<Integer> hueGradientDim;
    pub hue_gradient_dim: Option<compat::Shared<dyn MutableDimension<i32>>>,
    /// java: protected MutableDimension<Integer> alphaGradientDim;
    pub alpha_gradient_dim: Option<compat::Shared<dyn MutableDimension<i32>>>,
    /// java: private boolean mouseDown;
    pub mouse_down: bool,
    /// java: private boolean hueSliderDown;
    pub hue_slider_down: bool,
    /// java: private boolean satLightGradientDown;
    pub sat_light_gradient_down: bool,
    /// java: private boolean alphaSliderDown;
    pub alpha_slider_down: bool,
    /// java: private int hueThumbX;
    pub hue_thumb_x: i32,
    /// java: private int satLightThumbX;
    pub sat_light_thumb_x: i32,
    /// java: private int alphaThumbX;
    pub alpha_thumb_x: i32,
    /// java: private boolean charTyped;
    pub char_typed: bool,
    /// java: private final int outline = 1;
    pub outline: i32,
    /// java: private final int previewPortion = 7;
    pub preview_portion: i32,
    /// java: private final int sliderHeight = 7;
    pub slider_height: i32,
    /// java: private final int paddingX = 1;
    pub padding_x: i32,
    /// java: private final int paddingY = 3;
    pub padding_y: i32,
    /// java: private float[] HSL;
    pub hsl: Vec<f32>,
    /// java: private float hue;
    pub hue: f32,
    /// java: private float saturation;
    pub saturation: f32,
    /// java: private float light;
    pub light: f32,
    /// java: private int alpha;
    pub alpha: i32,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for ColorPickerWidget {
    type Target = ControllerPopupWidgetBase<ColorController>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColorPickerWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColorPickerWidget {
    /// java: public ColorPickerWidget(ColorController control, YACLScreen screen, Dimension<Integer> dim, ColorController.ColorControllerElement entryWidget)
    pub fn new(mut control: ColorController, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>, mut entry_widget: compat::Shared<ColorControllerElement>) -> compat::Shared<Self> {
        let __init_controller = control.clone();
        let __init_entry_widget = entry_widget.clone();
        let __this = compat::shared(Self {
            base: ControllerPopupWidgetBase::<ColorController>::new(control.clone(), screen.clone(), dim.clone(), (entry_widget.clone() as compat::Shared<dyn ControllerWidget<compat::Object>>)),
            self_ref: std::sync::Weak::new(),
            controller: __init_controller,
            entry_widget: __init_entry_widget,
            color_picker_dim: None,
            preview_color_dim: None,
            saturation_light_dim: None,
            hue_gradient_dim: None,
            alpha_gradient_dim: None,
            mouse_down: false,
            hue_slider_down: false,
            sat_light_gradient_down: false,
            alpha_slider_down: false,
            hue_thumb_x: 0,
            sat_light_thumb_x: 0,
            alpha_thumb_x: 0,
            char_typed: false,
            outline: 1,
            preview_portion: 7,
            slider_height: 7,
            padding_x: 1,
            padding_y: 3,
            hsl: Default::default(),
            hue: 0.0,
            saturation: 0.0,
            light: 0.0,
            alpha: 0,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        ({ let __f = compat::dispatch(&__this, |__t| __t.set_dimension()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn AbstractWidget>), dim.clone()) });
        ColorPickerWidget::update_hsl(&__this);
        ColorPickerWidget::set_thumb_x(&__this);
        __this
    }

    /// java: private boolean isHoveringHueSlider(double mouseX, double mouseY)
    pub fn is_hovering_hue_slider(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return (mouse_y >= __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().y() && mouse_y <= __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().y_limit() && mouse_x >= __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().x() && mouse_x <= __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().x_limit());
    }

    /// java: public boolean clickedHueSlider(double mouseX, double mouseY)
    pub fn clicked_hue_slider(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        if compat::read(__this, |__t| __t.sat_light_gradient_down) || compat::read(__this, |__t| __t.alpha_slider_down) {
            return false;
        }
        if ColorPickerWidget::is_hovering_hue_slider(__this, mouse_x, mouse_y) {
            { let __v = true; compat::write(__this, |__t| __t.hue_slider_down = __v); }
        }
        if compat::read(__this, |__t| __t.hue_slider_down) {
            { let __v = ((mth::clamp_f64(mouse_x, __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().x(), __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().x_limit())) as i32); compat::write(__this, |__t| __t.hue_thumb_x = __v); }
        }
        return compat::read(__this, |__t| __t.hue_slider_down);
    }

    /// java: private boolean isHoveringSatLightGradient(double mouseX, double mouseY)
    pub fn is_hovering_sat_light_gradient(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return (mouse_y >= __this.lock().borrow().saturation_light_dim.as_ref().unwrap().y() && mouse_y <= __this.lock().borrow().saturation_light_dim.as_ref().unwrap().y_limit() && mouse_x >= __this.lock().borrow().saturation_light_dim.as_ref().unwrap().x() && mouse_x <= __this.lock().borrow().saturation_light_dim.as_ref().unwrap().x_limit());
    }

    /// java: public boolean clickedSatLightGradient(double mouseX, double mouseY)
    pub fn clicked_sat_light_gradient(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        if compat::read(__this, |__t| __t.hue_slider_down) || compat::read(__this, |__t| __t.alpha_slider_down) {
            return false;
        }
        if ColorPickerWidget::is_hovering_sat_light_gradient(__this, mouse_x, mouse_y) {
            { let __v = true; compat::write(__this, |__t| __t.sat_light_gradient_down = __v); }
        }
        if compat::read(__this, |__t| __t.sat_light_gradient_down) {
            { let __v = ((mth::clamp_f64(mouse_x, __this.lock().borrow().saturation_light_dim.as_ref().unwrap().x(), __this.lock().borrow().saturation_light_dim.as_ref().unwrap().x_limit())) as i32); compat::write(__this, |__t| __t.sat_light_thumb_x = __v); }
        }
        return compat::read(__this, |__t| __t.sat_light_gradient_down);
    }

    /// java: private boolean isHoveringAlphaSlider(double mouseX, double mouseY)
    pub fn is_hovering_alpha_slider(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        if compat::read(__this, |__t| __t.alpha_gradient_dim.clone()).is_none() {
            return false;
        }
        return (mouse_y >= __this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().y() && mouse_y <= __this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().y_limit() && mouse_x >= __this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().x() && mouse_x <= __this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().x_limit());
    }

    /// java: public boolean clickedAlphaSlider(double mouseX, double mouseY)
    pub fn clicked_alpha_slider(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        if compat::read(__this, |__t| __t.sat_light_gradient_down) || compat::read(__this, |__t| __t.hue_slider_down) {
            return false;
        }
        if ColorPickerWidget::is_hovering_alpha_slider(__this, mouse_x, mouse_y) {
            { let __v = true; compat::write(__this, |__t| __t.alpha_slider_down = __v); }
        }
        if compat::read(__this, |__t| __t.alpha_slider_down) {
            { let __v = ((mth::clamp_f64(mouse_x, __this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().x(), __this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().x_limit())) as i32); compat::write(__this, |__t| __t.alpha_thumb_x = __v); }
        }
        return compat::read(__this, |__t| __t.alpha_slider_down);
    }

    /// java: public void setColorFromMouseClick(double mouseX, double mouseY)
    pub fn set_color_from_mouse_click(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) {
        if ColorPickerWidget::clicked_sat_light_gradient(__this, mouse_x, mouse_y) {
            ColorPickerWidget::set_sat_light_from_mouse(__this, mouse_x, mouse_y);
        } else if ColorPickerWidget::clicked_hue_slider(__this, mouse_x, mouse_y) {
            ColorPickerWidget::set_hue_from_mouse(__this, mouse_x);
        } else if __this.lock().borrow().controller.allow_alpha() && ColorPickerWidget::clicked_alpha_slider(__this, mouse_x, mouse_y) {
            ColorPickerWidget::set_alpha_from_mouse(__this, mouse_x);
        }
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        if { let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), event.x(), event.y()) } {
            { let __v = true; compat::write(__this, |__t| __t.mouse_down = __v); }
            { let __v = false; compat::write(__this, |__t| __t.hue_slider_down = __v); }
            { let __v = false; compat::write(__this, |__t| __t.sat_light_gradient_down = __v); }
            { let __v = false; compat::write(__this, |__t| __t.alpha_slider_down = __v); }
            ColorPickerWidget::set_color_from_mouse_click(__this, event.x(), event.y());
            return true;
        } else if { let __r = &compat::read(__this, |__t| __t.entry_widget.clone()); let __f = compat::dispatch(__r, |__t| __t.is_mouse_over()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>), event.x(), event.y()) } {
            return ({ let __r = &compat::read(__this, |__t| __t.entry_widget.clone()); let __f = compat::dispatch(__r, |__t| __t.mouse_clicked()); __f(&compat::view(__r, |a| a as compat::Shared<dyn StringControllerElement>), event, double_click) });
        } else {
            __this.lock().borrow_mut().close();
            return false;
        }
    }

    /// java: @Override public boolean mouseReleased(@NonNull MouseButtonEvent event)
    pub fn mouse_released(__this: &compat::Shared<Self>, event: &MouseButtonEvent) -> bool {
        { let __v = false; compat::write(__this, |__t| __t.mouse_down = __v); }
        return false;
    }

    /// java: @Override public boolean mouseDragged(@NonNull MouseButtonEvent event, double dx, double dy)
    pub fn mouse_dragged(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut dx: f64, mut dy: f64) -> bool {
        if compat::read(__this, |__t| __t.mouse_down) || { let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), event.x(), event.y()) } {
            ColorPickerWidget::set_color_from_mouse_click(__this, event.x(), event.y());
            return true;
        }
        // TODO(fabric2seer): `mouseDragged` is not a method of ColorControllerElement
        //   java: return entryWidget.mouseDragged(event, dx, dy);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent event)
    pub fn char_typed(__this: &compat::Shared<Self>, event: &CharacterEvent) -> bool {
        { let __v = true; compat::write(__this, |__t| __t.char_typed = __v); }
        return ({ let __r = &compat::read(__this, |__t| __t.entry_widget.clone()); let __f = compat::dispatch(__r, |__t| __t.char_typed()); __f(&compat::view(__r, |a| a as compat::Shared<dyn StringControllerElement>), event) });
    }

    /// java: public void setThumbX()
    pub fn set_thumb_x(__this: &compat::Shared<Self>) {
        { let __v = ColorPickerWidget::get_hue_thumb_x(__this); compat::write(__this, |__t| __t.hue_thumb_x = __v); }
        { let __v = ColorPickerWidget::get_sat_light_thumb_x(__this); compat::write(__this, |__t| __t.sat_light_thumb_x = __v); }
        if __this.lock().borrow().controller.allow_alpha() {
            { let __v = ColorPickerWidget::get_alpha_thumb_x(__this); compat::write(__this, |__t| __t.alpha_thumb_x = __v); }
        }
    }

    /// java: protected int getHueThumbX()
    pub fn get_hue_thumb_x(__this: &compat::Shared<Self>) -> i32 {
        let mut min: i32 = __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().x();
        let mut max: i32 = __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().x_limit();
        let mut value: i32 = ((((min as f32) + (__this.lock().borrow().hue_gradient_dim.as_ref().unwrap().width() * compat::read(__this, |__t| __t.hue)))) as i32);
        return mth::clamp_i32(value, min, max);
    }

    /// java: protected int getSatLightThumbX()
    pub fn get_sat_light_thumb_x(__this: &compat::Shared<Self>) -> i32 {
        let mut min: i32 = __this.lock().borrow().saturation_light_dim.as_ref().unwrap().x();
        let mut max: i32 = __this.lock().borrow().saturation_light_dim.as_ref().unwrap().x_limit();
        let mut value: i32 = ((((min as f32) + (__this.lock().borrow().saturation_light_dim.as_ref().unwrap().width() * compat::read(__this, |__t| __t.saturation)))) as i32);
        return mth::clamp_i32(value, min, max);
    }

    /// java: protected int getSatLightThumbY()
    pub fn get_sat_light_thumb_y(__this: &compat::Shared<Self>) -> i32 {
        let mut min: i32 = __this.lock().borrow().saturation_light_dim.as_ref().unwrap().y();
        let mut max: i32 = __this.lock().borrow().saturation_light_dim.as_ref().unwrap().y_limit();
        let mut value: i32 = ((((min as f32) + (__this.lock().borrow().saturation_light_dim.as_ref().unwrap().height() * (1.0f32 - compat::read(__this, |__t| __t.light))))) as i32);
        return mth::clamp_i32(value, min, max);
    }

    /// java: protected int getAlphaThumbX()
    pub fn get_alpha_thumb_x(__this: &compat::Shared<Self>) -> i32 {
        let mut min: i32 = __this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().x();
        let mut max: i32 = __this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().x_limit();
        let mut value: i32 = max.wrapping_sub(((__this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().width()).wrapping_mul(compat::read(__this, |__t| __t.alpha))).wrapping_div(255));
        return mth::clamp_i32(value, min, max);
    }

    /// java: public void setHueFromMouse(double mouseX)
    pub fn set_hue_from_mouse(__this: &compat::Shared<Self>, mut mouse_x: f64) {
        if mouse_x < __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().x() {
            { let __v = 0.0f32; compat::write(__this, |__t| __t.hue = __v); }
        } else if mouse_x > __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().x_limit() {
            { let __v = 1.0f32; compat::write(__this, |__t| __t.hue = __v); }
        } else {
            let mut new_hue: f32 = ((((mouse_x - __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().x())) as f32) / __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().width());
            { let __v = mth::clamp_f32(new_hue, 0.0f32, 1.0f32); compat::write(__this, |__t| __t.hue = __v); }
        }
        ColorPickerWidget::set_color_controller_from_hsl(__this);
    }

    /// java: public void setSatLightFromMouse(double mouseX, double mouseY)
    pub fn set_sat_light_from_mouse(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) {
        if mouse_x < __this.lock().borrow().saturation_light_dim.as_ref().unwrap().x() {
            { let __v = 0.0f32; compat::write(__this, |__t| __t.saturation = __v); }
        } else if mouse_x > __this.lock().borrow().saturation_light_dim.as_ref().unwrap().x_limit() {
            { let __v = 1.0f32; compat::write(__this, |__t| __t.saturation = __v); }
        } else {
            let mut new_sat: f32 = ((((mouse_x - __this.lock().borrow().saturation_light_dim.as_ref().unwrap().x())) as f32) / __this.lock().borrow().saturation_light_dim.as_ref().unwrap().width());
            { let __v = mth::clamp_f32(new_sat, 0.0f32, 1.0f32); compat::write(__this, |__t| __t.saturation = __v); }
        }
        if mouse_y < __this.lock().borrow().saturation_light_dim.as_ref().unwrap().y() {
            { let __v = 1.0f32; compat::write(__this, |__t| __t.light = __v); }
        } else if mouse_y > __this.lock().borrow().saturation_light_dim.as_ref().unwrap().y_limit() {
            { let __v = 0.0f32; compat::write(__this, |__t| __t.light = __v); }
        } else {
            let mut new_light: f32 = ((((mouse_y - __this.lock().borrow().saturation_light_dim.as_ref().unwrap().y())) as f32) / __this.lock().borrow().saturation_light_dim.as_ref().unwrap().height());
            { let __v = mth::clamp_f32((1.0f32 - new_light), 0.0f32, 1.0f32); compat::write(__this, |__t| __t.light = __v); }
        }
        ColorPickerWidget::set_color_controller_from_hsl(__this);
    }

    /// java: public void setAlphaFromMouse(double mouseX)
    pub fn set_alpha_from_mouse(__this: &compat::Shared<Self>, mut mouse_x: f64) {
        if mouse_x < __this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().x() {
            { let __v = 255; compat::write(__this, |__t| __t.alpha = __v); }
        } else if mouse_x > __this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().x_limit() {
            { let __v = 0; compat::write(__this, |__t| __t.alpha = __v); }
        } else {
            let mut new_alpha: i32 = (((((mouse_x - __this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().x_limit()) / __this.lock().borrow().alpha_gradient_dim.as_ref().unwrap().width()) * (-255 as f64))) as i32);
            { let __v = mth::clamp_i32(new_alpha, 0, 255); compat::write(__this, |__t| __t.alpha = __v); }
        }
        ColorPickerWidget::set_color_controller_from_hsl(__this);
    }

    /// java: public void setColorControllerFromHSL()
    pub fn set_color_controller_from_hsl(__this: &compat::Shared<Self>) {
        let mut true_hue: f32 = ((((compat::read(__this, |__t| __t.hue_thumb_x)).wrapping_sub(__this.lock().borrow().color_picker_dim.as_ref().unwrap().x())) as f32) / __this.lock().borrow().color_picker_dim.as_ref().unwrap().width());
        let mut color: Color = Color::get_hsb_color(true_hue, compat::read(__this, |__t| __t.saturation), compat::read(__this, |__t| __t.light));
        // TODO(fabric2seer): `.getRed(..)` on a Color
        //   java: Color returnColor = new Color(color.getRed(), color.getGreen(), color.getBlue(), alpha);
        let mut return_color: Color = todo!("fabric2seer: the initialiser of `returnColor` (TODO above)");
        { let (__a0,) = (return_color.clone(),); __this.lock().borrow().controller.option().lock().borrow_mut().request_set(__a0) };
    }

    /// java: protected void updateHSL()
    pub fn update_hsl(__this: &compat::Shared<Self>) {
        { let __v = ColorPickerWidget::get_hsl(__this); compat::write(__this, |__t| __t.hsl = __v); }
        { let __v = ColorPickerWidget::hue(__this); compat::write(__this, |__t| __t.hue = __v); }
        { let __v = ColorPickerWidget::saturation(__this); compat::write(__this, |__t| __t.saturation = __v); }
        { let __v = ColorPickerWidget::light(__this); compat::write(__this, |__t| __t.light = __v); }
        { let __v = ColorPickerWidget::get_alpha(__this); compat::write(__this, |__t| __t.alpha = __v); }
        if compat::read(__this, |__t| __t.char_typed) {
            ColorPickerWidget::set_thumb_x(__this);
            { let __v = false; compat::write(__this, |__t| __t.char_typed = __v); }
        }
    }

    /// java: protected float[] getHSL()
    pub fn get_hsl(__this: &compat::Shared<Self>) -> Vec<f32> {
        let mut pending_value: Color = __this.lock().borrow().controller.option().lock().borrow().pending_value();
        // TODO(fabric2seer): `.getRed(..)` on a Color
        //   java: return Color.RGBtoHSB(pendingValue.getRed(), pendingValue.getGreen(), pendingValue.getBlue(), null);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: protected float hue()
    pub fn hue(__this: &compat::Shared<Self>) -> f32 {
        return __this.lock().borrow().hsl[(0) as usize];
    }

    /// java: protected float saturation()
    pub fn saturation(__this: &compat::Shared<Self>) -> f32 {
        return __this.lock().borrow().hsl[(1) as usize];
    }

    /// java: protected float light()
    pub fn light(__this: &compat::Shared<Self>) -> f32 {
        return __this.lock().borrow().hsl[(2) as usize];
    }

    /// java: protected int getAlpha()
    pub fn get_alpha(__this: &compat::Shared<Self>) -> i32 {
        // TODO(fabric2seer): `.getAlpha(..)` on a T
        //   java: return controller.option().pendingValue().getAlpha();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: protected float getRgbFromHueX()
    pub fn get_rgb_from_hue_x(__this: &compat::Shared<Self>) -> f32 {
        let mut true_hue: f32 = ((((compat::read(__this, |__t| __t.hue_thumb_x)).wrapping_sub(__this.lock().borrow().color_picker_dim.as_ref().unwrap().x())) as f32) / __this.lock().borrow().color_picker_dim.as_ref().unwrap().width());
        // TODO(fabric2seer): static `Color.HSBtoRGB` has no Rust counterpart in the port
        // rust: return Color::hs_bto_rgb(true_hue, 1, 1);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: protected int getRgbWithoutAlpha()
    pub fn get_rgb_without_alpha(__this: &compat::Shared<Self>) -> i32 {
        let mut pending_color: Color = __this.lock().borrow().controller.option().lock().borrow().pending_value();
        // TODO(fabric2seer): `.getRed(..)` on a Color
        //   java: Color returnColor = new Color(pendingColor.getRed(), pendingColor.getGreen(), pendingColor.getBlue(), 255);
        let mut return_color: Color = todo!("fabric2seer: the initialiser of `returnColor` (TODO above)");
        // TODO(fabric2seer): `.getRGB(..)` on a Color
        //   java: return returnColor.getRGB();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl ControllerPopupWidget<crate::dev::isxander::yacl3::gui::controllers::color_controller::ColorController> for ColorPickerWidget {
    fn controller_popup_widget_base(&self) -> &ControllerPopupWidgetBase<crate::dev::isxander::yacl3::gui::controllers::color_controller::ColorController> {
        &self.base
    }
    fn controller_popup_widget_base_mut(&mut self) -> &mut ControllerPopupWidgetBase<crate::dev::isxander::yacl3::gui::controllers::color_controller::ColorController> {
        &mut self.base
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor guiGraphics, int mouseX, int mouseY, float partialTick)
    fn extract_background(&mut self, gui_graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut partial_tick: f32) {
        { let __v = ColorControllerElement::is_mouse_over_color_preview(&self.entry_widget, (mouse_x) as f64, (mouse_y) as f64); compat::write(&self.entry_widget, |__t| __t.hovered_over_color_preview = __v); }
    }

    /// java: @Override public void close()
    fn close(&mut self) {
        ColorControllerElement::remove_color_picker(&self.entry_widget);
    }

    /// java: @Override public Component popupTitle()
    fn popup_title(&self) -> MutableComponent {
        return component::translatable("yacl.control.color.color_picker_title");
    }

}

impl ControllerWidget<compat::Object> for ColorPickerWidget {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<compat::Object> {
        &self.base.base
    }
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<compat::Object> {
        &mut self.base.base
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ColorPickerWidget::update_hsl(&self.self_ref.upgrade().unwrap());
        let mut thumb_width: i32 = 4;
        let mut thumb_height: i32 = 4;
        let __pose_1 = gui_graphics_extractor::push_pose();
        let mut x1: i32 = (self.color_picker_dim.as_ref().unwrap().lock().borrow().x()).wrapping_sub(5);
        let mut y1: i32 = (self.color_picker_dim.as_ref().unwrap().lock().borrow().y()).wrapping_sub(5);
        let mut width1: i32 = (self.color_picker_dim.as_ref().unwrap().lock().borrow().width()).wrapping_add(10);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), COLOR_PICKER_SPRITE.get_mut(), x1, y1, width1, (self.color_picker_dim.as_ref().unwrap().lock().borrow().height()).wrapping_add(10));
        // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
        //   java: graphics.fill(previewColorDim.x() - outline, previewColorDim.y() - outline, previewColorDim.xLimit() + outline, previewColorDim.yLimit() + outline, Color.black.getRGB());
        if self.controller.allow_alpha() {
            let mut x: i32 = self.preview_color_dim.as_ref().unwrap().lock().borrow().x();
            let mut y: i32 = self.preview_color_dim.as_ref().unwrap().lock().borrow().y();
            let mut width: i32 = self.preview_color_dim.as_ref().unwrap().lock().borrow().width();
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TRANSPARENT_SPRITE.get_mut(), x, y, width, self.preview_color_dim.as_ref().unwrap().lock().borrow().height());
        }
        // TODO(fabric2seer): `.getRGB(..)` on a T
        //   java: graphics.fill(previewColorDim.x(), previewColorDim.y(), previewColorDim.xLimit(), previewColorDim.yLimit(), controller.option().pendingValue().getRGB());
        // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
        //   java: graphics.fill(saturationLightDim.x() - outline, saturationLightDim.y() - outline, saturationLightDim.xLimit() + outline, saturationLightDim.yLimit() + outline, Color.black.getRGB());
        ({ let __r = &ColorGradientRenderState::create_horizontal(graphics, self.saturation_light_dim.as_ref().unwrap().lock().borrow().x(), self.saturation_light_dim.as_ref().unwrap().lock().borrow().y(), self.saturation_light_dim.as_ref().unwrap().lock().borrow().x_limit(), self.saturation_light_dim.as_ref().unwrap().lock().borrow().y_limit(), 0xFFFFFFFFu32 as i32, ((((ColorPickerWidget::get_rgb_from_hue_x(&self.self_ref.upgrade().unwrap())) as i32) & 0x00FFFFFF) | 0xFF000000u32 as i32)); let __f = compat::dispatch(__r, |__t| __t.submit()); __f(&compat::view(__r, |a| a as compat::Shared<dyn YACLGuiElementRenderState>), graphics) });
        // TODO(fabric2seer): `GuiGraphicsExtractor.fillGradient` has no Rust method in the port (looked for `fill_gradient` in GuiGraphicsExtractor and its supertypes)
        // rust: graphics.fill_gradient(&self.saturation_light_dim.as_ref().unwrap().lock().borrow().x(), &self.saturation_light_dim.as_ref().unwrap().lock().borrow().y(), &self.saturation_light_dim.as_ref().unwrap().lock().borrow().x_limit(), &self.saturation_light_dim.as_ref().unwrap().lock().borrow().y_limit(), 0x00000000, 0xFF000000u32 as i32);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (self.sat_light_thumb_x.wrapping_sub(thumb_width.wrapping_div(2))).wrapping_sub(2), ((ColorPickerWidget::get_sat_light_thumb_y(&self.self_ref.upgrade().unwrap())).wrapping_add(thumb_height.wrapping_div(2))).wrapping_add(2), (self.sat_light_thumb_x.wrapping_add(thumb_width.wrapping_div(2))).wrapping_add(1), ((ColorPickerWidget::get_sat_light_thumb_y(&self.self_ref.upgrade().unwrap())).wrapping_sub(thumb_height.wrapping_div(2))).wrapping_sub(1), 0xFF404040u32 as i32);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (self.sat_light_thumb_x.wrapping_sub(thumb_width.wrapping_div(2))).wrapping_sub(1), ((ColorPickerWidget::get_sat_light_thumb_y(&self.self_ref.upgrade().unwrap())).wrapping_add(thumb_height.wrapping_div(2))).wrapping_add(1), self.sat_light_thumb_x.wrapping_add(thumb_width.wrapping_div(2)), (ColorPickerWidget::get_sat_light_thumb_y(&self.self_ref.upgrade().unwrap())).wrapping_sub(thumb_height.wrapping_div(2)), -1);
        // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
        //   java: graphics.fill(hueGradientDim.x() - outline, hueGradientDim.y() - outline, hueGradientDim.xLimit() + outline, hueGradientDim.yLimit() + outline, Color.black.getRGB());
        ({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.draw_rainbow_gradient()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>), graphics, self.hue_gradient_dim.as_ref().unwrap().lock().borrow().x(), self.hue_gradient_dim.as_ref().unwrap().lock().borrow().y(), self.hue_gradient_dim.as_ref().unwrap().lock().borrow().x_limit(), self.hue_gradient_dim.as_ref().unwrap().lock().borrow().y_limit()) });
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (self.hue_thumb_x.wrapping_sub(thumb_width.wrapping_div(2))).wrapping_sub(1), ((self.hue_gradient_dim.as_ref().unwrap().lock().borrow().y()).wrapping_sub(self.outline)).wrapping_sub(1), (self.hue_thumb_x.wrapping_add(thumb_width.wrapping_div(2))).wrapping_add(1), ((self.hue_gradient_dim.as_ref().unwrap().lock().borrow().y_limit()).wrapping_add(self.outline)).wrapping_add(1), 0xFF404040u32 as i32);
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.hue_thumb_x.wrapping_sub(thumb_width.wrapping_div(2)), (self.hue_gradient_dim.as_ref().unwrap().lock().borrow().y()).wrapping_sub(self.outline), self.hue_thumb_x.wrapping_add(thumb_width.wrapping_div(2)), (self.hue_gradient_dim.as_ref().unwrap().lock().borrow().y_limit()).wrapping_add(self.outline), -1);
        if self.controller.allow_alpha() {
            // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
            //   java: graphics.fill(alphaGradientDim.x() - outline, alphaGradientDim.y() - outline, alphaGradientDim.xLimit() + outline, alphaGradientDim.yLimit() + outline, Color.black.getRGB());
            let mut x: i32 = self.alpha_gradient_dim.as_ref().unwrap().lock().borrow().x();
            let mut y: i32 = self.alpha_gradient_dim.as_ref().unwrap().lock().borrow().y();
            let mut width: i32 = self.alpha_gradient_dim.as_ref().unwrap().lock().borrow().width();
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), TRANSPARENT_SPRITE.get_mut(), x, y, width, self.slider_height);
            ({ let __r = &ColorGradientRenderState::create_horizontal(graphics, self.alpha_gradient_dim.as_ref().unwrap().lock().borrow().x(), self.alpha_gradient_dim.as_ref().unwrap().lock().borrow().y(), self.alpha_gradient_dim.as_ref().unwrap().lock().borrow().x_limit(), self.alpha_gradient_dim.as_ref().unwrap().lock().borrow().y_limit(), ((ColorPickerWidget::get_rgb_without_alpha(&self.self_ref.upgrade().unwrap()) & 0x00FFFFFF) | 0xFF000000u32 as i32), 0x00000000); let __f = compat::dispatch(__r, |__t| __t.submit()); __f(&compat::view(__r, |a| a as compat::Shared<dyn YACLGuiElementRenderState>), graphics) });
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (self.alpha_thumb_x.wrapping_sub(thumb_width.wrapping_div(2))).wrapping_sub(1), ((self.alpha_gradient_dim.as_ref().unwrap().lock().borrow().y()).wrapping_sub(self.outline)).wrapping_sub(1), (self.alpha_thumb_x.wrapping_add(thumb_width.wrapping_div(2))).wrapping_add(1), ((self.alpha_gradient_dim.as_ref().unwrap().lock().borrow().y_limit()).wrapping_add(self.outline)).wrapping_add(1), 0xFF404040u32 as i32);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.alpha_thumb_x.wrapping_sub(thumb_width.wrapping_div(2)), (self.alpha_gradient_dim.as_ref().unwrap().lock().borrow().y()).wrapping_sub(self.outline), self.alpha_thumb_x.wrapping_add(thumb_width.wrapping_div(2)), (self.alpha_gradient_dim.as_ref().unwrap().lock().borrow().y_limit()).wrapping_add(self.outline), -1);
        }
        gui_graphics_extractor::pop_pose(__pose_1);
        if ColorPickerWidget::is_hovering_hue_slider(&self.self_ref.upgrade().unwrap(), (mouse_x) as f64, (mouse_y) as f64) {
            // TODO(fabric2seer): static `CursorTypes.RESIZE_EW` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&CursorTypes::RESIZE_EW);
        } else if ColorPickerWidget::is_hovering_alpha_slider(&self.self_ref.upgrade().unwrap(), (mouse_x) as f64, (mouse_y) as f64) {
            // TODO(fabric2seer): static `CursorTypes.RESIZE_EW` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&CursorTypes::RESIZE_EW);
        } else if ColorPickerWidget::is_hovering_sat_light_gradient(&self.self_ref.upgrade().unwrap(), (mouse_x) as f64, (mouse_y) as f64) {
            // TODO(fabric2seer): static `CursorTypes.CROSSHAIR` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&CursorTypes::CROSSHAIR);
        }
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        return 0;
    }

}

impl AbstractWidget for ColorPickerWidget {
    fn abstract_widget_base(&self) -> &AbstractWidgetBase {
        &self.base.base.base
    }
    fn abstract_widget_base_mut(&mut self) -> &mut AbstractWidgetBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY) (dispatch: the body for this class)
    fn is_mouse_over(&self) -> fn(&compat::Shared<dyn AbstractWidget>, f64, f64) -> bool {
        ColorPickerWidget::is_mouse_over_dyn
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim) (dispatch: the body for this class)
    fn set_dimension(&self) -> fn(&compat::Shared<dyn AbstractWidget>, compat::Shared<dyn Dimension<i32>>) {
        ColorPickerWidget::set_dimension_dyn
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

impl ColorPickerWidget {
    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    pub fn is_mouse_over_impl(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return (mouse_x >= (((__this.lock().borrow().color_picker_dim.as_ref().unwrap().x()).wrapping_sub(compat::read(__this, |__t| __t.outline))).wrapping_sub(3) as f64) && mouse_x <= (((__this.lock().borrow().color_picker_dim.as_ref().unwrap().x_limit()).wrapping_add(compat::read(__this, |__t| __t.outline))).wrapping_add(3) as f64) && mouse_y >= (((__this.lock().borrow().color_picker_dim.as_ref().unwrap().y()).wrapping_sub(compat::read(__this, |__t| __t.outline))).wrapping_sub(3) as f64) && mouse_y <= (((__this.lock().borrow().color_picker_dim.as_ref().unwrap().y_limit()).wrapping_add(compat::read(__this, |__t| __t.outline))).wrapping_add(3) as f64));
    }

    /// ColorPickerWidget.isMouseOver as the dispatch of AbstractWidget.isMouseOver
    pub fn is_mouse_over_dyn(__this: &compat::Shared<dyn AbstractWidget>, __p0: f64, __p1: f64) -> bool {
        Self::is_mouse_over_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim)
    pub fn set_dimension_impl(__this: &compat::Shared<Self>, mut dim: compat::Shared<dyn Dimension<i32>>) {
        <dyn AbstractWidget>::set_dimension_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), dim.clone());
        let mut color_picker_height: i32 = ((dim.lock().borrow().height()).wrapping_mul(2)).wrapping_add(7);
        let mut color_picker_x: i32 = (dim.lock().borrow().center_x()).wrapping_sub((__this.lock().borrow().get_x_padding()).wrapping_mul(2));
        let mut color_picker_y: i32 = ((dim.lock().borrow().y()).wrapping_sub(color_picker_height)).wrapping_sub(compat::read(__this, |__t| __t.slider_height));
        let mut alpha_slider_height: i32 = 0;
        if __this.lock().borrow().controller.allow_alpha() {
            alpha_slider_height = ((compat::read(__this, |__t| __t.slider_height)).wrapping_add(compat::read(__this, |__t| __t.outline))).wrapping_add(compat::read(__this, |__t| __t.padding_y));
            color_picker_height = color_picker_height.wrapping_add(alpha_slider_height);
            color_picker_y = color_picker_y.wrapping_sub(alpha_slider_height);
        }
        if color_picker_y < compat::read(__this, |__t| __t.screen.clone()).lock().borrow().tab_area.as_ref().unwrap().top() {
            color_picker_y = (dim.lock().borrow().y_limit()).wrapping_add(compat::read(__this, |__t| __t.slider_height));
        }
        { let __v = Some(dimension::of_int(color_picker_x, color_picker_y, (dim.lock().borrow().x_limit()).wrapping_sub(color_picker_x), color_picker_height)); compat::write(__this, |__t| __t.color_picker_dim = __v); }
        { let __v = Some(dimension::of_int(__this.lock().borrow().color_picker_dim.as_ref().unwrap().x(), __this.lock().borrow().color_picker_dim.as_ref().unwrap().y(), (((__this.lock().borrow().color_picker_dim.as_ref().unwrap().x()).wrapping_add((__this.lock().borrow().color_picker_dim.as_ref().unwrap().x_limit()).wrapping_div(compat::read(__this, |__t| __t.preview_portion)))).wrapping_sub(compat::read(__this, |__t| __t.padding_x))).wrapping_sub(__this.lock().borrow().color_picker_dim.as_ref().unwrap().x()), ((((__this.lock().borrow().color_picker_dim.as_ref().unwrap().y_limit()).wrapping_sub(compat::read(__this, |__t| __t.slider_height))).wrapping_sub(compat::read(__this, |__t| __t.padding_y))).wrapping_sub(__this.lock().borrow().color_picker_dim.as_ref().unwrap().y())).wrapping_sub(alpha_slider_height))); compat::write(__this, |__t| __t.preview_color_dim = __v); }
        { let __v = Some(dimension::of_int((((__this.lock().borrow().color_picker_dim.as_ref().unwrap().x()).wrapping_add((__this.lock().borrow().color_picker_dim.as_ref().unwrap().x_limit()).wrapping_div(compat::read(__this, |__t| __t.preview_portion)))).wrapping_add(compat::read(__this, |__t| __t.padding_x))).wrapping_add(1), __this.lock().borrow().color_picker_dim.as_ref().unwrap().y(), (__this.lock().borrow().color_picker_dim.as_ref().unwrap().x_limit()).wrapping_sub((((__this.lock().borrow().color_picker_dim.as_ref().unwrap().x()).wrapping_add((__this.lock().borrow().color_picker_dim.as_ref().unwrap().x_limit()).wrapping_div(compat::read(__this, |__t| __t.preview_portion)))).wrapping_add(compat::read(__this, |__t| __t.padding_x))).wrapping_add(1)), ((((__this.lock().borrow().color_picker_dim.as_ref().unwrap().y_limit()).wrapping_sub(compat::read(__this, |__t| __t.slider_height))).wrapping_sub(compat::read(__this, |__t| __t.padding_y))).wrapping_sub(__this.lock().borrow().color_picker_dim.as_ref().unwrap().y())).wrapping_sub(alpha_slider_height))); compat::write(__this, |__t| __t.saturation_light_dim = __v); }
        { let __v = Some(dimension::of_int(__this.lock().borrow().color_picker_dim.as_ref().unwrap().x(), ((__this.lock().borrow().color_picker_dim.as_ref().unwrap().y_limit()).wrapping_sub(compat::read(__this, |__t| __t.slider_height))).wrapping_sub(alpha_slider_height), __this.lock().borrow().color_picker_dim.as_ref().unwrap().width(), compat::read(__this, |__t| __t.slider_height))); compat::write(__this, |__t| __t.hue_gradient_dim = __v); }
        if __this.lock().borrow().controller.allow_alpha() {
            { let __v = Some(dimension::of_int(__this.lock().borrow().hue_gradient_dim.as_ref().unwrap().x(), (__this.lock().borrow().hue_gradient_dim.as_ref().unwrap().y()).wrapping_add(alpha_slider_height), __this.lock().borrow().hue_gradient_dim.as_ref().unwrap().width(), compat::read(__this, |__t| __t.slider_height))); compat::write(__this, |__t| __t.alpha_gradient_dim = __v); }
        }
    }

    /// ColorPickerWidget.setDimension as the dispatch of AbstractWidget.setDimension
    pub fn set_dimension_dyn(__this: &compat::Shared<dyn AbstractWidget>, __p0: compat::Shared<dyn Dimension<i32>>) {
        Self::set_dimension_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
