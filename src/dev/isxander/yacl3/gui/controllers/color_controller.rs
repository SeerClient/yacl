//! Ported from dev.isxander.yacl3.gui.controllers.ColorController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::{self, Dimension};
use crate::dev::isxander::yacl3::api::utils::mutable_dimension::MutableDimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::color_picker_widget::{self, ColorPickerWidget};
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::{
    self as string_element, StringControllerElement, StringControllerElementClass,
};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::platform::yacl_config::HANDLER;
use compat::{CursorKind, Graphics};
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::argb;
use std::cell::{Cell, RefCell};
use std::sync::Weak;

/// java: public record ColorController(Option<Color> option, boolean allowAlpha) implements IStringController<Color>
///
/// A color controller that uses a hex color field as input.
pub struct ColorController {
    /// java: Option<Color> option
    pub option: Arc<dyn YOption<Color>>,
    /// java: boolean allowAlpha
    pub allow_alpha: bool,
}

impl ColorController {
    /// java: public ColorController(Option<Color> option) - `allowAlpha` defaulting to false
    pub fn new(option: Arc<dyn YOption<Color>>) -> Self {
        Self { option, allow_alpha: false }
    }

    /// java: public ColorController (the canonical record constructor)
    pub fn new_1(option: Arc<dyn YOption<Color>>, allow_alpha: bool) -> Arc<dyn Controller<Color>> {
        Arc::new(Self { option, allow_alpha })
    }

    /// java: the record accessor `allowAlpha()`
    pub fn allow_alpha(&self) -> bool {
        self.allow_alpha
    }

    /// java: private String toHex(int value)
    fn to_hex(&self, value: i32) -> String {
        let mut hex = compat::integer_to_string_radix(value, 16).to_uppercase();
        if compat::str_len(&hex) == 1 {
            hex = format!("0{hex}");
        }
        hex
    }
}

impl Controller<Color> for ColorController {
    /// java: @Override public Option<Color> option()
    fn option(&self) -> Arc<dyn YOption<Color>> {
        self.option.clone()
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        let pending = self.option.pending_value();
        let mut text = component::literal("#");
        text = text.append(component::literal(&self.to_hex(pending.get_red())).with_style_format(ChatFormatting::Red));
        text = text.append(component::literal(&self.to_hex(pending.get_green())).with_style_format(ChatFormatting::Green));
        text = text.append(component::literal(&self.to_hex(pending.get_blue())).with_style_format(ChatFormatting::Blue));
        if self.allow_alpha() {
            text = text.append_str(&self.to_hex(pending.get_alpha()));
        }
        text
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        ColorControllerElement::new(self, screen, widget_dimension)
    }
}

impl IStringController<Color> for ColorController {
    /// java: @Override public String getString()
    fn get_string(&self) -> String {
        self.format_value().get_string()
    }

    /// java: @Override public void setFromString(String value)
    fn set_from_string(&self, value: &str) {
        let mut value = value.to_string();
        if value.starts_with('#') {
            value = compat::substring(&value, 1);
        }

        let parse = |from: i32, to: i32| -> i32 {
            let digits = compat::substring2(&value, from, to);
            i32::from_str_radix(&digits, 16).unwrap_or_else(|_| panic!("NumberFormatException: For input string: \"{digits}\""))
        };
        let red = parse(0, 2);
        let green = parse(2, 4);
        let blue = parse(4, 6);

        if self.allow_alpha() {
            let alpha = parse(6, 8);
            self.option.request_set(Color::new_with_alpha(red, green, blue, alpha));
        } else {
            self.option.request_set(Color::new(red, green, blue));
        }
    }
}

/// `StringBuilder.setCharAt(index, ch)` over char positions.
fn sb_set_char_at(builder: &mut String, index: i32, ch: char) {
    let mut chars: Vec<char> = builder.chars().collect();
    // StringIndexOutOfBoundsException for a bad index
    chars[index as usize] = ch;
    *builder = chars.into_iter().collect();
}

/// java: public static class ColorControllerElement extends StringControllerElement
pub struct ColorControllerElement {
    /// java: extends StringControllerElement
    pub string: StringControllerElement,
    /// `this`, for the colour picker it creates
    this: Weak<ColorControllerElement>,
    /// java: private final ColorController colorController;
    color_controller: Arc<ColorController>,
    /// java: private ColorPickerWidget colorPickerWidget;
    color_picker_widget: RefCell<Option<Arc<ColorPickerWidget>>>,
    /// java: protected MutableDimension<Integer> colorPreviewDim;
    pub color_preview_dim: RefCell<Option<Arc<dyn MutableDimension<i32>>>>,
    /// java: private final List<Character> allowedChars;
    allowed_chars: Vec<char>,
    /// java: public boolean hoveredOverColorPreview = false;
    pub hovered_over_color_preview: Cell<bool>,
    /// java: private boolean colorPickerVisible = false;
    color_picker_visible: Cell<bool>,
    /// java: private int previewOutlineFadeTicks = 0;
    preview_outline_fade_ticks: Cell<i32>,
}

impl ColorControllerElement {
    /// java: public ColorControllerElement(ColorController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(control: Arc<ColorController>, screen: &Arc<YACLScreen>, dim: Arc<dyn Dimension<i32>>) -> Arc<ColorControllerElement> {
        let element = Arc::new_cyclic(|this: &Weak<ColorControllerElement>| ColorControllerElement {
            // super(control, screen, dim, true);
            string: StringControllerElement::new::<Color, ColorController>(control.clone(), screen, dim.clone(), true),
            this: this.clone(),
            color_controller: control,
            color_picker_widget: RefCell::new(None),
            color_preview_dim: RefCell::new(None),
            allowed_chars: vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'],
            hovered_over_color_preview: Cell::new(false),
            color_picker_visible: Cell::new(false),
            preview_outline_fade_ticks: Cell::new(0),
        });
        // the super constructor's `setDimension(dim)` dispatches to this class's override
        StringControllerElementClass::set_dimension(&*element, dim);
        element
    }

    /// `colorPreviewDim`
    fn color_preview_dim(&self) -> Arc<dyn MutableDimension<i32>> {
        self.color_preview_dim.borrow().clone().expect("ColorControllerElement.colorPreviewDim before setDimension")
    }

    /// java: protected void setSelectionLength()
    fn set_selection_length(&self) {
        let s = &self.string;
        let caret_pos = s.caret_pos.get();
        s.selection_length.set(if caret_pos < compat::str_len(&s.input_field()) && caret_pos > 0 { 1 } else { 0 });
    }

    /// java: public boolean isMouseOverColorPreview(double mouseX, double mouseY)
    pub fn is_mouse_over_color_preview(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.color_preview_dim().is_point_inside(mouse_x as i32, mouse_y as i32)
    }

    /// java: public void createOrRemoveColorPicker()
    pub fn create_or_remove_color_picker(&self) {
        self.color_picker_visible.set(!self.color_picker_visible.get());

        if self.color_picker_visible.get() {
            let picker = self.create_color_picker();
            *self.color_picker_widget.borrow_mut() = Some(picker.clone());
            self.string.controller_widget.screen().add_popup_controller_widget(picker);
        } else {
            self.remove_color_picker();
        }
    }

    /// java: public Color getPreviewOutlineColor(boolean colorPreviewHovered)
    pub fn get_preview_outline_color(&self, color_preview_hovered: bool) -> Color {
        let outline_color = Color::from_rgb(0xFF000000u32 as i32);
        let highlighted_color = self.get_highlighted_outline_color();
        if !self.string.controller_widget.hovered.get() && !color_preview_hovered {
            self.preview_outline_fade_ticks.set(0);
            return outline_color;
        }

        let fade_in_ticks = 80;
        let fade_out_ticks = fade_in_ticks + 120;

        if color_preview_hovered {
            // white/light grey if the color preview is being hovered
            self.preview_outline_fade_ticks.set(0);
            return highlighted_color;
        } else if compat::read(&HANDLER.get().instance(), |config| config.show_color_picker_indicator) {
            let ticks = self.preview_outline_fade_ticks.get();
            if ticks <= fade_in_ticks {
                // fade to white
                return self.get_faded_color(outline_color, highlighted_color, ticks, fade_in_ticks);
            } else if ticks <= fade_out_ticks {
                // fade to black
                return self.get_faded_color(highlighted_color, outline_color, ticks - fade_in_ticks, fade_out_ticks - fade_in_ticks);
            }

            if ticks >= fade_in_ticks + fade_out_ticks + 10 {
                // reset fade
                self.preview_outline_fade_ticks.set(0);
            }
        }

        outline_color
    }

    /// java: private Color getFadedColor(Color original, Color fadeToColor, int fadeTick, int maxFadeTicks)
    fn get_faded_color(&self, original: Color, fade_to_color: Color, fade_tick: i32, max_fade_ticks: i32) -> Color {
        let red = fade_to_color.get_red() - original.get_red();
        let green = fade_to_color.get_green() - original.get_green();
        let blue = fade_to_color.get_blue() - original.get_blue();

        Color::new(
            original.get_red() + ((red * fade_tick) / max_fade_ticks),
            original.get_green() + ((green * fade_tick) / max_fade_ticks),
            original.get_blue() + ((blue * fade_tick) / max_fade_ticks),
        )
    }

    /// java: private Color getHighlightedOutlineColor()
    fn get_highlighted_outline_color(&self) -> Color {
        // Brightness detector in case a developer has their starting color bright. Makes the outline indicating to a
        // user that the mini color preview can be clicked a light grey rather than white. For reference, there is about
        // a 10 digit moving room in saturation and light.
        let pending_value = self.color_controller.option().pending_value();
        let hsl = Color::rgb_to_hsb(pending_value.get_red(), pending_value.get_green(), pending_value.get_blue(), None);
        let mut highlighted_color = Color::from_rgb(0xFFFFFFFFu32 as i32);
        if hsl[1] < 0.1 && hsl[2] > 0.9 {
            highlighted_color = Color::from_rgb(0xFFC6C6C6u32 as i32);
        }
        highlighted_color
    }

    /// java: public ColorPickerWidget colorPickerWidget()
    pub fn color_picker_widget(&self) -> Option<Arc<ColorPickerWidget>> {
        self.color_picker_widget.borrow().clone()
    }

    /// java: public boolean colorPickerVisible()
    pub fn color_picker_visible(&self) -> bool {
        self.color_picker_visible.get()
    }

    /// java: public ColorPickerWidget createColorPicker()
    pub fn create_color_picker(&self) -> Arc<ColorPickerWidget> {
        ColorPickerWidget::new(
            self.color_controller.clone(),
            &self.string.controller_widget.screen(),
            StringControllerElementClass::get_dimension(self),
            self.this.clone(),
        )
    }

    /// java: public void removeColorPicker()
    pub fn remove_color_picker(&self) {
        self.string.controller_widget.screen().clear_popup_controller_widget();
        self.color_picker_visible.set(false);
        *self.color_picker_widget.borrow_mut() = None;
        // set to false in favor of the manual checking here to be done
        self.hovered_over_color_preview.set(false);
    }
}

impl StringControllerElementClass for ColorControllerElement {
    fn string_element(&self) -> &StringControllerElement {
        &self.string
    }

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let cw = &self.string.controller_widget;
        cw.hovered.set(StringControllerElementClass::is_mouse_over(self, mouse_x as f64, mouse_y as f64));
        if StringControllerElementClass::is_hovered(self) {
            let preview = self.color_preview_dim();
            preview.r#move(-self.string.input_field_bounds().width() - 8, -2);
            preview.expand(4, 4);
            self.preview_outline_fade_ticks.set(self.preview_outline_fade_ticks.get() + 1);
            string_element::extract_value_text(self, graphics, mouse_x, mouse_y, a);
        }

        let preview = self.color_preview_dim();
        let preview_color = self.color_controller.option().pending_value().get_rgb();
        if argb::alpha(preview_color) < 255 {
            graphics.blit_sprite(&color_picker_widget::transparent_sprite(), preview.x(), preview.y(), preview.width(), preview.height());
        }
        graphics.fill(preview.x(), preview.y(), preview.x_limit(), preview.y_limit(), preview_color);

        let is_mouse_over_color_preview = self.is_mouse_over_color_preview(mouse_x as f64, mouse_y as f64);
        graphics.outline(
            preview.x(),
            preview.y(),
            preview.width(),
            preview.height(),
            self.get_preview_outline_color(self.hovered_over_color_preview.get() || is_mouse_over_color_preview).get_rgb(),
        );

        if is_mouse_over_color_preview {
            graphics.request_cursor(if StringControllerElementClass::is_available(self) {
                CursorKind::PointingHand
            } else {
                CursorKind::NotAllowed
            });
        }
    }

    /// java: @Override public void write(String string)
    fn write(&self, string: &str) {
        let mut string = string.to_string();
        if string.starts_with("0x") {
            string = compat::substring(&string, 2);
        }
        for chr in string.chars() {
            if !self.allowed_chars.contains(&compat::char_to_lower_case(chr)) {
                return;
            }
        }

        let s = &self.string;
        let caret_pos = s.caret_pos.get();
        if caret_pos == 0 {
            return;
        }

        let trimmed = compat::substring2(&string, 0, (compat::str_len(&s.input_field()) - caret_pos).min(compat::str_len(&string)));
        let trimmed_len = compat::str_len(&trimmed);
        if self.modify_input(&|builder: &mut String| string_element::sb_replace(builder, caret_pos, caret_pos + trimmed_len, &trimmed)) {
            s.caret_pos.set(s.caret_pos.get() + trimmed_len);
            self.set_selection_length();
            self.update_control();
        }
    }

    /// java: @Override protected void doBackspace()
    fn do_backspace(&self) {
        let s = &self.string;
        let caret_pos = s.caret_pos.get();
        if caret_pos > 1 && self.modify_input(&|builder: &mut String| sb_set_char_at(builder, caret_pos - 1, '0')) {
            s.caret_pos.set(s.caret_pos.get() - 1);
            self.update_control();
        }
    }

    /// java: @Override protected void doDelete()
    fn do_delete(&self) {
        let caret_pos = self.string.caret_pos.get();
        if caret_pos >= 1 && self.modify_input(&|builder: &mut String| sb_set_char_at(builder, caret_pos, '0')) {
            self.update_control();
        }
    }

    /// java: @Override protected boolean doCut()
    fn do_cut(&self) -> bool {
        false
    }

    /// java: @Override protected boolean doCopy()
    fn do_copy(&self) -> bool {
        false
    }

    /// java: @Override protected boolean doSelectAll()
    fn do_select_all(&self) -> bool {
        false
    }

    /// java: @Override protected int getDefaultCaretPos()
    fn get_default_caret_pos(&self) -> i32 {
        if self.color_controller.allow_alpha() { 3 } else { 1 }
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim)
    fn set_dimension(&self, dim: Arc<dyn Dimension<i32>>) {
        string_element::set_dimension(self, dim.clone());

        let preview_size = (dim.height() - StringControllerElementClass::get_y_padding(self) * 2) / 2;
        *self.color_preview_dim.borrow_mut() = Some(dimension::of_int(
            dim.x_limit() - StringControllerElementClass::get_x_padding(self) - preview_size,
            dim.center_y() - preview_size / 2,
            preview_size,
            preview_size,
        ));

        let picker = self.color_picker_widget.borrow().clone();
        if let Some(picker) = picker {
            ControllerWidget::set_dimension(&*picker, ControllerWidget::get_dimension(&*picker).with_y(StringControllerElementClass::get_dimension(self).y()));
            // checks if the color controller is being partially rendered offscreen
            let own = StringControllerElementClass::get_dimension(self);
            let tab_area = self.string.controller_widget.screen().tab_area();
            if own.y() < tab_area.top() || own.y_limit() > tab_area.bottom() {
                self.remove_color_picker();
            }
        }
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        let s = &self.string;
        let prev_selection_length = s.selection_length.get();
        s.selection_length.set(0);
        if string_element::key_pressed(self, event) {
            s.caret_pos.set(1.max(s.caret_pos.get()));
            self.set_selection_length();
            return true;
        } else {
            s.selection_length.set(prev_selection_length);
        }
        false
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked(&self, event: &MouseButtonEvent, _double_click: bool) -> bool {
        // Detects if the user has clicked the color preview
        if self.is_mouse_over_color_preview(event.x(), event.y()) {
            StringControllerElementClass::play_down_sound(self);
            self.create_or_remove_color_picker();
            let handler = HANDLER.get();
            if compat::read(&handler.instance(), |config| config.show_color_picker_indicator) {
                compat::write(&handler.instance(), |config| config.show_color_picker_indicator = false);
                handler.save();
            }
        }

        let s = &self.string;
        s.caret_pos.set(1.max(s.caret_pos.get()));
        self.set_selection_length();
        true
    }

    /// java: @Override public void unfocus()
    fn unfocus(&self) {
        if self.color_picker_visible.get() {
            self.remove_color_picker();
        }
        self.preview_outline_fade_ticks.set(0);
        string_element::unfocus(self);
    }
}

/// `ColorPickerWidget`'s view of the entry widget it was opened from.
impl ColorControllerElement {
    /// `entryWidget.isMouseOver(x, y)` - the element's `isMouseOver`
    pub fn entry_is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        StringControllerElementClass::is_mouse_over(self, mouse_x, mouse_y)
    }
}
