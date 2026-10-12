//! Ported from dev.isxander.yacl3.gui.controllers.string.StringControllerElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option_event_listener::Event;
use crate::dev::isxander::yacl3::api::utils::dimension;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;
use crate::dev::isxander::yacl3::gui::utils::undo_redo_helper::UndoRedoHelper;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::controller::Controller as _;
use crate::dev::isxander::yacl3::api::option::Option as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class StringControllerElement extends ControllerWidget<IStringController<?>>
pub struct StringControllerElementBase {
    /// java: extends ControllerWidget
    pub base: ControllerWidgetBase<compat::Shared<dyn IStringController<compat::Object>>>,
    /// java: protected final boolean instantApply;
    pub instant_apply: bool,
    /// java: protected String inputField;
    pub input_field: String,
    /// java: protected Dimension<Integer> inputFieldBounds;
    pub input_field_bounds: Option<compat::Shared<dyn Dimension<i32>>>,
    /// java: protected boolean inputFieldFocused;
    pub input_field_focused: bool,
    /// java: protected int caretPos;
    pub caret_pos: i32,
    /// java: protected int previousCaretPos;
    pub previous_caret_pos: i32,
    /// java: protected int selectionLength;
    pub selection_length: i32,
    /// java: protected int renderOffset;
    pub render_offset: i32,
    /// java: protected UndoRedoHelper undoRedoHelper;
    pub undo_redo_helper: Option<UndoRedoHelper>,
    /// java: protected float ticks;
    pub ticks: f32,
    /// java: protected float caretTicks;
    pub caret_ticks: f32,
    /// java: private final Component emptyText;
    pub empty_text: MutableComponent,
}

impl std::ops::Deref for StringControllerElementBase {
    type Target = ControllerWidgetBase<compat::Shared<dyn IStringController<compat::Object>>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for StringControllerElementBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl StringControllerElementBase {
    /// java: public StringControllerElement(IStringController<?> control, YACLScreen screen, Dimension<Integer> dim, boolean instantApply) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut control: compat::Shared<dyn IStringController<compat::Object>>, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>, mut instant_apply: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn StringControllerElement>, __R)) -> (compat::Shared<dyn StringControllerElement>, __R) {
        let __init_instant_apply = instant_apply;
        let __init_input_field = control.lock().borrow().get_string();
        let __init_input_field_focused = false;
        let __init_selection_length = 0;
        let __init_empty_text = component::literal("Click to type...").with_style_format(ChatFormatting::Gray);
        let this = Self {
            base: ControllerWidgetBase::<compat::Shared<dyn IStringController<compat::Object>>>::new(control.clone(), screen.clone(), dim.clone()),
            instant_apply: __init_instant_apply,
            input_field: __init_input_field,
            input_field_bounds: None,
            input_field_focused: __init_input_field_focused,
            caret_pos: 0,
            previous_caret_pos: 0,
            selection_length: __init_selection_length,
            render_offset: 0,
            undo_redo_helper: None,
            ticks: 0.0,
            caret_ticks: 0.0,
            empty_text: __init_empty_text,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        let __this_1 = compat::RenderThread::new(__this.clone());
        { let (__a0,) = (Arc::new(move |opt, event| {
            if event == Event::StateChange {
                { let __v = control.lock().borrow().get_string(); compat::write(&(*__this_1.get()), |__t| __t.string_controller_element_base_mut().input_field = __v); }
            }
        }),); control.lock().borrow().option().lock().borrow_mut().add_event_listener(__a0) };
        ({ let __f = compat::dispatch(&__this, |__t| __t.set_dimension()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn AbstractWidget>), dim.clone()) });
        (__this, __r)
    }

    /// java: public StringControllerElement(IStringController<?> control, YACLScreen screen, Dimension<Integer> dim, boolean instantApply)
    pub fn new(mut control: compat::Shared<dyn IStringController<compat::Object>>, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>, mut instant_apply: bool) -> compat::Shared<Self> {
        Self::new_in(control, screen, dim, instant_apply, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn StringControllerElement>), __h)
        }).1
    }

}

/// java: the virtual methods of StringControllerElement (implemented by StringControllerElementBase and every subclass)
pub trait StringControllerElement: ControllerWidget {
    fn string_controller_element_base(&self) -> &StringControllerElementBase;
    fn string_controller_element_base_mut(&mut self) -> &mut StringControllerElementBase;

    /// java: private boolean isHoveredInputField(double mouseX, double mouseY) (dispatch: the body for this class)
    fn is_hovered_input_field(&self) -> fn(&compat::Shared<dyn StringControllerElement>, f64, f64) -> bool {
        <dyn StringControllerElement>::is_hovered_input_field_body
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn StringControllerElement>, &MouseButtonEvent, bool) -> bool {
        <dyn StringControllerElement>::mouse_clicked_body
    }

    /// java: protected int getDefaultCaretPos() (dispatch: the body for this class)
    fn get_default_caret_pos(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> i32 {
        <dyn StringControllerElement>::get_default_caret_pos_body
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event) (dispatch: the body for this class)
    fn key_pressed(&self) -> fn(&compat::Shared<dyn StringControllerElement>, &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        <dyn StringControllerElement>::key_pressed_body
    }

    /// java: protected boolean doPaste() (dispatch: the body for this class)
    fn do_paste(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> bool {
        <dyn StringControllerElement>::do_paste_body
    }

    /// java: protected boolean doCopy() (dispatch: the body for this class)
    fn do_copy(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> bool {
        <dyn StringControllerElement>::do_copy_body
    }

    /// java: protected boolean doCut() (dispatch: the body for this class)
    fn do_cut(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> bool {
        <dyn StringControllerElement>::do_cut_body
    }

    /// java: protected boolean doSelectAll() (dispatch: the body for this class)
    fn do_select_all(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> bool {
        <dyn StringControllerElement>::do_select_all_body
    }

    /// java: protected void checkRenderOffset() (dispatch: the body for this class)
    fn check_render_offset(&self) -> fn(&compat::Shared<dyn StringControllerElement>) {
        <dyn StringControllerElement>::check_render_offset_body
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent event) (dispatch: the body for this class)
    fn char_typed(&self) -> fn(&compat::Shared<dyn StringControllerElement>, &CharacterEvent) -> bool {
        <dyn StringControllerElement>::char_typed_body
    }

    /// java: protected void doBackspace() (dispatch: the body for this class)
    fn do_backspace(&self) -> fn(&compat::Shared<dyn StringControllerElement>) {
        <dyn StringControllerElement>::do_backspace_body
    }

    /// java: protected void doDelete() (dispatch: the body for this class)
    fn do_delete(&self) -> fn(&compat::Shared<dyn StringControllerElement>) {
        <dyn StringControllerElement>::do_delete_body
    }

    /// java: public void write(String string) (dispatch: the body for this class)
    fn write(&self) -> fn(&compat::Shared<dyn StringControllerElement>, String) {
        <dyn StringControllerElement>::write_body
    }

    /// java: public boolean modifyInput(Consumer<StringBuilder> consumer) (dispatch: the body for this class)
    fn modify_input(&self) -> fn(&compat::Shared<dyn StringControllerElement>, Arc<dyn Fn(String)>) -> bool {
        <dyn StringControllerElement>::modify_input_body
    }

    /// java: protected void updateUndoHistory() (dispatch: the body for this class)
    fn update_undo_history(&self) -> fn(&compat::Shared<dyn StringControllerElement>) {
        <dyn StringControllerElement>::update_undo_history_body
    }

    /// java: public int getUnshiftedLength() (dispatch: the body for this class)
    fn get_unshifted_length(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> i32 {
        <dyn StringControllerElement>::get_unshifted_length_body
    }

    /// java: public int getMaxUnwrapLength() (dispatch: the body for this class)
    fn get_max_unwrap_length(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> i32 {
        <dyn StringControllerElement>::get_max_unwrap_length_body
    }

    /// java: public int getSelectionStart() (dispatch: the body for this class)
    fn get_selection_start(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> i32 {
        <dyn StringControllerElement>::get_selection_start_body
    }

    /// java: public int getSelectionEnd() (dispatch: the body for this class)
    fn get_selection_end(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> i32 {
        <dyn StringControllerElement>::get_selection_end_body
    }

    /// java: protected String getSelection() (dispatch: the body for this class)
    fn get_selection(&self) -> fn(&compat::Shared<dyn StringControllerElement>) -> String {
        <dyn StringControllerElement>::get_selection_body
    }

    /// java: protected int findSpaceIndex(boolean reverse) (dispatch: the body for this class)
    fn find_space_index(&self) -> fn(&compat::Shared<dyn StringControllerElement>, bool) -> i32 {
        <dyn StringControllerElement>::find_space_index_body
    }

    /// java: protected void updateControl() (dispatch: the body for this class)
    fn update_control(&self) -> fn(&compat::Shared<dyn StringControllerElement>) {
        <dyn StringControllerElement>::update_control_body
    }

}

impl dyn StringControllerElement {
    /// java: @Override protected void extractHoveredControl(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_hovered_control_body(__this: &compat::Shared<dyn StringControllerElement>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
    }

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_value_text_body(__this: &compat::Shared<dyn StringControllerElement>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut value_text: MutableComponent = __this.lock().borrow().get_value_text();
        if !(__this.lock().borrow().is_hovered()) {
            value_text = (component::literal(&GuiUtils::shorten_string(value_text.get_string(), &compat::read(__this, |__t| __t.string_controller_element_base().text_renderer.clone()).unwrap(), ({ let __f = compat::dispatch(__this, |__t| __t.get_max_unwrap_length()); __f(__this) }), "...".to_string()))).set_style(value_text.get_style().clone());
        }
        let mut text_x: i32 = ((((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x_limit()).wrapping_sub({ let (__q0,) = (&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(value_text.clone()),); __this.lock().borrow().string_controller_element_base().text_renderer.as_ref().unwrap().width_of_cached(__q0) })).wrapping_add(compat::read(__this, |__t| __t.string_controller_element_base().render_offset))).wrapping_sub(__this.lock().borrow().get_x_padding());
        // TODO(fabric2seer): `GuiGraphicsExtractor.enableScissor` has no Rust method in the port (looked for `enable_scissor` in GuiGraphicsExtractor and its supertypes)
        // rust: graphics.enable_scissor(&__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().x(), (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().y()).wrapping_sub(2), (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().x_limit()).wrapping_add(1), (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().y_limit()).wrapping_add(4));
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(value_text.clone()), text_x, __this.lock().borrow().get_text_y(), __this.lock().borrow().get_value_color(), true);
        if __this.lock().borrow().is_hovered() {
            { let __v = (compat::read(__this, |__t| __t.string_controller_element_base().ticks) + a); compat::write(__this, |__t| __t.string_controller_element_base_mut().ticks = __v); }
            let mut text: String = __this.lock().borrow().get_value_text().get_string();
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), __this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().x(), __this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().y_limit(), __this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().x_limit(), (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().y_limit()).wrapping_add(1), -1);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().x()).wrapping_add(1), (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().y_limit()).wrapping_add(1), (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().x_limit()).wrapping_add(1), (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().y_limit()).wrapping_add(2), 0xFF404040u32 as i32);
            if compat::read(__this, |__t| __t.string_controller_element_base().input_field_focused) || compat::read(__this, |__t| __t.string_controller_element_base().focused) {
                if compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) > compat::str_len(&text) {
                    { let __v = compat::str_len(&text); compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                }
                let mut caret_x: i32 = text_x.wrapping_add({ let (__q0,) = (&compat::substring2(&text, 0, compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)),); __this.lock().borrow().string_controller_element_base().text_renderer.as_ref().unwrap().width(__q0) });
                if text.is_empty() {
                    caret_x = (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().x()).wrapping_add((__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().width()).wrapping_div(2));
                }
                if compat::read(__this, |__t| __t.string_controller_element_base().selection_length) != 0 {
                    let mut selection_x: i32 = text_x.wrapping_add({ let (__q0,) = (&compat::substring2(&text, 0, (compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)).wrapping_add(compat::read(__this, |__t| __t.string_controller_element_base().selection_length))),); __this.lock().borrow().string_controller_element_base().text_renderer.as_ref().unwrap().width(__q0) });
                    graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), caret_x, (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().y()).wrapping_sub(2), selection_x, (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().y_limit()).wrapping_sub(1), 0x803030FFu32 as i32);
                }
                if compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) != compat::read(__this, |__t| __t.string_controller_element_base().previous_caret_pos) {
                    { let __v = compat::read(__this, |__t| __t.string_controller_element_base().caret_pos); compat::write(__this, |__t| __t.string_controller_element_base_mut().previous_caret_pos = __v); }
                    { let __v = ((0) as f32) as f32; compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_ticks = __v); }
                }
                // TODO(fabric2seer): the condition: an assignment used as a value
                //   java: (caretTicks += a) % 20 <= 10
                if todo!("fabric2seer: the condition (TODO above)") {
                    graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), caret_x, (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().y()).wrapping_sub(2), caret_x.wrapping_add(1), (__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().y_limit()).wrapping_sub(1), -1);
                }
            }
        }
        graphics.disable_scissor();
        if { let __f = compat::dispatch(__this, |__t| __t.is_hovered_input_field()); __f(__this, (mouse_x) as f64, (mouse_y) as f64) } {
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&(if __this.lock().borrow().is_available() { IBEAM } else { NOT_ALLOWED }));
        } else if compat::read(__this, |__t| __t.string_controller_element_base().hovered) {
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&(if __this.lock().borrow().is_available() { POINTING_HAND } else { NOT_ALLOWED }));
        }
    }

    /// java: private boolean isHoveredInputField(double mouseX, double mouseY)
    pub fn is_hovered_input_field_body(__this: &compat::Shared<dyn StringControllerElement>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return { let (__a0, __a1,) = (&((mouse_x) as i32), &((mouse_y) as i32),); __this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().is_point_inside(__a0, __a1) };
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked_body(__this: &compat::Shared<dyn StringControllerElement>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        if __this.lock().borrow().is_available() && { let (__a0, __a1,) = (&((event.x()) as i32), &((event.y()) as i32),); (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).is_point_inside(__a0, __a1) } {
            { let __v = true; compat::write(__this, |__t| __t.string_controller_element_base_mut().input_field_focused = __v); }
            if !({ let __f = compat::dispatch(__this, |__t| __t.is_hovered_input_field()); __f(__this, event.x(), event.y()) }) {
                { let __v = ({ let __f = compat::dispatch(__this, |__t| __t.get_default_caret_pos()); __f(__this) }); compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
            } else {
                let mut text_x: i32 = ((event.x()) as i32).wrapping_sub((__this.lock().borrow().string_controller_element_base().input_field_bounds.as_ref().unwrap().x_limit()).wrapping_sub({ let (__q0,) = (&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__this.lock().borrow().get_value_text()),); __this.lock().borrow().string_controller_element_base().text_renderer.as_ref().unwrap().width_of_cached(__q0) }));
                let mut pos: i32 = -1;
                let mut current_width: i32 = 0;
                for ch in __this.lock().borrow().string_controller_element_base().input_field.chars().collect::<Vec<char>>() {
                    pos += 1;
                    let mut char_length: i32 = { let (__q0,) = (&ch.to_string(),); __this.lock().borrow().string_controller_element_base().text_renderer.as_ref().unwrap().width(__q0) };
                    if current_width.wrapping_add(char_length.wrapping_div(2)) > text_x {
                        { let __v = pos; compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                        break;
                    } else if pos == (compat::str_len(&__this.lock().borrow().string_controller_element_base().input_field)).wrapping_sub(1) {
                        { let __v = pos.wrapping_add(1); compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                    }
                    current_width = current_width.wrapping_add(char_length);
                }
                { let __v = 0; compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
            }
            return true;
        } else {
            ({ let __f = compat::dispatch(__this, |__t| __t.unfocus()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) });
        }
        return false;
    }

    /// java: protected int getDefaultCaretPos()
    pub fn get_default_caret_pos_body(__this: &compat::Shared<dyn StringControllerElement>) -> i32 {
        return compat::str_len(&__this.lock().borrow().string_controller_element_base().input_field);
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    pub fn key_pressed_body(__this: &compat::Shared<dyn StringControllerElement>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if !(compat::read(__this, |__t| __t.string_controller_element_base().input_field_focused)) {
            return false;
        }
        match event.key() {
            256 | mcrust::com::mojang::blaze3d::platform::input_constants::KEY_RETURN => {
                ({ let __f = compat::dispatch(__this, |__t| __t.unfocus()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) });
                return true;
            }
            263 => {
                if event.has_shift_down() {
                    if event.has_control_down() {
                        let mut space_char: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.find_space_index()); __f(__this, true) });
                        { let __v = (compat::read(__this, |__t| __t.string_controller_element_base().selection_length)).wrapping_add((compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)).wrapping_sub(space_char)); compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
                        { let __v = space_char; compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                    } else if compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) > 0 {
                        { let __v = compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) - 1; compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                        { let __v = (compat::read(__this, |__t| __t.string_controller_element_base().selection_length)).wrapping_add(1); compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
                    }
                    ({ let __f = compat::dispatch(__this, |__t| __t.check_render_offset()); __f(__this) });
                } else {
                    if compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) > 0 {
                        if event.has_control_down() {
                            { let __v = ({ let __f = compat::dispatch(__this, |__t| __t.find_space_index()); __f(__this, true) }); compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                        } else {
                            if compat::read(__this, |__t| __t.string_controller_element_base().selection_length) != 0 {
                                { let __v = (compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)).wrapping_add((compat::read(__this, |__t| __t.string_controller_element_base().selection_length)).min(0)); compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                            } else {
                                { let __v = compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) - 1; compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                            }
                        }
                    }
                    ({ let __f = compat::dispatch(__this, |__t| __t.check_render_offset()); __f(__this) });
                    { let __v = 0; compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
                }
                return true;
            }
            262 => {
                if event.has_shift_down() {
                    if event.has_control_down() {
                        let mut space_char: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.find_space_index()); __f(__this, false) });
                        { let __v = (compat::read(__this, |__t| __t.string_controller_element_base().selection_length)).wrapping_sub(space_char.wrapping_sub(compat::read(__this, |__t| __t.string_controller_element_base().caret_pos))); compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
                        { let __v = space_char; compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                    } else if compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) < compat::str_len(&__this.lock().borrow().string_controller_element_base().input_field) {
                        { let __v = compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) + 1; compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                        { let __v = (compat::read(__this, |__t| __t.string_controller_element_base().selection_length)).wrapping_sub(1); compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
                    }
                    ({ let __f = compat::dispatch(__this, |__t| __t.check_render_offset()); __f(__this) });
                } else {
                    if compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) < compat::str_len(&__this.lock().borrow().string_controller_element_base().input_field) {
                        if event.has_control_down() {
                            { let __v = ({ let __f = compat::dispatch(__this, |__t| __t.find_space_index()); __f(__this, false) }); compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                        } else {
                            if compat::read(__this, |__t| __t.string_controller_element_base().selection_length) != 0 {
                                { let __v = (compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)).wrapping_add((compat::read(__this, |__t| __t.string_controller_element_base().selection_length)).max(0)); compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                            } else {
                                { let __v = compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) + 1; compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                            }
                        }
                        ({ let __f = compat::dispatch(__this, |__t| __t.check_render_offset()); __f(__this) });
                    }
                    { let __v = 0; compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
                }
                return true;
            }
            259 => {
                ({ let __f = compat::dispatch(__this, |__t| __t.do_backspace()); __f(__this) });
                return true;
            }
            261 => {
                ({ let __f = compat::dispatch(__this, |__t| __t.do_delete()); __f(__this) });
                return true;
            }
            269 => {
                if event.has_shift_down() {
                    { let __v = (compat::read(__this, |__t| __t.string_controller_element_base().selection_length)).wrapping_sub((compat::str_len(&__this.lock().borrow().string_controller_element_base().input_field)).wrapping_sub(compat::read(__this, |__t| __t.string_controller_element_base().caret_pos))); compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
                } else {
                    { let __v = 0; compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
                }
                { let __v = compat::str_len(&__this.lock().borrow().string_controller_element_base().input_field); compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                ({ let __f = compat::dispatch(__this, |__t| __t.check_render_offset()); __f(__this) });
                return true;
            }
            268 => {
                if event.has_shift_down() {
                    { let __v = (compat::read(__this, |__t| __t.string_controller_element_base().selection_length)).wrapping_add(compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)); compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
                    { let __v = 0; compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                } else {
                    { let __v = 0; compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                    { let __v = 0; compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
                }
                ({ let __f = compat::dispatch(__this, |__t| __t.check_render_offset()); __f(__this) });
                return true;
            }
            _ => {}
        }
        if event.is_paste() {
            return ({ let __f = compat::dispatch(__this, |__t| __t.do_paste()); __f(__this) });
        } else if event.is_copy() {
            return ({ let __f = compat::dispatch(__this, |__t| __t.do_copy()); __f(__this) });
        } else if event.is_cut() {
            return ({ let __f = compat::dispatch(__this, |__t| __t.do_cut()); __f(__this) });
        } else if event.is_select_all() {
            return ({ let __f = compat::dispatch(__this, |__t| __t.do_select_all()); __f(__this) });
        }
        return false;
    }

    /// java: protected boolean doPaste()
    pub fn do_paste_body(__this: &compat::Shared<dyn StringControllerElement>) -> bool {
        // TODO(fabric2seer): `get_clipboard` takes 1 argument(s) in the port (`clipboard: &mut dyn ClipboardAccess`), Java passes 0
        // rust: ({ let __f = compat::dispatch(__this, |__t| __t.write()); __f(__this, compat::minecraft().keyboard_handler.get_clipboard()) });
        ({ let __f = compat::dispatch(__this, |__t| __t.update_undo_history()); __f(__this) });
        return true;
    }

    /// java: protected boolean doCopy()
    pub fn do_copy_body(__this: &compat::Shared<dyn StringControllerElement>) -> bool {
        if compat::read(__this, |__t| __t.string_controller_element_base().selection_length) != 0 {
            // TODO(fabric2seer): `set_clipboard` takes 2 argument(s) in the port (`clipboard: &mut dyn ClipboardAccess, value: &str`), Java passes 1
            // rust: compat::minecraft().keyboard_handler.set_clipboard(&mut ({ let __f = compat::dispatch(__this, |__t| __t.get_selection()); __f(__this) }));
            return true;
        }
        return false;
    }

    /// java: protected boolean doCut()
    pub fn do_cut_body(__this: &compat::Shared<dyn StringControllerElement>) -> bool {
        if compat::read(__this, |__t| __t.string_controller_element_base().selection_length) != 0 {
            // TODO(fabric2seer): `set_clipboard` takes 2 argument(s) in the port (`clipboard: &mut dyn ClipboardAccess, value: &str`), Java passes 1
            // rust: compat::minecraft().keyboard_handler.set_clipboard(&mut ({ let __f = compat::dispatch(__this, |__t| __t.get_selection()); __f(__this) }));
            ({ let __f = compat::dispatch(__this, |__t| __t.write()); __f(__this, "".to_string()) });
            ({ let __f = compat::dispatch(__this, |__t| __t.update_undo_history()); __f(__this) });
            return true;
        }
        return false;
    }

    /// java: protected boolean doSelectAll()
    pub fn do_select_all_body(__this: &compat::Shared<dyn StringControllerElement>) -> bool {
        { let __v = compat::str_len(&__this.lock().borrow().string_controller_element_base().input_field); compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
        ({ let __f = compat::dispatch(__this, |__t| __t.check_render_offset()); __f(__this) });
        { let __v = (-compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)); compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
        return true;
    }

    /// java: protected void checkRenderOffset()
    pub fn check_render_offset_body(__this: &compat::Shared<dyn StringControllerElement>) {
        if { let (__q0,) = (&compat::read(__this, |__t| __t.string_controller_element_base().input_field.clone()),); __this.lock().borrow().string_controller_element_base().text_renderer.as_ref().unwrap().width(__q0) } < ({ let __f = compat::dispatch(__this, |__t| __t.get_unshifted_length()); __f(__this) }) {
            { let __v = 0; compat::write(__this, |__t| __t.string_controller_element_base_mut().render_offset = __v); }
            return;
        }
        let mut text_x: i32 = (((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x_limit()).wrapping_sub({ let (__q0,) = (&compat::read(__this, |__t| __t.string_controller_element_base().input_field.clone()),); __this.lock().borrow().string_controller_element_base().text_renderer.as_ref().unwrap().width(__q0) })).wrapping_sub(__this.lock().borrow().get_x_padding());
        let mut caret_x: i32 = text_x.wrapping_add({ let (__q0,) = (&compat::substring2(&__this.lock().borrow().string_controller_element_base().input_field, 0, compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)),); __this.lock().borrow().string_controller_element_base().text_renderer.as_ref().unwrap().width(__q0) });
        let mut min_x: i32 = (((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x_limit()).wrapping_sub(__this.lock().borrow().get_x_padding())).wrapping_sub({ let __f = compat::dispatch(__this, |__t| __t.get_unshifted_length()); __f(__this) });
        let mut max_x: i32 = min_x.wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.get_unshifted_length()); __f(__this) });
        if caret_x.wrapping_add(compat::read(__this, |__t| __t.string_controller_element_base().render_offset)) < min_x {
            { let __v = min_x.wrapping_sub(caret_x); compat::write(__this, |__t| __t.string_controller_element_base_mut().render_offset = __v); }
        } else if caret_x.wrapping_add(compat::read(__this, |__t| __t.string_controller_element_base().render_offset)) > max_x {
            { let __v = max_x.wrapping_sub(caret_x); compat::write(__this, |__t| __t.string_controller_element_base_mut().render_offset = __v); }
        }
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent event)
    pub fn char_typed_body(__this: &compat::Shared<dyn StringControllerElement>, event: &CharacterEvent) -> bool {
        if !(compat::read(__this, |__t| __t.string_controller_element_base().input_field_focused)) {
            return false;
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.write()); __f(__this, event.codepoint_as_string()) });
        ({ let __f = compat::dispatch(__this, |__t| __t.update_undo_history()); __f(__this) });
        return true;
    }

    /// java: protected void doBackspace()
    pub fn do_backspace_body(__this: &compat::Shared<dyn StringControllerElement>) {
        if compat::read(__this, |__t| __t.string_controller_element_base().selection_length) != 0 {
            ({ let __f = compat::dispatch(__this, |__t| __t.write()); __f(__this, "".to_string()) });
        } else if compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) > 0 {
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            if { let __f = compat::dispatch(__this, |__t| __t.modify_input()); __f(__this, Arc::new(move |builder| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { compat::delete_char_at(&mut builder, (compat::read((&__this_1), |__t| __t.string_controller_element_base().caret_pos)).wrapping_sub(1)); } })) } {
                { let __v = compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) - 1; compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                ({ let __f = compat::dispatch(__this, |__t| __t.check_render_offset()); __f(__this) });
            }
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.update_undo_history()); __f(__this) });
    }

    /// java: protected void doDelete()
    pub fn do_delete_body(__this: &compat::Shared<dyn StringControllerElement>) {
        if compat::read(__this, |__t| __t.string_controller_element_base().selection_length) != 0 {
            ({ let __f = compat::dispatch(__this, |__t| __t.write()); __f(__this, "".to_string()) });
        } else if compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) < compat::str_len(&__this.lock().borrow().string_controller_element_base().input_field) {
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            ({ let __f = compat::dispatch(__this, |__t| __t.modify_input()); __f(__this, Arc::new(move |builder| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { compat::delete_char_at(&mut builder, compat::read((&__this_1), |__t| __t.string_controller_element_base().caret_pos)); } })) });
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.update_undo_history()); __f(__this) });
    }

    /// java: public void write(String string)
    pub fn write_body(__this: &compat::Shared<dyn StringControllerElement>, mut string: String) {
        if compat::read(__this, |__t| __t.string_controller_element_base().selection_length) == 0 {
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            if { let __f = compat::dispatch(__this, |__t| __t.modify_input()); __f(__this, Arc::new(move |builder| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { compat::insert_str(&mut builder, compat::read((&__this_1), |__t| __t.string_controller_element_base().caret_pos), &string.to_string()); } })) } {
                { let __v = (compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)).wrapping_add(compat::str_len(&string)); compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                ({ let __f = compat::dispatch(__this, |__t| __t.check_render_offset()); __f(__this) });
            }
        } else {
            let mut start: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.get_selection_start()); __f(__this) });
            let mut end: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.get_selection_end()); __f(__this) });
            // TODO(fabric2seer): the condition: StringBuilder.replace
            //   java: modifyInput(builder -> builder.replace(start, end, string))
            if todo!("fabric2seer: the condition (TODO above)") {
                { let __v = start.wrapping_add(compat::str_len(&string)); compat::write(__this, |__t| __t.string_controller_element_base_mut().caret_pos = __v); }
                { let __v = 0; compat::write(__this, |__t| __t.string_controller_element_base_mut().selection_length = __v); }
                ({ let __f = compat::dispatch(__this, |__t| __t.check_render_offset()); __f(__this) });
            }
        }
    }

    /// java: public boolean modifyInput(Consumer<StringBuilder> consumer)
    pub fn modify_input_body(__this: &compat::Shared<dyn StringControllerElement>, consumer: Arc<dyn Fn(String)>) -> bool {
        let mut temp: String = compat::read(__this, |__t| __t.string_controller_element_base().input_field.clone());
        (consumer)(temp.clone());
        // TODO(fabric2seer): the condition: `.isInputValid(..)` on a T
        //   java: !control.isInputValid(temp.toString())
        if todo!("fabric2seer: the condition (TODO above)") {
            return false;
        }
        { let __v = temp.clone(); compat::write(__this, |__t| __t.string_controller_element_base_mut().input_field = __v); }
        if compat::read(__this, |__t| __t.string_controller_element_base().instant_apply) {
            ({ let __f = compat::dispatch(__this, |__t| __t.update_control()); __f(__this) });
        }
        return true;
    }

    /// java: protected void updateUndoHistory()
    pub fn update_undo_history_body(__this: &compat::Shared<dyn StringControllerElement>) {
    }

    /// java: public int getUnshiftedLength()
    pub fn get_unshifted_length_body(__this: &compat::Shared<dyn StringControllerElement>) -> i32 {
        if __this.lock().borrow().string_controller_element_base().option_name_string.is_empty() {
            return ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).width()).wrapping_sub((__this.lock().borrow().get_x_padding()).wrapping_mul(2));
        }
        return (((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).width()).wrapping_div(8)).wrapping_mul(5);
    }

    /// java: public int getMaxUnwrapLength()
    pub fn get_max_unwrap_length_body(__this: &compat::Shared<dyn StringControllerElement>) -> i32 {
        if __this.lock().borrow().string_controller_element_base().option_name_string.is_empty() {
            return ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).width()).wrapping_sub((__this.lock().borrow().get_x_padding()).wrapping_mul(2));
        }
        return ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).width()).wrapping_div(2);
    }

    /// java: public int getSelectionStart()
    pub fn get_selection_start_body(__this: &compat::Shared<dyn StringControllerElement>) -> i32 {
        return (compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)).min((compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)).wrapping_add(compat::read(__this, |__t| __t.string_controller_element_base().selection_length)));
    }

    /// java: public int getSelectionEnd()
    pub fn get_selection_end_body(__this: &compat::Shared<dyn StringControllerElement>) -> i32 {
        return (compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)).max((compat::read(__this, |__t| __t.string_controller_element_base().caret_pos)).wrapping_add(compat::read(__this, |__t| __t.string_controller_element_base().selection_length)));
    }

    /// java: protected String getSelection()
    pub fn get_selection_body(__this: &compat::Shared<dyn StringControllerElement>) -> String {
        return compat::substring2(&__this.lock().borrow().string_controller_element_base().input_field, ({ let __f = compat::dispatch(__this, |__t| __t.get_selection_start()); __f(__this) }), ({ let __f = compat::dispatch(__this, |__t| __t.get_selection_end()); __f(__this) }));
    }

    /// java: protected int findSpaceIndex(boolean reverse)
    pub fn find_space_index_body(__this: &compat::Shared<dyn StringControllerElement>, mut reverse: bool) -> i32 {
        let mut i: i32;
        let mut from_index: i32 = compat::read(__this, |__t| __t.string_controller_element_base().caret_pos);
        if reverse {
            if compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) > 0 {
                from_index = from_index.wrapping_sub(2);
            }
            i = (compat::last_index_of_from(&__this.lock().borrow().string_controller_element_base().input_field, " ", from_index)).wrapping_add(1);
        } else {
            if compat::read(__this, |__t| __t.string_controller_element_base().caret_pos) < compat::str_len(&__this.lock().borrow().string_controller_element_base().input_field) {
                from_index = from_index.wrapping_add(1);
            }
            i = (compat::index_of_from(&__this.lock().borrow().string_controller_element_base().input_field, " ", from_index)).wrapping_add(1);
            if i == 0 {
                i = compat::str_len(&__this.lock().borrow().string_controller_element_base().input_field);
            }
        }
        return i;
    }

    /// java: @Override public void setFocused(boolean focused)
    pub fn set_focused_body(__this: &compat::Shared<dyn StringControllerElement>, mut focused: bool) {
        __this.base.set_focused(focused);
        { let __v = focused; compat::write(__this, |__t| __t.string_controller_element_base_mut().input_field_focused = __v); }
    }

    /// java: @Override public void unfocus()
    pub fn unfocus_body(__this: &compat::Shared<dyn StringControllerElement>) {
        __this.base.unfocus();
        { let __v = false; compat::write(__this, |__t| __t.string_controller_element_base_mut().input_field_focused = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.string_controller_element_base_mut().render_offset = __v); }
        if !(compat::read(__this, |__t| __t.string_controller_element_base().instant_apply)) {
            ({ let __f = compat::dispatch(__this, |__t| __t.update_control()); __f(__this) });
        }
    }

    /// StringControllerElement.unfocus as the dispatch of AbstractWidget.unfocus for the concrete class `__C`
    pub fn unfocus_from<__C: StringControllerElement + 'static>(__this: &compat::Shared<dyn AbstractWidget>) {
        <dyn StringControllerElement>::unfocus_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn StringControllerElement>))
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim)
    pub fn set_dimension_body(__this: &compat::Shared<dyn StringControllerElement>, mut dim: compat::Shared<dyn Dimension<i32>>) {
        <dyn AbstractWidget>::set_dimension_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), dim.clone());
        let mut width: i32 = 6i32.max(({ let (__q0,) = (&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__this.lock().borrow().get_value_text()),); __this.lock().borrow().string_controller_element_base().text_renderer.as_ref().unwrap().width_of_cached(__q0) }).min(({ let __f = compat::dispatch(__this, |__t| __t.get_unshifted_length()); __f(__this) })));
        { let __v = Some((dimension::of_int(((dim.lock().borrow().x_limit()).wrapping_sub(__this.lock().borrow().get_x_padding())).wrapping_sub(width), (dim.lock().borrow().center_y()).wrapping_sub((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_div(2)), width, mcrust::net::minecraft::client::gui::font::LINE_HEIGHT) as compat::Shared<dyn Dimension<compat::Object>>)); compat::write(__this, |__t| __t.string_controller_element_base_mut().input_field_bounds = __v); }
    }

    /// StringControllerElement.setDimension as the dispatch of AbstractWidget.setDimension for the concrete class `__C`
    pub fn set_dimension_from<__C: StringControllerElement + 'static>(__this: &compat::Shared<dyn AbstractWidget>, __p0: compat::Shared<dyn Dimension<i32>>) {
        <dyn StringControllerElement>::set_dimension_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn StringControllerElement>), __p0)
    }

    /// java: @Override public boolean isHovered()
    pub fn is_hovered_body(__this: &compat::Shared<dyn StringControllerElement>) -> bool {
        return (__this.base.is_hovered() || compat::read(__this, |__t| __t.string_controller_element_base().input_field_focused));
    }

    /// java: protected void updateControl()
    pub fn update_control_body(__this: &compat::Shared<dyn StringControllerElement>) {
        // TODO(fabric2seer): `.setFromString(..)` on a T
        //   java: control.setFromString(inputField);
    }

    /// java: @Override protected int getUnhoveredControlWidth()
    pub fn get_unhovered_control_width_body(__this: &compat::Shared<dyn StringControllerElement>) -> i32 {
        return (if !(__this.lock().borrow().is_hovered()) { (__this.lock().borrow().get_hovered_control_width()).min(({ let __f = compat::dispatch(__this, |__t| __t.get_max_unwrap_length()); __f(__this) })) } else { __this.lock().borrow().get_hovered_control_width() });
    }

    /// java: @Override protected int getHoveredControlWidth()
    pub fn get_hovered_control_width_body(__this: &compat::Shared<dyn StringControllerElement>) -> i32 {
        return ({ let (__q0,) = (&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__this.lock().borrow().get_value_text()),); __this.lock().borrow().string_controller_element_base().text_renderer.as_ref().unwrap().width_of_cached(__q0) }).min(({ let __f = compat::dispatch(__this, |__t| __t.get_unshifted_length()); __f(__this) }));
    }

    /// java: @Override protected Component getValueText()
    pub fn get_value_text_body(__this: &compat::Shared<dyn StringControllerElement>) -> MutableComponent {
        if !(compat::read(__this, |__t| __t.string_controller_element_base().input_field_focused)) && __this.lock().borrow().string_controller_element_base().input_field.is_empty() {
            return compat::read(__this, |__t| __t.string_controller_element_base().empty_text.clone());
        }
        // TODO(fabric2seer): `.formatValue(..)` on a T
        //   java: return instantApply || !inputFieldFocused ? control.formatValue() : Component.literal(inputField);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl StringControllerElement for StringControllerElementBase {
    fn string_controller_element_base(&self) -> &StringControllerElementBase {
        self
    }
    fn string_controller_element_base_mut(&mut self) -> &mut StringControllerElementBase {
        self
    }

}

impl ControllerWidget<compat::Shared<dyn crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController<compat::Object>>> for StringControllerElementBase {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<compat::Shared<dyn crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController<compat::Object>>> {
        &self.base
    }
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<compat::Shared<dyn crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController<compat::Object>>> {
        &mut self.base
    }

    /// java: @Override protected void extractHoveredControl(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_hovered_control(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
    }

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut value_text: MutableComponent = self.get_value_text();
        if !self.is_hovered() {
            // TODO(fabric2seer): `getMaxUnwrapLength(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
            //   java: valueText = Component.literal(GuiUtils.shortenString(valueText.getString(), textRenderer, getMaxUnwrapLength(), "...")).setStyle(valueText.getStyle());
        }
        // TODO(fabric2seer): `getDimension(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: int textX = getDimension().xLimit() - textRenderer.width(valueText) + renderOffset - getXPadding();
        let mut text_x: i32 = todo!("fabric2seer: the initialiser of `textX` (TODO above)");
        // TODO(fabric2seer): `GuiGraphicsExtractor.enableScissor` has no Rust method in the port (looked for `enable_scissor` in GuiGraphicsExtractor and its supertypes)
        // rust: graphics.enable_scissor(&self.input_field_bounds.as_ref().unwrap().lock().borrow().x(), (self.input_field_bounds.as_ref().unwrap().lock().borrow().y()).wrapping_sub(2), (self.input_field_bounds.as_ref().unwrap().lock().borrow().x_limit()).wrapping_add(1), (self.input_field_bounds.as_ref().unwrap().lock().borrow().y_limit()).wrapping_add(4));
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(value_text.clone()), text_x, self.get_text_y(), self.get_value_color(), true);
        if self.is_hovered() {
            self.ticks = (self.ticks + a);
            let mut text: String = self.get_value_text().get_string();
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.input_field_bounds.as_ref().unwrap().lock().borrow().x(), self.input_field_bounds.as_ref().unwrap().lock().borrow().y_limit(), self.input_field_bounds.as_ref().unwrap().lock().borrow().x_limit(), (self.input_field_bounds.as_ref().unwrap().lock().borrow().y_limit()).wrapping_add(1), -1);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (self.input_field_bounds.as_ref().unwrap().lock().borrow().x()).wrapping_add(1), (self.input_field_bounds.as_ref().unwrap().lock().borrow().y_limit()).wrapping_add(1), (self.input_field_bounds.as_ref().unwrap().lock().borrow().x_limit()).wrapping_add(1), (self.input_field_bounds.as_ref().unwrap().lock().borrow().y_limit()).wrapping_add(2), 0xFF404040u32 as i32);
            if self.input_field_focused || self.focused {
                if self.caret_pos > compat::str_len(&text) {
                    self.caret_pos = compat::str_len(&text);
                }
                let mut caret_x: i32 = text_x.wrapping_add(self.text_renderer.as_ref().unwrap().width(&compat::substring2(&text, 0, self.caret_pos)));
                if text.is_empty() {
                    caret_x = (self.input_field_bounds.as_ref().unwrap().lock().borrow().x()).wrapping_add((self.input_field_bounds.as_ref().unwrap().lock().borrow().width()).wrapping_div(2));
                }
                if self.selection_length != 0 {
                    let mut selection_x: i32 = text_x.wrapping_add(self.text_renderer.as_ref().unwrap().width(&compat::substring2(&text, 0, self.caret_pos.wrapping_add(self.selection_length))));
                    graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), caret_x, (self.input_field_bounds.as_ref().unwrap().lock().borrow().y()).wrapping_sub(2), selection_x, (self.input_field_bounds.as_ref().unwrap().lock().borrow().y_limit()).wrapping_sub(1), 0x803030FFu32 as i32);
                }
                if self.caret_pos != self.previous_caret_pos {
                    self.previous_caret_pos = self.caret_pos;
                    self.caret_ticks = ((0) as f32) as f32;
                }
                // TODO(fabric2seer): the condition: an assignment used as a value
                //   java: (caretTicks += a) % 20 <= 10
                if todo!("fabric2seer: the condition (TODO above)") {
                    graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), caret_x, (self.input_field_bounds.as_ref().unwrap().lock().borrow().y()).wrapping_sub(2), caret_x.wrapping_add(1), (self.input_field_bounds.as_ref().unwrap().lock().borrow().y_limit()).wrapping_sub(1), -1);
                }
            }
        }
        graphics.disable_scissor();
        // TODO(fabric2seer): the condition: `isHoveredInputField(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: this.isHoveredInputField(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&(if self.is_available() { IBEAM } else { NOT_ALLOWED }));
        } else if self.hovered {
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&(if self.is_available() { POINTING_HAND } else { NOT_ALLOWED }));
        }
    }

    /// java: @Override public void setFocused(boolean focused)
    fn set_focused(&mut self, mut focused: bool) {
        self.base.set_focused(focused);
        self.input_field_focused = focused;
    }

    /// java: @Override public boolean isHovered()
    fn is_hovered(&self) -> bool {
        return (self.base.is_hovered() || self.input_field_focused);
    }

    /// java: @Override protected int getUnhoveredControlWidth()
    fn get_unhovered_control_width(&self) -> i32 {
        // TODO(fabric2seer): `getMaxUnwrapLength(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: return !isHovered() ? Math.min(getHoveredControlWidth(), getMaxUnwrapLength()) : getHoveredControlWidth();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        // TODO(fabric2seer): `getUnshiftedLength(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: return Math.min(textRenderer.width(getValueText()), getUnshiftedLength());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override protected Component getValueText()
    fn get_value_text(&self) -> MutableComponent {
        if !self.input_field_focused && self.input_field.is_empty() {
            return self.empty_text.clone();
        }
        // TODO(fabric2seer): `.formatValue(..)` on a T
        //   java: return instantApply || !inputFieldFocused ? control.formatValue() : Component.literal(inputField);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl AbstractWidget for StringControllerElementBase {
    fn abstract_widget_base(&self) -> &AbstractWidgetBase {
        &self.base.base
    }
    fn abstract_widget_base_mut(&mut self) -> &mut AbstractWidgetBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void unfocus() (dispatch: the body for this class)
    fn unfocus(&self) -> fn(&compat::Shared<dyn AbstractWidget>) {
        <dyn StringControllerElement>::unfocus_from::<Self>
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim) (dispatch: the body for this class)
    fn set_dimension(&self) -> fn(&compat::Shared<dyn AbstractWidget>, compat::Shared<dyn Dimension<i32>>) {
        <dyn StringControllerElement>::set_dimension_from::<Self>
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
