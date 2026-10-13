//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.AbstractDropdownControllerElement.
//!
//! `public abstract class AbstractDropdownControllerElement<T, U> extends StringControllerElement`. A subclass holds an
//! `AbstractDropdownControllerElement<U>` (the class's fields, its `StringControllerElement` half inside) and implements
//! - `StringControllerElementClass`, forwarding the nine methods this class overrides (`mouse_clicked`, `set_focused`,
//!   `unfocus`, `key_pressed`, `char_typed`, `get_value_color`, `modify_input`, `extract_render_state`,
//!   `set_dimension`) to the functions below (Rust has no inheritance between the two class levels: PATTERNS_Y2.md P3);
//! - `AbstractDropdownControllerElementClass`, the methods this class adds.
//!
//! `DropdownWidget` reaches the element through `DropdownElement`, which every such class is.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::ControllerPopupWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::{self, ControllerWidget};
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::dropdown_widget::DropdownWidget;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::{
    self as string_element, Base, StringControllerElement, StringControllerElementClass,
};
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::Graphics;
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::cell::{Cell, RefCell};
use std::sync::Weak;

/// `AbstractDropdownController<T>` as the element holds it (`dropdownController`): the members it calls that do not
/// mention `T`, and the `DropdownWidget` it builds over the same controller.
#[derive(Clone)]
pub struct DropdownControl {
    is_value_valid: Rc<dyn Fn(&str) -> bool>,
    get_valid_value_2: Rc<dyn Fn(&str, i32) -> String>,
    popup_base: Rc<dyn Fn(&Arc<YACLScreen>, Arc<dyn Dimension<i32>>, Weak<dyn ControllerWidget>) -> ControllerPopupWidgetBase>,
}

impl DropdownControl {
    /// The controller seen through the wildcard.
    pub fn of<T: Clone + 'static, C: AbstractDropdownController<T> + ?Sized + 'static>(control: &Arc<C>) -> DropdownControl {
        let (a, b, c) = (control.clone(), control.clone(), control.clone());
        DropdownControl {
            is_value_valid: Rc::new(move |value| a.is_value_valid(value)),
            get_valid_value_2: Rc::new(move |value, offset| b.get_valid_value_2(value, offset)),
            popup_base: Rc::new(move |screen, dim, entry| ControllerPopupWidgetBase::new::<T, C>(&c, screen, dim, entry)),
        }
    }

    /// java: public boolean isValueValid(String value)
    pub fn is_value_valid(&self, value: &str) -> bool {
        (self.is_value_valid)(value)
    }

    /// java: protected String getValidValue(String value)
    pub fn get_valid_value(&self, value: &str) -> String {
        (self.get_valid_value_2)(value, 0)
    }

    /// java: protected String getValidValue(String value, int offset)
    pub fn get_valid_value_2(&self, value: &str, offset: i32) -> String {
        (self.get_valid_value_2)(value, offset)
    }

    /// `super(control, screen, dim, entryWidget)` of a `DropdownWidget` over this controller.
    pub fn popup_base(&self, screen: &Arc<YACLScreen>, dim: Arc<dyn Dimension<i32>>, entry: Weak<dyn ControllerWidget>) -> ControllerPopupWidgetBase {
        (self.popup_base)(screen, dim, entry)
    }
}

/// java: the fields of `AbstractDropdownControllerElement<T, U>`
pub struct AbstractDropdownControllerElement<U: 'static> {
    /// java: extends StringControllerElement
    pub string: StringControllerElement,
    /// java: private final AbstractDropdownController<T> dropdownController;
    pub dropdown_controller: DropdownControl,
    /// java: protected DropdownWidget<T> dropdownWidget;
    pub dropdown_widget: RefCell<Option<Arc<DropdownWidget>>>,
    /// java: protected boolean dropdownVisible = false;
    pub dropdown_visible: Cell<bool>,
    /// java: protected List<U> matchingValues = null; - stores a cached list of matching values
    pub matching_values: RefCell<Option<Vec<U>>>,
}

impl<U: 'static> AbstractDropdownControllerElement<U> {
    /// java: public AbstractDropdownControllerElement(AbstractDropdownController<T> control, YACLScreen screen,
    /// Dimension<Integer> dim)
    ///
    /// The constructor's `option.addListener((opt, val) -> this.matchingValues = this.computeMatchingValues())` needs
    /// `this`: the subclass registers it with `register_matching_values_listener` once it is built.
    pub fn new<T: Clone + 'static, C: AbstractDropdownController<T> + ?Sized + 'static>(
        control: Arc<C>,
        screen: &Arc<YACLScreen>,
        dim: Arc<dyn Dimension<i32>>,
    ) -> Self {
        Self {
            dropdown_controller: DropdownControl::of(&control),
            string: StringControllerElement::new::<T, C>(control, screen, dim, false),
            dropdown_widget: RefCell::new(None),
            dropdown_visible: Cell::new(false),
            matching_values: RefCell::new(None),
        }
    }

    /// `dropdownWidget`
    pub fn dropdown_widget(&self) -> Option<Arc<DropdownWidget>> {
        self.dropdown_widget.borrow().clone()
    }
}

/// java: `this.dropdownController.option.addListener((opt, val) -> this.matchingValues = this.computeMatchingValues())`
pub fn register_matching_values_listener<W: AbstractDropdownControllerElementClass + 'static>(this: &Arc<W>) {
    let weak = Arc::downgrade(this);
    this.dropdown_element().string.controller_widget.option.add_any_listener(Arc::new(move |_opt: &dyn AnyOption| {
        if let Some(this) = weak.upgrade() {
            let values = this.compute_matching_values();
            *this.dropdown_element().matching_values.borrow_mut() = Some(values);
        }
    }));
}

/// java: the methods `AbstractDropdownControllerElement<T, U>` adds, each defaulting to its body
pub trait AbstractDropdownControllerElementClass: StringControllerElementClass {
    /// `U`
    type Value: Clone + 'static;

    /// The class's fields.
    fn dropdown_element(&self) -> &AbstractDropdownControllerElement<Self::Value>;

    /// `this` as the `DropdownWidget` holds it (and as its popup's entry widget).
    fn this_dropdown(&self) -> (Weak<dyn DropdownElement>, Weak<dyn ControllerWidget>);

    /// java: public abstract List<U> computeMatchingValues();
    fn compute_matching_values(&self) -> Vec<Self::Value>;

    /// java: public abstract String getString(U object);
    fn get_string_of(&self, object: &Self::Value) -> String;

    /// java: protected void extractDropdownEntry(GuiGraphicsExtractor graphics, Dimension<Integer> entryDimension, U value)
    fn extract_dropdown_entry(&self, graphics: &Graphics<'_>, entry_dimension: &dyn Dimension<i32>, value: &Self::Value) {
        extract_dropdown_entry(self, graphics, entry_dimension, value)
    }

    /// java: protected int getTextY(Dimension<Integer> dim)
    fn get_text_y_of(&self, dim: &dyn Dimension<i32>) -> i32 {
        (dim.y() as f32 + dim.height() as f32 / 2.0 - mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32 / 2.0) as i32
    }

    /// java: public Component shortenString(String value)
    fn shorten_string(&self, value: &str) -> MutableComponent {
        let font = compat::font();
        component::literal(&GuiUtils::shorten_string(
            value.to_string(),
            &font,
            StringControllerElementClass::get_dimension(self).width() - 20,
            "...",
        ))
    }

    /// java: protected int getDecorationPadding() - `super.getXPadding()`, `ControllerWidget`'s body (a `super`
    /// call does not dispatch)
    fn get_decoration_padding(&self) -> i32 {
        5
    }

    /// java: protected int getDropdownEntryPadding()
    fn get_dropdown_entry_padding(&self) -> i32 {
        0
    }

    /// java: public boolean matchingValue(String value)
    fn matching_value(&self, value: &str) -> bool {
        value.to_lowercase().contains(&self.dropdown_element().string.input_field().to_lowercase())
    }

    /// java: public void ensureValidValue()
    fn ensure_valid_value(&self) {
        ensure_valid_value(self)
    }

    /// java: public void createDropdownWidget()
    fn create_dropdown_widget(&self) {
        create_dropdown_widget(self)
    }

    /// java: public void removeDropdownWidget()
    fn remove_dropdown_widget(&self) {
        remove_dropdown_widget(self)
    }
}

/// java: void extractDropdownEntry(GuiGraphicsExtractor graphics, Dimension<Integer> entryDimension, int index)
pub fn extract_dropdown_entry_at<W: AbstractDropdownControllerElementClass + ?Sized>(
    this: &W,
    graphics: &Graphics<'_>,
    entry_dimension: &dyn Dimension<i32>,
    index: i32,
) {
    let value = this.dropdown_element().matching_values.borrow().as_ref().and_then(|values| values.get(index as usize).cloned());
    // `matchingValues.get(index)` throws for an index past the end
    let value = value.expect("AbstractDropdownControllerElement.matchingValues.get(index) out of range");
    this.extract_dropdown_entry(graphics, entry_dimension, &value);
}

/// java: protected void extractDropdownEntry(GuiGraphicsExtractor graphics, Dimension<Integer> entryDimension, U value)
pub fn extract_dropdown_entry<W: AbstractDropdownControllerElementClass + ?Sized>(
    this: &W,
    graphics: &Graphics<'_>,
    entry_dimension: &dyn Dimension<i32>,
    value: &W::Value,
) {
    let entry = this.get_string_of(value);
    let text = if compat::is_blank(&entry) {
        component::translatable("yacl.control.text.blank").with_style_format(ChatFormatting::Gray)
    } else {
        this.shorten_string(&entry)
    };
    let width = compat::font().width_of(&FormattedText::from(text.clone()));
    graphics.text_component(
        &text,
        entry_dimension.x_limit() - width - this.get_dropdown_entry_padding(),
        this.get_text_y_of(entry_dimension),
        -1,
        true,
    );
}

/// java: public void ensureValidValue()
pub fn ensure_valid_value<W: AbstractDropdownControllerElementClass + ?Sized>(this: &W) {
    let d = this.dropdown_element();
    let input_field = d.string.input_field();
    if !d.dropdown_controller.is_value_valid(&input_field) {
        match d.dropdown_widget() {
            None => d.string.set_input_field(d.dropdown_controller.get_valid_value(&input_field)),
            Some(widget) => {
                d.string.set_input_field(d.dropdown_controller.get_valid_value_2(&input_field, widget.selected_index()));
                widget.reset_selected_index();
            }
        }
        d.string.caret_pos.set(this.get_default_caret_pos());
        let values = this.compute_matching_values();
        *d.matching_values.borrow_mut() = Some(values);
    }
}

/// java: public void createDropdownWidget()
pub fn create_dropdown_widget<W: AbstractDropdownControllerElementClass + ?Sized>(this: &W) {
    let d = this.dropdown_element();
    d.dropdown_visible.set(true);
    let screen = d.string.controller_widget.screen();
    let (element, entry) = this.this_dropdown();
    let widget = DropdownWidget::new(&d.dropdown_controller, &screen, StringControllerElementClass::get_dimension(this), element, entry);
    *d.dropdown_widget.borrow_mut() = Some(widget.clone());
    screen.add_popup_controller_widget(widget);
}

/// java: public void removeDropdownWidget()
pub fn remove_dropdown_widget<W: AbstractDropdownControllerElementClass + ?Sized>(this: &W) {
    this.ensure_valid_value();
    let d = this.dropdown_element();
    d.string.controller_widget.screen().clear_popup_controller_widget();
    d.dropdown_visible.set(false);
    *d.dropdown_widget.borrow_mut() = None;
}

// ---- the `StringControllerElement` methods this class overrides ---------------------------------------------------

/// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
pub fn mouse_clicked<W: AbstractDropdownControllerElementClass + ?Sized>(this: &W, event: &MouseButtonEvent, double_click: bool) -> bool {
    if string_element::mouse_clicked(this, event, double_click) {
        if !this.dropdown_element().dropdown_visible.get() {
            this.create_dropdown_widget();
            this.do_select_all();
        }
        return true;
    }
    false
}

/// java: @Override public void setFocused(boolean focused)
pub fn set_focused<W: AbstractDropdownControllerElementClass + ?Sized>(this: &W, focused: bool) {
    if focused {
        this.do_select_all();
        string_element::set_focused(this, true);
    } else {
        StringControllerElementClass::unfocus(this);
    }
}

/// java: @Override public void unfocus()
pub fn unfocus<W: AbstractDropdownControllerElementClass + ?Sized>(this: &W) {
    if this.dropdown_element().dropdown_visible.get() {
        this.remove_dropdown_widget();
    }
    string_element::unfocus(this);
}

/// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
pub fn key_pressed<W: AbstractDropdownControllerElementClass + ?Sized>(this: &W, event: &KeyEvent) -> bool {
    let d = this.dropdown_element();
    if !d.string.input_field_focused.get() {
        return false;
    }

    if d.dropdown_visible.get() {
        if let Some(widget) = d.dropdown_widget() {
            match event.key() {
                input_constants::KEY_DOWN => {
                    widget.select_next_entry();
                    return true;
                }
                input_constants::KEY_UP => {
                    widget.select_previous_entry();
                    return true;
                }
                input_constants::KEY_TAB => {
                    if event.has_shift_down() {
                        widget.select_previous_entry();
                    } else {
                        widget.select_next_entry();
                    }
                    return true;
                }
                _ => {}
            }
        }
    } else if event.key() == input_constants::KEY_RETURN || event.key() == input_constants::KEY_NUMPADENTER {
        this.create_dropdown_widget();
        return true;
    }

    string_element::key_pressed(this, event)
}

/// java: @Override public boolean charTyped(@NonNull CharacterEvent event)
pub fn char_typed<W: AbstractDropdownControllerElementClass + ?Sized>(this: &W, event: &CharacterEvent) -> bool {
    let d = this.dropdown_element();
    if !d.string.input_field_focused.get() {
        return false;
    }
    if !d.dropdown_visible.get() {
        this.create_dropdown_widget();
    }
    string_element::char_typed(this, event)
}

/// java: @Override protected int getValueColor()
pub fn get_value_color<W: AbstractDropdownControllerElementClass + ?Sized>(this: &W) -> i32 {
    let d = this.dropdown_element();
    if d.string.input_field_focused.get() && !d.dropdown_controller.is_value_valid(&d.string.input_field()) {
        return 0xFFF06080u32 as i32;
    }
    controller_widget::get_value_color(&Base(this))
}

/// java: @Override public boolean modifyInput(Consumer<StringBuilder> builder)
pub fn modify_input<W: AbstractDropdownControllerElementClass + ?Sized>(this: &W, builder: &dyn Fn(&mut String)) -> bool {
    let success = string_element::modify_input(this, builder);
    if success {
        let values = this.compute_matching_values();
        *this.dropdown_element().matching_values.borrow_mut() = Some(values);
    }
    success
}

/// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
pub fn extract_render_state<W: AbstractDropdownControllerElementClass + ?Sized>(this: &W, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
    if this.dropdown_element().matching_values.borrow().is_none() {
        let values = this.compute_matching_values();
        *this.dropdown_element().matching_values.borrow_mut() = Some(values);
    }
    controller_widget::extract_render_state(&Base(this), graphics, mouse_x, mouse_y, a);
}

/// java: @Override public void setDimension(Dimension<Integer> dim)
pub fn set_dimension<W: AbstractDropdownControllerElementClass + ?Sized>(this: &W, dim: Arc<dyn Dimension<i32>>) {
    string_element::set_dimension(this, dim);

    if let Some(widget) = this.dropdown_element().dropdown_widget() {
        let own = StringControllerElementClass::get_dimension(this);
        ControllerWidget::set_dimension(&*widget, ControllerWidget::get_dimension(&*widget).with_y(own.y()));
        // checks if the popup is being partially rendered offscreen
        let tab_area = this.dropdown_element().string.controller_widget.screen().tab_area();
        if own.y() < tab_area.top() || own.y_limit() > tab_area.bottom() {
            this.remove_dropdown_widget();
        }
    }
}

/// `AbstractDropdownControllerElement<T, ?>` as `DropdownWidget` holds it.
pub trait DropdownElement {
    /// `dropdownElement.matchingValues.size()`
    fn dropdown_length(&self) -> i32;
    /// `dropdownElement.extractDropdownEntry(graphics, entryDimension, i)`
    fn extract_dropdown_entry_at(&self, graphics: &Graphics<'_>, entry_dimension: &dyn Dimension<i32>, index: i32);
    /// `dropdownElement.getDecorationPadding()`
    fn decoration_padding(&self) -> i32;
    /// `dropdownElement.getDimension()`
    fn element_dimension(&self) -> Arc<dyn Dimension<i32>>;
    /// `dropdownElement.isMouseOver(x, y)`
    fn element_is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool;
    /// `dropdownElement.mouseClicked(event, doubleClick)`
    fn element_mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool;
    /// `dropdownElement.unfocus()`
    fn element_unfocus(&self);
    /// `dropdownElement.charTyped(event)`
    fn element_char_typed(&self, event: &CharacterEvent) -> bool;
    /// `dropdownElement.removeDropdownWidget()`
    fn element_remove_dropdown_widget(&self);
}

impl<W: AbstractDropdownControllerElementClass + ?Sized> DropdownElement for W {
    fn dropdown_length(&self) -> i32 {
        self.dropdown_element().matching_values.borrow().as_ref().map_or(0, |values| values.len() as i32)
    }
    fn extract_dropdown_entry_at(&self, graphics: &Graphics<'_>, entry_dimension: &dyn Dimension<i32>, index: i32) {
        extract_dropdown_entry_at(self, graphics, entry_dimension, index)
    }
    fn decoration_padding(&self) -> i32 {
        self.get_decoration_padding()
    }
    fn element_dimension(&self) -> Arc<dyn Dimension<i32>> {
        StringControllerElementClass::get_dimension(self)
    }
    fn element_is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        StringControllerElementClass::is_mouse_over(self, mouse_x, mouse_y)
    }
    fn element_mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        StringControllerElementClass::mouse_clicked(self, event, double_click)
    }
    fn element_unfocus(&self) {
        StringControllerElementClass::unfocus(self)
    }
    fn element_char_typed(&self, event: &CharacterEvent) -> bool {
        StringControllerElementClass::char_typed(self, event)
    }
    fn element_remove_dropdown_widget(&self) {
        self.remove_dropdown_widget()
    }
}

/// The nine `StringControllerElementClass` methods `AbstractDropdownControllerElement` overrides, forwarded to its
/// bodies above - expanded inside a subclass's `impl StringControllerElementClass for ..` block.
#[macro_export]
macro_rules! abstract_dropdown_controller_element_overrides {
    () => {
        fn mouse_clicked(
            &self,
            event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
            double_click: bool,
        ) -> bool {
            $crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::mouse_clicked(self, event, double_click)
        }
        fn set_focused(&self, focused: bool) {
            $crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::set_focused(self, focused)
        }
        fn unfocus(&self) {
            $crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::unfocus(self)
        }
        fn key_pressed(&self, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
            $crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::key_pressed(self, event)
        }
        fn char_typed(&self, event: &mcrust::net::minecraft::client::input::character_event::CharacterEvent) -> bool {
            $crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::char_typed(self, event)
        }
        fn get_value_color(&self) -> i32 {
            $crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::get_value_color(self)
        }
        fn modify_input(&self, builder: &dyn Fn(&mut String)) -> bool {
            $crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::modify_input(self, builder)
        }
        fn extract_render_state(&self, graphics: &$crate::compat::Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
            $crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::extract_render_state(
                self, graphics, mouse_x, mouse_y, a,
            )
        }
        fn set_dimension(
            &self,
            dim: std::sync::Arc<dyn $crate::dev::isxander::yacl3::api::utils::dimension::Dimension<i32>>,
        ) {
            $crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::set_dimension(self, dim)
        }
    };
}
