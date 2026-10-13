//! Ported from dev.isxander.yacl3.gui.controllers.ColorPickerWidget.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::{self, Dimension};
use crate::dev::isxander::yacl3::api::utils::mutable_dimension::MutableDimension;
use crate::dev::isxander::yacl3::gui::abstract_widget;
use crate::dev::isxander::yacl3::gui::controllers::color_controller::{ColorController, ColorControllerElement};
use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::{self, ControllerPopupWidget, ControllerPopupWidgetBase};
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::{ControllerWidget, ControllerWidgetBase};
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElementClass;
use crate::dev::isxander::yacl3::gui::render::color_gradient_render_state::ColorGradientRenderState;
use crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState as _;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use compat::{CursorKind, Graphics};
use mcrust::java::awt::color::{self as awt_color, Color};
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::mth;
use std::cell::{Cell, RefCell};
use std::sync::Weak;

/// java: public static final Identifier COLOR_PICKER_SPRITE = YACLPlatform.rl("controller/colorpicker");
pub fn color_picker_sprite() -> Identifier {
    YACLPlatform::rl("controller/colorpicker")
}

/// java: public static final Identifier TRANSPARENT_SPRITE = YACLPlatform.rl("controller/transparent");
pub fn transparent_sprite() -> Identifier {
    YACLPlatform::rl("controller/transparent")
}

type Dim = RefCell<Option<Arc<dyn MutableDimension<i32>>>>;

fn get(dim: &Dim) -> Arc<dyn MutableDimension<i32>> {
    dim.borrow().clone().expect("ColorPickerWidget dimension before setDimension")
}

/// java: public class ColorPickerWidget extends ControllerPopupWidget<ColorController>
pub struct ColorPickerWidget {
    /// java: extends ControllerPopupWidget<ColorController>
    pub popup: ControllerPopupWidgetBase,
    /// java: private final ColorController controller;
    controller: Arc<ColorController>,
    /// java: private final ColorController.ColorControllerElement entryWidget; (held weakly: the element holds this)
    entry_widget: Weak<ColorControllerElement>,
    /// java: protected MutableDimension<Integer> colorPickerDim;
    pub color_picker_dim: Dim,
    /// java: protected MutableDimension<Integer> previewColorDim;
    pub preview_color_dim: Dim,
    /// java: protected MutableDimension<Integer> saturationLightDim;
    pub saturation_light_dim: Dim,
    /// java: protected MutableDimension<Integer> hueGradientDim;
    pub hue_gradient_dim: Dim,
    /// java: protected MutableDimension<Integer> alphaGradientDim;
    pub alpha_gradient_dim: Dim,
    /// java: private boolean mouseDown;
    mouse_down: Cell<bool>,
    /// java: private boolean hueSliderDown;
    hue_slider_down: Cell<bool>,
    /// java: private boolean satLightGradientDown;
    sat_light_gradient_down: Cell<bool>,
    /// java: private boolean alphaSliderDown;
    alpha_slider_down: Cell<bool>,
    /// java: private int hueThumbX;
    hue_thumb_x: Cell<i32>,
    /// java: private int satLightThumbX;
    sat_light_thumb_x: Cell<i32>,
    /// java: private int alphaThumbX;
    alpha_thumb_x: Cell<i32>,
    /// java: private boolean charTyped;
    char_typed: Cell<bool>,
    /// java: private float[] HSL;
    hsl: Cell<[f32; 3]>,
    /// java: private float hue;
    hue: Cell<f32>,
    /// java: private float saturation;
    saturation: Cell<f32>,
    /// java: private float light;
    light: Cell<f32>,
    /// java: private int alpha;
    alpha: Cell<i32>,
}

impl ColorPickerWidget {
    /// java: private final int outline = 1; - the width of the outline between each color picker element (color
    /// preview, saturation/light gradient, hue gradient)
    const OUTLINE: i32 = 1;
    /// java: private final int previewPortion = 7; - the main color preview's portion of the color picker as a whole
    const PREVIEW_PORTION: i32 = 7;
    /// java: private final int sliderHeight = 7; - the height in pixels of the hue slider
    const SLIDER_HEIGHT: i32 = 7;
    /// java: private final int paddingX = 1; - the x padding between the color preview and saturation/light gradient
    const PADDING_X: i32 = 1;
    /// java: private final int paddingY = 3; - the y padding between the hue gradient and color preview & sat/light
    const PADDING_Y: i32 = 3;

    /// java: public ColorPickerWidget(ColorController control, YACLScreen screen, Dimension<Integer> dim,
    /// ColorController.ColorControllerElement entryWidget)
    pub fn new(
        control: Arc<ColorController>,
        screen: &Arc<YACLScreen>,
        dim: Arc<dyn Dimension<i32>>,
        entry_widget: Weak<ColorControllerElement>,
    ) -> Arc<ColorPickerWidget> {
        let entry_dyn: Weak<dyn ControllerWidget> = entry_widget.clone();
        let widget = Arc::new(ColorPickerWidget {
            popup: ControllerPopupWidgetBase::new::<Color, ColorController>(&control, screen, dim.clone(), entry_dyn),
            controller: control,
            entry_widget,
            color_picker_dim: RefCell::new(None),
            preview_color_dim: RefCell::new(None),
            saturation_light_dim: RefCell::new(None),
            hue_gradient_dim: RefCell::new(None),
            alpha_gradient_dim: RefCell::new(None),
            mouse_down: Cell::new(false),
            hue_slider_down: Cell::new(false),
            sat_light_gradient_down: Cell::new(false),
            alpha_slider_down: Cell::new(false),
            hue_thumb_x: Cell::new(0),
            sat_light_thumb_x: Cell::new(0),
            alpha_thumb_x: Cell::new(0),
            char_typed: Cell::new(false),
            hsl: Cell::new([0.0; 3]),
            hue: Cell::new(0.0),
            saturation: Cell::new(0.0),
            light: Cell::new(0.0),
            alpha: Cell::new(0),
        });
        ControllerWidget::set_dimension(&*widget, dim);
        widget.update_hsl();
        widget.set_thumb_x();
        widget
    }

    /// `entryWidget`
    fn entry(&self) -> Arc<ColorControllerElement> {
        self.entry_widget.upgrade().expect("a colour picker outlived its colour controller")
    }

    /// java: private boolean isHoveringHueSlider(double mouseX, double mouseY)
    fn is_hovering_hue_slider(&self, mouse_x: f64, mouse_y: f64) -> bool {
        let d = get(&self.hue_gradient_dim);
        mouse_y >= d.y() as f64 && mouse_y <= d.y_limit() as f64 && mouse_x >= d.x() as f64 && mouse_x <= d.x_limit() as f64
    }

    /// java: public boolean clickedHueSlider(double mouseX, double mouseY)
    pub fn clicked_hue_slider(&self, mouse_x: f64, mouse_y: f64) -> bool {
        if self.sat_light_gradient_down.get() || self.alpha_slider_down.get() {
            return false;
        }

        if self.is_hovering_hue_slider(mouse_x, mouse_y) {
            self.hue_slider_down.set(true);
        }

        if self.hue_slider_down.get() {
            let d = get(&self.hue_gradient_dim);
            self.hue_thumb_x.set(mth::clamp_f64(mouse_x, d.x() as f64, d.x_limit() as f64) as i32);
        }
        self.hue_slider_down.get()
    }

    /// java: private boolean isHoveringSatLightGradient(double mouseX, double mouseY)
    fn is_hovering_sat_light_gradient(&self, mouse_x: f64, mouse_y: f64) -> bool {
        let d = get(&self.saturation_light_dim);
        mouse_y >= d.y() as f64 && mouse_y <= d.y_limit() as f64 && mouse_x >= d.x() as f64 && mouse_x <= d.x_limit() as f64
    }

    /// java: public boolean clickedSatLightGradient(double mouseX, double mouseY)
    pub fn clicked_sat_light_gradient(&self, mouse_x: f64, mouse_y: f64) -> bool {
        if self.hue_slider_down.get() || self.alpha_slider_down.get() {
            return false;
        }

        if self.is_hovering_sat_light_gradient(mouse_x, mouse_y) {
            self.sat_light_gradient_down.set(true);
        }

        if self.sat_light_gradient_down.get() {
            let d = get(&self.saturation_light_dim);
            self.sat_light_thumb_x.set(mth::clamp_f64(mouse_x, d.x() as f64, d.x_limit() as f64) as i32);
        }
        self.sat_light_gradient_down.get()
    }

    /// java: private boolean isHoveringAlphaSlider(double mouseX, double mouseY)
    fn is_hovering_alpha_slider(&self, mouse_x: f64, mouse_y: f64) -> bool {
        let Some(d) = self.alpha_gradient_dim.borrow().clone() else {
            return false;
        };
        mouse_y >= d.y() as f64 && mouse_y <= d.y_limit() as f64 && mouse_x >= d.x() as f64 && mouse_x <= d.x_limit() as f64
    }

    /// java: public boolean clickedAlphaSlider(double mouseX, double mouseY)
    pub fn clicked_alpha_slider(&self, mouse_x: f64, mouse_y: f64) -> bool {
        if self.sat_light_gradient_down.get() || self.hue_slider_down.get() {
            return false;
        }

        if self.is_hovering_alpha_slider(mouse_x, mouse_y) {
            self.alpha_slider_down.set(true);
        }

        if self.alpha_slider_down.get() {
            let d = get(&self.alpha_gradient_dim);
            self.alpha_thumb_x.set(mth::clamp_f64(mouse_x, d.x() as f64, d.x_limit() as f64) as i32);
        }
        self.alpha_slider_down.get()
    }

    /// java: public void setColorFromMouseClick(double mouseX, double mouseY)
    pub fn set_color_from_mouse_click(&self, mouse_x: f64, mouse_y: f64) {
        if self.clicked_sat_light_gradient(mouse_x, mouse_y) {
            self.set_sat_light_from_mouse(mouse_x, mouse_y);
        } else if self.clicked_hue_slider(mouse_x, mouse_y) {
            self.set_hue_from_mouse(mouse_x);
        } else if self.controller.allow_alpha() && self.clicked_alpha_slider(mouse_x, mouse_y) {
            self.set_alpha_from_mouse(mouse_x);
        }
    }

    /// java: public void setThumbX()
    pub fn set_thumb_x(&self) {
        // Sets the thumb x for both hue and sat/light
        self.hue_thumb_x.set(self.get_hue_thumb_x());
        self.sat_light_thumb_x.set(self.get_sat_light_thumb_x());
        if self.controller.allow_alpha() {
            self.alpha_thumb_x.set(self.get_alpha_thumb_x());
        }
    }

    /// java: protected int getHueThumbX()
    pub fn get_hue_thumb_x(&self) -> i32 {
        let d = get(&self.hue_gradient_dim);
        let min = d.x();
        let max = d.x_limit();
        let value = (min as f32 + d.width() as f32 * self.hue.get()) as i32;
        mth::clamp_i32(value, min, max)
    }

    /// java: protected int getSatLightThumbX()
    pub fn get_sat_light_thumb_x(&self) -> i32 {
        let d = get(&self.saturation_light_dim);
        let min = d.x();
        let max = d.x_limit();
        let value = (min as f32 + (d.width() as f32 * self.saturation.get())) as i32;
        mth::clamp_i32(value, min, max)
    }

    /// java: protected int getSatLightThumbY()
    pub fn get_sat_light_thumb_y(&self) -> i32 {
        let d = get(&self.saturation_light_dim);
        let min = d.y();
        let max = d.y_limit();
        let value = (min as f32 + (d.height() as f32 * (1.0 - self.light.get()))) as i32;
        mth::clamp_i32(value, min, max)
    }

    /// java: protected int getAlphaThumbX()
    pub fn get_alpha_thumb_x(&self) -> i32 {
        let d = get(&self.alpha_gradient_dim);
        let min = d.x();
        let max = d.x_limit();
        let value = max - (d.width() * self.alpha.get() / 255);
        mth::clamp_i32(value, min, max)
    }

    /// java: public void setHueFromMouse(double mouseX)
    pub fn set_hue_from_mouse(&self, mouse_x: f64) {
        // Changes the hue of the pending color based on the mouseX's pos. relative to the colorPickerDim's x/xLimit
        let d = get(&self.hue_gradient_dim);
        if mouse_x < d.x() as f64 {
            self.hue.set(0.0);
        } else if mouse_x > d.x_limit() as f64 {
            self.hue.set(1.0);
        } else {
            let new_hue = (mouse_x - d.x() as f64) as f32 / d.width() as f32;
            self.hue.set(mth::clamp_f32(new_hue, 0.0, 1.0));
        }

        self.set_color_controller_from_hsl();
    }

    /// java: public void setSatLightFromMouse(double mouseX, double mouseY)
    pub fn set_sat_light_from_mouse(&self, mouse_x: f64, mouse_y: f64) {
        let d = get(&self.saturation_light_dim);
        if mouse_x < d.x() as f64 {
            self.saturation.set(0.0);
        } else if mouse_x > d.x_limit() as f64 {
            self.saturation.set(1.0);
        } else {
            let new_sat = (mouse_x - d.x() as f64) as f32 / d.width() as f32;
            self.saturation.set(mth::clamp_f32(new_sat, 0.0, 1.0));
        }

        if mouse_y < d.y() as f64 {
            self.light.set(1.0);
        } else if mouse_y > d.y_limit() as f64 {
            self.light.set(0.0);
        } else {
            let new_light = (mouse_y - d.y() as f64) as f32 / d.height() as f32;
            self.light.set(mth::clamp_f32(1.0 - new_light, 0.0, 1.0));
        }

        self.set_color_controller_from_hsl();
    }

    /// java: public void setAlphaFromMouse(double mouseX)
    pub fn set_alpha_from_mouse(&self, mouse_x: f64) {
        // Changes the alpha of the pending color based on the mouseX's pos.
        let d = get(&self.alpha_gradient_dim);
        if mouse_x < d.x() as f64 {
            self.alpha.set(255);
        } else if mouse_x > d.x_limit() as f64 {
            self.alpha.set(0);
        } else {
            let new_alpha = ((mouse_x - d.x_limit() as f64) / d.width() as f64 * -255.0) as i32;
            self.alpha.set(mth::clamp_i32(new_alpha, 0, 255));
        }

        self.set_color_controller_from_hsl();
    }

    /// java: public void setColorControllerFromHSL()
    pub fn set_color_controller_from_hsl(&self) {
        // Updates the current color controller's pending value based from HSL to RGB
        let picker = get(&self.color_picker_dim);
        let true_hue = (self.hue_thumb_x.get() - picker.x()) as f32 / picker.width() as f32;
        let color = Color::get_hsb_color(true_hue, self.saturation.get(), self.light.get());
        let return_color = Color::new_with_alpha(color.get_red(), color.get_green(), color.get_blue(), self.alpha.get());
        self.controller.option().request_set(return_color);
    }

    /// java: protected void updateHSL()
    pub fn update_hsl(&self) {
        self.hsl.set(self.get_hsl());
        self.hue.set(self.hue());
        self.saturation.set(self.saturation());
        self.light.set(self.light());
        self.alpha.set(self.get_alpha());
        if self.char_typed.get() {
            self.set_thumb_x();
            self.char_typed.set(false);
        }
    }

    /// java: protected float[] getHSL()
    pub fn get_hsl(&self) -> [f32; 3] {
        let pending_value = self.controller.option().pending_value();
        Color::rgb_to_hsb(pending_value.get_red(), pending_value.get_green(), pending_value.get_blue(), None)
    }

    /// java: protected float hue() - gets the hue of the pending value
    pub fn hue(&self) -> f32 {
        self.hsl.get()[0]
    }

    /// java: protected float saturation() - gets the saturation of the pending value
    pub fn saturation(&self) -> f32 {
        self.hsl.get()[1]
    }

    /// java: protected float light() - gets the light/brightness/value of the pending value
    pub fn light(&self) -> f32 {
        self.hsl.get()[2]
    }

    /// java: protected int getAlpha()
    pub fn get_alpha(&self) -> i32 {
        self.controller.option().pending_value().get_alpha()
    }

    /// java: protected float getRgbFromHueX()
    pub fn get_rgb_from_hue_x(&self) -> f32 {
        let picker = get(&self.color_picker_dim);
        let true_hue = (self.hue_thumb_x.get() - picker.x()) as f32 / picker.width() as f32;
        Color::hsb_to_rgb(true_hue, 1.0, 1.0) as f32
    }

    /// java: protected int getRgbWithoutAlpha()
    pub fn get_rgb_without_alpha(&self) -> i32 {
        let pending_color = self.controller.option().pending_value();
        let return_color = Color::new_with_alpha(pending_color.get_red(), pending_color.get_green(), pending_color.get_blue(), 255);
        return_color.get_rgb()
    }
}

impl ControllerWidget for ColorPickerWidget {
    fn controller_widget(&self) -> &ControllerWidgetBase {
        &self.popup.cw
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        self.update_hsl();

        let thumb_width = 4;
        let thumb_height = 4;
        let outline = Self::OUTLINE;
        let black = awt_color::black().get_rgb();
        let picker = get(&self.color_picker_dim);
        let preview = get(&self.preview_color_dim);
        let sat_light = get(&self.saturation_light_dim);
        let hue = get(&self.hue_gradient_dim);

        graphics.push_pose();

        // Background
        graphics.blit_sprite(&color_picker_sprite(), picker.x() - 5, picker.y() - 5, picker.width() + 10, picker.height() + 10);

        // Main color preview
        // outline
        graphics.fill(preview.x() - outline, preview.y() - outline, preview.x_limit() + outline, preview.y_limit() + outline, black);
        // transparent texture - must be rendered BEFORE the main color preview
        if self.controller.allow_alpha() {
            graphics.blit_sprite(&transparent_sprite(), preview.x(), preview.y(), preview.width(), preview.height());
        }
        // main color preview
        graphics.fill(preview.x(), preview.y(), preview.x_limit(), preview.y_limit(), self.controller.option().pending_value().get_rgb());

        // Saturation/light gradient
        // outline
        graphics.fill(sat_light.x() - outline, sat_light.y() - outline, sat_light.x_limit() + outline, sat_light.y_limit() + outline, black);
        // White to pending color's RGB from hue, left to right
        ColorGradientRenderState::create_horizontal(
            &graphics.extractor,
            sat_light.x(),
            sat_light.y(),
            sat_light.x_limit(),
            sat_light.y_limit(),
            0xFFFFFFFFu32 as i32,
            ((self.get_rgb_from_hue_x() as i32) & 0x00FFFFFF) | 0xFF000000u32 as i32,
        )
        .submit(&graphics.extractor);
        // Transparent to black, top to bottom
        graphics.fill_gradient(sat_light.x(), sat_light.y(), sat_light.x_limit(), sat_light.y_limit(), 0x00000000, 0xFF000000u32 as i32);

        let sat_light_thumb_x = self.sat_light_thumb_x.get();
        let sat_light_thumb_y = self.get_sat_light_thumb_y();
        // Sat/light thumb shadow
        graphics.fill(
            sat_light_thumb_x - thumb_width / 2 - 2,
            sat_light_thumb_y + thumb_height / 2 + 2,
            sat_light_thumb_x + thumb_width / 2 + 1,
            sat_light_thumb_y - thumb_height / 2 - 1,
            0xFF404040u32 as i32,
        );
        // Sat/light thumb - extra 1 pixel on left and top to make it centered
        graphics.fill(
            sat_light_thumb_x - thumb_width / 2 - 1,
            sat_light_thumb_y + thumb_height / 2 + 1,
            sat_light_thumb_x + thumb_width / 2,
            sat_light_thumb_y - thumb_height / 2,
            -1,
        );

        // Hue gradient
        // outline
        graphics.fill(hue.x() - outline, hue.y() - outline, hue.x_limit() + outline, hue.y_limit() + outline, black);
        // Hue rainbow gradient
        abstract_widget::draw_rainbow_gradient(graphics, hue.x(), hue.y(), hue.x_limit(), hue.y_limit());

        let hue_thumb_x = self.hue_thumb_x.get();
        // Hue slider thumb shadow
        graphics.fill(
            hue_thumb_x - thumb_width / 2 - 1,
            hue.y() - outline - 1,
            hue_thumb_x + thumb_width / 2 + 1,
            hue.y_limit() + outline + 1,
            0xFF404040u32 as i32,
        );
        // Hue slider thumb
        graphics.fill(hue_thumb_x - thumb_width / 2, hue.y() - outline, hue_thumb_x + thumb_width / 2, hue.y_limit() + outline, -1);

        if self.controller.allow_alpha() {
            let alpha = get(&self.alpha_gradient_dim);
            // outline
            graphics.fill(alpha.x() - outline, alpha.y() - outline, alpha.x_limit() + outline, alpha.y_limit() + outline, black);
            // Transparent texture
            graphics.blit_sprite(&transparent_sprite(), alpha.x(), alpha.y(), alpha.width(), Self::SLIDER_HEIGHT);
            // Pending color to transparent
            ColorGradientRenderState::create_horizontal(
                &graphics.extractor,
                alpha.x(),
                alpha.y(),
                alpha.x_limit(),
                alpha.y_limit(),
                (self.get_rgb_without_alpha() & 0x00FFFFFF) | 0xFF000000u32 as i32,
                0x00000000,
            )
            .submit(&graphics.extractor);

            let alpha_thumb_x = self.alpha_thumb_x.get();
            // Alpha slider thumb shadow
            graphics.fill(
                alpha_thumb_x - thumb_width / 2 - 1,
                alpha.y() - outline - 1,
                alpha_thumb_x + thumb_width / 2 + 1,
                alpha.y_limit() + outline + 1,
                0xFF404040u32 as i32,
            );
            // Alpha slider thumb
            graphics.fill(alpha_thumb_x - thumb_width / 2, alpha.y() - outline, alpha_thumb_x + thumb_width / 2, alpha.y_limit() + outline, -1);
        }

        graphics.pop_pose();

        if self.is_hovering_hue_slider(mouse_x as f64, mouse_y as f64) {
            graphics.request_cursor(CursorKind::ResizeEw);
        } else if self.is_hovering_alpha_slider(mouse_x as f64, mouse_y as f64) {
            graphics.request_cursor(CursorKind::ResizeEw);
        } else if self.is_hovering_sat_light_gradient(mouse_x as f64, mouse_y as f64) {
            graphics.request_cursor(CursorKind::Crosshair);
        }
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        if ControllerWidget::is_mouse_over(self, event.x(), event.y()) {
            self.mouse_down.set(true);
            self.hue_slider_down.set(false);
            self.sat_light_gradient_down.set(false);
            self.alpha_slider_down.set(false);
            self.set_color_from_mouse_click(event.x(), event.y());
            return true;
        }
        let entry = self.entry();
        if entry.entry_is_mouse_over(event.x(), event.y()) {
            StringControllerElementClass::mouse_clicked(&*entry, event, double_click)
        } else {
            // removes color picker
            ControllerPopupWidget::close(self);
            false
        }
    }

    /// java: @Override public boolean mouseReleased(@NonNull MouseButtonEvent event)
    fn mouse_released(&self, _event: &MouseButtonEvent) -> bool {
        self.mouse_down.set(false);
        false
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        // Checks if the mouse is either over the color picker or the color controller. The addition/subtraction of the
        // outline and extra 3 pixels is to account for both the outline and the background
        let d = get(&self.color_picker_dim);
        let outline = Self::OUTLINE;
        mouse_x >= (d.x() - outline - 3) as f64
            && mouse_x <= (d.x_limit() + outline + 3) as f64
            && mouse_y >= (d.y() - outline - 3) as f64
            && mouse_y <= (d.y_limit() + outline + 3) as f64
    }

    /// java: @Override public boolean mouseDragged(@NonNull MouseButtonEvent event, double dx, double dy)
    fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        if self.mouse_down.get() || ControllerWidget::is_mouse_over(self, event.x(), event.y()) {
            self.set_color_from_mouse_click(event.x(), event.y());
            return true;
        }

        StringControllerElementClass::mouse_dragged(&*self.entry(), event, dx, dy)
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent event)
    fn char_typed(&self, event: &CharacterEvent) -> bool {
        // Done to allow for typing whilst the color picker is visible
        self.char_typed.set(true);
        StringControllerElementClass::char_typed(&*self.entry(), event)
    }

    /// java: ControllerPopupWidget.keyPressed(KeyEvent) - `entryWidget.keyPressed(event)`
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        controller_popup_widget::key_pressed(self, event)
    }

    /// java: ControllerPopupWidget.getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        controller_popup_widget::get_hovered_control_width(self)
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim)
    fn set_dimension(&self, dim: Arc<dyn Dimension<i32>>) {
        // super.setDimension(dim);
        self.popup.cw.base.set_dimension(dim.clone());

        let slider_height = Self::SLIDER_HEIGHT;
        let mut color_picker_height = (dim.height() * 2) + 7;
        let color_picker_x = dim.center_x() - self.get_x_padding() * 2;
        let mut color_picker_y = dim.y() - color_picker_height - slider_height;
        let mut alpha_slider_height = 0;
        if self.controller.allow_alpha() {
            alpha_slider_height = slider_height + Self::OUTLINE + Self::PADDING_Y;
            color_picker_height += alpha_slider_height;
            color_picker_y -= alpha_slider_height;
        }

        // Check if the color picker should be moved to beneath the controller. Add additional numbers after
        // colorPickerY to reduce the "strictness" of this detection
        if color_picker_y < self.popup.cw.screen().tab_area().top() {
            color_picker_y = dim.y_limit() + slider_height;
        }

        // A single dimension for the entire color picker as a whole. Division is used for the main color preview,
        // saturation/light picker, and hue slider to determine their dimensions
        let picker = dimension::of_int(color_picker_x, color_picker_y, dim.x_limit() - color_picker_x, color_picker_height);
        let portion = Self::PREVIEW_PORTION;
        let padding_x = Self::PADDING_X;
        let padding_y = Self::PADDING_Y;
        let preview = dimension::of_int(
            picker.x(),
            picker.y(),
            (picker.x() + (picker.x_limit() / portion) - padding_x) - picker.x(),
            (picker.y_limit() - slider_height - padding_y) - picker.y() - alpha_slider_height,
        );
        let sat_light = dimension::of_int(
            picker.x() + (picker.x_limit() / portion) + padding_x + 1,
            picker.y(),
            picker.x_limit() - (picker.x() + (picker.x_limit() / portion) + padding_x + 1),
            (picker.y_limit() - slider_height - padding_y) - picker.y() - alpha_slider_height,
        );
        let hue = dimension::of_int(picker.x(), picker.y_limit() - slider_height - alpha_slider_height, picker.width(), slider_height);
        if self.controller.allow_alpha() {
            *self.alpha_gradient_dim.borrow_mut() =
                Some(dimension::of_int(hue.x(), hue.y() + alpha_slider_height, hue.width(), slider_height));
        }
        *self.color_picker_dim.borrow_mut() = Some(picker);
        *self.preview_color_dim.borrow_mut() = Some(preview);
        *self.saturation_light_dim.borrow_mut() = Some(sat_light);
        *self.hue_gradient_dim.borrow_mut() = Some(hue);
    }
}

impl ControllerPopupWidget for ColorPickerWidget {
    fn controller_popup_widget(&self) -> &ControllerPopupWidgetBase {
        &self.popup
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor guiGraphics, int mouseX, int mouseY, float partialTick)
    fn extract_background(&self, _graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, _partial_tick: f32) {
        let entry = self.entry();
        entry.hovered_over_color_preview.set(entry.is_mouse_over_color_preview(mouse_x as f64, mouse_y as f64));
    }

    /// java: @Override public void close()
    fn close(&self) {
        self.entry().remove_color_picker();
    }

    /// java: @Override public Component popupTitle()
    fn popup_title(&self) -> MutableComponent {
        component::translatable("yacl.control.color.color_picker_title")
    }
}
