//! Ported from dev.isxander.yacl3.gui.OptionListWidget.
//!
//! `public class OptionListWidget extends YACLSelectionList<OptionListWidget.Entry>`: the list half is
//! `YACLSelectionList<dyn Entry>`, whose overridable bodies this widget replaces through `SelectionListClass`. The inner
//! entry classes (`Entry`, `OptionEntry`, `GroupSeparatorEntry`, `ListGroupSeparatorEntry`, `EmptyListLabel`) hold the
//! list weakly (`OptionListWidget.this`); a group separator holds its child entries and a child holds its separator
//! weakly.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::api::list_option::AnyListOption;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::utils::dimension::{self, Dimension};
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::multi_line_label;
use crate::dev::isxander::yacl3::gui::description_with_name::DescriptionWithName;
use crate::dev::isxander::yacl3::gui::low_profile_button_widget::LowProfileButtonWidget;
use crate::dev::isxander::yacl3::gui::text_scaled_button_widget::TextScaledButtonWidget;
use crate::dev::isxander::yacl3::gui::tooltip_button_widget::TooltipButtonWidget;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::gui::yacl_selection_list::{
    self as selection_list, Entry as SelectionEntry, EntryBase, SelectionListClass, YACLSelectionList,
};
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER;
use compat::{Graphics, GuiEventListener};
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::SCROLLBAR_WIDTH;
use mcrust::net::minecraft::client::gui::components::multi_line_label::MultiLineLabel;
use mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
use mcrust::net::minecraft::client::gui::narration::narrated_element_type::NarratedElementType;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_direction::ScreenDirection;
use mcrust::net::minecraft::client::gui::text_alignment::TextAlignment;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use std::cell::{Cell, RefCell};
use std::sync::Weak;

/// java: public class OptionListWidget extends YACLSelectionList<OptionListWidget.Entry>
pub struct OptionListWidget {
    /// java: extends YACLSelectionList<OptionListWidget.Entry>
    pub list: YACLSelectionList<dyn Entry>,
    /// java: private final YACLScreen yaclScreen; (held weakly: the screen holds its widgets)
    yacl_screen: Weak<YACLScreen>,
    /// java: private final ConfigCategory category;
    category: Arc<dyn ConfigCategory>,
    /// java: private String searchQuery = "";
    search_query: RefCell<String>,
    /// java: private final Consumer<DescriptionWithName> hoverEvent;
    hover_event: Box<dyn Fn(DescriptionWithName)>,
    /// java: private DescriptionWithName lastHoveredOption;
    last_hovered_option: RefCell<Option<DescriptionWithName>>,
    /// `OptionListWidget.this` for the inner classes
    this: Weak<OptionListWidget>,
}

impl OptionListWidget {
    /// java: public OptionListWidget(YACLScreen screen, ConfigCategory category, Minecraft client, int x, int y,
    /// int width, int height, Consumer<DescriptionWithName> hoverEvent) - `super(client, width, height, y)`
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        screen: &Arc<YACLScreen>,
        category: Arc<dyn ConfigCategory>,
        _x: i32,
        y: i32,
        width: i32,
        height: i32,
        hover_event: impl Fn(DescriptionWithName) + 'static,
    ) -> Arc<OptionListWidget> {
        let widget = Arc::new_cyclic(|this| OptionListWidget {
            list: YACLSelectionList::new(width, height, y),
            yacl_screen: Arc::downgrade(screen),
            category: category.clone(),
            search_query: RefCell::new(String::new()),
            hover_event: Box::new(hover_event),
            last_hovered_option: RefCell::new(None),
            this: this.clone(),
        });

        widget.refresh_options();

        for group in category.groups() {
            if let Some(list_option) = group.as_list_option() {
                let weak = widget.this.clone();
                let listened = list_option.clone();
                let category = category.clone();
                list_option.add_refresh_listener(Arc::new(move || {
                    if let Some(this) = weak.upgrade() {
                        this.refresh_list_entries(&listened, &category);
                    }
                }));
            }
        }
        widget
    }

    /// `yaclScreen`
    fn yacl_screen(&self) -> Arc<YACLScreen> {
        self.yacl_screen.upgrade().expect("an option list outlived its screen")
    }

    /// java: public void refreshOptions()
    pub fn refresh_options(&self) {
        self.list.clear_entries();

        let yacl_screen = self.yacl_screen();
        for group in self.category.groups() {
            let group_separator_entry: Option<Arc<dyn GroupSeparatorEntryClass>> = if !group.is_root() {
                let separator: Arc<dyn GroupSeparatorEntryClass> = match group.as_list_option() {
                    Some(list_option) => ListGroupSeparatorEntry::new(self, list_option, &yacl_screen),
                    None => GroupSeparatorEntry::new(self, group.clone(), &yacl_screen),
                };
                self.add_entry(separator.clone());
                Some(separator)
            } else {
                None
            };

            let mut option_entries: Vec<Arc<dyn Entry>> = Vec::new();

            // add empty entry to make sure users know it's empty not just bugging out
            if let Some(separator) = &group_separator_entry
                && let Some(list_separator) = separator.dyn_any().downcast_ref::<ListGroupSeparatorEntry>()
                && OptionGroup::options(&*list_separator.list_option).is_empty()
            {
                let empty_list_label = EmptyListLabel::new(self, list_separator, &self.category);
                self.add_entry(empty_list_label.clone());
                option_entries.push(empty_list_label);
            }

            for option in OptionGroup::options(&*group) {
                let widget = option.provide_controller_widget(&yacl_screen, self.get_default_entry_dimension());
                let entry = OptionEntry::new(self, option, self.category.clone(), group.clone(), group_separator_entry.as_ref(), widget);
                self.add_entry(entry.clone());
                option_entries.push(entry);
            }

            if let Some(separator) = &group_separator_entry {
                separator.set_child_entries(option_entries);
            }
        }

        selection_list::set_scroll_amount(self, 0.0);
        selection_list::reposition_entries(self);
    }

    /// java: @Override protected int addEntry(Entry entry) - "instead of using super.defaultEntryHeight, use the
    /// height the entry wants to be - our entries set their height in the constructor"
    pub fn add_entry(&self, entry: Arc<dyn Entry>) -> i32 {
        let height = entry.get_height();
        selection_list::add_entry(self, entry, height)
    }

    /// java: private void refreshListEntries(ListOption<?> listOption, ConfigCategory category)
    fn refresh_list_entries(&self, list_option: &Arc<dyn AnyListOption>, category: &Arc<dyn ConfigCategory>) {
        // find group separator for group
        let group_separator: Option<Arc<dyn Entry>> = self.list.children().into_iter().find(|e| {
            e.dyn_any()
                .downcast_ref::<ListGroupSeparatorEntry>()
                .is_some_and(|gs| std::ptr::eq(Arc::as_ptr(&gs.list_option) as *const (), Arc::as_ptr(list_option) as *const ()))
        });

        let Some(group_separator) = group_separator else {
            LOGGER.warn(&format!(
                "Can't find group seperator to refresh list option entries for list option {}",
                AnyOption::name(&**list_option).get_string()
            ));
            return;
        };
        let separator = group_separator.dyn_any().downcast_ref::<ListGroupSeparatorEntry>().expect("found above");

        let old_children = std::mem::take(&mut *separator.fields.child_entries.borrow_mut());
        for entry in &old_children {
            selection_list::remove_entry(self, entry);
        }

        // if no entries, below loop won't run where addEntryBelow() recaches viewable children
        if OptionGroup::options(&**list_option).is_empty() {
            let empty_list_label: Arc<dyn Entry> = EmptyListLabel::new(self, separator, category);
            self.add_entry_below(&group_separator, empty_list_label.clone());
            separator.fields.child_entries.borrow_mut().push(empty_list_label);
            return;
        }

        let yacl_screen = self.yacl_screen();
        let separator_dyn: Arc<dyn GroupSeparatorEntryClass> =
            separator.this.upgrade().expect("the separator is in the list");
        let group: Arc<dyn OptionGroup> = list_option.clone();
        let mut last_entry = group_separator.clone();
        for list_option_entry in OptionGroup::options(&**list_option) {
            let widget = list_option_entry.provide_controller_widget(&yacl_screen, self.get_default_entry_dimension());
            let option_entry: Arc<dyn Entry> =
                OptionEntry::new(self, list_option_entry, category.clone(), group.clone(), Some(&separator_dyn), widget);
            self.add_entry_below(&last_entry, option_entry.clone());
            separator.fields.child_entries.borrow_mut().push(option_entry.clone());
            last_entry = option_entry;
        }
    }

    /// java: public Dimension<Integer> getDefaultEntryDimension()
    pub fn get_default_entry_dimension(&self) -> Arc<dyn Dimension<i32>> {
        dimension::of_int(self.get_row_left(), 0, self.get_row_width(), 20)
    }

    /// java: public void expandAllGroups()
    pub fn expand_all_groups(&self) {
        for entry in self.list.children() {
            if let Some(group_separator_entry) = as_group_separator(&entry) {
                group_separator_entry.set_expanded(true);
            }
        }
    }

    /// java: public void updateSearchQuery(String query)
    pub fn update_search_query(&self, query: &str) {
        *self.search_query.borrow_mut() = query.to_string();
        for entry in self.list.children() {
            entry.update_search_query(query);
        }
        self.expand_all_groups();
        selection_list::reposition_entries(self);
    }

    /// java: public void addEntryAtIndex(int index, Entry entry)
    pub fn add_entry_at_index(&self, index: usize, entry: Arc<dyn Entry>) {
        self.list.insert_child(index, entry);
        selection_list::reposition_entries(self);
    }

    /// java: public void addEntryBelow(Entry below, Entry entry)
    pub fn add_entry_below(&self, below: &Arc<dyn Entry>, entry: Arc<dyn Entry>) {
        let idx = self.list.index_of(below).map_or(0, |i| i + 1);

        if idx == 0 {
            panic!("IllegalStateException: The entry to insert below does not exist!");
        }

        self.add_entry_at_index(idx, entry);
    }

    /// java: public void addEntryBelowWithoutScroll(Entry below, Entry entry)
    pub fn add_entry_below_without_scroll(&self, below: &Arc<dyn Entry>, entry: Arc<dyn Entry>) {
        let d = self.list.content_height() as f64 - self.list.scroll_amount();
        self.add_entry_below(below, entry);
        selection_list::set_scroll_amount(self, self.list.content_height() as f64 - d);
    }

    /// java: private void setHoverDescription(DescriptionWithName description)
    fn set_hover_description(&self, description: DescriptionWithName) {
        let changed = match &*self.last_hovered_option.borrow() {
            None => true,
            // `description != lastHoveredOption` - the record's equals: its name and description
            Some(last) => last.name != description.name || !Arc::ptr_eq(&last.description, &description.description),
        };
        if changed {
            *self.last_hovered_option.borrow_mut() = Some(description.clone());
            (self.hover_event)(description);
        }
    }

    /// java: protected boolean isValidMouseClick(int button)
    pub fn is_valid_mouse_click(&self, button: i32) -> bool {
        button == input_constants::MOUSE_BUTTON_LEFT || button == input_constants::MOUSE_BUTTON_RIGHT || button == input_constants::MOUSE_BUTTON_MIDDLE
    }

    /// java: @Override protected Entry nextEntry(@NotNull ScreenDirection direction, @NotNull Predicate<Entry>
    /// predicate, Entry selected) - "ensure we don't focus unviewable entries"
    pub fn next_entry(&self, direction: ScreenDirection, predicate: &dyn Fn(&Arc<dyn Entry>) -> bool, selected: Option<&Arc<dyn Entry>>) -> Option<Arc<dyn Entry>> {
        selection_list::next_entry(self, direction, &|entry| entry.is_viewable() && predicate(entry), selected)
    }
}

impl SelectionListClass for OptionListWidget {
    type E = dyn Entry;

    fn list(&self) -> &YACLSelectionList<dyn Entry> {
        &self.list
    }

    /// java: @Override public int getRowLeft()
    fn get_row_left(&self) -> i32 {
        // super.getRowLeft()
        self.list.get_x() + self.list.get_width() / 2 - self.get_row_width() / 2 - SCROLLBAR_WIDTH
    }

    /// java: @Override public int getRowWidth() - 10 padding each side
    fn get_row_width(&self) -> i32 {
        self.list.get_width() - SCROLLBAR_WIDTH - 20
    }

    /// java: @Override protected void extractListBackground(@NonNull GuiGraphicsExtractor graphics)
    fn extract_list_background(&self, _graphics: &Graphics<'_>) {}
}

impl GuiEventListener for OptionListWidget {
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        selection_list::extract_render_state(self, graphics, mouse_x, mouse_y, a);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent event, boolean bl)
    fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        let at_position = self.list.get_entry_at_position(event.x(), event.y());
        for child in self.list.children() {
            if !at_position.as_ref().is_some_and(|at| selection_list::same(at, &child))
                && let Some(option_entry) = child.dyn_any().downcast_ref::<OptionEntry>()
            {
                option_entry.widget.unfocus();
            }
        }

        selection_list::mouse_clicked(self, event, double_click)
    }

    fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
        selection_list::mouse_released(self, event)
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    fn mouse_scrolled(&self, mouse_x: f64, mouse_y: f64, horizontal: f64, vertical: f64) -> bool {
        selection_list::mouse_scrolled(self, vertical);

        let mut index = 0;
        while let Some(child) = self.list.child(index) {
            if child.mouse_scrolled(mouse_x, mouse_y, horizontal, vertical) {
                break;
            }
            index += 1;
        }

        true
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent event, double deltaX, double deltaY)
    fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        if let Some(focused) = self.list.get_focused()
            && self.list.is_dragging()
            && self.is_valid_mouse_click(event.button())
        {
            return focused.mouse_dragged(event, dx, dy);
        }
        selection_list::mouse_dragged(self, event, dx, dy)
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyEvent)
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        let mut index = 0;
        while let Some(child) = self.list.child(index) {
            if child.key_pressed(event) {
                return true;
            }
            index += 1;
        }

        selection_list::key_pressed(self, event)
    }

    /// java: @Override public boolean charTyped(CharacterEvent characterEvent)
    fn char_typed(&self, event: &CharacterEvent) -> bool {
        let mut index = 0;
        while let Some(child) = self.list.child(index) {
            if child.char_typed(event) {
                return true;
            }
            index += 1;
        }

        selection_list::char_typed(self, event)
    }

    fn is_mouse_over(&self, x: f64, y: f64) -> bool {
        selection_list::is_mouse_over(self, x, y)
    }

    fn set_focused(&self, focused: bool) {
        selection_list::set_focused(self, focused)
    }

    fn is_focused(&self) -> bool {
        selection_list::is_focused(self)
    }
}

/// `entry instanceof GroupSeparatorEntry groupSeparatorEntry`
fn as_group_separator(entry: &Arc<dyn Entry>) -> Option<Arc<dyn GroupSeparatorEntryClass>> {
    if let Some(separator) = entry.dyn_any().downcast_ref::<GroupSeparatorEntry>() {
        return separator.this.upgrade().map(|s| s as Arc<dyn GroupSeparatorEntryClass>);
    }
    if let Some(separator) = entry.dyn_any().downcast_ref::<ListGroupSeparatorEntry>() {
        return separator.this.upgrade().map(|s| s as Arc<dyn GroupSeparatorEntryClass>);
    }
    None
}

// ---- java: public abstract class Entry extends YACLSelectionList.Entry<Entry> --------------------------------------

/// `OptionListWidget.Entry`'s field
pub struct EntryFields {
    /// java: protected boolean searchQueryMatches = true;
    pub search_query_matches: Cell<bool>,
    /// `OptionListWidget.this`
    pub list: Weak<OptionListWidget>,
}

impl EntryFields {
    /// java: public Entry() - `super(OptionListWidget.this)`
    pub fn new(list: &OptionListWidget) -> EntryFields {
        EntryFields { search_query_matches: Cell::new(true), list: list.this.clone() }
    }

    /// `OptionListWidget.this`
    pub fn list(&self) -> Arc<OptionListWidget> {
        self.list.upgrade().expect("an option list entry outlived its list")
    }
}

/// java: public abstract class Entry extends YACLSelectionList.Entry<Entry> - its overridable methods, each defaulting
/// to its body.
pub trait Entry: SelectionEntry {
    /// The class's field.
    fn entry_fields(&self) -> &EntryFields;

    /// java: public boolean updateSearchQuery(String searchQuery)
    fn update_search_query(&self, search_query: &str) -> bool {
        let matches = search_query.is_empty();
        if self.entry_fields().search_query_matches.get() != matches {
            self.entry_fields().search_query_matches.set(matches);
            self.refresh_visibility_state();
        }
        self.entry_fields().search_query_matches.get()
    }

    /// java: public boolean isViewable()
    fn is_viewable(&self) -> bool {
        self.entry_fields().search_query_matches.get()
    }

    /// java: protected void refreshVisibilityState()
    fn refresh_visibility_state(&self) {
        if self.is_viewable() {
            self.on_became_viewable();
        } else {
            self.on_became_hidden();
        }
    }

    /// java: protected void onBecameViewable()
    fn on_became_viewable(&self) {}

    /// java: protected void onBecameHidden()
    fn on_became_hidden(&self) {
        self.entry_base().set_height(0);
    }
}

/// java: @Override public int getHeight() - `Entry`'s override, for every subclass
pub fn entry_get_height<E: Entry + ?Sized>(this: &E) -> i32 {
    if !this.is_viewable() {
        return 0;
    }
    this.entry_base().height()
}

// ---- java: public class OptionEntry extends Entry ------------------------------------------------------------------

/// java: public class OptionEntry extends Entry
pub struct OptionEntry {
    base: EntryBase,
    fields: EntryFields,
    /// java: public final Option<?> option;
    pub option: Arc<dyn AnyOption>,
    /// java: public final ConfigCategory category;
    pub category: Arc<dyn ConfigCategory>,
    /// java: public final OptionGroup group;
    pub group: Arc<dyn OptionGroup>,
    /// java: public final @Nullable GroupSeparatorEntry groupSeparatorEntry;
    pub group_separator_entry: Option<Weak<dyn GroupSeparatorEntryClass>>,
    /// java: public final AbstractWidget widget;
    pub widget: Arc<dyn AbstractWidget>,
    /// java: private final TextScaledButtonWidget resetButton;
    reset_button: Option<Arc<TextScaledButtonWidget>>,
    /// java: private final String categoryName;
    #[allow(dead_code)]
    category_name: String,
    /// java: private final String groupName;
    group_name: String,
}

impl OptionEntry {
    /// java: public OptionEntry(Option<?> option, ConfigCategory category, OptionGroup group, @Nullable
    /// GroupSeparatorEntry groupSeparatorEntry, AbstractWidget widget)
    pub fn new(
        list: &OptionListWidget,
        option: Arc<dyn AnyOption>,
        category: Arc<dyn ConfigCategory>,
        group: Arc<dyn OptionGroup>,
        group_separator_entry: Option<&Arc<dyn GroupSeparatorEntryClass>>,
        widget: Arc<dyn AbstractWidget>,
    ) -> Arc<dyn Entry> {
        let category_name = ConfigCategory::name(&*category).get_string().to_lowercase();
        let group_name = OptionGroup::name(&*group).get_string().to_lowercase();
        let reset_button = if option.can_reset_to_default() && widget.can_reset() {
            widget.set_dimension(widget.get_dimension().expanded(-20, 0));
            let reset_option = option.clone();
            let button = Arc::new(TextScaledButtonWidget::new_8(
                &list.yacl_screen(),
                widget.get_dimension().x_limit(),
                -50,
                20,
                20,
                2.0,
                component::literal("\u{21BB}"),
                move |_button| reset_option.request_set_default(),
            ));
            // option.addListener((opt, val) -> this.resetButton.active = !opt.isPendingValueDefault() && opt.available());
            let listened = Arc::downgrade(&button);
            option.add_any_listener(Arc::new(move |opt: &dyn AnyOption| {
                if let Some(button) = listened.upgrade() {
                    button.button().fields.active.set(!opt.is_pending_value_default() && opt.available());
                }
            }));
            button.button().fields.active.set(!option.is_pending_value_default() && option.available());
            Some(button)
        } else {
            None
        };
        let entry = Arc::new(OptionEntry {
            base: EntryBase::new(),
            fields: EntryFields::new(list),
            option,
            category,
            group,
            group_separator_entry: group_separator_entry.map(Arc::downgrade),
            widget,
            reset_button,
            category_name,
            group_name,
        });
        entry.update_height();
        entry
    }

    /// `groupSeparatorEntry`
    fn group_separator(&self) -> Option<Arc<dyn GroupSeparatorEntryClass>> {
        self.group_separator_entry.as_ref().and_then(Weak::upgrade)
    }

    /// java: private void updateHeight()
    fn update_height(&self) {
        let reset_height = self.reset_button.as_ref().map_or(0, |button| button.button().fields.get_height());
        self.base.set_height(self.widget.get_dimension().height().max(reset_height) + 2);
    }

    /// `DescriptionWithName.of(option.name(), option.description())`
    fn description(&self) -> DescriptionWithName {
        DescriptionWithName::of(&self.option.name(), self.option.description())
    }
}

impl SelectionEntry for OptionEntry {
    fn entry_base(&self) -> &EntryBase {
        &self.base
    }

    fn get_height(&self) -> i32 {
        entry_get_height(self)
    }

    /// java: @Override public void extractContent(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY,
    /// boolean hovered, float a)
    fn extract_content(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, _hovered: bool, a: f32) {
        if !self.is_viewable() {
            return;
        }

        self.update_height();

        self.widget.set_dimension(self.widget.get_dimension().with_y(self.base.get_y()));

        self.widget.extract_render_state(graphics, mouse_x, mouse_y, a);

        if let Some(reset_button) = &self.reset_button {
            reset_button.button().fields.set_y(self.base.get_y());
            reset_button.extract_render_state(graphics, mouse_x, mouse_y, a);
        }

        if SelectionEntry::is_mouse_over(self, mouse_x as f64, mouse_y as f64) {
            self.fields.list().set_hover_description(self.description());
        }
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    fn mouse_scrolled(&self, mouse_x: f64, mouse_y: f64, horizontal: f64, vertical: f64) -> bool {
        self.widget.mouse_scrolled(mouse_x, mouse_y, horizontal, vertical)
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        self.widget.key_pressed(event)
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent event)
    fn char_typed(&self, event: &CharacterEvent) -> bool {
        self.widget.char_typed(event)
    }

    /// java: @Override public void setFocused(boolean focused)
    fn set_focused(&self, focused: bool) {
        if focused {
            self.fields.list().set_hover_description(self.description());
        }
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    fn children(&self) -> Vec<Arc<dyn GuiEventListener>> {
        let widget: Arc<dyn GuiEventListener> = self.widget.clone();
        match &self.reset_button {
            None => vec![widget],
            Some(reset_button) => {
                let reset: Arc<dyn GuiEventListener> = reset_button.clone();
                vec![widget, reset]
            }
        }
    }

    /// java: @Override public List<? extends NarratableEntry> narratables() - the widget (and the reset button)
    fn update_narration(&self, output: &mut dyn NarrationElementOutput) {
        self.widget.update_narration(output);
    }
}

impl Entry for OptionEntry {
    fn entry_fields(&self) -> &EntryFields {
        &self.fields
    }

    /// java: @Override public boolean updateSearchQuery(String searchQuery)
    fn update_search_query(&self, search_query: &str) -> bool {
        self.fields.search_query_matches.set(
            search_query.is_empty() || self.group_name.contains(search_query) || self.widget.matches_search(search_query),
        );
        self.refresh_visibility_state();
        self.fields.search_query_matches.get()
    }

    /// java: @Override public boolean isViewable()
    fn is_viewable(&self) -> bool {
        self.fields.search_query_matches.get() && self.group_separator().is_none_or(|separator| separator.is_expanded())
    }

    /// java: @Override protected void onBecameViewable()
    fn on_became_viewable(&self) {
        self.update_height();
    }
}

// ---- java: public class GroupSeparatorEntry extends Entry ----------------------------------------------------------

/// java: the fields of `GroupSeparatorEntry`
pub struct GroupSeparatorFields {
    /// java: protected final OptionGroup group;
    pub group: Arc<dyn OptionGroup>,
    /// java: protected final MultiLineLabel wrappedName;
    pub wrapped_name: MultiLineLabel,
    /// java: protected final MultiLineLabel wrappedTooltip;
    pub wrapped_tooltip: MultiLineLabel,
    /// java: protected final LowProfileButtonWidget expandMinimizeButton;
    pub expand_minimize_button: Arc<LowProfileButtonWidget>,
    /// java: protected final Screen screen;
    pub screen: Weak<YACLScreen>,
    /// java: protected boolean groupExpanded;
    pub group_expanded: Cell<bool>,
    /// java: protected List<Entry> childEntries = new ArrayList<>();
    pub child_entries: RefCell<Vec<Arc<dyn Entry>>>,
}

impl GroupSeparatorFields {
    /// java: private GroupSeparatorEntry(OptionGroup group, Screen screen) - the fields (the button presses
    /// `onExpandButtonPress()` on `this`)
    fn new(list: &OptionListWidget, group: Arc<dyn OptionGroup>, screen: &Arc<YACLScreen>, this: Weak<dyn GroupSeparatorEntryClass>) -> Self {
        let wrapped_name = multi_line_label(&OptionGroup::name(&*group), list.get_row_width() - 45);
        let wrapped_tooltip = multi_line_label(&OptionGroup::tooltip(&*group), screen.width() / 3 * 2 - 10);
        let group_expanded = !group.collapsed();
        let expand_minimize_button = Arc::new(LowProfileButtonWidget::new(0, 0, 20, 20, component::empty(), move |_btn| {
            if let Some(this) = this.upgrade() {
                this.on_expand_button_press();
            }
        }));
        GroupSeparatorFields {
            group,
            wrapped_name,
            wrapped_tooltip,
            expand_minimize_button,
            screen: Arc::downgrade(screen),
            group_expanded: Cell::new(group_expanded),
            child_entries: RefCell::new(Vec::new()),
        }
    }

    /// java: private int getYPadding()
    fn get_y_padding(&self) -> i32 {
        6
    }
}

/// java: the overridable methods of `GroupSeparatorEntry` (`ListGroupSeparatorEntry` overrides them)
pub trait GroupSeparatorEntryClass: Entry {
    /// The class's fields.
    fn group_separator(&self) -> &GroupSeparatorFields;

    /// java: public boolean isExpanded()
    fn is_expanded(&self) -> bool {
        self.group_separator().group_expanded.get()
    }

    /// java: public void setExpanded(boolean expanded)
    fn set_expanded(&self, expanded: bool) {
        set_expanded(self, expanded)
    }

    /// java: protected void onExpandButtonPress()
    fn on_expand_button_press(&self) {
        self.set_expanded(!self.is_expanded());
    }

    /// java: protected void updateExpandMinimizeText()
    fn update_expand_minimize_text(&self) {
        update_expand_minimize_text(self)
    }

    /// java: public void setChildEntries(List<? extends Entry> childEntries)
    fn set_child_entries(&self, child_entries: Vec<Arc<dyn Entry>>) {
        *self.group_separator().child_entries.borrow_mut() = child_entries;
    }
}

/// java: public void setExpanded(boolean expanded)
pub fn set_expanded<G: GroupSeparatorEntryClass + ?Sized>(this: &G, expanded: bool) {
    let fields = this.group_separator();
    if fields.group_expanded.get() == expanded {
        return;
    }

    fields.group_expanded.set(expanded);
    this.update_expand_minimize_text();
    let children = fields.child_entries.borrow().clone();
    for child in children {
        child.refresh_visibility_state();
    }
    selection_list::reposition_entries(&*this.entry_fields().list());
}

/// java: protected void updateExpandMinimizeText()
pub fn update_expand_minimize_text<G: GroupSeparatorEntryClass + ?Sized>(this: &G) {
    this.group_separator()
        .expand_minimize_button
        .button
        .fields
        .set_message(component::literal(if this.is_expanded() { "\u{25BC}" } else { "\u{25B6}" }));
}

/// java: private void updateHeight()
fn group_update_height<G: GroupSeparatorEntryClass + ?Sized>(this: &G) {
    let fields = this.group_separator();
    this.entry_base().set_height(fields.wrapped_name.get_line_count().max(1) * LINE_HEIGHT + fields.get_y_padding() * 2);
}

/// java: GroupSeparatorEntry.extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
fn group_extract_content<G: GroupSeparatorEntryClass + ?Sized>(this: &G, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
    if !this.is_viewable() {
        return;
    }

    group_update_height(this);

    let fields = this.group_separator();
    let base = this.entry_base();
    let button = &fields.expand_minimize_button.button.fields;
    let button_y = base.get_y() + this.get_height() / 2 - button.get_height() / 2 + 1;

    button.set_y(button_y);
    button.set_x(base.get_x());
    fields.expand_minimize_button.extract_render_state(graphics, mouse_x, mouse_y, a);

    // wrappedName.visitLines(TextAlignment.CENTER, x + width / 2, y + getYPadding(), font.lineHeight, graphics.textRenderer())
    fields.wrapped_name.visit_lines(
        TextAlignment::Center,
        base.get_x() + base.get_width() / 2,
        base.get_y() + fields.get_y_padding(),
        LINE_HEIGHT,
        &mut |x, y, text: &FormattedText| graphics.text(text, x, y, -1, true),
    );

    if SelectionEntry::is_mouse_over(this, mouse_x as f64, mouse_y as f64) {
        let group = &fields.group;
        this.entry_fields().list().set_hover_description(DescriptionWithName::of(&OptionGroup::name(&**group), OptionGroup::description(&**group)));
    }
}

/// java: @Override public boolean updateSearchQuery(String searchQuery)
fn group_update_search_query<G: GroupSeparatorEntryClass + ?Sized>(this: &G, search_query: &str) -> bool {
    let children = this.group_separator().child_entries.borrow().clone();
    let matches = search_query.is_empty() || children.iter().any(|e| e.update_search_query(search_query));
    this.entry_fields().search_query_matches.set(matches);
    matches
}

/// java: @Override public void setFocused(boolean focused)
fn group_set_focused<G: GroupSeparatorEntryClass + ?Sized>(this: &G, focused: bool) {
    if focused {
        let group = &this.group_separator().group;
        this.entry_fields().list().set_hover_description(DescriptionWithName::of(&OptionGroup::name(&**group), OptionGroup::description(&**group)));
    }
}

/// java: @Override public List<? extends NarratableEntry> narratables() - the group's title and tooltip
fn group_update_narration<G: GroupSeparatorEntryClass + ?Sized>(this: &G, output: &mut dyn NarrationElementOutput) {
    let group = &this.group_separator().group;
    output.add_component(NarratedElementType::Title, FormattedText::from(OptionGroup::name(&**group)));
    output.add_component(NarratedElementType::Hint, FormattedText::from(OptionGroup::tooltip(&**group)));
}

/// java: public class GroupSeparatorEntry extends Entry
pub struct GroupSeparatorEntry {
    base: EntryBase,
    fields_entry: EntryFields,
    /// the class's fields
    pub fields: GroupSeparatorFields,
    /// `this`
    this: Weak<GroupSeparatorEntry>,
}

impl GroupSeparatorEntry {
    /// java: private GroupSeparatorEntry(OptionGroup group, Screen screen)
    pub fn new(list: &OptionListWidget, group: Arc<dyn OptionGroup>, screen: &Arc<YACLScreen>) -> Arc<dyn GroupSeparatorEntryClass> {
        let entry = Arc::new_cyclic(|this: &Weak<GroupSeparatorEntry>| {
            let this_dyn: Weak<dyn GroupSeparatorEntryClass> = this.clone();
            GroupSeparatorEntry {
                base: EntryBase::new(),
                fields_entry: EntryFields::new(list),
                fields: GroupSeparatorFields::new(list, group, screen, this_dyn),
                this: this.clone(),
            }
        });
        entry.update_expand_minimize_text();
        group_update_height(&*entry);
        entry
    }
}

impl SelectionEntry for GroupSeparatorEntry {
    fn entry_base(&self) -> &EntryBase {
        &self.base
    }

    fn get_height(&self) -> i32 {
        entry_get_height(self)
    }

    fn extract_content(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, _hovered: bool, a: f32) {
        group_extract_content(self, graphics, mouse_x, mouse_y, a)
    }

    fn set_focused(&self, focused: bool) {
        group_set_focused(self, focused)
    }

    /// java: @Override public @NotNull List<? extends GuiEventListener> children()
    fn children(&self) -> Vec<Arc<dyn GuiEventListener>> {
        let button: Arc<dyn GuiEventListener> = self.fields.expand_minimize_button.clone();
        vec![button]
    }

    fn update_narration(&self, output: &mut dyn NarrationElementOutput) {
        group_update_narration(self, output)
    }
}

impl Entry for GroupSeparatorEntry {
    fn entry_fields(&self) -> &EntryFields {
        &self.fields_entry
    }

    fn update_search_query(&self, search_query: &str) -> bool {
        group_update_search_query(self, search_query)
    }
}

impl GroupSeparatorEntryClass for GroupSeparatorEntry {
    fn group_separator(&self) -> &GroupSeparatorFields {
        &self.fields
    }
}

// ---- java: public class ListGroupSeparatorEntry extends GroupSeparatorEntry ---------------------------------------

/// java: public class ListGroupSeparatorEntry extends GroupSeparatorEntry
pub struct ListGroupSeparatorEntry {
    base: EntryBase,
    fields_entry: EntryFields,
    /// the `GroupSeparatorEntry` fields
    pub fields: GroupSeparatorFields,
    /// java: private final ListOption<?> listOption;
    pub list_option: Arc<dyn AnyListOption>,
    /// java: private final TextScaledButtonWidget resetListButton;
    reset_list_button: Arc<TextScaledButtonWidget>,
    /// java: private final TooltipButtonWidget addListButton;
    add_list_button: Arc<TooltipButtonWidget>,
    /// `this`
    this: Weak<ListGroupSeparatorEntry>,
}

impl ListGroupSeparatorEntry {
    /// java: private ListGroupSeparatorEntry(ListOption<?> group, Screen screen)
    pub fn new(list: &OptionListWidget, group: Arc<dyn AnyListOption>, screen: &Arc<YACLScreen>) -> Arc<dyn GroupSeparatorEntryClass> {
        let row_right = selection_list::get_row_right(list);
        let entry = Arc::new_cyclic(|this: &Weak<ListGroupSeparatorEntry>| {
            let this_dyn: Weak<dyn GroupSeparatorEntryClass> = this.clone();
            let as_group: Arc<dyn OptionGroup> = group.clone();
            let reset_group = group.clone();
            let reset_list_button = Arc::new(TextScaledButtonWidget::new_8(
                screen,
                row_right - 20,
                -50,
                20,
                20,
                1.0,
                component::literal("\u{21BB}"),
                move |_button| reset_group.request_set_default(),
            ));
            let add_group = group.clone();
            let add_this = this.clone();
            let add_list_button = Arc::new(TooltipButtonWidget::new(
                screen,
                reset_list_button.button().fields.get_x() - 20,
                -50,
                20,
                20,
                component::literal("+"),
                Some(component::translatable("yacl.list.add_top")),
                move |_btn| {
                    add_group.insert_new_entry_any();
                    if let Some(this) = add_this.upgrade() {
                        this.set_expanded(true);
                    }
                },
            ));
            ListGroupSeparatorEntry {
                base: EntryBase::new(),
                fields_entry: EntryFields::new(list),
                fields: GroupSeparatorFields::new(list, as_group, screen, this_dyn),
                list_option: group.clone(),
                reset_list_button,
                add_list_button,
                this: this.clone(),
            }
        });

        // (the super constructor's) updateExpandMinimizeText(); updateHeight();
        entry.update_expand_minimize_text();
        group_update_height(&*entry);

        // group.addListener((opt, val) -> this.resetListButton.active = !opt.isPendingValueDefault() && opt.available());
        let listened = Arc::downgrade(&entry.reset_list_button);
        group.add_any_listener(Arc::new(move |opt: &dyn AnyOption| {
            if let Some(button) = listened.upgrade() {
                button.button().fields.active.set(!opt.is_pending_value_default() && opt.available());
            }
        }));
        entry.reset_list_button.button().fields.active.set(!group.is_pending_value_default() && AnyOption::available(&*group));

        entry.update_expand_minimize_text();
        entry.minimize_if_unavailable();
        entry
    }

    /// java: private void minimizeIfUnavailable()
    fn minimize_if_unavailable(&self) {
        if !AnyOption::available(&*self.list_option) && self.is_expanded() {
            self.set_expanded(false);
        }
    }
}

impl SelectionEntry for ListGroupSeparatorEntry {
    fn entry_base(&self) -> &EntryBase {
        &self.base
    }

    fn get_height(&self) -> i32 {
        entry_get_height(self)
    }

    /// java: @Override public void extractContent(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY,
    /// boolean hovered, float a)
    fn extract_content(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, _hovered: bool, a: f32) {
        if !self.is_viewable() {
            return;
        }

        // update every render because option could become available/unavailable at any time
        self.update_expand_minimize_text();

        group_extract_content(self, graphics, mouse_x, mouse_y, a);

        let button_y = self.fields.expand_minimize_button.button.fields.get_y();

        self.reset_list_button.button().fields.set_y(button_y);
        self.add_list_button.button.fields.set_y(button_y);

        self.reset_list_button.extract_render_state(graphics, mouse_x, mouse_y, a);
        self.add_list_button.extract_render_state(graphics, mouse_x, mouse_y, a);
    }

    fn set_focused(&self, focused: bool) {
        group_set_focused(self, focused)
    }

    /// java: @Override public @NotNull List<? extends GuiEventListener> children()
    fn children(&self) -> Vec<Arc<dyn GuiEventListener>> {
        let expand: Arc<dyn GuiEventListener> = self.fields.expand_minimize_button.clone();
        let add: Arc<dyn GuiEventListener> = self.add_list_button.clone();
        let reset: Arc<dyn GuiEventListener> = self.reset_list_button.clone();
        vec![expand, add, reset]
    }

    fn update_narration(&self, output: &mut dyn NarrationElementOutput) {
        group_update_narration(self, output)
    }
}

impl Entry for ListGroupSeparatorEntry {
    fn entry_fields(&self) -> &EntryFields {
        &self.fields_entry
    }

    fn update_search_query(&self, search_query: &str) -> bool {
        group_update_search_query(self, search_query)
    }
}

impl GroupSeparatorEntryClass for ListGroupSeparatorEntry {
    fn group_separator(&self) -> &GroupSeparatorFields {
        &self.fields
    }

    /// java: @Override protected void updateExpandMinimizeText()
    fn update_expand_minimize_text(&self) {
        update_expand_minimize_text(self);
        let expand = &self.fields.expand_minimize_button.button.fields;
        expand.active.set(AnyOption::available(&*self.list_option));
        self.add_list_button.button.fields.active.set(
            expand.active.get() && self.list_option.number_of_entries() < self.list_option.maximum_number_of_entries(),
        );
    }

    /// java: @Override public void setExpanded(boolean expanded)
    fn set_expanded(&self, expanded: bool) {
        set_expanded(self, AnyOption::available(&*self.list_option) && expanded);
    }
}

// ---- java: public class EmptyListLabel extends Entry ----------------------------------------------------------------

/// java: public class EmptyListLabel extends Entry
pub struct EmptyListLabel {
    base: EntryBase,
    fields: EntryFields,
    /// java: private final ListGroupSeparatorEntry parent; (held weakly: the separator holds its children)
    parent: Weak<ListGroupSeparatorEntry>,
    /// java: private final String groupName;
    group_name: String,
    /// java: private final String categoryName;
    #[allow(dead_code)]
    category_name: String,
}

impl EmptyListLabel {
    /// java: public EmptyListLabel(ListGroupSeparatorEntry parent, ConfigCategory category)
    pub fn new(list: &OptionListWidget, parent: &ListGroupSeparatorEntry, category: &Arc<dyn ConfigCategory>) -> Arc<dyn Entry> {
        let label = EmptyListLabel {
            base: EntryBase::new(),
            fields: EntryFields::new(list),
            parent: parent.this.clone(),
            group_name: OptionGroup::name(&*parent.fields.group).get_string().to_lowercase(),
            category_name: ConfigCategory::name(&**category).get_string().to_lowercase(),
        };
        label.base.set_height(11);
        Arc::new(label)
    }
}

impl SelectionEntry for EmptyListLabel {
    fn entry_base(&self) -> &EntryBase {
        &self.base
    }

    fn get_height(&self) -> i32 {
        entry_get_height(self)
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean
    /// hovered, float a)
    fn extract_content(&self, graphics: &Graphics<'_>, _mouse_x: i32, _mouse_y: i32, _hovered: bool, _a: f32) {
        if !self.is_viewable() {
            return;
        }

        let text = component::translatable("yacl.list.empty").with_style_formats(&[ChatFormatting::DarkGray, ChatFormatting::Italic]);
        graphics.centered_text(&FormattedText::from(text), self.base.get_x() + self.base.get_width() / 2, self.base.get_y(), -1);
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    fn children(&self) -> Vec<Arc<dyn GuiEventListener>> {
        Vec::new()
    }
}

impl Entry for EmptyListLabel {
    fn entry_fields(&self) -> &EntryFields {
        &self.fields
    }

    /// java: @Override public boolean updateSearchQuery(String searchQuery)
    fn update_search_query(&self, search_query: &str) -> bool {
        let matches = search_query.is_empty() || self.group_name.contains(search_query);
        self.fields.search_query_matches.set(matches);
        matches
    }

    /// java: @Override public boolean isViewable()
    fn is_viewable(&self) -> bool {
        self.parent.upgrade().is_some_and(|parent| parent.is_expanded()) && self.fields.search_query_matches.get()
    }

    /// java: @Override protected void onBecameViewable()
    fn on_became_viewable(&self) {
        self.base.set_height(11);
    }

    /// java: @Override protected void onBecameHidden()
    fn on_became_hidden(&self) {
        self.base.set_height(0);
        self.base.set_height(0);
    }
}

/// `OptionListWidget` for a field Java reads from the inner classes (`yaclScreen`).
impl OptionListWidget {
    /// java: `OptionListWidget.this.searchQuery`
    pub fn search_query(&self) -> String {
        self.search_query.borrow().clone()
    }
}
