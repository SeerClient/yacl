//! Ported from dev.isxander.yacl3.gui.controllers.string.StringControllerElement.
//!
//! `class StringControllerElement extends ControllerWidget<IStringController<?>>`. A subclass (the dropdown elements,
//! the colour element, a mod's own) holds a `StringControllerElement` and implements `StringControllerElementClass`,
//! which declares every overridable method of this class AND of `ControllerWidget` with this class's bodies as the
//! defaults; `ControllerWidget` is then implemented for it (the blanket impl at the bottom), so a base body that calls
//! `this.unfocus()` / `this.modifyInput(..)` reaches the subclass's override (PATTERNS_Y2.md P3).
//!
//! Text positions (`caretPos`, `selectionLength`, `renderOffset`) count chars, as Java's `String` indices do (YACL's
//! fields hold no surrogate pairs in practice).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::option_event_listener::Event;
use crate::dev::isxander::yacl3::api::utils::dimension::{self, Dimension};
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::{self, ControllerWidget, ControllerWidgetBase};
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;
use crate::dev::isxander::yacl3::gui::utils::undo_redo_helper::UndoRedoHelper;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::{CursorKind, Graphics};
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarrationPriority;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::cell::{Cell, RefCell};

/// `IStringController<?>` as the element holds it: the controller's members that do not mention `T`.
#[derive(Clone)]
pub struct StringControl {
    get_string: Rc<dyn Fn() -> String>,
    set_from_string: Rc<dyn Fn(&str)>,
    is_input_valid: Rc<dyn Fn(&str) -> bool>,
}

impl StringControl {
    /// The controller seen through the wildcard.
    pub fn of<T: Clone + 'static, C: IStringController<T> + ?Sized + 'static>(control: &Arc<C>) -> StringControl {
        let (a, b, c) = (control.clone(), control.clone(), control.clone());
        StringControl {
            get_string: Rc::new(move || a.get_string()),
            set_from_string: Rc::new(move |value: &str| b.set_from_string(value)),
            is_input_valid: Rc::new(move |input: &str| c.is_input_valid(input)),
        }
    }

    /// java: String getString();
    pub fn get_string(&self) -> String {
        (self.get_string)()
    }

    /// java: void setFromString(String value);
    pub fn set_from_string(&self, value: &str) {
        (self.set_from_string)(value)
    }

    /// java: boolean isInputValid(String input);
    pub fn is_input_valid(&self, input: &str) -> bool {
        (self.is_input_valid)(input)
    }
}

/// `StringBuilder.insert(index, str)` over char positions.
pub fn sb_insert(builder: &mut String, index: i32, value: &str) {
    compat::insert_str(builder, index, value);
}

/// `StringBuilder.deleteCharAt(index)` over char positions.
pub fn sb_delete_char_at(builder: &mut String, index: i32) {
    compat::delete_char_at(builder, index);
}

/// `StringBuilder.replace(start, end, str)` over char positions.
pub fn sb_replace(builder: &mut String, start: i32, end: i32, value: &str) {
    let len = compat::str_len(builder);
    let end = end.min(len);
    let head = compat::substring2(builder, 0, start);
    let tail = compat::substring(builder, end);
    *builder = format!("{head}{value}{tail}");
}

fn width(text: &str) -> i32 {
    compat::font().width(text)
}

fn width_of(text: &MutableComponent) -> i32 {
    compat::font().width_of(&FormattedText::from(text.clone()))
}

/// java: the fields of `StringControllerElement`
pub struct StringControllerElement {
    /// java: extends ControllerWidget<IStringController<?>>
    pub controller_widget: ControllerWidgetBase,
    /// java: protected final T control;
    pub control: StringControl,
    /// java: protected final boolean instantApply;
    pub instant_apply: bool,
    /// java: protected String inputField; (shared with the option listener that writes it)
    pub input_field: Rc<RefCell<String>>,
    /// java: protected Dimension<Integer> inputFieldBounds;
    pub input_field_bounds: RefCell<Option<Arc<dyn Dimension<i32>>>>,
    /// java: protected boolean inputFieldFocused;
    pub input_field_focused: Cell<bool>,
    /// java: protected int caretPos;
    pub caret_pos: Cell<i32>,
    /// java: protected int previousCaretPos;
    pub previous_caret_pos: Cell<i32>,
    /// java: protected int selectionLength;
    pub selection_length: Cell<i32>,
    /// java: protected int renderOffset;
    pub render_offset: Cell<i32>,
    /// java: protected UndoRedoHelper undoRedoHelper;
    pub undo_redo_helper: RefCell<Option<UndoRedoHelper>>,
    /// java: protected float ticks;
    pub ticks: Cell<f32>,
    /// java: protected float caretTicks;
    pub caret_ticks: Cell<f32>,
    /// java: private final Component emptyText;
    empty_text: MutableComponent,
}

impl StringControllerElement {
    /// java: public StringControllerElement(IStringController<?> control, YACLScreen screen, Dimension<Integer> dim, boolean instantApply)
    ///
    /// The constructor's `setDimension(dim)` is this class's body; a subclass whose `setDimension` override must run
    /// there calls its own after building itself.
    pub fn new<T: Clone + 'static, C: IStringController<T> + ?Sized + 'static>(
        control: Arc<C>,
        screen: &Arc<YACLScreen>,
        dim: Arc<dyn Dimension<i32>>,
        instant_apply: bool,
    ) -> StringControllerElement {
        let controller_widget = ControllerWidgetBase::new(&control, screen, dim.clone());
        let string_control = StringControl::of(&control);
        let input_field = Rc::new(RefCell::new(string_control.get_string()));
        // java: control.option().addEventListener((opt, event) -> { if (event == STATE_CHANGE) inputField = control.getString(); });
        let listener_field = Rc::downgrade(&input_field);
        let listener_control = string_control.clone();
        controller_widget.option.add_any_event_listener(Arc::new(move |_opt: &dyn AnyOption, event: Event| {
            if event == Event::StateChange
                && let Some(field) = listener_field.upgrade()
            {
                let value = listener_control.get_string();
                *field.borrow_mut() = value;
            }
        }));
        let element = StringControllerElement {
            controller_widget,
            control: string_control,
            instant_apply,
            input_field,
            input_field_bounds: RefCell::new(None),
            input_field_focused: Cell::new(false),
            caret_pos: Cell::new(0),
            previous_caret_pos: Cell::new(0),
            selection_length: Cell::new(0),
            render_offset: Cell::new(0),
            undo_redo_helper: RefCell::new(None),
            ticks: Cell::new(0.0),
            caret_ticks: Cell::new(0.0),
            empty_text: component::literal("Click to type...").with_style_format(ChatFormatting::Gray),
        };
        // java: setDimension(dim);
        set_dimension(&element, dim);
        element
    }

    /// `inputField`
    pub fn input_field(&self) -> String {
        self.input_field.borrow().clone()
    }

    /// `inputField = value`
    pub fn set_input_field(&self, value: String) {
        *self.input_field.borrow_mut() = value;
    }

    /// `inputFieldBounds`
    pub fn input_field_bounds(&self) -> Arc<dyn Dimension<i32>> {
        self.input_field_bounds.borrow().clone().expect("StringControllerElement.inputFieldBounds before setDimension")
    }
}

impl StringControllerElementClass for StringControllerElement {
    fn string_element(&self) -> &StringControllerElement {
        self
    }
}

/// java: the overridable methods of `StringControllerElement` and of the `ControllerWidget` / `AbstractWidget` under it,
/// each defaulting to this class's body.
pub trait StringControllerElementClass: compat::DynAny {
    /// The class's fields.
    fn string_element(&self) -> &StringControllerElement;

    // ---- ControllerWidget's (and AbstractWidget's) members, with this class's bodies ---------------------------

    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        controller_widget::extract_render_state(&Base(self), graphics, mouse_x, mouse_y, a)
    }
    fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        mouse_clicked(self, event, double_click)
    }
    fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
        let _ = event;
        false
    }
    fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        let _ = (event, dx, dy);
        false
    }
    fn mouse_scrolled(&self, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        let _ = (x, y, scroll_x, scroll_y);
        false
    }
    fn mouse_moved(&self, x: f64, y: f64) {
        let _ = (x, y);
    }
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        key_pressed(self, event)
    }
    fn char_typed(&self, event: &CharacterEvent) -> bool {
        char_typed(self, event)
    }
    fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.string_element().controller_widget.base.is_mouse_over(mouse_x, mouse_y)
    }
    fn set_focused(&self, focused: bool) {
        set_focused(self, focused)
    }
    fn is_focused(&self) -> bool {
        self.string_element().controller_widget.focused.get()
    }
    fn should_take_focus_after_interaction(&self) -> bool {
        true
    }
    fn can_reset(&self) -> bool {
        true
    }
    fn set_dimension(&self, dim: Arc<dyn Dimension<i32>>) {
        set_dimension(self, dim)
    }
    fn get_dimension(&self) -> Arc<dyn Dimension<i32>> {
        self.string_element().controller_widget.base.get_dimension()
    }
    fn narration_priority(&self) -> NarrationPriority {
        controller_widget::narration_priority(&Base(self))
    }
    fn unfocus(&self) {
        unfocus(self)
    }
    fn matches_search(&self, query: &str) -> bool {
        controller_widget::matches_search(&Base(self), query)
    }
    fn update_narration(&self, builder: &mut dyn NarrationElementOutput) {
        controller_widget::update_narration(&Base(self), builder)
    }
    fn play_down_sound(&self) {
        compat::play_down_sound();
    }
    fn extract_hovered_control(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = (graphics, mouse_x, mouse_y, a);
    }
    fn extract_value_text(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        extract_value_text(self, graphics, mouse_x, mouse_y, a)
    }
    fn get_control_width(&self) -> i32 {
        controller_widget::get_control_width(&Base(self))
    }
    fn is_hovered(&self) -> bool {
        is_hovered(self)
    }
    fn get_hovered_control_width(&self) -> i32 {
        get_hovered_control_width(self)
    }
    fn get_unhovered_control_width(&self) -> i32 {
        get_unhovered_control_width(self)
    }
    fn get_x_padding(&self) -> i32 {
        5
    }
    fn get_y_padding(&self) -> i32 {
        2
    }
    fn get_value_text(&self) -> MutableComponent {
        get_value_text(self)
    }
    fn is_available(&self) -> bool {
        self.string_element().controller_widget.option.available()
    }
    fn get_value_color(&self) -> i32 {
        controller_widget::get_value_color(&Base(self))
    }
    fn get_text_y(&self) -> i32 {
        controller_widget::get_text_y(&Base(self))
    }

    // ---- StringControllerElement's own -----------------------------------------------------------------------

    /// java: protected int getDefaultCaretPos()
    fn get_default_caret_pos(&self) -> i32 {
        compat::str_len(&self.string_element().input_field())
    }
    /// java: protected boolean doPaste()
    fn do_paste(&self) -> bool {
        self.write(&compat::get_clipboard());
        self.update_undo_history();
        true
    }
    /// java: protected boolean doCopy()
    fn do_copy(&self) -> bool {
        if self.string_element().selection_length.get() != 0 {
            compat::set_clipboard(&self.get_selection());
            return true;
        }
        false
    }
    /// java: protected boolean doCut()
    fn do_cut(&self) -> bool {
        if self.string_element().selection_length.get() != 0 {
            compat::set_clipboard(&self.get_selection());
            self.write("");
            self.update_undo_history();
            return true;
        }
        false
    }
    /// java: protected boolean doSelectAll()
    fn do_select_all(&self) -> bool {
        let s = self.string_element();
        s.caret_pos.set(compat::str_len(&s.input_field()));
        self.check_render_offset();
        s.selection_length.set(-s.caret_pos.get());
        true
    }
    /// java: protected void checkRenderOffset()
    fn check_render_offset(&self) {
        check_render_offset(self)
    }
    /// java: protected void doBackspace()
    fn do_backspace(&self) {
        let s = self.string_element();
        if s.selection_length.get() != 0 {
            self.write("");
        } else if s.caret_pos.get() > 0 {
            let at = s.caret_pos.get() - 1;
            if self.modify_input(&|builder: &mut String| sb_delete_char_at(builder, at)) {
                s.caret_pos.set(s.caret_pos.get() - 1);
                self.check_render_offset();
            }
        }
        self.update_undo_history();
    }
    /// java: protected void doDelete()
    fn do_delete(&self) {
        let s = self.string_element();
        if s.selection_length.get() != 0 {
            self.write("");
        } else if s.caret_pos.get() < compat::str_len(&s.input_field()) {
            let at = s.caret_pos.get();
            self.modify_input(&|builder: &mut String| sb_delete_char_at(builder, at));
        }
        self.update_undo_history();
    }
    /// java: public void write(String string)
    fn write(&self, string: &str) {
        write(self, string)
    }
    /// java: public boolean modifyInput(Consumer<StringBuilder> consumer)
    fn modify_input(&self, consumer: &dyn Fn(&mut String)) -> bool {
        modify_input(self, consumer)
    }
    /// java: protected void updateUndoHistory()
    fn update_undo_history(&self) {
        // undoRedoHelper.save(inputField, caretPos, selectionLength); - commented out in YACL
    }
    /// java: public int getUnshiftedLength()
    fn get_unshifted_length(&self) -> i32 {
        let s = self.string_element();
        let dim = self.get_dimension();
        if s.controller_widget.option_name_string.is_empty() {
            return dim.width() - self.get_x_padding() * 2;
        }
        dim.width() / 8 * 5
    }
    /// java: public int getMaxUnwrapLength()
    fn get_max_unwrap_length(&self) -> i32 {
        let s = self.string_element();
        let dim = self.get_dimension();
        if s.controller_widget.option_name_string.is_empty() {
            return dim.width() - self.get_x_padding() * 2;
        }
        dim.width() / 2
    }
    /// java: public int getSelectionStart()
    fn get_selection_start(&self) -> i32 {
        let s = self.string_element();
        s.caret_pos.get().min(s.caret_pos.get() + s.selection_length.get())
    }
    /// java: public int getSelectionEnd()
    fn get_selection_end(&self) -> i32 {
        let s = self.string_element();
        s.caret_pos.get().max(s.caret_pos.get() + s.selection_length.get())
    }
    /// java: protected String getSelection()
    fn get_selection(&self) -> String {
        compat::substring2(&self.string_element().input_field(), self.get_selection_start(), self.get_selection_end())
    }
    /// java: protected int findSpaceIndex(boolean reverse)
    fn find_space_index(&self, reverse: bool) -> i32 {
        let s = self.string_element();
        let input_field = s.input_field();
        let caret_pos = s.caret_pos.get();
        let mut from_index = caret_pos;
        if reverse {
            if caret_pos > 0 {
                from_index -= 2;
            }
            compat::last_index_of_from(&input_field, " ", from_index) + 1
        } else {
            if caret_pos < compat::str_len(&input_field) {
                from_index += 1;
            }
            let mut i = compat::index_of_from(&input_field, " ", from_index) + 1;
            if i == 0 {
                i = compat::str_len(&input_field);
            }
            i
        }
    }
    /// java: protected void updateControl()
    fn update_control(&self) {
        let s = self.string_element();
        s.control.set_from_string(&s.input_field());
    }
}

/// `super.x(..)` from a `StringControllerElement` body into `ControllerWidget`'s: the element seen as a
/// `ControllerWidget` whose overridable methods are the element class's (so the base body's virtual calls still reach
/// the most-derived override).
pub struct Base<'a, W: StringControllerElementClass + ?Sized>(pub &'a W);

impl<W: StringControllerElementClass + ?Sized> ControllerWidget for Base<'_, W> {
    fn controller_widget(&self) -> &ControllerWidgetBase {
        &self.0.string_element().controller_widget
    }
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        StringControllerElementClass::extract_render_state(self.0, graphics, mouse_x, mouse_y, a)
    }
    fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        StringControllerElementClass::is_mouse_over(self.0, mouse_x, mouse_y)
    }
    fn get_dimension(&self) -> Arc<dyn Dimension<i32>> {
        StringControllerElementClass::get_dimension(self.0)
    }
    fn extract_hovered_control(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        StringControllerElementClass::extract_hovered_control(self.0, graphics, mouse_x, mouse_y, a)
    }
    fn extract_value_text(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        StringControllerElementClass::extract_value_text(self.0, graphics, mouse_x, mouse_y, a)
    }
    fn get_control_width(&self) -> i32 {
        StringControllerElementClass::get_control_width(self.0)
    }
    fn is_hovered(&self) -> bool {
        StringControllerElementClass::is_hovered(self.0)
    }
    fn get_hovered_control_width(&self) -> i32 {
        StringControllerElementClass::get_hovered_control_width(self.0)
    }
    fn get_unhovered_control_width(&self) -> i32 {
        StringControllerElementClass::get_unhovered_control_width(self.0)
    }
    fn get_x_padding(&self) -> i32 {
        StringControllerElementClass::get_x_padding(self.0)
    }
    fn get_y_padding(&self) -> i32 {
        StringControllerElementClass::get_y_padding(self.0)
    }
    fn get_value_text(&self) -> MutableComponent {
        StringControllerElementClass::get_value_text(self.0)
    }
    fn is_available(&self) -> bool {
        StringControllerElementClass::is_available(self.0)
    }
    fn get_value_color(&self) -> i32 {
        StringControllerElementClass::get_value_color(self.0)
    }
    fn get_text_y(&self) -> i32 {
        StringControllerElementClass::get_text_y(self.0)
    }
}

/// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
pub fn extract_value_text<W: StringControllerElementClass + ?Sized>(this: &W, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
    let s = this.string_element();
    let mut value_text = this.get_value_text();
    if !this.is_hovered() {
        let font = compat::font();
        value_text = component::literal(&GuiUtils::shorten_string(value_text.get_string(), &font, this.get_max_unwrap_length(), "..."))
            .set_style(value_text.get_style().clone());
    }

    let dim = this.get_dimension();
    let bounds = s.input_field_bounds();
    let text_x = dim.x_limit() - width_of(&value_text) + s.render_offset.get() - this.get_x_padding();
    graphics.enable_scissor(bounds.x(), bounds.y() - 2, bounds.x_limit() + 1, bounds.y_limit() + 4);
    graphics.text_component(&value_text, text_x, this.get_text_y(), this.get_value_color(), true);

    if this.is_hovered() {
        s.ticks.set(s.ticks.get() + a);

        let text = this.get_value_text().get_string();

        graphics.fill(bounds.x(), bounds.y_limit(), bounds.x_limit(), bounds.y_limit() + 1, -1);
        graphics.fill(bounds.x() + 1, bounds.y_limit() + 1, bounds.x_limit() + 1, bounds.y_limit() + 2, 0xFF404040u32 as i32);

        if s.input_field_focused.get() || s.controller_widget.focused.get() {
            let text_len = compat::str_len(&text);
            if s.caret_pos.get() > text_len {
                s.caret_pos.set(text_len);
            }

            let mut caret_x = text_x + width(&compat::substring2(&text, 0, s.caret_pos.get()));
            if text.is_empty() {
                caret_x = bounds.x() + bounds.width() / 2;
            }

            if s.selection_length.get() != 0 {
                let selection_x = text_x + width(&compat::substring2(&text, 0, s.caret_pos.get() + s.selection_length.get()));
                graphics.fill(caret_x, bounds.y() - 2, selection_x, bounds.y_limit() - 1, 0x803030FFu32 as i32);
            }

            if s.caret_pos.get() != s.previous_caret_pos.get() {
                s.previous_caret_pos.set(s.caret_pos.get());
                s.caret_ticks.set(0.0);
            }

            s.caret_ticks.set(s.caret_ticks.get() + a);
            if s.caret_ticks.get() % 20.0 <= 10.0 {
                graphics.fill(caret_x, bounds.y() - 2, caret_x + 1, bounds.y_limit() - 1, -1);
            }
        }
    }
    graphics.disable_scissor();

    if is_hovered_input_field(this, mouse_x as f64, mouse_y as f64) {
        graphics.request_cursor(if this.is_available() { CursorKind::IBeam } else { CursorKind::NotAllowed });
    } else if s.controller_widget.hovered.get() {
        graphics.request_cursor(if this.is_available() { CursorKind::PointingHand } else { CursorKind::NotAllowed });
    }
}

/// java: private boolean isHoveredInputField(double mouseX, double mouseY)
pub fn is_hovered_input_field<W: StringControllerElementClass + ?Sized>(this: &W, mouse_x: f64, mouse_y: f64) -> bool {
    this.string_element().input_field_bounds().is_point_inside(mouse_x as i32, mouse_y as i32)
}

/// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
pub fn mouse_clicked<W: StringControllerElementClass + ?Sized>(this: &W, event: &MouseButtonEvent, _double_click: bool) -> bool {
    let s = this.string_element();
    if this.is_available() && this.get_dimension().is_point_inside(event.x() as i32, event.y() as i32) {
        s.input_field_focused.set(true);

        if !is_hovered_input_field(this, event.x(), event.y()) {
            s.caret_pos.set(this.get_default_caret_pos());
        } else {
            // gets the appropriate caret position for where you click
            let text_x = event.x() as i32 - (s.input_field_bounds().x_limit() - width_of(&this.get_value_text()));
            let input_field = s.input_field();
            let len = compat::str_len(&input_field);
            let mut pos: i32 = -1;
            let mut current_width = 0;
            for ch in input_field.chars() {
                pos += 1;
                let char_length = width(&ch.to_string());
                if current_width + char_length / 2 > text_x {
                    // if more than halfway past the characters select in front of that char
                    s.caret_pos.set(pos);
                    break;
                } else if pos == len - 1 {
                    // if we have reached the end and no matches, it must be the second half of the char so the last position
                    s.caret_pos.set(pos + 1);
                }
                current_width += char_length;
            }

            s.selection_length.set(0);
        }
        return true;
    } else {
        this.unfocus();
    }
    false
}

/// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
pub fn key_pressed<W: StringControllerElementClass + ?Sized>(this: &W, event: &KeyEvent) -> bool {
    let s = this.string_element();
    if !s.input_field_focused.get() {
        return false;
    }

    match event.key() {
        input_constants::KEY_ESCAPE | input_constants::KEY_RETURN => {
            this.unfocus();
            return true;
        }
        input_constants::KEY_LEFT => {
            if event.has_shift_down() {
                if event.has_control_down() {
                    let space_char = this.find_space_index(true);
                    s.selection_length.set(s.selection_length.get() + s.caret_pos.get() - space_char);
                    s.caret_pos.set(space_char);
                } else if s.caret_pos.get() > 0 {
                    s.caret_pos.set(s.caret_pos.get() - 1);
                    s.selection_length.set(s.selection_length.get() + 1);
                }
                this.check_render_offset();
            } else {
                if s.caret_pos.get() > 0 {
                    if event.has_control_down() {
                        s.caret_pos.set(this.find_space_index(true));
                    } else if s.selection_length.get() != 0 {
                        s.caret_pos.set(s.caret_pos.get() + s.selection_length.get().min(0));
                    } else {
                        s.caret_pos.set(s.caret_pos.get() - 1);
                    }
                }
                this.check_render_offset();
                s.selection_length.set(0);
            }
            return true;
        }
        input_constants::KEY_RIGHT => {
            let len = compat::str_len(&s.input_field());
            if event.has_shift_down() {
                if event.has_control_down() {
                    let space_char = this.find_space_index(false);
                    s.selection_length.set(s.selection_length.get() - (space_char - s.caret_pos.get()));
                    s.caret_pos.set(space_char);
                } else if s.caret_pos.get() < len {
                    s.caret_pos.set(s.caret_pos.get() + 1);
                    s.selection_length.set(s.selection_length.get() - 1);
                }
                this.check_render_offset();
            } else {
                if s.caret_pos.get() < len {
                    if event.has_control_down() {
                        s.caret_pos.set(this.find_space_index(false));
                    } else if s.selection_length.get() != 0 {
                        s.caret_pos.set(s.caret_pos.get() + s.selection_length.get().max(0));
                    } else {
                        s.caret_pos.set(s.caret_pos.get() + 1);
                    }
                    this.check_render_offset();
                }
                s.selection_length.set(0);
            }
            return true;
        }
        input_constants::KEY_BACKSPACE => {
            this.do_backspace();
            return true;
        }
        input_constants::KEY_DELETE => {
            this.do_delete();
            return true;
        }
        input_constants::KEY_END => {
            let len = compat::str_len(&s.input_field());
            if event.has_shift_down() {
                s.selection_length.set(s.selection_length.get() - (len - s.caret_pos.get()));
            } else {
                s.selection_length.set(0);
            }
            s.caret_pos.set(len);
            this.check_render_offset();
            return true;
        }
        input_constants::KEY_HOME => {
            if event.has_shift_down() {
                s.selection_length.set(s.selection_length.get() + s.caret_pos.get());
                s.caret_pos.set(0);
            } else {
                s.caret_pos.set(0);
                s.selection_length.set(0);
            }
            this.check_render_offset();
            return true;
        }
        _ => {}
    }

    if event.is_paste() {
        return this.do_paste();
    } else if event.is_copy() {
        return this.do_copy();
    } else if event.is_cut() {
        return this.do_cut();
    } else if event.is_select_all() {
        return this.do_select_all();
    }

    false
}

/// java: protected void checkRenderOffset()
pub fn check_render_offset<W: StringControllerElementClass + ?Sized>(this: &W) {
    let s = this.string_element();
    let input_field = s.input_field();
    if width(&input_field) < this.get_unshifted_length() {
        s.render_offset.set(0);
        return;
    }

    let dim = this.get_dimension();
    let text_x = dim.x_limit() - width(&input_field) - this.get_x_padding();
    let caret_x = text_x + width(&compat::substring2(&input_field, 0, s.caret_pos.get()));

    let min_x = dim.x_limit() - this.get_x_padding() - this.get_unshifted_length();
    let max_x = min_x + this.get_unshifted_length();

    if caret_x + s.render_offset.get() < min_x {
        s.render_offset.set(min_x - caret_x);
    } else if caret_x + s.render_offset.get() > max_x {
        s.render_offset.set(max_x - caret_x);
    }
}

/// java: @Override public boolean charTyped(@NonNull CharacterEvent event)
pub fn char_typed<W: StringControllerElementClass + ?Sized>(this: &W, event: &CharacterEvent) -> bool {
    if !this.string_element().input_field_focused.get() {
        return false;
    }
    this.write(&event.codepoint_as_string());
    this.update_undo_history();
    true
}

/// java: public void write(String string)
pub fn write<W: StringControllerElementClass + ?Sized>(this: &W, string: &str) {
    let s = this.string_element();
    if s.selection_length.get() == 0 {
        let at = s.caret_pos.get();
        if this.modify_input(&|builder: &mut String| sb_insert(builder, at, string)) {
            s.caret_pos.set(s.caret_pos.get() + compat::str_len(string));
            this.check_render_offset();
        }
    } else {
        let start = this.get_selection_start();
        let end = this.get_selection_end();
        if this.modify_input(&|builder: &mut String| sb_replace(builder, start, end, string)) {
            s.caret_pos.set(start + compat::str_len(string));
            s.selection_length.set(0);
            this.check_render_offset();
        }
    }
}

/// java: public boolean modifyInput(Consumer<StringBuilder> consumer)
pub fn modify_input<W: StringControllerElementClass + ?Sized>(this: &W, consumer: &dyn Fn(&mut String)) -> bool {
    let s = this.string_element();
    let mut temp = s.input_field();
    consumer(&mut temp);
    if !s.control.is_input_valid(&temp) {
        return false;
    }
    s.set_input_field(temp);
    if s.instant_apply {
        this.update_control();
    }
    true
}

/// java: @Override public void setFocused(boolean focused)
pub fn set_focused<W: StringControllerElementClass + ?Sized>(this: &W, focused: bool) {
    let s = this.string_element();
    // super.setFocused(focused);
    s.controller_widget.focused.set(focused);
    s.input_field_focused.set(focused);
}

/// java: @Override public void unfocus()
pub fn unfocus<W: StringControllerElementClass + ?Sized>(this: &W) {
    let s = this.string_element();
    // super.unfocus();
    s.controller_widget.focused.set(false);
    s.input_field_focused.set(false);
    s.render_offset.set(0);
    if !s.instant_apply {
        this.update_control();
    }
}

/// java: @Override public void setDimension(Dimension<Integer> dim)
pub fn set_dimension<W: StringControllerElementClass + ?Sized>(this: &W, dim: Arc<dyn Dimension<i32>>) {
    let s = this.string_element();
    // super.setDimension(dim);
    s.controller_widget.base.set_dimension(dim.clone());

    let width = 6.max(width_of(&this.get_value_text()).min(this.get_unshifted_length()));
    let line_height = mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
    let bounds: Arc<dyn Dimension<i32>> =
        dimension::of_int(dim.x_limit() - this.get_x_padding() - width, dim.center_y() - line_height / 2, width, line_height);
    *s.input_field_bounds.borrow_mut() = Some(bounds);
}

/// java: @Override public boolean isHovered()
pub fn is_hovered<W: StringControllerElementClass + ?Sized>(this: &W) -> bool {
    controller_widget::is_hovered(&Base(this)) || this.string_element().input_field_focused.get()
}

/// java: @Override protected int getUnhoveredControlWidth()
pub fn get_unhovered_control_width<W: StringControllerElementClass + ?Sized>(this: &W) -> i32 {
    if !this.is_hovered() { this.get_hovered_control_width().min(this.get_max_unwrap_length()) } else { this.get_hovered_control_width() }
}

/// java: @Override protected int getHoveredControlWidth()
pub fn get_hovered_control_width<W: StringControllerElementClass + ?Sized>(this: &W) -> i32 {
    width_of(&this.get_value_text()).min(this.get_unshifted_length())
}

/// java: @Override protected Component getValueText()
pub fn get_value_text<W: StringControllerElementClass + ?Sized>(this: &W) -> MutableComponent {
    let s = this.string_element();
    let focused = s.input_field_focused.get();
    if !focused && s.input_field.borrow().is_empty() {
        return s.empty_text.clone();
    }
    if s.instant_apply || !focused { s.controller_widget.control_format_value() } else { component::literal(&s.input_field()) }
}

/// Every `StringControllerElementClass` is a `ControllerWidget` (Java's `extends`): the class's methods.
mod dispatch {
    use super::StringControllerElementClass;
    use crate::compat;
    use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
    use crate::dev::isxander::yacl3::gui::controllers::controller_widget::{ControllerWidget, ControllerWidgetBase};
    use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarrationPriority;
    use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
    use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
    use mcrust::net::minecraft::client::input::key_event::KeyEvent;
    use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
    use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
    use std::sync::Arc;

    impl<W: StringControllerElementClass + ?Sized> ControllerWidget for W {
        fn controller_widget(&self) -> &ControllerWidgetBase {
            &StringControllerElementClass::string_element(self).controller_widget
        }
        fn extract_render_state(&self, graphics: &compat::Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
            StringControllerElementClass::extract_render_state(self, graphics, mouse_x, mouse_y, a)
        }
        fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
            StringControllerElementClass::mouse_clicked(self, event, double_click)
        }
        fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
            StringControllerElementClass::mouse_released(self, event)
        }
        fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
            StringControllerElementClass::mouse_dragged(self, event, dx, dy)
        }
        fn mouse_scrolled(&self, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
            StringControllerElementClass::mouse_scrolled(self, x, y, scroll_x, scroll_y)
        }
        fn mouse_moved(&self, x: f64, y: f64) {
            StringControllerElementClass::mouse_moved(self, x, y)
        }
        fn key_pressed(&self, event: &KeyEvent) -> bool {
            StringControllerElementClass::key_pressed(self, event)
        }
        fn char_typed(&self, event: &CharacterEvent) -> bool {
            StringControllerElementClass::char_typed(self, event)
        }
        fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
            StringControllerElementClass::is_mouse_over(self, mouse_x, mouse_y)
        }
        fn set_focused(&self, focused: bool) {
            StringControllerElementClass::set_focused(self, focused)
        }
        fn is_focused(&self) -> bool {
            StringControllerElementClass::is_focused(self)
        }
        fn should_take_focus_after_interaction(&self) -> bool {
            StringControllerElementClass::should_take_focus_after_interaction(self)
        }
        fn can_reset(&self) -> bool {
            StringControllerElementClass::can_reset(self)
        }
        fn set_dimension(&self, dim: Arc<dyn Dimension<i32>>) {
            StringControllerElementClass::set_dimension(self, dim)
        }
        fn get_dimension(&self) -> Arc<dyn Dimension<i32>> {
            StringControllerElementClass::get_dimension(self)
        }
        fn narration_priority(&self) -> NarrationPriority {
            StringControllerElementClass::narration_priority(self)
        }
        fn unfocus(&self) {
            StringControllerElementClass::unfocus(self)
        }
        fn matches_search(&self, query: &str) -> bool {
            StringControllerElementClass::matches_search(self, query)
        }
        fn update_narration(&self, builder: &mut dyn NarrationElementOutput) {
            StringControllerElementClass::update_narration(self, builder)
        }
        fn play_down_sound(&self) {
            StringControllerElementClass::play_down_sound(self)
        }
        fn extract_hovered_control(&self, graphics: &compat::Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
            StringControllerElementClass::extract_hovered_control(self, graphics, mouse_x, mouse_y, a)
        }
        fn extract_value_text(&self, graphics: &compat::Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
            StringControllerElementClass::extract_value_text(self, graphics, mouse_x, mouse_y, a)
        }
        fn get_control_width(&self) -> i32 {
            StringControllerElementClass::get_control_width(self)
        }
        fn is_hovered(&self) -> bool {
            StringControllerElementClass::is_hovered(self)
        }
        fn get_hovered_control_width(&self) -> i32 {
            StringControllerElementClass::get_hovered_control_width(self)
        }
        fn get_unhovered_control_width(&self) -> i32 {
            StringControllerElementClass::get_unhovered_control_width(self)
        }
        fn get_x_padding(&self) -> i32 {
            StringControllerElementClass::get_x_padding(self)
        }
        fn get_y_padding(&self) -> i32 {
            StringControllerElementClass::get_y_padding(self)
        }
        fn get_value_text(&self) -> MutableComponent {
            StringControllerElementClass::get_value_text(self)
        }
        fn is_available(&self) -> bool {
            StringControllerElementClass::is_available(self)
        }
        fn get_value_color(&self) -> i32 {
            StringControllerElementClass::get_value_color(self)
        }
        fn get_text_y(&self) -> i32 {
            StringControllerElementClass::get_text_y(self)
        }
    }
}
