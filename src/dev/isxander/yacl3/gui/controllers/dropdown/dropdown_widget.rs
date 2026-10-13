//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.DropdownWidget.
//!
//! `public class DropdownWidget<T> extends ControllerPopupWidget<AbstractDropdownController<T>>`: `T` appears only in
//! the controller the popup's `ControllerWidget` half is built over (`DropdownControl::popup_base`), so the Rust type
//! is not generic.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::{self, Dimension};
use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::{self, ControllerPopupWidget, ControllerPopupWidgetBase};
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::{ControllerWidget, ControllerWidgetBase};
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::{DropdownControl, DropdownElement};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::Graphics;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use std::cell::{Cell, RefCell};
use std::sync::Weak;

/// java: public static final int MAX_SHOWN_NUMBER_OF_ITEMS = 7;
pub const MAX_SHOWN_NUMBER_OF_ITEMS: i32 = 7;
/// java: public static final int DROPDOWN_PADDING = 2;
pub const DROPDOWN_PADDING: i32 = 2;

/// java: public class DropdownWidget<T> extends ControllerPopupWidget<AbstractDropdownController<T>>
pub struct DropdownWidget {
    /// java: extends ControllerPopupWidget<AbstractDropdownController<T>>
    pub popup: ControllerPopupWidgetBase,
    /// java: private final AbstractDropdownControllerElement<T, ?> dropdownElement; (held weakly: the element holds this)
    dropdown_element: Weak<dyn DropdownElement>,
    /// java: protected Dimension<Integer> dropdownDim;
    pub dropdown_dim: RefCell<Option<Arc<dyn Dimension<i32>>>>,
    /// java: protected int firstVisibleIndex = 0; - scroll offset in the list of possible values
    pub first_visible_index: Cell<i32>,
    /// java: protected int selectedIndex = 0; - the current selection position: the item at this position in the
    /// dropdown list will be chosen as the accepted value when the element is closed
    pub selected_index: Cell<i32>,
}

impl DropdownWidget {
    /// java: public DropdownWidget(AbstractDropdownController<T> control, YACLScreen screen, Dimension<Integer> dim,
    /// AbstractDropdownControllerElement<T, ?> dropdownElement)
    pub fn new(
        control: &DropdownControl,
        screen: &Arc<YACLScreen>,
        dim: Arc<dyn Dimension<i32>>,
        dropdown_element: Weak<dyn DropdownElement>,
        entry_widget: Weak<dyn ControllerWidget>,
    ) -> Arc<DropdownWidget> {
        let widget = Arc::new(DropdownWidget {
            popup: control.popup_base(screen, dim.clone(), entry_widget),
            dropdown_element,
            dropdown_dim: RefCell::new(None),
            first_visible_index: Cell::new(0),
            selected_index: Cell::new(0),
        });
        ControllerWidget::set_dimension(&*widget, dim);
        widget
    }

    /// `dropdownElement`
    fn element(&self) -> Arc<dyn DropdownElement> {
        self.dropdown_element.upgrade().expect("a dropdown outlived its element")
    }

    /// `dropdownDim`
    fn dropdown_dim(&self) -> Arc<dyn Dimension<i32>> {
        self.dropdown_dim.borrow().clone().expect("DropdownWidget.dropdownDim before setDimension")
    }

    /// java: public int entryHeight()
    pub fn entry_height(&self) -> i32 {
        self.element().element_dimension().height()
    }

    /// java: public int dropdownLength()
    pub fn dropdown_length(&self) -> i32 {
        self.element().dropdown_length()
    }

    /// java: public int numberOfVisibleItems()
    pub fn number_of_visible_items(&self) -> i32 {
        MAX_SHOWN_NUMBER_OF_ITEMS.min(self.dropdown_length())
    }

    /// java: public int lastVisibleIndex()
    pub fn last_visible_index(&self) -> i32 {
        (self.first_visible_index.get() + MAX_SHOWN_NUMBER_OF_ITEMS).min(self.dropdown_length())
    }

    /// java: public int selectedIndex()
    pub fn selected_index(&self) -> i32 {
        self.selected_index.get()
    }

    /// java: public void resetSelectedIndex()
    pub fn reset_selected_index(&self) {
        self.selected_index.set(0);
    }

    /// java: public int selectedVisibleIndex() - the offset of the selected element relative to the first visible entry
    pub fn selected_visible_index(&self) -> i32 {
        self.selected_index.get() - self.first_visible_index.get()
    }

    /// java: public void selectVisibleItem(int visibleIndex)
    pub fn select_visible_item(&self, visible_index: i32) {
        self.selected_index.set((self.first_visible_index.get() + visible_index).min(self.dropdown_length() - 1));
    }

    /// java: public void selectNextEntry()
    pub fn select_next_entry(&self) {
        if self.selected_index.get() == self.dropdown_length() - 1 {
            self.selected_index.set(0);
        } else {
            self.selected_index.set(self.selected_index.get() + 1);
        }
        if self.selected_index.get() - self.first_visible_index.get() >= MAX_SHOWN_NUMBER_OF_ITEMS / 2 {
            self.center_on_selected_item();
        }
    }

    /// java: public void selectPreviousEntry()
    pub fn select_previous_entry(&self) {
        if self.selected_index.get() == 0 {
            self.selected_index.set(self.dropdown_length() - 1);
        } else {
            self.selected_index.set(self.selected_index.get() - 1);
        }
        if self.selected_index.get() - self.first_visible_index.get() <= MAX_SHOWN_NUMBER_OF_ITEMS / 2 {
            self.center_on_selected_item();
        }
    }

    /// java: private void centerOnSelectedItem()
    fn center_on_selected_item(&self) {
        // Limit the visible options to allow scrolling through the suggestion list
        let mut begin = 0.max(self.selected_index.get() - MAX_SHOWN_NUMBER_OF_ITEMS / 2);
        let mut end = begin + MAX_SHOWN_NUMBER_OF_ITEMS;
        if end >= self.dropdown_length() {
            end = self.dropdown_length();
            begin = 0.max(end - MAX_SHOWN_NUMBER_OF_ITEMS);
        }
        self.first_visible_index.set(begin);
    }

    /// java: public void scrollDown()
    pub fn scroll_down(&self) {
        if self.first_visible_index.get() + 1 + MAX_SHOWN_NUMBER_OF_ITEMS <= self.dropdown_length() {
            self.first_visible_index.set(self.first_visible_index.get() + 1);
        }
        if self.selected_index.get() < self.first_visible_index.get() {
            self.selected_index.set(self.first_visible_index.get());
        }
    }

    /// java: public void scrollUp()
    pub fn scroll_up(&self) {
        if self.first_visible_index.get() > 0 {
            self.first_visible_index.set(self.first_visible_index.get() - 1);
        }
        if self.selected_index.get() > self.first_visible_index.get() + MAX_SHOWN_NUMBER_OF_ITEMS - 1 {
            self.selected_index.set(self.first_visible_index.get() + MAX_SHOWN_NUMBER_OF_ITEMS - 1);
        }
    }
}

impl ControllerWidget for DropdownWidget {
    fn controller_widget(&self) -> &ControllerWidgetBase {
        &self.popup.cw
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim)
    fn set_dimension(&self, dim: Arc<dyn Dimension<i32>>) {
        // super.setDimension(dim);
        self.popup.cw.base.set_dimension(dim.clone());

        // Set up the dropdown above the controller ...
        let dropdown_height = dim.height() * self.number_of_visible_items();
        let mut dropdown_y = dim.y() - dropdown_height - DROPDOWN_PADDING;
        // ... unless it doesn't fit, then place it below
        if dropdown_y < self.popup.cw.screen().tab_area().top() {
            dropdown_y = dim.y_limit() + DROPDOWN_PADDING;
        }

        let dropdown_dim: Arc<dyn Dimension<i32>> = dimension::of_int(dim.x(), dropdown_y, dim.width(), dropdown_height);
        *self.dropdown_dim.borrow_mut() = Some(dropdown_dim);
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&self, graphics: &Graphics<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
        if self.dropdown_length() == 0 {
            return;
        }

        let dropdown_dim = self.dropdown_dim();
        let element = self.element();
        // Background
        graphics.blit_with_color(
            &Identifier::with_default_namespace("textures/gui/menu_background.png"),
            dropdown_dim.x(),
            dropdown_dim.y(),
            0.0,
            0.0,
            dropdown_dim.width(),
            dropdown_dim.height(),
            32,
            32,
            0xFF3F3F3Fu32 as i32,
        );
        graphics.outline(dropdown_dim.x(), dropdown_dim.y(), dropdown_dim.width(), dropdown_dim.height(), -1);

        // Highlight the currently selected element
        let entry_height = self.entry_height();
        let y = dropdown_dim.y() + 2 + entry_height * self.selected_visible_index();
        graphics.fill(dropdown_dim.x(), y, dropdown_dim.x_limit(), y + entry_height, 0x7F000000);
        graphics.outline(dropdown_dim.x(), y, dropdown_dim.width(), entry_height, -1);

        // Render all visible elements
        let entry_dimension = dimension::of_int(
            dropdown_dim.x() - element.decoration_padding(),
            dropdown_dim.y() + 2,
            dropdown_dim.width(),
            entry_height,
        );
        for i in self.first_visible_index.get()..self.last_visible_index() {
            element.extract_dropdown_entry_at(graphics, &*entry_dimension, i);
            entry_dimension.r#move(0, entry_height);
        }
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        let element = self.element();
        if ControllerWidget::is_mouse_over(self, event.x(), event.y()) {
            // Closes and cleans up the dropdown
            element.element_unfocus();
            true
        } else if element.element_is_mouse_over(event.x(), event.y()) {
            element.element_mouse_clicked(event, double_click)
        } else {
            ControllerPopupWidget::close(self);
            false
        }
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    fn mouse_scrolled(&self, mouse_x: f64, mouse_y: f64, _horizontal: f64, vertical: f64) -> bool {
        if ControllerWidget::is_mouse_over(self, mouse_x, mouse_y) {
            if vertical < 0.0 {
                self.scroll_down();
            } else {
                self.scroll_up();
            }
            return true;
        }
        // super.mouseScrolled(..) - GuiEventListener's `false`
        false
    }

    /// java: @Override public void mouseMoved(double mouseX, double mouseY)
    fn mouse_moved(&self, mouse_x: f64, mouse_y: f64) {
        if ControllerWidget::is_mouse_over(self, mouse_x, mouse_y) {
            let index = ((mouse_y - self.dropdown_dim().y() as f64) / self.entry_height() as f64) as i32;
            self.select_visible_item(index);
        }
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.dropdown_dim().is_point_inside(mouse_x as i32, mouse_y as i32)
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent event)
    fn char_typed(&self, event: &CharacterEvent) -> bool {
        // Done to allow for typing whilst the color picker is visible
        self.element().element_char_typed(event)
    }

    /// java: ControllerPopupWidget.keyPressed(KeyEvent)
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        controller_popup_widget::key_pressed(self, event)
    }

    /// java: ControllerPopupWidget.getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        controller_popup_widget::get_hovered_control_width(self)
    }
}

impl ControllerPopupWidget for DropdownWidget {
    fn controller_popup_widget(&self) -> &ControllerPopupWidgetBase {
        &self.popup
    }

    /// java: @Override public void close()
    fn close(&self) {
        self.element().element_remove_dropdown_widget();
    }

    /// java: @Override public Component popupTitle()
    fn popup_title(&self) -> MutableComponent {
        component::translatable("yacl.control.dropdown.dropdown_widget_title")
    }
}
