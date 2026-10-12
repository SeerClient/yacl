//! Ported from dev.isxander.yacl3.impl.ListOptionEntryImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::list_option::ListOption;
use crate::dev::isxander::yacl3::api::list_option_entry::{self, ListOptionEntry};
use crate::dev::isxander::yacl3::api::option::{AnyOption, Option as YOption};
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_event_listener::{Event, OptionEventListener};
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::list_entry_widget::ListEntryWidget;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::hidden_name_list_option_entry::HiddenNameListOptionEntry;
use crate::dev::isxander::yacl3::r#impl::list_option_impl::{EntryControllerFunction, ListOptionImpl};
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::cell::RefCell;
use std::sync::Weak;

/// java: @ApiStatus.Internal public final class ListOptionEntryImpl<T> implements ListOptionEntry<T>
pub struct ListOptionEntryImpl<T: Clone + PartialEq + 'static> {
    /// java: private final ListOptionImpl<T> group; (the entry's parent - held weakly, the group holds the entry)
    pub group: Weak<ListOptionImpl<T>>,
    /// java: private T value;
    pub value: RefCell<T>,
    /// java: private final Binding<T> binding;
    pub binding: Arc<dyn Binding<T>>,
    /// java: private final Controller<T> controller;
    pub controller: Arc<dyn Controller<T>>,
}

impl<T: Clone + PartialEq + 'static> ListOptionEntryImpl<T> {
    /// java: ListOptionEntryImpl(ListOptionImpl<T> group, T initialValue, @NotNull Function<ListOptionEntry<T>, Controller<T>> controlGetter)
    pub fn new(group: Weak<ListOptionImpl<T>>, initial_value: T, control_getter: EntryControllerFunction<T>) -> Arc<ListOptionEntryImpl<T>> {
        Arc::new_cyclic(|this: &Weak<ListOptionEntryImpl<T>>| {
            // java: this.binding = new EntryBinding();
            let binding: Arc<dyn Binding<T>> = Arc::new(EntryBinding { entry: this.clone() });
            // java: this.controller = new EntryController<>(controlGetter.apply(new HiddenNameListOptionEntry<>(this)), this);
            let this_entry: Weak<dyn ListOptionEntry<T>> = this.clone();
            let hidden: Arc<dyn ListOptionEntry<T>> = Arc::new(HiddenNameListOptionEntry::new(this_entry));
            let controller: Arc<dyn Controller<T>> = Arc::new(EntryController { controller: control_getter(hidden), entry: this.clone() });
            ListOptionEntryImpl { group, value: RefCell::new(initial_value), binding, controller }
        })
    }

    fn group(&self) -> Arc<ListOptionImpl<T>> {
        self.group.upgrade().expect("a list option entry outlived its list option")
    }
}

impl<T: Clone + PartialEq + 'static> AnyOption for ListOptionEntryImpl<T> {
    /// java: @Override public @NotNull Component name()
    fn name(&self) -> MutableComponent {
        OptionGroup::name(&*self.group())
    }

    /// java: @Override public @NotNull OptionDescription description()
    fn description(&self) -> Arc<dyn OptionDescription> {
        OptionGroup::description(&*self.group())
    }

    /// java: @Override public @NotNull Component tooltip()
    fn tooltip(&self) -> MutableComponent {
        OptionGroup::tooltip(&*self.group())
    }

    /// java: @Override public boolean available()
    fn available(&self) -> bool {
        list_option_entry::parent_available(self)
    }

    /// java: @Override public void setAvailable(boolean available)
    fn set_available(&self, _available: bool) {}

    /// java: ListOptionEntry.flags() - `parentGroup().flags()`
    fn flags(&self) -> Vec<Arc<dyn OptionFlag>> {
        list_option_entry::parent_flags(self)
    }

    /// java: @Override public boolean changed()
    fn changed(&self) -> bool {
        false
    }

    /// java: @Override public boolean applyValue()
    fn apply_value(&self) -> bool {
        false
    }

    /// java: @Override public void forgetPendingValue()
    fn forget_pending_value(&self) {}

    /// java: @Override public void requestSetDefault()
    fn request_set_default(&self) {}

    /// java: @Override public boolean isPendingValueDefault()
    fn is_pending_value_default(&self) -> bool {
        false
    }

    /// java: @Override public boolean canResetToDefault()
    fn can_reset_to_default(&self) -> bool {
        false
    }

    fn provide_controller_widget(&self, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        self.controller().provide_widget(screen, widget_dimension)
    }

    /// java: @Override public void addEventListener(..) - empty
    fn add_any_event_listener(&self, _listener: Arc<dyn Fn(&dyn AnyOption, Event)>) {}

    /// java: @Override public void addListener(..) - empty
    fn add_any_listener(&self, _changed_listener: Arc<dyn Fn(&dyn AnyOption)>) {}
}

impl<T: Clone + PartialEq + 'static> YOption<T> for ListOptionEntryImpl<T> {
    /// java: @Override public @NotNull Controller<T> controller()
    fn controller(&self) -> Arc<dyn Controller<T>> {
        self.controller.clone()
    }

    /// java: @Override public @NotNull StateManager<T> stateManager()
    fn state_manager(&self) -> Arc<dyn StateManager<T>> {
        panic!("UnsupportedOperationException: ListOptionEntryImpl does not support state managers");
    }

    /// java: @Override public @NotNull Binding<T> binding()
    fn binding(&self) -> Arc<dyn Binding<T>> {
        self.binding.clone()
    }

    /// java: @Override public @NotNull T pendingValue()
    fn pending_value(&self) -> T {
        self.value.borrow().clone()
    }

    /// java: @Override public void requestSet(@NotNull T value)
    fn request_set(&self, value: T) {
        self.binding.set_value(value);
    }

    /// java: @Override public void addEventListener(OptionEventListener<T> listener) - empty
    fn add_event_listener(&self, _listener: Arc<dyn OptionEventListener<T>>) {}

    /// java: @Override public void addListener(BiConsumer<Option<T>, T> changedListener) - empty
    fn add_listener(&self, _changed_listener: Arc<dyn Fn(&dyn YOption<T>, T)>) {}
}

impl<T: Clone + PartialEq + 'static> ListOptionEntry<T> for ListOptionEntryImpl<T> {
    /// java: @Override public ListOption<T> parentGroup()
    fn parent_group(&self) -> Arc<dyn ListOption<T>> {
        self.group()
    }
}

/// java: @ApiStatus.Internal public record EntryController<T>(Controller<T> controller, ListOptionEntryImpl<T> entry) implements Controller<T>
pub struct EntryController<T: Clone + PartialEq + 'static> {
    pub controller: Arc<dyn Controller<T>>,
    /// the entry (held weakly: the entry holds this controller)
    pub entry: Weak<ListOptionEntryImpl<T>>,
}

impl<T: Clone + PartialEq + 'static> Controller<T> for EntryController<T> {
    /// java: @Override public Option<T> option()
    fn option(&self) -> Arc<dyn YOption<T>> {
        self.controller.option()
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        self.controller.format_value()
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        let entry: Arc<dyn ListOptionEntry<T>> = self.entry.upgrade().expect("a list entry's controller outlived the entry");
        let entry_widget = self.controller.clone().provide_widget(screen, widget_dimension);
        ListEntryWidget::new(screen, entry, entry_widget)
    }
}

/// java: private class EntryBinding implements Binding<T>
pub struct EntryBinding<T: Clone + PartialEq + 'static> {
    entry: Weak<ListOptionEntryImpl<T>>,
}

impl<T: Clone + PartialEq + 'static> Binding<T> for EntryBinding<T> {
    /// java: @Override public void setValue(T newValue)
    fn set_value(&self, new_value: T) {
        let entry = self.entry.upgrade().expect("a list entry's binding outlived the entry");
        *entry.value.borrow_mut() = new_value;
        entry.group().trigger_listener(Event::Other, true);
    }

    /// java: @Override public T getValue()
    fn get_value(&self) -> T {
        let entry = self.entry.upgrade().expect("a list entry's binding outlived the entry");
        let value = entry.value.borrow().clone();
        value
    }

    /// java: @Override public T defaultValue()
    fn default_value(&self) -> T {
        panic!("UnsupportedOperationException");
    }
}
