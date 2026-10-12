//! Ported from dev.isxander.yacl3.gui.OptionListWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::api::list_option::ListOption;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::utils::dimension;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::description_with_name::DescriptionWithName;
use crate::dev::isxander::yacl3::gui::low_profile_button_widget::LowProfileButtonWidget;
use crate::dev::isxander::yacl3::gui::text_scaled_button_widget::TextScaledButtonWidget;
use crate::dev::isxander::yacl3::gui::tooltip_button_widget::TooltipButtonWidget;
use crate::dev::isxander::yacl3::gui::tooltip_button_widget::TooltipButtonWidgetBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::gui::yacl_selection_list::YACLSelectionList;
use crate::dev::isxander::yacl3::gui::yacl_selection_list::YACLSelectionListBase;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::container_object_selection_list;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::multi_line_label::MultiLineLabel;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::gui::navigation::screen_direction::ScreenDirection;
use mcrust::net::minecraft::client::gui::text_alignment::TextAlignment;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use crate::dev::isxander::yacl3::api::controller::Controller as _;
use crate::dev::isxander::yacl3::api::option::Option as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class OptionListWidget extends YACLSelectionList<OptionListWidget.Entry>
pub struct OptionListWidget {
    /// java: extends YACLSelectionList
    pub base: YACLSelectionListBase<(_, _)>,
    /// java: private final YACLScreen yaclScreen;
    pub yacl_screen: compat::Shared<YACLScreen>,
    /// java: private final ConfigCategory category;
    pub category: compat::Shared<dyn ConfigCategory>,
    /// java: private String searchQuery = "";
    pub search_query: String,
    /// java: private final Consumer<DescriptionWithName> hoverEvent;
    pub hover_event: Arc<dyn Fn(DescriptionWithName)>,
    /// java: private DescriptionWithName lastHoveredOption;
    pub last_hovered_option: Option<DescriptionWithName>,
}

impl std::ops::Deref for OptionListWidget {
    type Target = YACLSelectionListBase<(_, _)>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for OptionListWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl OptionListWidget {
    /// java: public OptionListWidget(YACLScreen screen, ConfigCategory category, Minecraft client, int x, int y, int width, int height, Consumer<DescriptionWithName> hoverEvent)
    pub fn new(mut screen: compat::Shared<YACLScreen>, mut category: compat::Shared<dyn ConfigCategory>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut x: i32, mut y: i32, mut width: i32, mut height: i32, hover_event: Arc<dyn Fn(DescriptionWithName)>) -> compat::Shared<Self> {
        let __init_yacl_screen = screen.clone();
        let __init_category = category.clone();
        let __init_hover_event = hover_event.clone();
        let __this = compat::shared(Self {
            base: YACLSelectionListBase::<(_, _)>::new(client, width, height, y),
            yacl_screen: __init_yacl_screen,
            category: __init_category,
            search_query: "".to_string(),
            hover_event: __init_hover_event,
            last_hovered_option: None,
        });
        OptionListWidget::refresh_options(&__this);
        for group in ({ let __f = compat::dispatch(&category, |__t| __t.groups()); __f(&category) }) {
            if let Some(list_option) = group.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn ListOption<compat::Object>>>() {
                let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
                { let (__a0,) = (Arc::new(move || { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { OptionListWidget::refresh_list_entries(&__this_1, list_option, &category); } }),); list_option.lock().borrow_mut().add_refresh_listener(__a0) };
            }
        }
        __this
    }

    /// java: public void refreshOptions()
    pub fn refresh_options(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| { let __l = &mut __t.base.base; __l.clear_entries() });
        for group in ({ let __r = &compat::read(__this, |__t| __t.category.clone()); let __f = compat::dispatch(__r, |__t| __t.groups()); __f(__r) }) {
            let mut group_separator_entry: Option<compat::Shared<dyn GroupSeparatorEntry>> = None;
            if !({ let __f = compat::dispatch(&group, |__t| __t.is_root()); __f(&group) }) {
                // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
                //   java: groupSeparatorEntry = group instanceof ListOption<?> listOption
                //   java:                         ? new ListGroupSeparatorEntry(listOption, yaclScreen)
                //   java:                         : new GroupSeparatorEntry(group, yaclScreen);
                OptionListWidget::add_entry(__this, group_separator_entry.as_ref().unwrap());
            } else {
                group_separator_entry = None;
            }
            let mut option_entries: Vec<(_, _)> = Vec::new();
            if let Some(list_group_separator_entry) = group_separator_entry.as_ref().unwrap().lock().borrow().as_any().downcast_ref::<compat::Shared<ListGroupSeparatorEntry>>() {
                if (({ let __r = &compat::read(list_group_separator_entry, |__t| __t.list_option.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.options()); __f(&compat::view(__r, |a| a as compat::Shared<dyn OptionGroup>)) })).is_empty() {
                    let mut empty_list_label: compat::Shared<EmptyListLabel> = EmptyListLabel::new(list_group_separator_entry.clone(), compat::read(__this, |__t| __t.category.clone()));
                    OptionListWidget::add_entry(__this, &empty_list_label);
                    option_entries.push(empty_list_label.clone());
                }
            }
            for option in ({ let __f = compat::dispatch(&group, |__t| __t.options()); __f(&group) }) {
                let mut entry: compat::Shared<OptionEntry> = OptionEntry::new(option.clone(), compat::read(__this, |__t| __t.category.clone()), group.clone(), group_separator_entry.clone(), { let (__a0, __a1,) = (&compat::read(__this, |__t| __t.yacl_screen.clone()), &OptionListWidget::get_default_entry_dimension(__this),); option.lock().borrow().controller().lock().borrow().provide_widget(__a0, __a1) });
                OptionListWidget::add_entry(__this, &entry);
                option_entries.push(entry.clone());
            }
            if let Some(group_separator_entry) = &group_separator_entry {
                ({ let __f = compat::dispatch(group_separator_entry, |__t| __t.set_child_entries()); __f(group_separator_entry, &option_entries) });
            }
        }
        { let (__q0,) = ((0) as f64,); __this.lock().borrow_mut().set_scroll_amount(__q0) };
        __this.lock().borrow_mut().reposition_entries();
    }

    /// java: @Override protected int addEntry(Entry entry)
    pub fn add_entry(__this: &compat::Shared<Self>, entry: &(_, _)) -> i32 {
        // TODO(fabric2seer): no overload of `addEntry` takes 2 argument(s)
        //   java: return this.addEntry(entry, entry.getHeight());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private void refreshListEntries(ListOption<?> listOption, ConfigCategory category)
    pub fn refresh_list_entries(__this: &compat::Shared<Self>, list_option: &compat::Shared<dyn ListOption<compat::Object>>, category: &compat::Shared<dyn ConfigCategory>) {
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: ListGroupSeparatorEntry groupSeparator = this.children().stream().filter(e -> e instanceof ListGroupSeparatorEntry gs && gs.group == listOption).map(ListGroupSeparatorEntry.class::cast).findAny().orElse(null);
        let mut group_separator: compat::Shared<ListGroupSeparatorEntry> = todo!("fabric2seer: the initialiser of `groupSeparator` (TODO above)");
        if false /* java: group_separator == null: never null here */ {
            logger().warn(&format!("Can't find group seperator to refresh list option entries for list option {}", compat::jstring(&({ let __f = compat::dispatch(list_option, |__t| __t.name()); __f(&compat::view(list_option, |a| a as compat::Shared<dyn OptionGroup>)) }))));
            return;
        }
        for entry in compat::read(&group_separator, |__t| __t.child_entries.clone()) {
            let __entry = entry.clone();
            compat::write(__this, |__t| { let __l = &mut __t.base.base; { if let Some(__i) = compat::entry_index::<dyn Entry>(__l, &__entry) { __l.remove_entry(__i); } } });
        }
        group_separator.lock().borrow_mut().child_entries.clear();
        if (({ let __f = compat::dispatch(list_option, |__t| __t.options()); __f(&compat::view(list_option, |a| a as compat::Shared<dyn OptionGroup>)) })).is_empty() {
            let mut empty_list_label: compat::Shared<EmptyListLabel>;
            OptionListWidget::add_entry_below(__this, &group_separator, &{ empty_list_label = EmptyListLabel::new(group_separator.clone(), category.clone()); empty_list_label.clone() });
            let __h0 = empty_list_label.clone();
            group_separator.lock().borrow_mut().child_entries.push(__h0);
            return;
        }
        let mut last_entry: (_, _) = group_separator.clone();
        for list_option_entry in ({ let __f = compat::dispatch(list_option, |__t| __t.options()); __f(&compat::view(list_option, |a| a as compat::Shared<dyn OptionGroup>)) }) {
            let mut option_entry: compat::Shared<OptionEntry> = OptionEntry::new((list_option_entry.clone() as compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>), category.clone(), (list_option.clone() as compat::Shared<dyn OptionGroup>), Some((group_separator.clone() as compat::Shared<dyn GroupSeparatorEntry>)), { let (__a0, __a1,) = (&compat::read(__this, |__t| __t.yacl_screen.clone()), &OptionListWidget::get_default_entry_dimension(__this),); list_option_entry.lock().borrow().controller().lock().borrow().provide_widget(__a0, __a1) });
            OptionListWidget::add_entry_below(__this, &last_entry, &option_entry);
            let __h0 = option_entry.clone();
            group_separator.lock().borrow_mut().child_entries.push(__h0);
            last_entry = option_entry.clone();
        }
    }

    /// java: public Dimension<Integer> getDefaultEntryDimension()
    pub fn get_default_entry_dimension(__this: &compat::Shared<Self>) -> compat::Shared<dyn Dimension<i32>> {
        return (dimension::of_int(OptionListWidget::get_row_left(__this), 0, OptionListWidget::get_row_width(__this), 20) as compat::Shared<dyn Dimension<compat::Object>>);
    }

    /// java: public void expandAllGroups()
    pub fn expand_all_groups(__this: &compat::Shared<Self>) {
        for entry in compat::write(__this, |__t| { let __l = &mut __t.base.base; compat::list_entries::<dyn Entry>(__l) }) {
            if let Some(group_separator_entry) = entry.as_any().downcast_ref::<compat::Shared<dyn GroupSeparatorEntry>>() {
                ({ let __f = compat::dispatch(group_separator_entry, |__t| __t.set_expanded()); __f(group_separator_entry, true) });
            }
        }
    }

    /// java: @Override public int getRowLeft()
    pub fn get_row_left(__this: &compat::Shared<Self>) -> i32 {
        return (compat::super_answer::<i32>()).wrapping_sub(compat::read(__this, |__t| __t.scrollbar_width()));
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(__this: &compat::Shared<Self>) -> i32 {
        return ((__this.lock().borrow().get_width()).wrapping_sub(compat::read(__this, |__t| __t.scrollbar_width()))).wrapping_sub(20);
    }

    /// java: public void updateSearchQuery(String query)
    pub fn update_search_query(__this: &compat::Shared<Self>, mut query: String) {
        { let __v = query.clone(); compat::write(__this, |__t| __t.search_query = __v); }
        for entry in compat::write(__this, |__t| { let __l = &mut __t.base.base; compat::list_entries::<dyn Entry>(__l) }) {
            // TODO(fabric2seer): Map.Entry.updateSearchQuery
            //   java: entry.updateSearchQuery(query);
        }
        OptionListWidget::expand_all_groups(__this);
        __this.lock().borrow_mut().reposition_entries();
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent event, boolean bl)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut bl: bool) -> bool {
        for child in compat::write(__this, |__t| { let __l = &mut __t.base.base; compat::list_entries::<dyn Entry>(__l) }) {
            if child != { let (__q0, __q1,) = (event.x(), event.y(),); __this.lock().borrow().get_entry_at_position(__q0, __q1) }.unwrap() && let Some(option_entry) = child.as_any().downcast_ref::<compat::Shared<OptionEntry>>() {
                ({ let __r = &compat::read(option_entry, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.unfocus()); __f(__r) });
            }
        }
        // TODO(fabric2seer): `AbstractContainerWidget::mouse_clicked` ports `mouseClicked`, but the port's `AbstractSelectionList` holds no `AbstractContainerWidget` (no Deref, no `base` field) to call it on
        // rust: return { let (__q0, __q1,) = (event.clone(), bl,); __this.lock().borrow_mut().base.list.mouse_clicked(__q0, __q1) };
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal: f64, mut vertical: f64) -> bool {
        // TODO(fabric2seer): `mouse_scrolled` takes 1 argument(s) in the port (`scroll_y: f64`), Java passes 4
        // rust: __this.lock().borrow_mut().base.list.mouse_scrolled(mouse_x, mouse_y, horizontal, vertical);
        for child in compat::write(__this, |__t| { let __l = &mut __t.base.base; compat::list_entries::<dyn Entry>(__l) }) {
            // TODO(fabric2seer): the condition: Map.Entry.mouseScrolled
            //   java: child.mouseScrolled(..)
            if todo!("fabric2seer: the condition (TODO above)") {
                break;
            }
        }
        return true;
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent event, double deltaX, double deltaY)
    pub fn mouse_dragged(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut delta_x: f64, mut delta_y: f64) -> bool {
        // TODO(fabric2seer): `AbstractContainerWidget::is_dragging` ports `isDragging`, but the port's `AbstractSelectionList` holds no `AbstractContainerWidget` (no Deref, no `base` field) to call it on
        if __this.lock().borrow().get_focused().is_some() && __this.lock().borrow().is_dragging() && OptionListWidget::is_valid_mouse_click(__this, event.button()) {
            let mut l: Box<dyn GuiEventListener> = __this.lock().borrow().get_focused().unwrap();
            // TODO(fabric2seer): `mouse_dragged` takes 5 argument(s) in the port (`_button: i32, _x: f64, _y: f64, _dx: f64, _dy: f64`), Java passes 3
            // rust: return l.mouse_dragged(event.clone(), delta_x, delta_y);
            return todo!("fabric2seer: see the TODO above");
        }
        // TODO(fabric2seer): `mouse_dragged` takes 2 argument(s) in the port (`event_y: f64, dy: f64`), Java passes 3
        // rust: return { let (__q0, __q1, __q2,) = (event.clone(), delta_x, delta_y,); __this.lock().borrow_mut().base.list.mouse_dragged(__q0, __q1, __q2) };
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyEvent)
    pub fn key_pressed(__this: &compat::Shared<Self>, key_event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        for child in compat::write(__this, |__t| { let __l = &mut __t.base.base; compat::list_entries::<dyn Entry>(__l) }) {
            // TODO(fabric2seer): the condition: Map.Entry.keyPressed
            //   java: child.keyPressed(..)
            if todo!("fabric2seer: the condition (TODO above)") {
                return true;
            }
        }
        // TODO(fabric2seer): `key_pressed` takes 3 argument(s) in the port (`_key: i32, _scancode: i32, _modifiers: i32`), Java passes 1
        // TODO(fabric2seer): `key_pressed` is a method of the port's trait `GuiEventListener`, which the port's `AbstractSelectionList` does not implement
        // rust: return { let (__q0,) = (key_event.clone(),); __this.lock().borrow_mut().base.list.key_pressed(__q0) };
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean charTyped(CharacterEvent characterEvent)
    pub fn char_typed(__this: &compat::Shared<Self>, character_event: &CharacterEvent) -> bool {
        for child in compat::write(__this, |__t| { let __l = &mut __t.base.base; compat::list_entries::<dyn Entry>(__l) }) {
            // TODO(fabric2seer): the condition: Map.Entry.charTyped
            //   java: child.charTyped(..)
            if todo!("fabric2seer: the condition (TODO above)") {
                return true;
            }
        }
        // TODO(fabric2seer): `char_typed` takes 2 argument(s) in the port (`_codepoint: char, _modifiers: i32`), Java passes 1
        // TODO(fabric2seer): `char_typed` is a method of the port's trait `GuiEventListener`, which the port's `AbstractSelectionList` does not implement
        // rust: return { let (__q0,) = (character_event.clone(),); __this.lock().borrow_mut().base.list.char_typed(__q0) };
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: private List<Entry> superModifiableChildren()
    pub fn super_modifiable_children(__this: &compat::Shared<Self>) -> Vec<(_, _)> {
        return __this.children;
    }

    /// java: public void addEntryAtIndex(int index, Entry entry)
    pub fn add_entry_at_index(__this: &compat::Shared<Self>, mut index: i32, mut entry: (_, _)) {
        OptionListWidget::super_modifiable_children(__this).insert((index) as usize, entry.clone());
        __this.lock().borrow_mut().reposition_entries();
    }

    /// java: public void addEntryBelow(Entry below, Entry entry)
    pub fn add_entry_below(__this: &compat::Shared<Self>, below: &(_, _), entry: &(_, _)) {
        let mut idx: i32 = (OptionListWidget::super_modifiable_children(__this).iter().position(|e| *e == below.clone()).map_or(-1, |i| i as i32)).wrapping_add(1);
        if idx == 0 {
            panic!("IllegalStateException: {}", "The entry to insert below does not exist!");
        }
        OptionListWidget::add_entry_at_index(__this, idx, entry.clone());
    }

    /// java: public void addEntryBelowWithoutScroll(Entry below, Entry entry)
    pub fn add_entry_below_without_scroll(__this: &compat::Shared<Self>, below: &(_, _), entry: &(_, _)) {
        let mut d: f64 = (((__this.lock().borrow().content_height()) as f64) - __this.lock().borrow().scroll_amount());
        OptionListWidget::add_entry_below(__this, below, entry);
        { let (__q0,) = (((__this.lock().borrow().content_height() as f64) - d),); __this.lock().borrow_mut().set_scroll_amount(__q0) };
    }

    /// java: private void setHoverDescription(DescriptionWithName description)
    pub fn set_hover_description(__this: &compat::Shared<Self>, mut description: DescriptionWithName) {
        if !std::ptr::eq(&description, &compat::read(__this, |__t| __t.last_hovered_option.clone()).unwrap()) {
            { let __v = Some(description.clone()); compat::write(__this, |__t| __t.last_hovered_option = __v); }
            ((compat::read(__this, |__t| __t.hover_event.clone())))(description.clone());
        }
    }

    /// java: @Override protected void extractListBackground(@NonNull GuiGraphicsExtractor graphics)
    pub fn extract_list_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor) {
    }

    /// java: protected boolean isValidMouseClick(int button)
    pub fn is_valid_mouse_click(__this: &compat::Shared<Self>, mut button: i32) -> bool {
        return (button == 0 || button == 1 || button == 2);
    }

    /// java: @Override protected Entry nextEntry(@NotNull ScreenDirection direction, @NotNull Predicate<Entry> predicate, Entry selected)
    pub fn next_entry(__this: &compat::Shared<Self>, direction: &ScreenDirection, predicate: Arc<dyn Fn((_, _)) -> bool>, selected: &(_, _)) -> (_, _) {
        // TODO(fabric2seer): `.isViewable(..)` on a E
        //   java: return super.nextEntry(direction, entry -> entry.isViewable() && predicate.test(entry), selected);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl YACLSelectionList<(_, _)> for OptionListWidget {
    fn yacl_selection_list_base(&self) -> &YACLSelectionListBase<(_, _)> {
        &self.base
    }
    fn yacl_selection_list_base_mut(&mut self) -> &mut YACLSelectionListBase<(_, _)> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// OptionListWidget's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for OptionListWidget {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base.base
    }
}

/// OptionListWidget's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<OptionListWidget> {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| OptionListWidget::get_row_width(__h))
    }

    /// java: @Override public int getRowLeft()
    fn get_row_left(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| OptionListWidget::get_row_left(__h))
    }

    /// java: @Override protected void extractListBackground(@NonNull GuiGraphicsExtractor graphics)
    fn extract_list_background(&self, list: &AbstractSelectionList, context: &mut WidgetRenderContext<'_>, super_extract_list_background: &mut dyn FnMut(&mut WidgetRenderContext<'_>)) {
        let _ = super_extract_list_background; // the override replaces the base's body (it never calls it)
        self.ask(list, (), |__h| OptionListWidget::extract_list_background(__h, &context.graphics))
    }

}

/// java: public abstract class Entry extends YACLSelectionList.Entry<Entry>
pub struct EntryBase {
    /// java: extends Entry
    pub base: crate::dev::isxander::yacl3::gui::yacl_selection_list::EntryBase<(_, _)>,
    /// java: protected boolean searchQueryMatches = true;
    pub search_query_matches: bool,
}

impl std::ops::Deref for EntryBase {
    type Target = crate::dev::isxander::yacl3::gui::yacl_selection_list::EntryBase<(_, _)>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EntryBase {
    /// java: public Entry() (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(__wrap: impl FnOnce(Self) -> (compat::Shared<dyn Entry>, __R)) -> (compat::Shared<dyn Entry>, __R) {
        // TODO(fabric2seer): `OptionListWidget.this` (the enclosing instance of an inner class)
        //   java: super(OptionListWidget.this);
        let (__this, __r): (compat::Shared<dyn Entry>, __R) = todo!("constructor");
        (__this, __r)
    }

}

/// java: the virtual methods of Entry (implemented by EntryBase and every subclass)
pub trait Entry: crate::dev::isxander::yacl3::gui::yacl_selection_list::Entry + compat::DynAny {
    fn entry_base(&self) -> &EntryBase;
    fn entry_base_mut(&mut self) -> &mut EntryBase;

    /// java: public boolean updateSearchQuery(String searchQuery) (dispatch: the body for this class)
    fn update_search_query(&self) -> fn(&compat::Shared<dyn Entry>, &str) -> bool {
        <dyn Entry>::update_search_query_body
    }

    /// java: public boolean isViewable() (dispatch: the body for this class)
    fn is_viewable(&self) -> fn(&compat::Shared<dyn Entry>) -> bool {
        <dyn Entry>::is_viewable_body
    }

    /// java: @Override public int getHeight() (dispatch: the body for this class)
    fn get_height(&self) -> fn(&compat::Shared<dyn Entry>) -> i32 {
        <dyn Entry>::get_height_body
    }

    /// java: protected void refreshVisibilityState() (dispatch: the body for this class)
    fn refresh_visibility_state(&self) -> fn(&compat::Shared<dyn Entry>) {
        <dyn Entry>::refresh_visibility_state_body
    }

    /// java: protected void onBecameViewable() (dispatch: the body for this class)
    fn on_became_viewable(&self) -> fn(&compat::Shared<dyn Entry>) {
        <dyn Entry>::on_became_viewable_body
    }

    /// java: protected void onBecameHidden() (dispatch: the body for this class)
    fn on_became_hidden(&self) -> fn(&compat::Shared<dyn Entry>) {
        <dyn Entry>::on_became_hidden_body
    }

}

impl dyn Entry {
    /// java: public boolean updateSearchQuery(String searchQuery)
    pub fn update_search_query_body(__this: &compat::Shared<dyn Entry>, search_query: &str) -> bool {
        let mut matches: bool = search_query.is_empty();
        if compat::read(__this, |__t| __t.entry_base().search_query_matches) != matches {
            { let __v = matches; compat::write(__this, |__t| __t.entry_base_mut().search_query_matches = __v); }
            ({ let __f = compat::dispatch(__this, |__t| __t.refresh_visibility_state()); __f(__this) });
        }
        return compat::read(__this, |__t| __t.entry_base().search_query_matches);
    }

    /// java: public boolean isViewable()
    pub fn is_viewable_body(__this: &compat::Shared<dyn Entry>) -> bool {
        return compat::read(__this, |__t| __t.entry_base().search_query_matches);
    }

    /// java: @Override public int getHeight()
    pub fn get_height_body(__this: &compat::Shared<dyn Entry>) -> i32 {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_viewable()); __f(__this) }) {
            return 0;
        }
        return __this.lock().borrow_mut().entry_base_mut().base.get_height();
    }

    /// java: protected void refreshVisibilityState()
    pub fn refresh_visibility_state_body(__this: &compat::Shared<dyn Entry>) {
        if { let __f = compat::dispatch(__this, |__t| __t.is_viewable()); __f(__this) } {
            ({ let __f = compat::dispatch(__this, |__t| __t.on_became_viewable()); __f(__this) });
        } else {
            ({ let __f = compat::dispatch(__this, |__t| __t.on_became_hidden()); __f(__this) });
        }
    }

    /// java: protected void onBecameViewable()
    pub fn on_became_viewable_body(__this: &compat::Shared<dyn Entry>) {
    }

    /// java: protected void onBecameHidden()
    pub fn on_became_hidden_body(__this: &compat::Shared<dyn Entry>) {
        __this.lock().borrow_mut().entry_base_mut().set_height(0);
    }

}

/// java: public class OptionEntry extends Entry
pub struct OptionEntry {
    /// java: extends Entry
    pub base: EntryBase,
    /// java: public final Option<?> option;
    pub option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>,
    /// java: public final ConfigCategory category;
    pub category: compat::Shared<dyn ConfigCategory>,
    /// java: public final OptionGroup group;
    pub group: compat::Shared<dyn OptionGroup>,
    /// java: public final @Nullable GroupSeparatorEntry groupSeparatorEntry;
    pub group_separator_entry: Option<compat::Shared<dyn GroupSeparatorEntry>>,
    /// java: public final AbstractWidget widget;
    pub widget: compat::Shared<dyn AbstractWidget>,
    /// java: private final TextScaledButtonWidget resetButton;
    pub reset_button: Option<compat::Shared<TextScaledButtonWidget>>,
    /// java: private final String categoryName;
    pub category_name: String,
    /// java: private final String groupName;
    pub group_name: String,
}

impl std::ops::Deref for OptionEntry {
    type Target = EntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for OptionEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl OptionEntry {
    /// java: public OptionEntry(Option<?> option, ConfigCategory category, OptionGroup group, @Nullable GroupSeparatorEntry groupSeparatorEntry, AbstractWidget widget)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>, mut category: compat::Shared<dyn ConfigCategory>, mut group: compat::Shared<dyn OptionGroup>, mut group_separator_entry: Option<compat::Shared<dyn GroupSeparatorEntry>>, mut widget: compat::Shared<dyn AbstractWidget>) -> compat::Shared<Self> {
        let __init_option = option.clone();
        let __init_category = category.clone();
        let __init_group = group.clone();
        let __init_group_separator_entry = group_separator_entry.clone();
        let __init_widget = widget.clone();
        let __init_category_name = ({ let __f = compat::dispatch(&category, |__t| __t.name()); __f(&category) }).get_string().to_lowercase();
        let __init_group_name = ({ let __f = compat::dispatch(&group, |__t| __t.name()); __f(&group) }).get_string().to_lowercase();
        let (_, __this) = EntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                option: __init_option,
                category: __init_category,
                group: __init_group,
                group_separator_entry: __init_group_separator_entry,
                widget: __init_widget,
                reset_button: None,
                category_name: __init_category_name,
                group_name: __init_group_name,
            });
            ((__h.clone() as compat::Shared<dyn Entry>), __h)
        });
        if option.lock().borrow().can_reset_to_default() && { let __r = &compat::read(&__this, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.can_reset()); __f(__r) } {
            ({ let __r = &compat::read(&__this, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.set_dimension()); __f(__r, ({ let __r = &compat::read(&__this, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(__r) }).lock().borrow().expanded(&-20, &0)) });
            // TODO(fabric2seer): `yaclScreen` is a field of the enclosing instance (OptionListWidget.this) - an inner class has no outer reference in Rust
            //   java: this.resetButton = new TextScaledButtonWidget(yaclScreen, widget.getDimension().xLimit(), -50, 20, 20, 2f, Component.literal("\u21BB"), button -> {
            //   java:                     option.requestSetDefault();
            //   java:                 });
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
            { let (__a0,) = (Arc::new(move |opt, val| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let __v = (!(opt.lock().borrow().is_pending_value_default()) && opt.lock().borrow().available()); compat::write(&compat::read(&__this_1, |__t| __t.reset_button.clone()).unwrap(), |__t| __t.active = __v); } } }),); option.lock().borrow_mut().add_listener(__a0) };
            { let __v = (!(option.lock().borrow().is_pending_value_default()) && option.lock().borrow().available()); compat::write(&compat::read(&__this, |__t| __t.reset_button.clone()).unwrap(), |__t| __t.active = __v); }
        } else {
            { let __v = None; compat::write(&__this, |__t| __t.reset_button = __v); }
        }
        OptionEntry::update_height(&__this);
        __this
    }

    /// java: @Override public void extractContent(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_viewable()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Entry>)) }) {
            return;
        }
        OptionEntry::update_height(__this);
        ({ let __r = &compat::read(__this, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.set_dimension()); __f(__r, { let (__a0,) = (&__this.lock().borrow().get_y(),); ({ let __r = &compat::read(__this, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(__r) }).lock().borrow().with_y(__a0) }) });
        // TODO(fabric2seer): `extractRenderState` is not a method of AbstractWidget
        //   java: widget.extractRenderState(graphics, mouseX, mouseY, a);
        if compat::read(__this, |__t| __t.reset_button.clone()).is_some() {
            let __wa1 = __this.lock().borrow().get_y();
            let __o1 = compat::read(__this, |__t| __t.reset_button.clone()).unwrap();
            compat::write(&__o1, |__t| __t.set_y(__wa1));
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: let __wa2 = graphics.clone();
            // rust: let __wa3 = mouse_x;
            // rust: let __wa4 = mouse_y;
            // rust: let __wa5 = a;
            // rust: let __o2 = compat::read(__this, |__t| __t.reset_button.clone()).unwrap();
            // rust: compat::write(&__o2, |__t| __t.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa3, __wa4, __wa5));
        }
        if { let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().is_mouse_over(__q0, __q1) } {
            // TODO(fabric2seer): `setHoverDescription(..)` is a method of the enclosing instance (OptionListWidget.this) - an inner class has no outer reference in Rust
            //   java: setHoverDescription(DescriptionWithName.of(option.name(), option.description()));
        }
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal: f64, mut vertical: f64) -> bool {
        // TODO(fabric2seer): `mouseScrolled` is not a method of AbstractWidget
        //   java: return widget.mouseScrolled(mouseX, mouseY, horizontal, vertical);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    pub fn key_pressed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // TODO(fabric2seer): `keyPressed` is not a method of AbstractWidget
        //   java: return widget.keyPressed(event);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent event)
    pub fn char_typed(__this: &compat::Shared<Self>, event: &CharacterEvent) -> bool {
        // TODO(fabric2seer): `charTyped` is not a method of AbstractWidget
        //   java: return widget.charTyped(event);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private void updateHeight()
    pub fn update_height(__this: &compat::Shared<Self>) {
        let __o1 = compat::read(__this, |__t| __t.reset_button.clone()).unwrap();
        let __o2 = compat::read(__this, |__t| __t.reset_button.clone()).unwrap();
        let __o3 = compat::read(__this, |__t| __t.reset_button.clone()).unwrap();
        let __o4 = compat::read(__this, |__t| __t.reset_button.clone()).unwrap();
        { let (__q0,) = (((({ let __r = &compat::read(__this, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(__r) }).lock().borrow().height()).max((if compat::read(__this, |__t| __t.reset_button.clone()).is_some() { compat::write(&__o4, |__t| __t.get_height()) } else { 0 }))).wrapping_add(2),); __this.lock().borrow_mut().set_height(__q0) };
    }

    /// java: @Override public void setFocused(boolean focused)
    pub fn set_focused(__this: &compat::Shared<Self>, mut focused: bool) {
        // TODO(fabric2seer): `set_focused_clears_selection` takes 2 argument(s) in the port (`changed: bool, focused: Option<usize>`), Java passes 1
        // rust: container_object_selection_list::set_focused_clears_selection(focused);
        if focused {
            // TODO(fabric2seer): `setHoverDescription(..)` is a method of the enclosing instance (OptionListWidget.this) - an inner class has no outer reference in Rust
            //   java: setHoverDescription(DescriptionWithName.of(option.name(), option.description()));
        }
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<Box<dyn NarratableEntry>> {
        if compat::read(__this, |__t| __t.reset_button.clone()).is_none() {
            return vec![compat::read(__this, |__t| __t.widget.clone())];
        }
        return vec![compat::read(__this, |__t| __t.widget.clone()), compat::read(__this, |__t| __t.reset_button.clone()).unwrap()];
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        if compat::read(__this, |__t| __t.reset_button.clone()).is_none() {
            return vec![compat::read(__this, |__t| __t.widget.clone())];
        }
        return vec![compat::read(__this, |__t| __t.widget.clone()), compat::read(__this, |__t| __t.reset_button.clone()).unwrap()];
    }

}

impl Entry for OptionEntry {
    fn entry_base(&self) -> &EntryBase {
        &self.base
    }
    fn entry_base_mut(&mut self) -> &mut EntryBase {
        &mut self.base
    }

    /// java: @Override public boolean updateSearchQuery(String searchQuery) (dispatch: the body for this class)
    fn update_search_query(&self) -> fn(&compat::Shared<dyn Entry>, &str) -> bool {
        OptionEntry::update_search_query_dyn
    }

    /// java: @Override public boolean isViewable() (dispatch: the body for this class)
    fn is_viewable(&self) -> fn(&compat::Shared<dyn Entry>) -> bool {
        OptionEntry::is_viewable_dyn
    }

    /// java: @Override protected void onBecameViewable() (dispatch: the body for this class)
    fn on_became_viewable(&self) -> fn(&compat::Shared<dyn Entry>) {
        OptionEntry::on_became_viewable_dyn
    }

}

impl crate::dev::isxander::yacl3::gui::yacl_selection_list::Entry<compat::Object> for OptionEntry {
    fn entry_base(&self) -> &crate::dev::isxander::yacl3::gui::yacl_selection_list::EntryBase<compat::Object> {
        &self.base.base
    }
    fn entry_base_mut(&mut self) -> &mut crate::dev::isxander::yacl3::gui::yacl_selection_list::EntryBase<compat::Object> {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

impl OptionEntry {
    /// java: @Override public boolean updateSearchQuery(String searchQuery)
    pub fn update_search_query_impl(__this: &compat::Shared<Self>, search_query: &str) -> bool {
        { let __v = (search_query.is_empty() || __this.lock().borrow().group_name.contains(search_query) || { let __r = &compat::read(__this, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.matches_search()); __f(__r, search_query) }); compat::write(__this, |__t| __t.search_query_matches = __v); }
        ({ let __f = compat::dispatch(__this, |__t| __t.refresh_visibility_state()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Entry>)) });
        return compat::read(__this, |__t| __t.search_query_matches);
    }

    /// OptionEntry.updateSearchQuery as the dispatch of Entry.updateSearchQuery
    pub fn update_search_query_dyn(__this: &compat::Shared<dyn Entry>, __p0: &str) -> bool {
        Self::update_search_query_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isViewable()
    pub fn is_viewable_impl(__this: &compat::Shared<Self>) -> bool {
        return (<dyn Entry>::is_viewable_body(&compat::view(__this, |a| a as compat::Shared<dyn Entry>)) && (compat::read(__this, |__t| __t.group_separator_entry.clone()).is_none() || { let __r = &compat::read(__this, |__t| __t.group_separator_entry.clone()).unwrap(); let __f = compat::dispatch(__r, |__t| __t.is_expanded()); __f(__r) }));
    }

    /// OptionEntry.isViewable as the dispatch of Entry.isViewable
    pub fn is_viewable_dyn(__this: &compat::Shared<dyn Entry>) -> bool {
        Self::is_viewable_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void onBecameViewable()
    pub fn on_became_viewable_impl(__this: &compat::Shared<Self>) {
        <dyn Entry>::on_became_viewable_body(&compat::view(__this, |a| a as compat::Shared<dyn Entry>));
        OptionEntry::update_height(__this);
    }

    /// OptionEntry.onBecameViewable as the dispatch of Entry.onBecameViewable
    pub fn on_became_viewable_dyn(__this: &compat::Shared<dyn Entry>) {
        Self::on_became_viewable_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// OptionEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for OptionEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// OptionEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<OptionEntry> {
    /// java: @Override public void extractContent(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| OptionEntry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    fn key_pressed(&mut self, list: &mut ModSelectionList, index: usize, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| OptionEntry::key_pressed(__h, event))
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent event)
    fn char_typed(&mut self, list: &mut ModSelectionList, index: usize, event: &CharacterEvent) -> bool {
        self.lend(Some(list.list.entry(index)), |__h| OptionEntry::char_typed(__h, event))
    }

}

/// java: public class GroupSeparatorEntry extends Entry
pub struct GroupSeparatorEntryBase {
    /// java: extends Entry
    pub base: EntryBase,
    /// java: protected final OptionGroup group;
    pub group: compat::Shared<dyn OptionGroup>,
    /// java: protected final MultiLineLabel wrappedName;
    pub wrapped_name: Option<MultiLineLabel>,
    /// java: protected final MultiLineLabel wrappedTooltip;
    pub wrapped_tooltip: Option<MultiLineLabel>,
    /// java: protected final LowProfileButtonWidget expandMinimizeButton;
    pub expand_minimize_button: Option<compat::Shared<LowProfileButtonWidget>>,
    /// java: protected final Screen screen;
    pub screen: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: protected final Font font = Minecraft.getInstance().font;
    pub font: Font,
    /// java: protected boolean groupExpanded;
    pub group_expanded: bool,
    /// java: protected List<Entry> childEntries = new ArrayList<>();
    pub child_entries: Vec<(_, _)>,
}

impl std::ops::Deref for GroupSeparatorEntryBase {
    type Target = EntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for GroupSeparatorEntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GroupSeparatorEntryBase {
    /// java: private GroupSeparatorEntry(OptionGroup group, Screen screen) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut group: compat::Shared<dyn OptionGroup>, mut screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn GroupSeparatorEntry>, __R)) -> (compat::Shared<dyn GroupSeparatorEntry>, __R) {
        let __init_group = group.clone();
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this.screen = screen;
        let (_, (__this, __r)) = EntryBase::new_in(move |__base| {
            let this = Self {
                base: __base,
                group: __init_group,
                wrapped_name: None,
                wrapped_tooltip: None,
                expand_minimize_button: None,
                screen: todo!("fabric2seer: the field value (TODO above)"),
                font: compat::font(),
                group_expanded: false,
                child_entries: Vec::new(),
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn Entry>), (__h, __r))
        });
        // TODO(fabric2seer): `getRowWidth(..)` is a method of the enclosing instance (OptionListWidget.this) - an inner class has no outer reference in Rust
        //   java: this.wrappedName = MultiLineLabel.create(font, group.name(), getRowWidth() - 45);
        // TODO(fabric2seer): static `MultiLineLabel.create` has no Rust counterpart in the port
        // rust: { let __v = Some(MultiLineLabel::create(compat::read(&__this, |__t| __t.group_separator_entry_base().font.clone()), ({ let __f = compat::dispatch(&group, |__t| __t.tooltip()); __f(&group) }), ((screen.width.wrapping_div(3)).wrapping_mul(2)).wrapping_sub(10))); compat::write(&__this, |__t| __t.group_separator_entry_base_mut().wrapped_tooltip = __v); }
        { let __v = !({ let __f = compat::dispatch(&group, |__t| __t.collapsed()); __f(&group) }); compat::write(&__this, |__t| __t.group_separator_entry_base_mut().group_expanded = __v); }
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = Some(LowProfileButtonWidget::new(0, 0, 20, 20, component::empty(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { ({ let __f = compat::dispatch(&__this_1, |__t| __t.on_expand_button_press()); __f(&__this_1) }); } })))); compat::write(&__this, |__t| __t.group_separator_entry_base_mut().expand_minimize_button = __v); }
        ({ let __f = compat::dispatch(&__this, |__t| __t.update_expand_minimize_text()); __f(&__this) });
        ({ let __f = compat::dispatch(&__this, |__t| __t.update_height()); __f(&__this) });
        (__this, __r)
    }

    /// java: private GroupSeparatorEntry(OptionGroup group, Screen screen)
    pub fn new(mut group: compat::Shared<dyn OptionGroup>, mut screen: mcrust::net::minecraft::client::gui::screens::screen::Screen) -> compat::Shared<Self> {
        Self::new_in(group, screen, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn GroupSeparatorEntry>), __h)
        }).1
    }

}

/// java: the virtual methods of GroupSeparatorEntry (implemented by GroupSeparatorEntryBase and every subclass)
pub trait GroupSeparatorEntry: Entry {
    fn group_separator_entry_base(&self) -> &GroupSeparatorEntryBase;
    fn group_separator_entry_base_mut(&mut self) -> &mut GroupSeparatorEntryBase;

    /// java: @Override public void extractContent(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>, &GuiGraphicsExtractor, i32, i32, bool, f32) {
        <dyn GroupSeparatorEntry>::extract_content_body
    }

    /// java: public boolean isExpanded() (dispatch: the body for this class)
    fn is_expanded(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>) -> bool {
        <dyn GroupSeparatorEntry>::is_expanded_body
    }

    /// java: public void setExpanded(boolean expanded) (dispatch: the body for this class)
    fn set_expanded(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>, bool) {
        <dyn GroupSeparatorEntry>::set_expanded_body
    }

    /// java: protected void onExpandButtonPress() (dispatch: the body for this class)
    fn on_expand_button_press(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>) {
        <dyn GroupSeparatorEntry>::on_expand_button_press_body
    }

    /// java: protected void updateExpandMinimizeText() (dispatch: the body for this class)
    fn update_expand_minimize_text(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>) {
        <dyn GroupSeparatorEntry>::update_expand_minimize_text_body
    }

    /// java: public void setChildEntries(List<? extends Entry> childEntries) (dispatch: the body for this class)
    fn set_child_entries(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>, &Vec<(_, _)>) {
        <dyn GroupSeparatorEntry>::set_child_entries_body
    }

    /// java: private int getYPadding() (dispatch: the body for this class)
    fn get_y_padding(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>) -> i32 {
        <dyn GroupSeparatorEntry>::get_y_padding_body
    }

    /// java: @Override public void setFocused(boolean focused) (dispatch: the body for this class)
    fn set_focused(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>, bool) {
        <dyn GroupSeparatorEntry>::set_focused_body
    }

    /// java: private void updateHeight() (dispatch: the body for this class)
    fn update_height(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>) {
        <dyn GroupSeparatorEntry>::update_height_body
    }

    /// java: @Override public @NotNull List<? extends NarratableEntry> narratables() (dispatch: the body for this class)
    fn narratables(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>) -> Vec<Box<dyn NarratableEntry>> {
        <dyn GroupSeparatorEntry>::narratables_body
    }

    /// java: @Override public @NotNull List<? extends GuiEventListener> children() (dispatch: the body for this class)
    fn children(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>) -> Vec<Box<dyn GuiEventListener>> {
        <dyn GroupSeparatorEntry>::children_body
    }

}

impl dyn GroupSeparatorEntry {
    /// java: @Override public void extractContent(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content_body(__this: &compat::Shared<dyn GroupSeparatorEntry>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_viewable()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Entry>)) }) {
            return;
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.update_height()); __f(__this) });
        let __o1 = compat::read(__this, |__t| __t.group_separator_entry_base().expand_minimize_button.clone()).unwrap();
        let __o2 = compat::read(__this, |__t| __t.group_separator_entry_base().expand_minimize_button.clone()).unwrap();
        let mut button_y: i32 = (((__this.lock().borrow().group_separator_entry_base().get_y()).wrapping_add(({ let __f = compat::dispatch(__this, |__t| __t.get_height()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Entry>)) }).wrapping_div(2))).wrapping_sub((compat::write(&__o2, |__t| __t.get_height())).wrapping_div(2))).wrapping_add(1);
        let __wa1 = button_y;
        let __o3 = compat::read(__this, |__t| __t.group_separator_entry_base().expand_minimize_button.clone()).unwrap();
        compat::write(&__o3, |__t| __t.set_y(__wa1));
        let __wa2 = __this.lock().borrow().group_separator_entry_base().get_x();
        let __o4 = compat::read(__this, |__t| __t.group_separator_entry_base().expand_minimize_button.clone()).unwrap();
        compat::write(&__o4, |__t| __t.set_x(__wa2));
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
        // rust: let __wa3 = graphics.clone();
        // rust: let __wa4 = mouse_x;
        // rust: let __wa5 = mouse_y;
        // rust: let __wa6 = a;
        // rust: let __o5 = compat::read(__this, |__t| __t.group_separator_entry_base().expand_minimize_button.clone()).unwrap();
        // rust: compat::write(&__o5, |__t| __t.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa4, __wa5, __wa6));
        // TODO(fabric2seer): the port's `text_renderer` ports this Java method with other parameters (mouse_x, mouse_y for Java's )
        // TODO(fabric2seer): `text_renderer` takes 2 argument(s) in the port (`mouse_x: f64, mouse_y: f64`), Java passes 0
        // rust: { let (__q0, __q1, __q2, __q3, __q4,) = (TextAlignment::Center, (__this.lock().borrow().group_separator_entry_base().get_x()).wrapping_add((__this.lock().borrow().group_separator_entry_base().get_width()).wrapping_div(2)), (__this.lock().borrow().group_separator_entry_base().get_y()).wrapping_add({ let __f = compat::dispatch(__this, |__t| __t.get_y_padding()); __f(__this) }), mcrust::net::minecraft::client::gui::font::LINE_HEIGHT, &mut gui_graphics_extractor::text_renderer(),); __this.lock().borrow().group_separator_entry_base().wrapped_name.as_ref().unwrap().visit_lines(__q0, __q1, __q2, __q3, __q4) };
        if { let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().group_separator_entry_base().is_mouse_over(__q0, __q1) } {
            // TODO(fabric2seer): `setHoverDescription(..)` is a method of the enclosing instance (OptionListWidget.this) - an inner class has no outer reference in Rust
            //   java: setHoverDescription(DescriptionWithName.of(group.name(), group.description()));
        }
    }

    /// java: public boolean isExpanded()
    pub fn is_expanded_body(__this: &compat::Shared<dyn GroupSeparatorEntry>) -> bool {
        return compat::read(__this, |__t| __t.group_separator_entry_base().group_expanded);
    }

    /// java: public void setExpanded(boolean expanded)
    pub fn set_expanded_body(__this: &compat::Shared<dyn GroupSeparatorEntry>, mut expanded: bool) {
        if compat::read(__this, |__t| __t.group_separator_entry_base().group_expanded) == expanded {
            return;
        }
        { let __v = expanded; compat::write(__this, |__t| __t.group_separator_entry_base_mut().group_expanded = __v); }
        ({ let __f = compat::dispatch(__this, |__t| __t.update_expand_minimize_text()); __f(__this) });
        for __a0 in compat::read(__this, |__t| __t.group_separator_entry_base().child_entries.clone()) {
            // TODO(fabric2seer): Map.Entry.refreshVisibilityState
            //   java: Entry::refreshVisibilityState;
        }
        // TODO(fabric2seer): `Entry.repositionEntries` has no Rust method in the port (looked for `reposition_entries` in ContainerObjectSelectionList and JavaSrc)
        // rust: __this.lock().borrow().group_separator_entry_base().reposition_entries();
    }

    /// java: protected void onExpandButtonPress()
    pub fn on_expand_button_press_body(__this: &compat::Shared<dyn GroupSeparatorEntry>) {
        ({ let __f = compat::dispatch(__this, |__t| __t.set_expanded()); __f(__this, !({ let __f = compat::dispatch(__this, |__t| __t.is_expanded()); __f(__this) })) });
    }

    /// java: protected void updateExpandMinimizeText()
    pub fn update_expand_minimize_text_body(__this: &compat::Shared<dyn GroupSeparatorEntry>) {
        let __wa1 = component::literal((if { let __f = compat::dispatch(__this, |__t| __t.is_expanded()); __f(__this) } { "▼" } else { "▶" }));
        let __o1 = compat::read(__this, |__t| __t.group_separator_entry_base().expand_minimize_button.clone()).unwrap();
        compat::write(&__o1, |__t| __t.set_message_text(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__wa1.clone())));
    }

    /// java: public void setChildEntries(List<? extends Entry> childEntries)
    pub fn set_child_entries_body(__this: &compat::Shared<dyn GroupSeparatorEntry>, child_entries: &Vec<(_, _)>) {
        __this.lock().borrow_mut().group_separator_entry_base_mut().child_entries.clear();
        let __h0 = child_entries.iter().cloned();
        __this.lock().borrow_mut().group_separator_entry_base_mut().child_entries.extend(__h0);
    }

    /// java: @Override public boolean updateSearchQuery(String searchQuery)
    pub fn update_search_query_body(__this: &compat::Shared<dyn GroupSeparatorEntry>, search_query: &str) -> bool {
        // TODO(fabric2seer): an assignment used as a value
        //   java: return this.searchQueryMatches = searchQuery.isEmpty() || childEntries.stream().anyMatch(e -> e.updateSearchQuery(searchQuery));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// GroupSeparatorEntry.updateSearchQuery as the dispatch of Entry.updateSearchQuery for the concrete class `__C`
    pub fn update_search_query_from<__C: GroupSeparatorEntry + 'static>(__this: &compat::Shared<dyn Entry>, __p0: &str) -> bool {
        <dyn GroupSeparatorEntry>::update_search_query_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn GroupSeparatorEntry>), __p0)
    }

    /// java: private int getYPadding()
    pub fn get_y_padding_body(__this: &compat::Shared<dyn GroupSeparatorEntry>) -> i32 {
        return 6;
    }

    /// java: @Override public void setFocused(boolean focused)
    pub fn set_focused_body(__this: &compat::Shared<dyn GroupSeparatorEntry>, mut focused: bool) {
        // TODO(fabric2seer): `set_focused_clears_selection` takes 2 argument(s) in the port (`changed: bool, focused: Option<usize>`), Java passes 1
        // rust: container_object_selection_list::set_focused_clears_selection(focused);
        if focused {
            // TODO(fabric2seer): `setHoverDescription(..)` is a method of the enclosing instance (OptionListWidget.this) - an inner class has no outer reference in Rust
            //   java: setHoverDescription(DescriptionWithName.of(group.name(), group.description()));
        }
    }

    /// java: private void updateHeight()
    pub fn update_height_body(__this: &compat::Shared<dyn GroupSeparatorEntry>) {
        { let (__q0,) = ((((__this.lock().borrow().group_separator_entry_base().wrapped_name.as_ref().unwrap().get_line_count()).max(1)).wrapping_mul(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_add(({ let __f = compat::dispatch(__this, |__t| __t.get_y_padding()); __f(__this) }).wrapping_mul(2)),); __this.lock().borrow_mut().group_separator_entry_base_mut().set_height(__q0) };
    }

    /// java: @Override public @NotNull List<? extends NarratableEntry> narratables()
    pub fn narratables_body(__this: &compat::Shared<dyn GroupSeparatorEntry>) -> Vec<Box<dyn NarratableEntry>> {
        // TODO(fabric2seer): `new NarratableEntry(..) {..}` (an anonymous class with state or several methods)
        //   java: return ImmutableList.of(new NarratableEntry() {
        //   java:                 @Override
        //   java:                 public @NotNull NarrationPriority narrationPriority() {
        //   java:                     return NarrationPriority.HOVERED;
        //   java:                 }
        //   java:
        //   java:                 @Override
        //   java:                 public void updateNarration(NarrationElementOutput builder) {
        //   java:                     builder.add(NarratedElementType.TITLE, group.name());
        //   java:                     builder.add(NarratedElementType.HINT, group.tooltip());
        //   java:                 }
        //   java:             });
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public @NotNull List<? extends GuiEventListener> children()
    pub fn children_body(__this: &compat::Shared<dyn GroupSeparatorEntry>) -> Vec<Box<dyn GuiEventListener>> {
        return vec![compat::read(__this, |__t| __t.group_separator_entry_base().expand_minimize_button.clone()).unwrap()];
    }

}

impl GroupSeparatorEntry for GroupSeparatorEntryBase {
    fn group_separator_entry_base(&self) -> &GroupSeparatorEntryBase {
        self
    }
    fn group_separator_entry_base_mut(&mut self) -> &mut GroupSeparatorEntryBase {
        self
    }

}

impl Entry for GroupSeparatorEntryBase {
    fn entry_base(&self) -> &EntryBase {
        &self.base
    }
    fn entry_base_mut(&mut self) -> &mut EntryBase {
        &mut self.base
    }

    /// java: @Override public boolean updateSearchQuery(String searchQuery) (dispatch: the body for this class)
    fn update_search_query(&self) -> fn(&compat::Shared<dyn Entry>, &str) -> bool {
        <dyn GroupSeparatorEntry>::update_search_query_from::<Self>
    }

}

impl crate::dev::isxander::yacl3::gui::yacl_selection_list::Entry<compat::Object> for GroupSeparatorEntryBase {
    fn entry_base(&self) -> &crate::dev::isxander::yacl3::gui::yacl_selection_list::EntryBase<compat::Object> {
        &self.base.base
    }
    fn entry_base_mut(&mut self) -> &mut crate::dev::isxander::yacl3::gui::yacl_selection_list::EntryBase<compat::Object> {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public class ListGroupSeparatorEntry extends GroupSeparatorEntry
pub struct ListGroupSeparatorEntry {
    /// java: extends GroupSeparatorEntry
    pub base: GroupSeparatorEntryBase,
    /// java: private final ListOption<?> listOption;
    pub list_option: Option<compat::Shared<dyn ListOption<compat::Object>>>,
    /// java: private final TextScaledButtonWidget resetListButton;
    pub reset_list_button: Option<compat::Shared<TextScaledButtonWidget>>,
    /// java: private final TooltipButtonWidget addListButton;
    pub add_list_button: Option<compat::Shared<dyn TooltipButtonWidget>>,
}

impl std::ops::Deref for ListGroupSeparatorEntry {
    type Target = GroupSeparatorEntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ListGroupSeparatorEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ListGroupSeparatorEntry {
    /// java: private ListGroupSeparatorEntry(ListOption<?> group, Screen screen)
    pub fn new(mut group: compat::Shared<dyn ListOption<compat::Object>>, mut screen: mcrust::net::minecraft::client::gui::screens::screen::Screen) -> compat::Shared<Self> {
        // NOTE(fabric2seer): `listOption` is read by `setExpanded`, which GroupSeparatorEntry's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `addListButton` is read by `updateExpandMinimizeText`, which GroupSeparatorEntry's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: super(group, screen);
        let __this: compat::Shared<Self> = todo!("constructor");
        { let __v = Some(group.clone()); compat::write(&__this, |__t| __t.list_option = __v); }
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        // TODO(fabric2seer): `Entry.getRowRight` has no Rust method in the port (looked for `get_row_right` in ContainerObjectSelectionList and JavaSrc)
        // rust: { let __v = Some(TextScaledButtonWidget::new_8(screen.clone(), (__this.lock().borrow().get_row_right()).wrapping_sub(20), -50, 20, 20, 1.0f32, component::literal("↻"), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| {
        // rust: group.lock().borrow_mut().request_set_default();
        // rust: })))); compat::write(&__this, |__t| __t.reset_list_button = __v); }
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let (__a0,) = (Arc::new(move |opt, val| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let __v = (!(opt.lock().borrow().is_pending_value_default()) && opt.lock().borrow().available()); compat::write(&compat::read(&__this_1, |__t| __t.reset_list_button.clone()).unwrap(), |__t| __t.active = __v); } } }),); group.lock().borrow_mut().add_listener(__a0) };
        { let __v = (!(group.lock().borrow().is_pending_value_default()) && group.lock().borrow().available()); compat::write(&compat::read(&__this, |__t| __t.reset_list_button.clone()).unwrap(), |__t| __t.active = __v); }
        // TODO(fabric2seer): `yaclScreen` is a field of the enclosing instance (OptionListWidget.this) - an inner class has no outer reference in Rust
        //   java: this.addListButton = new TooltipButtonWidget(yaclScreen, resetListButton.getX() - 20, -50, 20, 20, Component.literal("+"), Component.translatable("yacl.list.add_top"), btn -> {
        //   java:                 group.insertNewEntry();
        //   java:                 setExpanded(true);
        //   java:             });
        ({ let __f = compat::dispatch(&__this, |__t| __t.update_expand_minimize_text()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn GroupSeparatorEntry>)) });
        ListGroupSeparatorEntry::minimize_if_unavailable(&__this);
        __this
    }

    /// java: private void minimizeIfUnavailable()
    pub fn minimize_if_unavailable(__this: &compat::Shared<Self>) {
        if !(__this.lock().borrow().list_option.as_ref().unwrap().available()) && { let __f = compat::dispatch(__this, |__t| __t.is_expanded()); __f(&compat::view(__this, |a| a as compat::Shared<dyn GroupSeparatorEntry>)) } {
            ({ let __f = compat::dispatch(__this, |__t| __t.set_expanded()); __f(&compat::view(__this, |a| a as compat::Shared<dyn GroupSeparatorEntry>), false) });
        }
    }

}

impl GroupSeparatorEntry for ListGroupSeparatorEntry {
    fn group_separator_entry_base(&self) -> &GroupSeparatorEntryBase {
        &self.base
    }
    fn group_separator_entry_base_mut(&mut self) -> &mut GroupSeparatorEntryBase {
        &mut self.base
    }

    /// java: @Override public void extractContent(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a) (dispatch: the body for this class)
    fn extract_content(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>, &GuiGraphicsExtractor, i32, i32, bool, f32) {
        ListGroupSeparatorEntry::extract_content_dyn
    }

    /// java: @Override protected void updateExpandMinimizeText() (dispatch: the body for this class)
    fn update_expand_minimize_text(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>) {
        ListGroupSeparatorEntry::update_expand_minimize_text_dyn
    }

    /// java: @Override public void setExpanded(boolean expanded) (dispatch: the body for this class)
    fn set_expanded(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>, bool) {
        ListGroupSeparatorEntry::set_expanded_dyn
    }

    /// java: @Override public @NotNull List<? extends GuiEventListener> children() (dispatch: the body for this class)
    fn children(&self) -> fn(&compat::Shared<dyn GroupSeparatorEntry>) -> Vec<Box<dyn GuiEventListener>> {
        ListGroupSeparatorEntry::children_dyn
    }

}

impl Entry for ListGroupSeparatorEntry {
    fn entry_base(&self) -> &EntryBase {
        &self.base.base
    }
    fn entry_base_mut(&mut self) -> &mut EntryBase {
        &mut self.base.base
    }

    /// java: @Override public boolean updateSearchQuery(String searchQuery) (dispatch: the body for this class)
    fn update_search_query(&self) -> fn(&compat::Shared<dyn Entry>, &str) -> bool {
        <dyn GroupSeparatorEntry>::update_search_query_from::<Self>
    }

}

impl crate::dev::isxander::yacl3::gui::yacl_selection_list::Entry<compat::Object> for ListGroupSeparatorEntry {
    fn entry_base(&self) -> &crate::dev::isxander::yacl3::gui::yacl_selection_list::EntryBase<compat::Object> {
        &self.base.base.base
    }
    fn entry_base_mut(&mut self) -> &mut crate::dev::isxander::yacl3::gui::yacl_selection_list::EntryBase<compat::Object> {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

impl ListGroupSeparatorEntry {
    /// java: @Override public void extractContent(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_viewable()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Entry>)) }) {
            return;
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.update_expand_minimize_text()); __f(&compat::view(__this, |a| a as compat::Shared<dyn GroupSeparatorEntry>)) });
        <dyn GroupSeparatorEntry>::extract_content_body(&compat::view(__this, |a| a as compat::Shared<dyn GroupSeparatorEntry>), graphics, mouse_x, mouse_y, hovered, a);
        let __o1 = compat::read(__this, |__t| __t.expand_minimize_button.clone()).unwrap();
        let mut button_y: i32 = compat::write(&__o1, |__t| __t.get_y());
        let __wa1 = button_y;
        let __o2 = compat::read(__this, |__t| __t.reset_list_button.clone()).unwrap();
        compat::write(&__o2, |__t| __t.set_y(__wa1));
        let __wa2 = button_y;
        let __o3 = compat::read(__this, |__t| __t.add_list_button.clone()).unwrap();
        compat::write(&__o3, |__t| __t.tooltip_button_widget_base_mut().set_y(__wa2));
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
        // rust: let __wa3 = graphics.clone();
        // rust: let __wa4 = mouse_x;
        // rust: let __wa5 = mouse_y;
        // rust: let __wa6 = a;
        // rust: let __o4 = compat::read(__this, |__t| __t.reset_list_button.clone()).unwrap();
        // rust: compat::write(&__o4, |__t| __t.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa4, __wa5, __wa6));
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
        // rust: let __wa7 = graphics.clone();
        // rust: let __wa8 = mouse_x;
        // rust: let __wa9 = mouse_y;
        // rust: let __wa10 = a;
        // rust: let __o5 = compat::read(__this, |__t| __t.add_list_button.clone()).unwrap();
        // rust: compat::write(&__o5, |__t| __t.tooltip_button_widget_base_mut().extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa8, __wa9, __wa10));
    }

    /// ListGroupSeparatorEntry.extractContent as the dispatch of GroupSeparatorEntry.extractContent
    pub fn extract_content_dyn(__this: &compat::Shared<dyn GroupSeparatorEntry>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: bool, __p4: f32) {
        Self::extract_content_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override protected void updateExpandMinimizeText()
    pub fn update_expand_minimize_text_impl(__this: &compat::Shared<Self>) {
        <dyn GroupSeparatorEntry>::update_expand_minimize_text_body(&compat::view(__this, |a| a as compat::Shared<dyn GroupSeparatorEntry>));
        { let __v = (compat::read(__this, |__t| __t.list_option.clone()).is_none() || __this.lock().borrow().list_option.as_ref().unwrap().available()); compat::write(&compat::read(__this, |__t| __t.expand_minimize_button.clone()).unwrap(), |__t| __t.active = __v); }
        if compat::read(__this, |__t| __t.add_list_button.clone()).is_some() {
            { let __v = (compat::read(&compat::read(__this, |__t| __t.expand_minimize_button.clone()).unwrap(), |__t| __t.active) && __this.lock().borrow().list_option.as_ref().unwrap().number_of_entries() < __this.lock().borrow().list_option.as_ref().unwrap().maximum_number_of_entries()); compat::write(&compat::read(__this, |__t| __t.add_list_button.clone()).unwrap(), |__t| __t.tooltip_button_widget_base_mut().active = __v); }
        }
    }

    /// ListGroupSeparatorEntry.updateExpandMinimizeText as the dispatch of GroupSeparatorEntry.updateExpandMinimizeText
    pub fn update_expand_minimize_text_dyn(__this: &compat::Shared<dyn GroupSeparatorEntry>) {
        Self::update_expand_minimize_text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void setExpanded(boolean expanded)
    pub fn set_expanded_impl(__this: &compat::Shared<Self>, mut expanded: bool) {
        <dyn GroupSeparatorEntry>::set_expanded_body(&compat::view(__this, |a| a as compat::Shared<dyn GroupSeparatorEntry>), (__this.lock().borrow().list_option.as_ref().unwrap().available() && expanded));
    }

    /// ListGroupSeparatorEntry.setExpanded as the dispatch of GroupSeparatorEntry.setExpanded
    pub fn set_expanded_dyn(__this: &compat::Shared<dyn GroupSeparatorEntry>, __p0: bool) {
        Self::set_expanded_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public @NotNull List<? extends GuiEventListener> children()
    pub fn children_impl(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return vec![compat::read(__this, |__t| __t.expand_minimize_button.clone()).unwrap(), compat::read(__this, |__t| __t.add_list_button.clone()).unwrap(), compat::read(__this, |__t| __t.reset_list_button.clone()).unwrap()];
    }

    /// ListGroupSeparatorEntry.children as the dispatch of GroupSeparatorEntry.children
    pub fn children_dyn(__this: &compat::Shared<dyn GroupSeparatorEntry>) -> Vec<Box<dyn GuiEventListener>> {
        Self::children_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// ListGroupSeparatorEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for ListGroupSeparatorEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base.base.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// ListGroupSeparatorEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<ListGroupSeparatorEntry> {
    /// java: @Override public void extractContent(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_content()); __f(&compat::view(__h, |a| a as compat::Shared<dyn GroupSeparatorEntry>), &context.graphics, mouse_x, mouse_y, hovered, a) }));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

/// java: public class EmptyListLabel extends Entry
pub struct EmptyListLabel {
    /// java: extends Entry
    pub base: EntryBase,
    /// java: private final ListGroupSeparatorEntry parent;
    pub parent: compat::Shared<ListGroupSeparatorEntry>,
    /// java: private final String groupName;
    pub group_name: String,
    /// java: private final String categoryName;
    pub category_name: String,
}

impl std::ops::Deref for EmptyListLabel {
    type Target = EntryBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EmptyListLabel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EmptyListLabel {
    /// java: public EmptyListLabel(ListGroupSeparatorEntry parent, ConfigCategory category)
    pub fn new(mut parent: compat::Shared<ListGroupSeparatorEntry>, mut category: compat::Shared<dyn ConfigCategory>) -> compat::Shared<Self> {
        let __init_parent = parent.clone();
        let __init_group_name = ({ let __r = &compat::read(&parent, |__t| __t.group.clone()); let __f = compat::dispatch(__r, |__t| __t.name()); __f(__r) }).get_string().to_lowercase();
        let __init_category_name = ({ let __f = compat::dispatch(&category, |__t| __t.name()); __f(&category) }).get_string().to_lowercase();
        let (_, __this) = EntryBase::new_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                parent: __init_parent,
                group_name: __init_group_name,
                category_name: __init_category_name,
            });
            ((__h.clone() as compat::Shared<dyn Entry>), __h)
        });
        __this.lock().borrow_mut().set_height(11);
        __this
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut a: f32) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_viewable()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Entry>)) }) {
            return;
        }
        gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(component::translatable("yacl.list.empty").with_style_formats(&[ChatFormatting::DarkGray, ChatFormatting::Italic])), (__this.lock().borrow().get_x()).wrapping_add((__this.lock().borrow().get_width()).wrapping_div(2)), __this.lock().borrow().get_y(), -1);
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return vec![];
    }

    /// java: @Override public List<? extends NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<Box<dyn NarratableEntry>> {
        return vec![];
    }

}

impl Entry for EmptyListLabel {
    fn entry_base(&self) -> &EntryBase {
        &self.base
    }
    fn entry_base_mut(&mut self) -> &mut EntryBase {
        &mut self.base
    }

    /// java: @Override public boolean updateSearchQuery(String searchQuery) (dispatch: the body for this class)
    fn update_search_query(&self) -> fn(&compat::Shared<dyn Entry>, &str) -> bool {
        EmptyListLabel::update_search_query_dyn
    }

    /// java: @Override public boolean isViewable() (dispatch: the body for this class)
    fn is_viewable(&self) -> fn(&compat::Shared<dyn Entry>) -> bool {
        EmptyListLabel::is_viewable_dyn
    }

    /// java: @Override protected void onBecameViewable() (dispatch: the body for this class)
    fn on_became_viewable(&self) -> fn(&compat::Shared<dyn Entry>) {
        EmptyListLabel::on_became_viewable_dyn
    }

    /// java: @Override protected void onBecameHidden() (dispatch: the body for this class)
    fn on_became_hidden(&self) -> fn(&compat::Shared<dyn Entry>) {
        EmptyListLabel::on_became_hidden_dyn
    }

}

impl crate::dev::isxander::yacl3::gui::yacl_selection_list::Entry<compat::Object> for EmptyListLabel {
    fn entry_base(&self) -> &crate::dev::isxander::yacl3::gui::yacl_selection_list::EntryBase<compat::Object> {
        &self.base.base
    }
    fn entry_base_mut(&mut self) -> &mut crate::dev::isxander::yacl3::gui::yacl_selection_list::EntryBase<compat::Object> {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

impl EmptyListLabel {
    /// java: @Override public boolean updateSearchQuery(String searchQuery)
    pub fn update_search_query_impl(__this: &compat::Shared<Self>, search_query: &str) -> bool {
        // TODO(fabric2seer): an assignment used as a value
        //   java: return this.searchQueryMatches = searchQuery.isEmpty() || groupName.contains(searchQuery);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// EmptyListLabel.updateSearchQuery as the dispatch of Entry.updateSearchQuery
    pub fn update_search_query_dyn(__this: &compat::Shared<dyn Entry>, __p0: &str) -> bool {
        Self::update_search_query_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isViewable()
    pub fn is_viewable_impl(__this: &compat::Shared<Self>) -> bool {
        return ({ let __r = &compat::read(__this, |__t| __t.parent.clone()); let __f = compat::dispatch(__r, |__t| __t.is_expanded()); __f(&compat::view(__r, |a| a as compat::Shared<dyn GroupSeparatorEntry>)) } && <dyn Entry>::is_viewable_body(&compat::view(__this, |a| a as compat::Shared<dyn Entry>)));
    }

    /// EmptyListLabel.isViewable as the dispatch of Entry.isViewable
    pub fn is_viewable_dyn(__this: &compat::Shared<dyn Entry>) -> bool {
        Self::is_viewable_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void onBecameViewable()
    pub fn on_became_viewable_impl(__this: &compat::Shared<Self>) {
        <dyn Entry>::on_became_viewable_body(&compat::view(__this, |a| a as compat::Shared<dyn Entry>));
        __this.lock().borrow_mut().set_height(11);
    }

    /// EmptyListLabel.onBecameViewable as the dispatch of Entry.onBecameViewable
    pub fn on_became_viewable_dyn(__this: &compat::Shared<dyn Entry>) {
        Self::on_became_viewable_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected void onBecameHidden()
    pub fn on_became_hidden_impl(__this: &compat::Shared<Self>) {
        <dyn Entry>::on_became_hidden_body(&compat::view(__this, |a| a as compat::Shared<dyn Entry>));
        __this.lock().borrow_mut().set_height(0);
    }

    /// EmptyListLabel.onBecameHidden as the dispatch of Entry.onBecameHidden
    pub fn on_became_hidden_dyn(__this: &compat::Shared<dyn Entry>) {
        Self::on_became_hidden_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// EmptyListLabel's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for EmptyListLabel {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base.base.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = &mut self.base.base;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// EmptyListLabel as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<EmptyListLabel> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| EmptyListLabel::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}
