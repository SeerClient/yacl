//! Ported from dev.isxander.yacl3.gui.SearchFieldWidget.
//!
//! `extends EditBox`: the YACL screen draws and dispatches its own widgets (PATTERNS_Y2.md P3), so the box is a
//! `compat::EditBoxWidget` (the game's `EditBox` state and look).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::{EditBoxWidget, Graphics, GuiEventListener};
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::cell::{Cell, RefCell};
use std::sync::Weak;

/// java: public class SearchFieldWidget extends EditBox
pub struct SearchFieldWidget {
    /// java: extends EditBox
    pub edit_box: EditBoxWidget,
    /// java: private Component emptyText;
    empty_text: RefCell<MutableComponent>,
    /// java: private final YACLScreen yaclScreen; (held weakly: the screen holds its widgets)
    #[allow(dead_code)]
    yacl_screen: Weak<YACLScreen>,
    /// java: private boolean isEmpty = true; (shared with the responder)
    is_empty: Rc<Cell<bool>>,
    /// java: private boolean doNotUpdate = false; (shared with the responder)
    do_not_update: Rc<Cell<bool>>,
}

impl SearchFieldWidget {
    /// java: public SearchFieldWidget(YACLScreen yaclScreen, Font font, int x, int y, int width, int height,
    /// Component text, Component emptyText, Consumer<String> updateConsumer)
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        yacl_screen: &Arc<YACLScreen>,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        text: MutableComponent,
        empty_text: MutableComponent,
        update_consumer: impl Fn(&str) + 'static,
    ) -> Self {
        let edit_box = EditBoxWidget::new(x, y, width, height, text);
        let is_empty = Rc::new(Cell::new(true));
        let do_not_update = Rc::new(Cell::new(false));
        // setResponder(this::update);
        let (responder_empty, responder_do_not_update) = (is_empty.clone(), do_not_update.clone());
        edit_box.set_responder(move |query: &str| update(&responder_empty, &responder_do_not_update, &update_consumer, query));
        Self { edit_box, empty_text: RefCell::new(empty_text), yacl_screen: Arc::downgrade(yacl_screen), is_empty, do_not_update }
    }

    /// java: public void setValueDoNotUpdate(String value)
    pub fn set_value_do_not_update(&self, value: &str) {
        self.do_not_update.set(true);
        self.edit_box.set_value(value);
        self.do_not_update.set(false);
    }

    /// java: public String getQuery()
    pub fn get_query(&self) -> String {
        self.edit_box.get_value().to_lowercase()
    }

    /// java: public boolean isEmpty()
    pub fn is_empty(&self) -> bool {
        self.is_empty.get()
    }

    /// java: public Component getEmptyText()
    pub fn get_empty_text(&self) -> MutableComponent {
        self.empty_text.borrow().clone()
    }

    /// java: public void setEmptyText(Component emptyText)
    pub fn set_empty_text(&self, empty_text: MutableComponent) {
        *self.empty_text.borrow_mut() = empty_text;
    }
}

/// java: private void update(String query)
fn update(is_empty: &Cell<bool>, do_not_update: &Cell<bool>, update_consumer: &dyn Fn(&str), query: &str) {
    let was_empty = is_empty.get();
    is_empty.set(query.is_empty());

    if is_empty.get() && was_empty {
        return;
    }

    if do_not_update.get() {
        return;
    }
    update_consumer(query);
}

impl GuiEventListener for SearchFieldWidget {
    /// java: @Override public void extractWidgetRenderState(@NonNull GuiGraphicsExtractor graphics, int mouseX,
    /// int mouseY, float a)
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.edit_box.extract_render_state(graphics, mouse_x, mouse_y, a);
        if self.edit_box.is_visible() && self.is_empty() {
            let height = self.edit_box.get_height();
            graphics.text_component(&self.empty_text.borrow(), self.edit_box.get_x() + 4, self.edit_box.get_y() + (height - 8) / 2, 0x707070, true);
        }
    }
    fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.edit_box.mouse_clicked(event, double_click)
    }
    fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
        self.edit_box.mouse_released(event)
    }
    fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        self.edit_box.mouse_dragged(event, dx, dy)
    }
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        self.edit_box.key_pressed(event)
    }
    fn char_typed(&self, event: &CharacterEvent) -> bool {
        self.edit_box.char_typed(event)
    }
    fn is_mouse_over(&self, x: f64, y: f64) -> bool {
        self.edit_box.is_mouse_over(x, y)
    }
    fn set_focused(&self, focused: bool) {
        self.edit_box.set_focused(focused)
    }
    fn is_focused(&self) -> bool {
        self.edit_box.is_focused()
    }
}
