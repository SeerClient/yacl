//! Ported from dev.isxander.yacl3.impl.ListOptionImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::{self, Binding};
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::list_option::{AnyListOption, Builder, ListOption};
use crate::dev::isxander::yacl3::api::list_option_entry::ListOptionEntry;
use crate::dev::isxander::yacl3::api::option::{self as yoption, AnyOption, Option as YOption};
use crate::dev::isxander::yacl3::api::option_description::{self, OptionDescription};
use crate::dev::isxander::yacl3::api::option_event_listener::{Event, OptionEventListener};
use crate::dev::isxander::yacl3::api::option_flag::{self, OptionFlag};
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::state_manager::{self, StateManager};
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::list_option_entry_impl::ListOptionEntryImpl;
use crate::dev::isxander::yacl3::r#impl::option_impl::OptionListeners;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::cell::{Cell, RefCell};
use std::sync::Weak;

/// `Function<ListOptionEntry<T>, Controller<T>>`
pub type EntryControllerFunction<T> = Arc<dyn Fn(Arc<dyn ListOptionEntry<T>>) -> Arc<dyn Controller<T>>>;

/// java: @ApiStatus.Internal public final class ListOptionImpl<T> implements ListOption<T>
pub struct ListOptionImpl<T: Clone + PartialEq + 'static> {
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final OptionDescription description;
    pub description: Arc<dyn OptionDescription>,
    /// java: private final StateManager<List<T>> stateManager;
    pub state_manager: Arc<dyn StateManager<Vec<T>>>,
    /// java: private final Supplier<T> initialValue;
    pub initial_value: Arc<dyn Fn() -> T>,
    /// java: private final List<ListOptionEntry<T>> entries;
    pub entries: RefCell<Vec<Arc<dyn ListOptionEntry<T>>>>,
    /// java: private final boolean collapsed;
    pub collapsed: bool,
    /// java: private boolean available;
    pub available: Cell<bool>,
    /// java: private final int minimumNumberOfEntries;
    pub minimum_number_of_entries: i32,
    /// java: private final int maximumNumberOfEntries;
    pub maximum_number_of_entries: i32,
    /// java: private final boolean insertEntriesAtEnd;
    pub insert_entries_at_end: bool,
    /// java: private final ImmutableSet<OptionFlag> flags;
    pub flags: Vec<Arc<dyn OptionFlag>>,
    /// java: private final EntryFactory entryFactory;
    entry_factory: EntryFactory<T>,
    /// java: private final List<OptionEventListener<List<T>>> listeners; private int currentListenerDepth = 0;
    pub listeners: Rc<OptionListeners<Vec<T>>>,
    /// java: private final List<Runnable> refreshListeners;
    pub refresh_listeners: RefCell<Vec<Arc<dyn Fn()>>>,
    this: Weak<ListOptionImpl<T>>,
}

impl<T: Clone + PartialEq + 'static> ListOptionImpl<T> {
    /// java: public ListOptionImpl(@NotNull Component name, @NotNull OptionDescription description, @NotNull StateManager<List<T>> stateManager,
    /// @NotNull Supplier<T> initialValue, @NotNull Function<ListOptionEntry<T>, Controller<T>> controllerFunction, ImmutableSet<OptionFlag> flags,
    /// boolean collapsed, boolean available, int minimumNumberOfEntries, int maximumNumberOfEntries, boolean insertEntriesAtEnd,
    /// Collection<OptionEventListener<List<T>>> listeners)
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: MutableComponent,
        description: Arc<dyn OptionDescription>,
        state_manager: Arc<dyn StateManager<Vec<T>>>,
        initial_value: Arc<dyn Fn() -> T>,
        controller_function: EntryControllerFunction<T>,
        flags: Vec<Arc<dyn OptionFlag>>,
        collapsed: bool,
        available: bool,
        minimum_number_of_entries: i32,
        maximum_number_of_entries: i32,
        insert_entries_at_end: bool,
        listeners: Vec<Arc<dyn OptionEventListener<Vec<T>>>>,
    ) -> Arc<ListOptionImpl<T>> {
        let option = Arc::new_cyclic(|this: &Weak<ListOptionImpl<T>>| {
            let entry_factory = EntryFactory { controller_function, group: this.clone() };
            // java: this.entries = createEntries(binding().getValue());
            let binding = binding_of(&state_manager);
            let entries = entry_factory.create_entries(binding.get_value());
            let this_option: Weak<dyn YOption<Vec<T>>> = this.clone();
            ListOptionImpl {
                name,
                description,
                state_manager,
                initial_value,
                entries: RefCell::new(entries),
                collapsed,
                available: Cell::new(available),
                minimum_number_of_entries,
                maximum_number_of_entries,
                insert_entries_at_end,
                flags,
                entry_factory,
                listeners: Rc::new(OptionListeners::new(listeners, this_option)),
                refresh_listeners: RefCell::new(Vec::new()),
                this: this.clone(),
            }
        });
        // java: this.stateManager.addListener((oldValue, newValue) -> triggerListener(OptionEventListener.Event.STATE_CHANGE, false));
        let listeners = Rc::downgrade(&option.listeners);
        option.state_manager.add_listener(Arc::new(move |_old_value: Vec<T>, _new_value: Vec<T>| {
            if let Some(listeners) = listeners.upgrade() {
                listeners.trigger_listener(Event::StateChange, false);
            }
        }));
        // java: triggerListener(OptionEventListener.Event.INITIAL, false);
        option.trigger_listener(Event::Initial, false);
        option
    }

    /// java: void triggerListener(OptionEventListener.Event event, boolean allowDepth)
    pub fn trigger_listener(&self, event: Event, allow_depth: bool) {
        self.listeners.trigger_listener(event, allow_depth);
    }

    /// java: private void onRefresh()
    fn on_refresh(&self) {
        let refresh_listeners = self.refresh_listeners.borrow().clone();
        for listener in refresh_listeners {
            listener();
        }
        self.trigger_listener(Event::Other, true);
    }

    fn this(&self) -> Arc<ListOptionImpl<T>> {
        self.this.upgrade().expect("ListOptionImpl reached after it was dropped")
    }
}

/// java: @Override @Deprecated public @NotNull Binding<List<T>> binding()'s body over the state manager
fn binding_of<T: Clone + 'static>(state_manager: &Arc<dyn StateManager<Vec<T>>>) -> Arc<dyn Binding<Vec<T>>> {
    match state_manager.get_binding_for_deprecation() {
        Some(binding) => binding,
        None => panic!(
            "UnsupportedOperationException: Binding is not available for this option - using a new state manager which does not directly expose the binding as it may not have one."
        ),
    }
}

impl<T: Clone + PartialEq + 'static> OptionGroup for ListOptionImpl<T> {
    /// java: @Override public @NotNull Component name()
    fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: @Override public @NotNull OptionDescription description()
    fn description(&self) -> Arc<dyn OptionDescription> {
        self.description.clone()
    }

    /// java: @Override public @NotNull Component tooltip()
    fn tooltip(&self) -> MutableComponent {
        self.description.text()
    }

    /// java: @Override public @NotNull ImmutableList<ListOptionEntry<T>> options()
    fn options(&self) -> Vec<Arc<dyn AnyOption>> {
        self.entries.borrow().iter().map(|e| e.clone() as Arc<dyn AnyOption>).collect()
    }

    /// java: @Override public boolean collapsed()
    fn collapsed(&self) -> bool {
        self.collapsed
    }

    /// java: @Override public boolean isRoot()
    fn is_root(&self) -> bool {
        false
    }

    fn as_list_option(&self) -> Option<Arc<dyn AnyListOption>> {
        Some(self.this())
    }
}

impl<T: Clone + PartialEq + 'static> AnyOption for ListOptionImpl<T> {
    fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    fn description(&self) -> Arc<dyn OptionDescription> {
        self.description.clone()
    }

    fn tooltip(&self) -> MutableComponent {
        self.description.text()
    }

    /// java: @Override public boolean available()
    fn available(&self) -> bool {
        self.available.get()
    }

    /// java: @Override public void setAvailable(boolean available)
    fn set_available(&self, available: bool) {
        let changed = self.available.get() != available;
        self.available.set(available);
        if changed {
            if !available {
                self.state_manager.sync();
            }
            self.trigger_listener(Event::AvailabilityChange, !available);
        }
    }

    /// java: @Override public @NotNull ImmutableSet<OptionFlag> flags()
    fn flags(&self) -> Vec<Arc<dyn OptionFlag>> {
        self.flags.clone()
    }

    /// java: @Override public boolean changed()
    fn changed(&self) -> bool {
        self.binding().get_value() != self.pending_value()
    }

    /// java: @Override public boolean applyValue()
    fn apply_value(&self) -> bool {
        if self.changed() {
            self.binding().set_value(self.pending_value());
            return true;
        }
        false
    }

    /// java: @Override public void forgetPendingValue()
    fn forget_pending_value(&self) {
        self.request_set(self.binding().get_value());
    }

    /// java: @Override public void requestSetDefault()
    fn request_set_default(&self) {
        self.request_set(self.binding().default_value());
    }

    /// java: @Override public boolean isPendingValueDefault()
    fn is_pending_value_default(&self) -> bool {
        self.binding().default_value() == self.pending_value()
    }

    /// `controller()` throws for a list option, so `controller().provideWidget(..)` does too.
    fn provide_controller_widget(&self, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        self.controller().provide_widget(screen, widget_dimension)
    }

    fn add_any_event_listener(&self, listener: Arc<dyn Fn(&dyn AnyOption, Event)>) {
        self.add_event_listener(yoption::any_event_listener(listener));
    }

    fn add_any_listener(&self, changed_listener: Arc<dyn Fn(&dyn AnyOption)>) {
        self.add_event_listener(yoption::any_listener(changed_listener));
    }

    fn as_list_option(&self) -> Option<Arc<dyn AnyListOption>> {
        Some(self.this())
    }
}

impl<T: Clone + PartialEq + 'static> YOption<Vec<T>> for ListOptionImpl<T> {
    /// java: @Override public @NotNull Controller<List<T>> controller()
    fn controller(&self) -> Arc<dyn Controller<Vec<T>>> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public @NotNull StateManager<List<T>> stateManager()
    fn state_manager(&self) -> Arc<dyn StateManager<Vec<T>>> {
        self.state_manager.clone()
    }

    /// java: @Override @Deprecated public @NotNull Binding<List<T>> binding()
    fn binding(&self) -> Arc<dyn Binding<Vec<T>>> {
        binding_of(&self.state_manager)
    }

    /// java: @Override public @NotNull ImmutableList<T> pendingValue()
    fn pending_value(&self) -> Vec<T> {
        let entries = self.entries.borrow().clone();
        entries.iter().map(|e| e.pending_value()).collect()
    }

    /// java: @Override public void requestSet(@NotNull List<T> value)
    fn request_set(&self, value: Vec<T>) {
        let entries = self.entry_factory.create_entries(value);
        *self.entries.borrow_mut() = entries;
        self.on_refresh();
    }

    /// java: @Override public void addEventListener(OptionEventListener<List<T>> listener)
    fn add_event_listener(&self, listener: Arc<dyn OptionEventListener<Vec<T>>>) {
        self.listeners.add(listener);
    }

    /// java: @Override @Deprecated public void addListener(BiConsumer<Option<List<T>>, List<T>> changedListener)
    fn add_listener(&self, changed_listener: Arc<dyn Fn(&dyn YOption<Vec<T>>, Vec<T>)>) {
        self.add_event_listener(yoption::listener_as_event_listener(changed_listener));
    }
}

impl<T: Clone + PartialEq + 'static> AnyListOption for ListOptionImpl<T> {
    /// java: @Override public int numberOfEntries()
    fn number_of_entries(&self) -> i32 {
        self.entries.borrow().len() as i32
    }

    /// java: @Override public int maximumNumberOfEntries()
    fn maximum_number_of_entries(&self) -> i32 {
        self.maximum_number_of_entries
    }

    /// java: @Override public int minimumNumberOfEntries()
    fn minimum_number_of_entries(&self) -> i32 {
        self.minimum_number_of_entries
    }

    fn insert_new_entry_any(&self) -> Arc<dyn AnyOption> {
        self.insert_new_entry()
    }

    /// java: @Override public void addRefreshListener(Runnable changedListener)
    fn add_refresh_listener(&self, changed_listener: Arc<dyn Fn()>) {
        self.refresh_listeners.borrow_mut().push(changed_listener);
    }
}

impl<T: Clone + PartialEq + 'static> ListOption<T> for ListOptionImpl<T> {
    fn list_entries(&self) -> Vec<Arc<dyn ListOptionEntry<T>>> {
        self.entries.borrow().clone()
    }

    /// java: @Override public ListOptionEntry<T> insertNewEntry()
    fn insert_new_entry(&self) -> Arc<dyn ListOptionEntry<T>> {
        let new_entry = self.entry_factory.create((self.initial_value)());
        if self.insert_entries_at_end {
            self.entries.borrow_mut().push(new_entry.clone());
        } else {
            // insert at top
            self.entries.borrow_mut().insert(0, new_entry.clone());
        }
        self.on_refresh();
        new_entry
    }

    /// java: @Override public void insertEntry(int index, ListOptionEntry<?> entry)
    fn insert_entry(&self, index: i32, entry: Arc<dyn ListOptionEntry<T>>) {
        self.entries.borrow_mut().insert(index as usize, entry);
        self.on_refresh();
    }

    /// java: @Override public int indexOf(ListOptionEntry<?> entry)
    fn index_of(&self, entry: &dyn ListOptionEntry<T>) -> i32 {
        self.entries
            .borrow()
            .iter()
            .position(|e| std::ptr::addr_eq(Arc::as_ptr(e), entry as *const dyn ListOptionEntry<T>))
            .map_or(-1, |i| i as i32)
    }

    /// java: @Override public void removeEntry(ListOptionEntry<?> entry)
    fn remove_entry(&self, entry: &dyn ListOptionEntry<T>) {
        let index = self.index_of(entry);
        if index >= 0 {
            self.entries.borrow_mut().remove(index as usize);
            self.on_refresh();
        }
    }
}

/// java: private class EntryFactory
struct EntryFactory<T: Clone + PartialEq + 'static> {
    /// java: private final Function<ListOptionEntry<T>, Controller<T>> controllerFunction;
    controller_function: EntryControllerFunction<T>,
    /// `ListOptionImpl.this`
    group: Weak<ListOptionImpl<T>>,
}

impl<T: Clone + PartialEq + 'static> EntryFactory<T> {
    /// java: public ListOptionEntry<T> create(T initialValue)
    fn create(&self, initial_value: T) -> Arc<dyn ListOptionEntry<T>> {
        ListOptionEntryImpl::new(self.group.clone(), initial_value, self.controller_function.clone())
    }

    /// java: private List<ListOptionEntry<T>> createEntries(Collection<T> values)
    fn create_entries(&self, values: Vec<T>) -> Vec<Arc<dyn ListOptionEntry<T>>> {
        values.into_iter().map(|v| self.create(v)).collect()
    }
}

/// java: @ApiStatus.Internal public static final class BuilderImpl<T> implements Builder<T>
pub struct BuilderImpl<T: Clone + 'static> {
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private OptionDescription description = OptionDescription.EMPTY;
    pub description: Arc<dyn OptionDescription>,
    /// java: private Function<ListOptionEntry<T>, Controller<T>> controllerFunction;
    pub controller_function: Option<EntryControllerFunction<T>>,
    /// java: private final Set<OptionFlag> flags = new HashSet<>();
    pub flags: Vec<Arc<dyn OptionFlag>>,
    /// java: private Supplier<T> initialValue;
    pub initial_value: Option<Arc<dyn Fn() -> T>>,
    /// java: private boolean collapsed = false;
    pub collapsed: bool,
    /// java: private boolean available = true;
    pub available: bool,
    /// java: private int minimumNumberOfEntries = 0;
    pub minimum_number_of_entries: i32,
    /// java: private int maximumNumberOfEntries = Integer.MAX_VALUE;
    pub maximum_number_of_entries: i32,
    /// java: private boolean insertEntriesAtEnd = false;
    pub insert_entries_at_end: bool,
    /// java: private final List<OptionEventListener<List<T>>> listeners = new ArrayList<>();
    pub listeners: Vec<Arc<dyn OptionEventListener<Vec<T>>>>,
    /// java: private Binding<List<T>> binding;
    pub binding: Option<Arc<dyn Binding<Vec<T>>>>,
    /// java: private StateManager<List<T>> stateManager;
    pub state_manager: Option<Arc<dyn StateManager<Vec<T>>>>,
}

impl<T: Clone + PartialEq + 'static> BuilderImpl<T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {
            name: component::empty(),
            description: option_description::EMPTY(),
            controller_function: None,
            flags: Vec::new(),
            initial_value: None,
            collapsed: false,
            available: true,
            minimum_number_of_entries: 0,
            maximum_number_of_entries: i32::MAX,
            insert_entries_at_end: false,
            listeners: Vec::new(),
            binding: None,
            state_manager: None,
        }
    }
}

impl<T: Clone + PartialEq + 'static> Default for BuilderImpl<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + PartialEq + 'static> Builder<T> for BuilderImpl<T> {
    fn name(mut self, name: MutableComponent) -> Self {
        self.name = name;
        self
    }

    fn description(mut self, description: Arc<dyn OptionDescription>) -> Self {
        self.description = description;
        self
    }

    fn initial(mut self, initial_value: impl Fn() -> T + 'static) -> Self {
        self.initial_value = Some(Arc::new(initial_value));
        self
    }

    fn initial_t(mut self, initial_value: T) -> Self {
        self.initial_value = Some(Arc::new(move || initial_value.clone()));
        self
    }

    /// java: `this.controllerFunction = opt -> controller.apply(opt).build();`
    fn controller<B: ControllerBuilder<T> + 'static>(mut self, controller: impl Fn(Arc<dyn YOption<T>>) -> B + 'static) -> Self {
        self.controller_function = Some(Arc::new(move |opt: Arc<dyn ListOptionEntry<T>>| {
            let opt: Arc<dyn YOption<T>> = opt;
            controller(opt).build()
        }));
        self
    }

    fn custom_controller(mut self, control: impl Fn(Arc<dyn ListOptionEntry<T>>) -> Arc<dyn Controller<T>> + 'static) -> Self {
        self.controller_function = Some(Arc::new(control));
        self
    }

    fn state(mut self, state_manager: Arc<dyn StateManager<Vec<T>>>) -> Self {
        assert!(self.binding.is_none(), "Cannot set state manager if binding is already set");
        self.state_manager = Some(state_manager);
        self
    }

    fn binding(mut self, binding: Arc<dyn Binding<Vec<T>>>) -> Self {
        assert!(self.state_manager.is_none(), "Cannot set binding if state manager is already set");
        self.binding = Some(binding);
        self
    }

    fn binding_3(mut self, def: Vec<T>, getter: impl Fn() -> Vec<T> + 'static, setter: impl Fn(Vec<T>) + 'static) -> Self {
        self.binding = Some(binding::generic(def, getter, setter));
        self
    }

    fn available(mut self, available: bool) -> Self {
        self.available = available;
        self
    }

    fn minimum_number_of_entries(mut self, number: i32) -> Self {
        self.minimum_number_of_entries = number;
        self
    }

    fn maximum_number_of_entries(mut self, number: i32) -> Self {
        self.maximum_number_of_entries = number;
        self
    }

    fn insert_entries_at_end(mut self, insert_at_end: bool) -> Self {
        self.insert_entries_at_end = insert_at_end;
        self
    }

    fn flag(mut self, flag: Vec<Arc<dyn OptionFlag>>) -> Self {
        for f in flag {
            option_flag::add_flag(&mut self.flags, f);
        }
        self
    }

    fn flags(mut self, flags: Vec<Arc<dyn OptionFlag>>) -> Self {
        for f in flags {
            option_flag::add_flag(&mut self.flags, f);
        }
        self
    }

    fn collapsed(mut self, collapsible: bool) -> Self {
        self.collapsed = collapsible;
        self
    }

    fn add_listener(mut self, listener: Arc<dyn OptionEventListener<Vec<T>>>) -> Self {
        self.listeners.push(listener);
        self
    }

    fn add_listeners(mut self, listeners: Vec<Arc<dyn OptionEventListener<Vec<T>>>>) -> Self {
        self.listeners.extend(listeners);
        self
    }

    fn listener(self, listener: Arc<dyn Fn(&dyn YOption<Vec<T>>, Vec<T>)>) -> Self {
        self.add_listener(yoption::listener_as_event_listener(listener))
    }

    fn listeners(self, listeners: Vec<Arc<dyn Fn(&dyn YOption<Vec<T>>, Vec<T>)>>) -> Self {
        let mapped: Vec<Arc<dyn OptionEventListener<Vec<T>>>> = listeners.into_iter().map(yoption::listener_as_event_listener).collect();
        self.add_listeners(mapped)
    }

    /// java: @Override public ListOption<T> build()
    fn build(self) -> Arc<dyn ListOption<T>> {
        let controller_function = self.controller_function.expect("`controller` must not be null");
        let initial_value = self.initial_value.expect("`initialValue` must not be null");
        assert!(self.state_manager.is_some() || self.binding.is_some(), "Either a state manager or binding must be set");
        let state_manager = match self.state_manager {
            Some(state_manager) => state_manager,
            None => state_manager::create_simple(self.binding.expect("checked above")),
        };
        ListOptionImpl::new(
            self.name,
            self.description,
            state_manager,
            initial_value,
            controller_function,
            self.flags,
            self.collapsed,
            self.available,
            self.minimum_number_of_entries,
            self.maximum_number_of_entries,
            self.insert_entries_at_end,
            self.listeners,
        )
    }
}
