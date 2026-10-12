//! Ported from dev.isxander.yacl3.gui.controllers.ColorController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::api::utils::mutable_dimension::MutableDimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::color_picker_widget;
use crate::dev::isxander::yacl3::gui::controllers::color_picker_widget::ColorPickerWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::ControllerPopupWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElementBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::platform::yacl_config;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::argb;
use crate::dev::isxander::yacl3::api::option::Option as _;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public record ColorController(Option<Color> option, boolean allowAlpha) implements IStringController<Color>
#[derive(Clone, Debug)]
pub struct ColorController {
    /// java: Option<Color> option
    pub option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Color>>,
    /// java: boolean allowAlpha
    pub allow_alpha: bool,
}

impl ColorController {
    /// java: public ColorController(Option<Color> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Color>>) -> Self {
        let mut this = Self::new_canonical(option.clone(), false);
        this
    }

    /// java: the canonical record constructor
    pub fn new_canonical(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Color>>, mut allow_alpha: bool) -> Self {
        Self { option, allow_alpha }
    }

    /// java: the record accessor `allowAlpha()`
    pub fn allow_alpha(&self) -> bool {
        self.allow_alpha
    }

    /// java: private String toHex(int value)
    pub fn to_hex(&self, mut value: i32) -> String {
        // TODO(fabric2seer): Integer.toString
        //   java: String hex = Integer.toString(value, 16).toUpperCase();
        let mut hex: String = todo!("fabric2seer: the initialiser of `hex` (TODO above)");
        if compat::str_len(&hex) == 1 {
            hex = format!("0{}", hex);
        }
        return hex;
    }

}

impl IStringController<mcrust::java::awt::color::Color> for ColorController {
    /// java: @Override public String getString()
    fn get_string(&self) -> String {
        return self.format_value().get_string();
    }

    /// java: @Override public void setFromString(String value)
    fn set_from_string(&self, mut value: String) {
        if value.starts_with("#") {
            value = compat::substring(&value, 1);
        }
        let mut red: i32 = i32::from_str_radix(&(compat::substring2(&value, 0, 2)), (16) as u32).expect("NumberFormatException");
        let mut green: i32 = i32::from_str_radix(&(compat::substring2(&value, 2, 4)), (16) as u32).expect("NumberFormatException");
        let mut blue: i32 = i32::from_str_radix(&(compat::substring2(&value, 4, 6)), (16) as u32).expect("NumberFormatException");
        if self.allow_alpha() {
            let mut alpha: i32 = i32::from_str_radix(&(compat::substring2(&value, 6, 8)), (16) as u32).expect("NumberFormatException");
            { let (__a0,) = (Color::new_with_alpha(red, green, blue, alpha),); self.option().lock().borrow_mut().request_set(__a0) };
        } else {
            { let (__a0,) = (Color::new(red, green, blue),); self.option().lock().borrow_mut().request_set(__a0) };
        }
    }

}

impl Controller<compat::Object> for ColorController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<Color> option()
    fn option(&self) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Color>> {
        return self.option.clone();
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        let mut text: MutableComponent = component::literal("#");
        // TODO(fabric2seer): `.getRed(..)` on a T
        //   java: text.append(Component.literal(toHex(option().pendingValue().getRed())).withStyle(ChatFormatting.RED));
        // TODO(fabric2seer): `.getGreen(..)` on a T
        //   java: text.append(Component.literal(toHex(option().pendingValue().getGreen())).withStyle(ChatFormatting.GREEN));
        // TODO(fabric2seer): `.getBlue(..)` on a T
        //   java: text.append(Component.literal(toHex(option().pendingValue().getBlue())).withStyle(ChatFormatting.BLUE));
        if self.allow_alpha() {
            // TODO(fabric2seer): `.getAlpha(..)` on a T
            //   java: text.append(toHex(option().pendingValue().getAlpha()));
        }
        return text;
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        return (ColorControllerElement::new(self.clone(), screen.clone(), widget_dimension.clone()) as compat::Shared<dyn AbstractWidget>);
    }

}

/// java: public static class ColorControllerElement extends StringControllerElement
pub struct ColorControllerElement {
    /// java: extends StringControllerElement
    pub base: StringControllerElementBase,
    /// java: private final ColorController colorController;
    pub color_controller: ColorController,
    /// java: private ColorPickerWidget colorPickerWidget;
    pub color_picker_widget: Option<compat::Shared<ColorPickerWidget>>,
    /// java: protected MutableDimension<Integer> colorPreviewDim;
    pub color_preview_dim: Option<compat::Shared<dyn MutableDimension<i32>>>,
    /// java: private final List<Character> allowedChars;
    pub allowed_chars: Vec<char>,
    /// java: public boolean hoveredOverColorPreview = false;
    pub hovered_over_color_preview: bool,
    /// java: private boolean colorPickerVisible = false;
    pub color_picker_visible: bool,
    /// java: private int previewOutlineFadeTicks = 0;
    pub preview_outline_fade_ticks: i32,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for ColorControllerElement {
    type Target = StringControllerElementBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColorControllerElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColorControllerElement {
    /// java: public ColorControllerElement(ColorController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(mut control: ColorController, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>) -> compat::Shared<Self> {
        // NOTE(fabric2seer): `colorPickerWidget` is read by `setDimension`, which StringControllerElement's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `colorPreviewDim` is read by `setDimension`, which StringControllerElement's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `colorPickerVisible` is read by `setDimension`, which StringControllerElement's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `hoveredOverColorPreview` is read by `setDimension`, which StringControllerElement's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let __init_color_controller = control.clone();
        let __init_allowed_chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
        let (_, __this) = StringControllerElementBase::new_in((compat::shared(control.clone()) as compat::Shared<dyn IStringController<compat::Object>>), screen.clone(), dim.clone(), true, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                color_controller: __init_color_controller,
                color_picker_widget: None,
                color_preview_dim: None,
                allowed_chars: __init_allowed_chars,
                hovered_over_color_preview: false,
                color_picker_visible: false,
                preview_outline_fade_ticks: 0,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn StringControllerElement>), __h)
        });
        __this
    }

    /// java: protected void setSelectionLength()
    pub fn set_selection_length(__this: &compat::Shared<Self>) {
        { let __v = (if compat::read(__this, |__t| __t.caret_pos) < compat::str_len(&__this.lock().borrow().input_field) && compat::read(__this, |__t| __t.caret_pos) > 0 { 1 } else { 0 }); compat::write(__this, |__t| __t.selection_length = __v); }
    }

    /// java: public boolean isMouseOverColorPreview(double mouseX, double mouseY)
    pub fn is_mouse_over_color_preview(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return { let (__a0, __a1,) = (&((mouse_x) as i32), &((mouse_y) as i32),); __this.lock().borrow().color_preview_dim.as_ref().unwrap().is_point_inside(__a0, __a1) };
    }

    /// java: public void createOrRemoveColorPicker()
    pub fn create_or_remove_color_picker(__this: &compat::Shared<Self>) {
        { let __v = !(compat::read(__this, |__t| __t.color_picker_visible)); compat::write(__this, |__t| __t.color_picker_visible = __v); }
        if compat::read(__this, |__t| __t.color_picker_visible) {
            { let __v = Some(ColorControllerElement::create_color_picker(__this)); compat::write(__this, |__t| __t.color_picker_widget = __v); }
            YACLScreen::add_popup_controller_widget(&compat::read(__this, |__t| __t.screen.clone()), (compat::read(__this, |__t| __t.color_picker_widget.clone()) as compat::Shared<dyn ControllerPopupWidget<compat::Object>>));
        } else {
            ColorControllerElement::remove_color_picker(__this);
        }
    }

    /// java: public Color getPreviewOutlineColor(boolean colorPreviewHovered)
    pub fn get_preview_outline_color(__this: &compat::Shared<Self>, mut color_preview_hovered: bool) -> Color {
        // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 1
        // rust: let mut outline_color: Color = Color::new(0xFF000000u32 as i32);
        let mut outline_color: Color = todo!("fabric2seer: the initialiser of `outlineColor` (TODO above)");
        let mut highlighted_color: Color = ColorControllerElement::get_highlighted_outline_color(__this);
        if !(compat::read(__this, |__t| __t.hovered)) && !color_preview_hovered {
            { let __v = 0; compat::write(__this, |__t| __t.preview_outline_fade_ticks = __v); }
            return outline_color;
        }
        let mut fade_in_ticks: i32 = 80;
        let mut fade_out_ticks: i32 = fade_in_ticks.wrapping_add(120);
        // TODO(fabric2seer): field `showColorPickerIndicator` of a T
        //   java: if (colorPreviewHovered) {
        //   java:                 //white/light grey if the color preview is being hovered
        //   java:                 previewOutlineFadeTicks = 0;
        //   java:                 return highlightedColor;
        //   java:             } else if (YACLConfig.HANDLER.instance().showColorPickerIndicator) {
        //   java:                 if (previewOutlineFadeTicks <= fadeInTicks) {
        //   java:                     //fade to white
        //   java:                     return getFadedColor(outlineColor, highlightedColor, previewOutlineFadeTicks, fadeInTicks);
        //   java:                 } else if (previewOutlineFadeTicks <= fadeOutTicks) {
        //   java:                     //fade to black
        //   java:                     return getFadedColor(highlightedColor, outlineColor, previewOutlineFadeTicks - fadeInTicks, fadeOutTicks - fadeInTicks);
        //   java:                 }
        //   java:
        //   java:                 if (previewOutlineFadeTicks >= fadeInTicks + fadeOutTicks + 10) {
        //   java:                     //reset fade
        //   java:                     previewOutlineFadeTicks = 0;
        //   java:                 }
        //   java:             }
        return outline_color;
    }

    /// java: private Color getFadedColor(Color original, Color fadeToColor, int fadeTick, int maxFadeTicks)
    pub fn get_faded_color(__this: &compat::Shared<Self>, original: &Color, fade_to_color: &Color, mut fade_tick: i32, mut max_fade_ticks: i32) -> Color {
        // TODO(fabric2seer): `.getRed(..)` on a Color
        //   java: int red = fadeToColor.getRed() - original.getRed();
        let mut red: i32 = todo!("fabric2seer: the initialiser of `red` (TODO above)");
        // TODO(fabric2seer): `.getGreen(..)` on a Color
        //   java: int green = fadeToColor.getGreen() - original.getGreen();
        let mut green: i32 = todo!("fabric2seer: the initialiser of `green` (TODO above)");
        // TODO(fabric2seer): `.getBlue(..)` on a Color
        //   java: int blue = fadeToColor.getBlue() - original.getBlue();
        let mut blue: i32 = todo!("fabric2seer: the initialiser of `blue` (TODO above)");
        // TODO(fabric2seer): `.getRed(..)` on a Color
        //   java: return new Color(
        //   java:                     original.getRed() + ((red * fadeTick) / maxFadeTicks),
        //   java:                     original.getGreen() + ((green * fadeTick) / maxFadeTicks),
        //   java:                     original.getBlue() + ((blue * fadeTick) / maxFadeTicks)
        //   java:             );
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private Color getHighlightedOutlineColor()
    pub fn get_highlighted_outline_color(__this: &compat::Shared<Self>) -> Color {
        let mut pending_value: Color = __this.lock().borrow().color_controller.option().lock().borrow().pending_value();
        // TODO(fabric2seer): `.getRed(..)` on a Color
        //   java: float[] HSL = Color.RGBtoHSB(pendingValue.getRed(), pendingValue.getGreen(), pendingValue.getBlue(), null);
        let mut hsl: Vec<f32> = todo!("fabric2seer: the initialiser of `HSL` (TODO above)");
        // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 1
        // rust: let mut highlighted_color: Color = Color::new(0xFFFFFFFFu32 as i32);
        let mut highlighted_color: Color = todo!("fabric2seer: the initialiser of `highlightedColor` (TODO above)");
        if hsl[(1) as usize] < 0.1f32 && hsl[(2) as usize] > 0.9f32 {
            // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 1
            // rust: highlighted_color = Color::new(0xFFC6C6C6u32 as i32);
        }
        return highlighted_color;
    }

    /// java: public ColorPickerWidget colorPickerWidget()
    pub fn color_picker_widget(__this: &compat::Shared<Self>) -> compat::Shared<ColorPickerWidget> {
        return compat::read(__this, |__t| __t.color_picker_widget.clone()).unwrap();
    }

    /// java: public boolean colorPickerVisible()
    pub fn color_picker_visible(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.color_picker_visible);
    }

    /// java: public ColorPickerWidget createColorPicker()
    pub fn create_color_picker(__this: &compat::Shared<Self>) -> compat::Shared<ColorPickerWidget> {
        return ColorPickerWidget::new(compat::read(__this, |__t| __t.color_controller.clone()), compat::read(__this, |__t| __t.screen.clone()), ({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }), __this.clone());
    }

    /// java: public void removeColorPicker()
    pub fn remove_color_picker(__this: &compat::Shared<Self>) {
        YACLScreen::clear_popup_controller_widget(&compat::read(__this, |__t| __t.screen.clone()));
        { let __v = false; compat::write(__this, |__t| __t.color_picker_visible = __v); }
        { let __v = None; compat::write(__this, |__t| __t.color_picker_widget = __v); }
        { let __v = false; compat::write(__this, |__t| __t.hovered_over_color_preview = __v); }
    }

}

impl StringControllerElement for ColorControllerElement {
    fn string_controller_element_base(&self) -> &StringControllerElementBase {
        &self.base
    }
    fn string_controller_element_base_mut(&mut self) -> &mut StringControllerElementBase {
        &mut self.base
    }

    /// java: @Override public void write(String string) (dispatch: the body for this class)
    fn write(&self) -> fn(&compat::Shared<dyn StringControllerElement>, String) {
        ColorControllerElement::write_dyn
    }

    /// java: @Override protected void doBackspace() (dispatch: the body for this class)
    fn do_backspace(&self) -> fn(&compat::Shared<dyn StringControllerElement>) {
        ColorControllerElement::do_backspace_dyn
    }

    /// java: @Override protected void doDelete() (dispatch: the body for this class)
    fn do_delete(&self) -> fn(&compat::Shared<dyn StringControllerElement>) {
        ColorControllerElement::do_delete_dyn
    }

    /// java: @Override protected boolean doCut() (dispatch: the body for this class)
    fn do_cut(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> bool {
        ColorControllerElement::do_cut_dyn
    }

    /// java: @Override protected boolean doCopy() (dispatch: the body for this class)
    fn do_copy(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> bool {
        ColorControllerElement::do_copy_dyn
    }

    /// java: @Override protected boolean doSelectAll() (dispatch: the body for this class)
    fn do_select_all(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> bool {
        ColorControllerElement::do_select_all_dyn
    }

    /// java: @Override protected int getDefaultCaretPos() (dispatch: the body for this class)
    fn get_default_caret_pos(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> i32 {
        ColorControllerElement::get_default_caret_pos_dyn
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event) (dispatch: the body for this class)
    fn key_pressed(&self) -> fn(&compat::Shared<dyn StringControllerElement>, &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        ColorControllerElement::key_pressed_dyn
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn StringControllerElement>, &MouseButtonEvent, bool) -> bool {
        ColorControllerElement::mouse_clicked_dyn
    }

}

impl ControllerWidget<compat::Object> for ColorControllerElement {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<compat::Object> {
        &self.base.base
    }
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<compat::Object> {
        &mut self.base.base
    }

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        self.hovered = ({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.is_mouse_over()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>), (mouse_x) as f64, (mouse_y) as f64) });
        if self.is_hovered() {
            { let __s0 = ((-self.input_field_bounds.as_ref().unwrap().lock().borrow().width()).wrapping_sub(8)).clone(); self.color_preview_dim.as_mut().unwrap().lock().borrow_mut().r#move(&__s0, &-2) };
            self.color_preview_dim.as_mut().unwrap().lock().borrow_mut().expand(&4, &4);
            self.preview_outline_fade_ticks += 1;
            self.base.extract_value_text(graphics, mouse_x, mouse_y, a);
        }
        // TODO(fabric2seer): `.getRGB(..)` on a T
        //   java: int previewColor = colorController.option().pendingValue().getRGB();
        let mut preview_color: i32 = todo!("fabric2seer: the initialiser of `previewColor` (TODO above)");
        if argb::alpha(preview_color) < 255 {
            let mut x: i32 = self.color_preview_dim.as_ref().unwrap().lock().borrow().x();
            let mut y: i32 = self.color_preview_dim.as_ref().unwrap().lock().borrow().y();
            let mut width: i32 = self.color_preview_dim.as_ref().unwrap().lock().borrow().width();
            gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), color_picker_widget::TRANSPARENT_SPRITE.get_mut(), x, y, width, self.color_preview_dim.as_ref().unwrap().lock().borrow().height());
        }
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.color_preview_dim.as_ref().unwrap().lock().borrow().x(), self.color_preview_dim.as_ref().unwrap().lock().borrow().y(), self.color_preview_dim.as_ref().unwrap().lock().borrow().x_limit(), self.color_preview_dim.as_ref().unwrap().lock().borrow().y_limit(), preview_color);
        let mut is_mouse_over_color_preview: bool = ColorControllerElement::is_mouse_over_color_preview(&self.self_ref.upgrade().unwrap(), (mouse_x) as f64, (mouse_y) as f64);
        // TODO(fabric2seer): `.getRGB(..)` on a Color
        //   java: graphics.outline(
        //   java:                     colorPreviewDim.x(),
        //   java:                     colorPreviewDim.y(),
        //   java:                     colorPreviewDim.width(),
        //   java:                     colorPreviewDim.height(),
        //   java:                     getPreviewOutlineColor(hoveredOverColorPreview || isMouseOverColorPreview).getRGB()
        //   java:             );
        if is_mouse_over_color_preview {
            // TODO(fabric2seer): static `CursorTypes.POINTING_HAND` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): static `CursorTypes.NOT_ALLOWED` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&(if self.is_available() { CursorTypes::POINTING_HAND.clone() } else { CursorTypes::NOT_ALLOWED.clone() }));
        }
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        // TODO(fabric2seer): `getUnshiftedLength(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: return Math.min(textRenderer.width(getValueText()), getUnshiftedLength());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl AbstractWidget for ColorControllerElement {
    fn abstract_widget_base(&self) -> &AbstractWidgetBase {
        &self.base.base.base
    }
    fn abstract_widget_base_mut(&mut self) -> &mut AbstractWidgetBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim) (dispatch: the body for this class)
    fn set_dimension(&self) -> fn(&compat::Shared<dyn AbstractWidget>, compat::Shared<dyn Dimension<i32>>) {
        ColorControllerElement::set_dimension_dyn
    }

    /// java: @Override public void unfocus() (dispatch: the body for this class)
    fn unfocus(&self) -> fn(&compat::Shared<dyn AbstractWidget>) {
        ColorControllerElement::unfocus_dyn
    }

    /// java: @Override public boolean canReset() (dispatch: the body for this class)
    fn can_reset(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> bool {
        <dyn ControllerWidget>::can_reset_from::<Self>
    }

    /// java: @Override public NarrationPriority narrationPriority() (dispatch: the body for this class)
    fn narration_priority(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> crate::unresolved::NarrationPriority {
        <dyn ControllerWidget>::narration_priority_from::<Self>
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

impl ColorControllerElement {
    /// java: @Override public void write(String string)
    pub fn write_impl(__this: &compat::Shared<Self>, mut string: String) {
        if string.starts_with("0x") {
            string = compat::substring(&string, 2);
        }
        for chr in string.chars().collect::<Vec<char>>() {
            if !(__this.lock().borrow().allowed_chars.iter().any(|e| *e == compat::char_lower(chr))) {
                return;
            }
        }
        if compat::read(__this, |__t| __t.caret_pos) == 0 {
            return;
        }
        let mut trimmed: String = compat::substring2(&string, 0, ((compat::str_len(&__this.lock().borrow().input_field)).wrapping_sub(compat::read(__this, |__t| __t.caret_pos))).min(compat::str_len(&string)));
        // TODO(fabric2seer): the condition: StringBuilder.replace
        //   java: modifyInput(builder -> builder.replace(caretPos, caretPos + trimmed.length(), trimmed))
        if todo!("fabric2seer: the condition (TODO above)") {
            { let __v = (compat::read(__this, |__t| __t.caret_pos)).wrapping_add(compat::str_len(&trimmed)); compat::write(__this, |__t| __t.caret_pos = __v); }
            ColorControllerElement::set_selection_length(__this);
            ({ let __f = compat::dispatch(__this, |__t| __t.update_control()); __f(&compat::view(__this, |a| a as compat::Shared<dyn StringControllerElement>)) });
        }
    }

    /// ColorControllerElement.write as the dispatch of StringControllerElement.write
    pub fn write_dyn(__this: &compat::Shared<dyn StringControllerElement>, __p0: String) {
        Self::write_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override protected void doBackspace()
    pub fn do_backspace_impl(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.caret_pos) > 1 {
            // TODO(fabric2seer): the condition: StringBuilder.setCharAt
            //   java: modifyInput(builder -> builder.setCharAt(caretPos - 1, '0'))
            if todo!("fabric2seer: the condition (TODO above)") {
                { let __v = compat::read(__this, |__t| __t.caret_pos) - 1; compat::write(__this, |__t| __t.caret_pos = __v); }
                ({ let __f = compat::dispatch(__this, |__t| __t.update_control()); __f(&compat::view(__this, |a| a as compat::Shared<dyn StringControllerElement>)) });
            }
        }
    }

    /// ColorControllerElement.doBackspace as the dispatch of StringControllerElement.doBackspace
    pub fn do_backspace_dyn(__this: &compat::Shared<dyn StringControllerElement>) {
        Self::do_backspace_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void doDelete()
    pub fn do_delete_impl(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.caret_pos) >= 1 {
            // TODO(fabric2seer): the condition: StringBuilder.setCharAt
            //   java: modifyInput(builder -> builder.setCharAt(caretPos, '0'))
            if todo!("fabric2seer: the condition (TODO above)") {
                ({ let __f = compat::dispatch(__this, |__t| __t.update_control()); __f(&compat::view(__this, |a| a as compat::Shared<dyn StringControllerElement>)) });
            }
        }
    }

    /// ColorControllerElement.doDelete as the dispatch of StringControllerElement.doDelete
    pub fn do_delete_dyn(__this: &compat::Shared<dyn StringControllerElement>) {
        Self::do_delete_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected boolean doCut()
    pub fn do_cut_impl(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

    /// ColorControllerElement.doCut as the dispatch of StringControllerElement.doCut
    pub fn do_cut_dyn(__this: &compat::Shared<dyn StringControllerElement>) -> bool {
        Self::do_cut_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected boolean doCopy()
    pub fn do_copy_impl(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

    /// ColorControllerElement.doCopy as the dispatch of StringControllerElement.doCopy
    pub fn do_copy_dyn(__this: &compat::Shared<dyn StringControllerElement>) -> bool {
        Self::do_copy_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected boolean doSelectAll()
    pub fn do_select_all_impl(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

    /// ColorControllerElement.doSelectAll as the dispatch of StringControllerElement.doSelectAll
    pub fn do_select_all_dyn(__this: &compat::Shared<dyn StringControllerElement>) -> bool {
        Self::do_select_all_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected int getDefaultCaretPos()
    pub fn get_default_caret_pos_impl(__this: &compat::Shared<Self>) -> i32 {
        return (if __this.lock().borrow().color_controller.allow_alpha() { 3 } else { 1 });
    }

    /// ColorControllerElement.getDefaultCaretPos as the dispatch of StringControllerElement.getDefaultCaretPos
    pub fn get_default_caret_pos_dyn(__this: &compat::Shared<dyn StringControllerElement>) -> i32 {
        Self::get_default_caret_pos_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    pub fn key_pressed_impl(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        let mut prev_selection_length: i32 = compat::read(__this, |__t| __t.selection_length);
        { let __v = 0; compat::write(__this, |__t| __t.selection_length = __v); }
        if <dyn StringControllerElement>::key_pressed_body(&compat::view(__this, |a| a as compat::Shared<dyn StringControllerElement>), event) {
            { let __v = 1i32.max(compat::read(__this, |__t| __t.caret_pos)); compat::write(__this, |__t| __t.caret_pos = __v); }
            ColorControllerElement::set_selection_length(__this);
            return true;
        } else {
            { let __v = prev_selection_length; compat::write(__this, |__t| __t.selection_length = __v); }
        }
        return false;
    }

    /// ColorControllerElement.keyPressed as the dispatch of StringControllerElement.keyPressed
    pub fn key_pressed_dyn(__this: &compat::Shared<dyn StringControllerElement>, __p0: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        Self::key_pressed_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked_impl(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        if ColorControllerElement::is_mouse_over_color_preview(__this, event.x(), event.y()) {
            ({ let __f = compat::dispatch(__this, |__t| __t.play_down_sound()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) });
            ColorControllerElement::create_or_remove_color_picker(__this);
            // TODO(fabric2seer): the condition: field `showColorPickerIndicator` of a T
            //   java: YACLConfig.HANDLER.instance(..).showColorPickerIndicator
            if todo!("fabric2seer: the condition (TODO above)") {
                // TODO(fabric2seer): field `showColorPickerIndicator` of a T
                //   java: YACLConfig.HANDLER.instance().showColorPickerIndicator = false;
                yacl_config::HANDLER.get_mut().lock().borrow().save();
            }
        }
        { let __v = 1i32.max(compat::read(__this, |__t| __t.caret_pos)); compat::write(__this, |__t| __t.caret_pos = __v); }
        ColorControllerElement::set_selection_length(__this);
        return true;
    }

    /// ColorControllerElement.mouseClicked as the dispatch of StringControllerElement.mouseClicked
    pub fn mouse_clicked_dyn(__this: &compat::Shared<dyn StringControllerElement>, __p0: &MouseButtonEvent, __p1: bool) -> bool {
        Self::mouse_clicked_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim)
    pub fn set_dimension_impl(__this: &compat::Shared<Self>, mut dim: compat::Shared<dyn Dimension<i32>>) {
        <dyn StringControllerElement>::set_dimension_body(&compat::view(__this, |a| a as compat::Shared<dyn StringControllerElement>), dim.clone());
        let mut preview_size: i32 = ((dim.lock().borrow().height()).wrapping_sub((__this.lock().borrow().get_y_padding()).wrapping_mul(2))).wrapping_div(2);
        { let __v = Some(dimension::of_int(((dim.lock().borrow().x_limit()).wrapping_sub(__this.lock().borrow().get_x_padding())).wrapping_sub(preview_size), (dim.lock().borrow().center_y()).wrapping_sub(preview_size.wrapping_div(2)), preview_size, preview_size)); compat::write(__this, |__t| __t.color_preview_dim = __v); }
        if compat::read(__this, |__t| __t.color_picker_widget.clone()).is_some() {
            ({ let __r = &compat::read(__this, |__t| __t.color_picker_widget.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>), { let (__a0,) = (&(({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y(),); (({ let __r = &compat::read(__this, |__t| __t.color_picker_widget.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).with_y(__a0) }) });
            if (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y() < compat::read(__this, |__t| __t.screen.clone()).lock().borrow().tab_area.as_ref().unwrap().top() || (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y_limit() > compat::read(__this, |__t| __t.screen.clone()).lock().borrow().tab_area.as_ref().unwrap().bottom() {
                ColorControllerElement::remove_color_picker(__this);
            }
        }
    }

    /// ColorControllerElement.setDimension as the dispatch of AbstractWidget.setDimension
    pub fn set_dimension_dyn(__this: &compat::Shared<dyn AbstractWidget>, __p0: compat::Shared<dyn Dimension<i32>>) {
        Self::set_dimension_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void unfocus()
    pub fn unfocus_impl(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.color_picker_visible) {
            ColorControllerElement::remove_color_picker(__this);
        }
        { let __v = 0; compat::write(__this, |__t| __t.preview_outline_fade_ticks = __v); }
        <dyn StringControllerElement>::unfocus_body(&compat::view(__this, |a| a as compat::Shared<dyn StringControllerElement>));
    }

    /// ColorControllerElement.unfocus as the dispatch of AbstractWidget.unfocus
    pub fn unfocus_dyn(__this: &compat::Shared<dyn AbstractWidget>) {
        Self::unfocus_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
