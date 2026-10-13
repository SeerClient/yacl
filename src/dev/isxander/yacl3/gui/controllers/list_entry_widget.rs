//! Ported from dev.isxander.yacl3.gui.controllers.ListEntryWidget.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::list_option::{AnyListOption, ListOption};
use crate::dev::isxander::yacl3::api::list_option_entry::ListOptionEntry;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::{AbstractWidget, AbstractWidgetBase};
use crate::dev::isxander::yacl3::gui::tooltip_button_widget::TooltipButtonWidget;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::{ContainerEventHandler, Graphics, GuiEventListener};
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use std::sync::Weak;

/// `ListOption<?>` / `ListOptionEntry<?>` as the widget holds them: the members it calls that mention `T`, over the
/// one entry.
struct EntryControl {
    /// `listOption.indexOf(listOptionEntry)`
    index_of: Box<dyn Fn() -> i32>,
    /// `listOption.removeEntry(listOptionEntry)`
    remove: Box<dyn Fn()>,
    /// `listOption.insertEntry(index, listOptionEntry)`
    insert: Box<dyn Fn(i32)>,
}

/// java: public class ListEntryWidget extends AbstractWidget implements ContainerEventHandler
pub struct ListEntryWidget {
    /// java: extends AbstractWidget
    pub base: AbstractWidgetBase,
    /// java: private final TooltipButtonWidget removeButton, moveUpButton, moveDownButton;
    remove_button: Arc<TooltipButtonWidget>,
    move_up_button: Arc<TooltipButtonWidget>,
    move_down_button: Arc<TooltipButtonWidget>,
    /// java: private final AbstractWidget entryWidget;
    entry_widget: Arc<dyn AbstractWidget>,
    /// java: private final ListOption<?> listOption;
    list_option: Arc<dyn AnyListOption>,
    /// java: private final ListOptionEntry<?> listOptionEntry;
    entry: EntryControl,
    /// java: private final String optionNameString;
    option_name_string: String,
    /// java: private GuiEventListener focused; private boolean dragging;
    container: ContainerEventHandler,
}

impl ListEntryWidget {
    /// java: public ListEntryWidget(YACLScreen screen, ListOptionEntry<?> listOptionEntry, AbstractWidget entryWidget)
    pub fn new<T: Clone + 'static>(
        screen: &Arc<YACLScreen>,
        list_option_entry: Arc<dyn ListOptionEntry<T>>,
        entry_widget: Arc<dyn AbstractWidget>,
    ) -> Arc<dyn AbstractWidget> {
        let list_option: Arc<dyn ListOption<T>> = list_option_entry.parent_group();
        let last_index = OptionGroup::options(&*list_option).len() as i32 - 1;
        let entry_dim = entry_widget.get_dimension();
        // -2 to remove the padding
        let own_dim = entry_dim.with_height(
            entry_dim.height().min(20) - if list_option.index_of(&*list_option_entry) == last_index { 0 } else { 2 },
        );

        let option_name_string = AnyOption::name(&*list_option_entry).get_string().to_lowercase();
        let dim = entry_dim;
        let moved: Arc<dyn Dimension<i32>> = {
            // dim.clone() - `Dimension.clone()`, a mutable copy (not the `Arc`)
            let copy = Dimension::clone(&*dim);
            copy.r#move(20 * 2, 0).expand(-20 * 3, 0);
            copy
        };
        entry_widget.set_dimension(moved);

        let any_list: Arc<dyn AnyListOption> = list_option.clone();
        let entry = {
            let (l1, e1) = (list_option.clone(), list_option_entry.clone());
            let (l2, e2) = (list_option.clone(), list_option_entry.clone());
            let (l3, e3) = (list_option.clone(), list_option_entry.clone());
            EntryControl {
                index_of: Box::new(move || l1.index_of(&*e1)),
                remove: Box::new(move || l2.remove_entry(&*e2)),
                insert: Box::new(move |index| l3.insert_entry(index, e3.clone())),
            }
        };

        let widget = Arc::new_cyclic(|this: &Weak<ListEntryWidget>| {
            let w = this.clone();
            let remove_button = TooltipButtonWidget::new(
                screen,
                dim.x_limit() - 20,
                dim.y(),
                20,
                20,
                component::literal("\u{274c}"),
                Some(component::translatable("yacl.list.remove")),
                move |_btn| {
                    if let Some(this) = w.upgrade() {
                        (this.entry.remove)();
                        this.update_button_states();
                    }
                },
            );
            let w = this.clone();
            let move_up_button = TooltipButtonWidget::new(
                screen,
                dim.x(),
                dim.y(),
                20,
                20,
                component::literal("\u{2191}"),
                Some(component::translatable("yacl.list.move_up")),
                move |_btn| {
                    if let Some(this) = w.upgrade() {
                        let index = (this.entry.index_of)() - 1;
                        if index >= 0 {
                            (this.entry.remove)();
                            (this.entry.insert)(index);
                            this.update_button_states();
                        }
                    }
                },
            );
            let w = this.clone();
            let move_down_button = TooltipButtonWidget::new(
                screen,
                dim.x() + 20,
                dim.y(),
                20,
                20,
                component::literal("\u{2193}"),
                Some(component::translatable("yacl.list.move_down")),
                move |_btn| {
                    if let Some(this) = w.upgrade() {
                        let index = (this.entry.index_of)() + 1;
                        if index < OptionGroup::options(&*this.list_option).len() as i32 {
                            (this.entry.remove)();
                            (this.entry.insert)(index);
                            this.update_button_states();
                        }
                    }
                },
            );
            ListEntryWidget {
                base: AbstractWidgetBase::new(own_dim),
                remove_button: Arc::new(remove_button),
                move_up_button: Arc::new(move_up_button),
                move_down_button: Arc::new(move_down_button),
                entry_widget,
                list_option: any_list,
                entry,
                option_name_string,
                container: ContainerEventHandler::new_plain(),
            }
        });
        widget.update_button_states();
        widget
    }

    /// java: protected void updateButtonStates()
    fn update_button_states(&self) {
        let list_option = &self.list_option;
        let available = AnyOption::available(&**list_option);
        let index = (self.entry.index_of)();
        self.remove_button.button.fields.active.set(available && list_option.number_of_entries() > list_option.minimum_number_of_entries());
        self.move_up_button.button.fields.active.set(index > 0 && available);
        self.move_down_button.button.fields.active.set(index < OptionGroup::options(&**list_option).len() as i32 - 1 && available);
    }

    /// java: public List<? extends GuiEventListener> children()
    pub fn children(&self) -> Vec<Arc<dyn GuiEventListener>> {
        let move_up: Arc<dyn GuiEventListener> = self.move_up_button.clone();
        let move_down: Arc<dyn GuiEventListener> = self.move_down_button.clone();
        let entry_widget: Arc<dyn GuiEventListener> = self.entry_widget.clone();
        let remove: Arc<dyn GuiEventListener> = self.remove_button.clone();
        vec![move_up, move_down, entry_widget, remove]
    }

    /// java: public @Nullable GuiEventListener getFocused()
    pub fn get_focused(&self) -> Option<Arc<dyn GuiEventListener>> {
        self.container.get_focused()
    }

    /// java: public void setFocused(@Nullable GuiEventListener focused) - a plain field write here (not
    /// `AbstractContainerEventHandler`'s)
    pub fn set_focused_child(&self, focused: Option<Arc<dyn GuiEventListener>>) {
        self.container.set_focused_field(focused);
    }
}

impl GuiEventListener for ListEntryWidget {
    /// java: @Override public void extractRenderState(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        // update every render in case option becomes available/unavailable
        self.update_button_states();

        let y = self.base.get_dimension().y();
        self.remove_button.button.fields.set_y(y);
        self.move_up_button.button.fields.set_y(y);
        self.move_down_button.button.fields.set_y(y);
        self.entry_widget.set_dimension(self.entry_widget.get_dimension().with_y(y));

        self.remove_button.extract_render_state(graphics, mouse_x, mouse_y, a);
        self.move_up_button.extract_render_state(graphics, mouse_x, mouse_y, a);
        self.move_down_button.extract_render_state(graphics, mouse_x, mouse_y, a);
        self.entry_widget.extract_render_state(graphics, mouse_x, mouse_y, a);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent mouseButtonEvent, boolean doubleClick) -
    /// `ContainerEventHandler.super.mouseClicked(..)`
    fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.container.mouse_clicked(&self.children(), event, double_click)
    }

    /// java: @Override public boolean mouseReleased(MouseButtonEvent mouseButtonEvent)
    fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
        self.container.mouse_released(event)
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent mouseButtonEvent, double dx, double dy)
    fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        self.container.mouse_dragged(event, dx, dy)
    }

    /// `ContainerEventHandler.mouseScrolled` (inherited)
    fn mouse_scrolled(&self, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        ContainerEventHandler::mouse_scrolled(&self.children(), x, y, scroll_x, scroll_y)
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyEvent)
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        self.container.key_pressed(event)
    }

    /// java: @Override public boolean charTyped(CharacterEvent characterEvent)
    fn char_typed(&self, event: &CharacterEvent) -> bool {
        self.container.char_typed(event)
    }

    /// java: AbstractWidget.isMouseOver(double mouseX, double mouseY)
    fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.base.is_mouse_over(mouse_x, mouse_y)
    }

    /// `ContainerEventHandler.setFocused(boolean)` - `if (!focused) setFocused(null)`
    fn set_focused(&self, focused: bool) {
        if !focused {
            self.set_focused_child(None);
        }
    }

    /// `ContainerEventHandler.isFocused()` - `getFocused() != null`
    fn is_focused(&self) -> bool {
        self.container.is_focused()
    }
}

impl AbstractWidget for ListEntryWidget {
    fn set_dimension(&self, dim: Arc<dyn Dimension<i32>>) {
        self.base.set_dimension(dim);
    }

    fn get_dimension(&self) -> Arc<dyn Dimension<i32>> {
        self.base.get_dimension()
    }

    /// java: @Override public void unfocus()
    fn unfocus(&self) {
        self.entry_widget.unfocus();
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder)
    fn update_narration(&self, builder: &mut dyn NarrationElementOutput) {
        self.entry_widget.update_narration(builder);
    }

    /// java: @Override public boolean matchesSearch(String query)
    fn matches_search(&self, query: &str) -> bool {
        self.option_name_string.contains(&query.to_lowercase())
    }
}
