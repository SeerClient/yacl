//! Ported from dev.isxander.yacl3.impl.ListOptionImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::list_option::Builder;
use crate::dev::isxander::yacl3::api::list_option::ListOption;
use crate::dev::isxander::yacl3::api::list_option_entry::ListOptionEntry;
use crate::dev::isxander::yacl3::api::option_description;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_event_listener::Event;
use crate::dev::isxander::yacl3::api::option_event_listener::OptionEventListener;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::state_manager;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::r#impl::list_option_entry_impl::ListOptionEntryImpl;
use crate::dev::isxander::yacl3::r#impl::provides_binding_for_deprecation::ProvidesBindingForDeprecation;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: @ApiStatus.Internal public final class ListOptionImpl<T> implements ListOption<T>
#[derive(Clone)]
pub struct ListOptionImpl<T> {
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final OptionDescription description;
    pub description: compat::Shared<dyn OptionDescription>,
    /// java: private final StateManager<List<T>> stateManager;
    pub state_manager: compat::Shared<dyn StateManager<Vec<T>>>,
    /// java: private final Supplier<T> initialValue;
    pub initial_value: Arc<dyn Fn() -> T>,
    /// java: private final List<ListOptionEntry<T>> entries;
    pub entries: Vec<compat::Shared<dyn ListOptionEntry<T>>>,
    /// java: private final boolean collapsed;
    pub collapsed: bool,
    /// java: private boolean available;
    pub available: bool,
    /// java: private final int minimumNumberOfEntries;
    pub minimum_number_of_entries: i32,
    /// java: private final int maximumNumberOfEntries;
    pub maximum_number_of_entries: i32,
    /// java: private final boolean insertEntriesAtEnd;
    pub insert_entries_at_end: bool,
    /// java: private final ImmutableSet<OptionFlag> flags;
    pub flags: std::collections::HashSet<compat::Shared<dyn OptionFlag>>,
    /// java: private final EntryFactory entryFactory;
    pub entry_factory: compat::Shared<EntryFactory>,
    /// java: private final List<OptionEventListener<List<T>>> listeners;
    pub listeners: Vec<compat::Shared<dyn OptionEventListener<Vec<T>>>>,
    /// java: private final List<Runnable> refreshListeners;
    pub refresh_listeners: Vec<Arc<dyn Fn()>>,
    /// java: private int currentListenerDepth = 0;
    pub current_listener_depth: i32,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl<T: Clone + 'static> ListOptionImpl<T> {
    /// java: public ListOptionImpl(@NotNull Component name, @NotNull OptionDescription description, @NotNull StateManager<List<T>> stateManager, @NotNull Supplier<T> initialValue, @NotNull Function<ListOptionEntry
    pub fn new(mut name: MutableComponent, mut description: compat::Shared<dyn OptionDescription>, mut state_manager: compat::Shared<dyn StateManager<Vec<T>>>, initial_value: Arc<dyn Fn() -> T>, controller_function: Arc<dyn Fn(compat::Shared<dyn ListOptionEntry<T>>) -> compat::Shared<dyn Controller<T>>>, mut flags: std::collections::HashSet<compat::Shared<dyn OptionFlag>>, mut collapsed: bool, mut available: bool, mut minimum_number_of_entries: i32, mut maximum_number_of_entries: i32, mut insert_entries_at_end: bool, mut listeners: Vec<compat::Shared<dyn OptionEventListener<Vec<T>>>>) -> compat::Shared<Self> {
        let __init_name = name.clone();
        let __init_description = description.clone();
        let __init_state_manager = state_manager.clone();
        let __init_initial_value = initial_value.clone();
        let __init_entry_factory = EntryFactory::new(controller_function.clone());
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            name: __init_name,
            description: __init_description,
            state_manager: __init_state_manager,
            initial_value: __init_initial_value,
            entries: Default::default(),
            collapsed: false,
            available: false,
            minimum_number_of_entries: 0,
            maximum_number_of_entries: 0,
            insert_entries_at_end: false,
            flags: Default::default(),
            entry_factory: __init_entry_factory,
            listeners: Default::default(),
            refresh_listeners: Default::default(),
            current_listener_depth: 0,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = ListOptionImpl::create_entries(&__this, &__this.lock().borrow().binding().lock().borrow().get_value()); compat::write(&__this, |__t| __t.entries = __v); }
        { let __v = collapsed; compat::write(&__this, |__t| __t.collapsed = __v); }
        { let __v = flags.clone(); compat::write(&__this, |__t| __t.flags = __v); }
        { let __v = available; compat::write(&__this, |__t| __t.available = __v); }
        { let __v = minimum_number_of_entries; compat::write(&__this, |__t| __t.minimum_number_of_entries = __v); }
        { let __v = maximum_number_of_entries; compat::write(&__this, |__t| __t.maximum_number_of_entries = __v); }
        { let __v = insert_entries_at_end; compat::write(&__this, |__t| __t.insert_entries_at_end = __v); }
        { let __v = Vec::new(); compat::write(&__this, |__t| __t.listeners = __v); }
        let __h0 = listeners.iter().cloned();
        __this.lock().borrow_mut().listeners.extend(__h0);
        { let __v = Vec::new(); compat::write(&__this, |__t| __t.refresh_listeners = __v); }
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let (__a0,) = (&Arc::new(move |old_value, new_value| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; ListOptionImpl::trigger_listener(&__this_1, Event::StateChange, false) }),); __this.lock().borrow_mut().state_manager.add_listener(__a0) };
        ListOptionImpl::trigger_listener(&__this, Event::Initial, false);
        __this
    }

    /// java: private List<ListOptionEntry<T>> createEntries(Collection<T> values)
    pub fn create_entries(__this: &compat::Shared<Self>, values: &Vec<T>) -> Vec<compat::Shared<dyn ListOptionEntry<T>>> {
        let __this_1 = compat::RenderThread::new(__this.clone());
        return values.iter().cloned().map(|__a0| EntryFactory::create(&compat::read(__this_1.get(), |__t| __t.entry_factory.clone()), &__a0)).collect::<Vec<_>>();
    }

    /// java: void triggerListener(OptionEventListener.Event event, boolean allowDepth)
    pub fn trigger_listener(__this: &compat::Shared<Self>, mut event: Event, mut allow_depth: bool) {
        if allow_depth || compat::read(__this, |__t| __t.current_listener_depth) == 0 {
            // TODO(fabric2seer): static call Validate.isTrue(..)
            //   java: Validate.isTrue(
            //   java:                     currentListenerDepth <= 10,
            //   java:                     "Listener depth exceeded 10! Possible cyclic listener pattern: a listener triggered an event that triggered the initial event etc etc."
            //   java:             );
            { let __v = compat::read(__this, |__t| __t.current_listener_depth) + 1; compat::write(__this, |__t| __t.current_listener_depth = __v); }
            for listener in compat::read(__this, |__t| __t.listeners.clone()) {
                { let (__a0, __a1,) = (&(__this.clone() as compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>), event,); listener.lock().borrow().on_event(__a0, __a1) };
            }
            { let __v = compat::read(__this, |__t| __t.current_listener_depth) - 1; compat::write(__this, |__t| __t.current_listener_depth = __v); }
        }
    }

    /// java: private void onRefresh()
    pub fn on_refresh(__this: &compat::Shared<Self>) {
        for __a0 in compat::read(__this, |__t| __t.refresh_listeners.clone()) {
            (__a0)();
        }
        ListOptionImpl::trigger_listener(__this, Event::Other, true);
    }

}

impl<T: Clone + 'static> ListOption<T> for ListOptionImpl<T> {
    /// java: @Override public void insertEntry(int index, ListOptionEntry<?> entry)
    fn insert_entry(&mut self, mut index: i32, entry: &compat::Shared<dyn ListOptionEntry<compat::Object>>) {
        self.entries.insert((index) as usize, entry.clone());
        ListOptionImpl::on_refresh(&self.self_ref.upgrade().unwrap());
    }

    /// java: @Override public ListOptionEntry<T> insertNewEntry()
    fn insert_new_entry(&mut self) -> compat::Shared<dyn ListOptionEntry<T>> {
        let mut new_entry: compat::Shared<dyn ListOptionEntry<T>> = EntryFactory::create(&self.entry_factory, &(self.initial_value)());
        if self.insert_entries_at_end {
            self.entries.push(new_entry.clone());
        } else {
            self.entries.insert((0) as usize, new_entry.clone());
        }
        ListOptionImpl::on_refresh(&self.self_ref.upgrade().unwrap());
        return new_entry;
    }

    /// java: @Override public void removeEntry(ListOptionEntry<?> entry)
    fn remove_entry(&mut self, entry: &compat::Shared<dyn ListOptionEntry<compat::Object>>) {
        if compat::remove_item(&mut self.entries, &entry.clone()) {
            ListOptionImpl::on_refresh(&self.self_ref.upgrade().unwrap());
        }
    }

    /// java: @Override public int indexOf(ListOptionEntry<?> entry)
    fn index_of(&self, entry: &compat::Shared<dyn ListOptionEntry<compat::Object>>) -> i32 {
        return self.entries.iter().position(|e| *e == entry.clone()).map_or(-1, |i| i as i32);
    }

    /// java: @Override public int numberOfEntries()
    fn number_of_entries(&self) -> i32 {
        return (self.entries.len() as i32);
    }

    /// java: @Override public int maximumNumberOfEntries()
    fn maximum_number_of_entries(&self) -> i32 {
        return self.maximum_number_of_entries;
    }

    /// java: @Override public int minimumNumberOfEntries()
    fn minimum_number_of_entries(&self) -> i32 {
        return self.minimum_number_of_entries;
    }

    /// java: @Override public void addRefreshListener(Runnable changedListener)
    fn add_refresh_listener(&mut self, changed_listener: Arc<dyn Fn()>) {
        self.refresh_listeners.push(changed_listener.clone());
    }

}

impl<T: Clone + 'static> OptionGroup for ListOptionImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @NotNull ImmutableList<ListOptionEntry<T>> options() (dispatch: the body for this class)
    fn options(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>> {
        ListOptionImpl::options_dyn
    }

    /// java: @Override public boolean collapsed() (dispatch: the body for this class)
    fn collapsed(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> bool {
        ListOptionImpl::collapsed_dyn
    }

    /// java: @Override public boolean isRoot() (dispatch: the body for this class)
    fn is_root(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> bool {
        ListOptionImpl::is_root_dyn
    }

    /// java: @Override public @NotNull Component name() (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option>) -> MutableComponent {
        ListOptionImpl::name_dyn
    }

    /// java: @Override public @NotNull OptionDescription description() (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option>) -> compat::Shared<dyn OptionDescription> {
        ListOptionImpl::description_dyn
    }

    /// java: @Override public @NotNull Component tooltip() (dispatch: the body for this class)
    fn tooltip(&self) -> fn(&compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option>) -> MutableComponent {
        ListOptionImpl::tooltip_dyn
    }

}

impl<T: Clone + 'static> crate::dev::isxander::yacl3::api::option::Option<compat::Object> for ListOptionImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @NotNull Component name()
    fn name(&self) -> MutableComponent {
        return self.name.clone();
    }

    /// java: @Override public @NotNull OptionDescription description()
    fn description(&self) -> compat::Shared<dyn OptionDescription> {
        return self.description.clone();
    }

    /// java: @Override public @NotNull Component tooltip()
    fn tooltip(&self) -> MutableComponent {
        return ({ let __r = &self.description(); let __f = compat::dispatch(__r, |__t| __t.text()); __f(__r) });
    }

    /// java: @Override public @NotNull Controller<List<T>> controller()
    fn controller(&self) -> compat::Shared<dyn Controller<Vec<T>>> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public @NotNull StateManager<List<T>> stateManager()
    fn state_manager(&self) -> compat::Shared<dyn StateManager<Vec<T>>> {
        return self.state_manager.clone();
    }

    /// java: @Override @Deprecated public @NotNull Binding<List<T>> binding()
    fn binding(&self) -> compat::Shared<dyn Binding<Vec<T>>> {
        if self.state_manager.lock().borrow().as_any().is::<compat::Shared<dyn ProvidesBindingForDeprecation<compat::Object>>>() {
            return (self.state_manager.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn ProvidesBindingForDeprecation<Vec<T>>>>().unwrap().lock().borrow()).get_binding();
        }
        panic!("UnsupportedOperationException: {}", "Binding is not available for this option - using a new state manager which does not directly expose the binding as it may not have one.");
    }

    /// java: @Override public @NotNull ImmutableSet<OptionFlag> flags()
    fn flags(&self) -> std::collections::HashSet<compat::Shared<dyn OptionFlag>> {
        return self.flags.clone();
    }

    /// java: @Override public @NotNull ImmutableList<T> pendingValue()
    fn pending_value(&self) -> Vec<T> {
        return (self.entries.iter().cloned().map(|__a0| __a0.lock().borrow().pending_value()).collect::<Vec<_>>()).iter().cloned().collect::<Vec<_>>();
    }

    /// java: @Override public void requestSet(@NotNull List<T> value)
    fn request_set(&mut self, mut value: Vec<T>) {
        self.entries.clear();
        self.entries.extend(ListOptionImpl::create_entries(&self.self_ref.upgrade().unwrap(), &value).into_iter());
        ListOptionImpl::on_refresh(&self.self_ref.upgrade().unwrap());
    }

    /// java: @Override public boolean changed()
    fn changed(&self) -> bool {
        return !(self.binding().lock().borrow().get_value() == self.pending_value());
    }

    /// java: @Override public boolean applyValue()
    fn apply_value(&self) -> bool {
        if self.changed() {
            { let __s0 = self.pending_value(); self.binding().lock().borrow_mut().set_value(__s0) };
            return true;
        }
        return false;
    }

    /// java: @Override public void forgetPendingValue()
    fn forget_pending_value(&mut self) {
        { let __s0 = self.binding().lock().borrow().get_value(); self.request_set(__s0) };
    }

    /// java: @Override public void requestSetDefault()
    fn request_set_default(&mut self) {
        { let __s0 = self.binding().lock().borrow().default_value(); self.request_set(__s0) };
    }

    /// java: @Override public boolean isPendingValueDefault()
    fn is_pending_value_default(&self) -> bool {
        return (self.binding().lock().borrow().default_value() == self.pending_value());
    }

    /// java: @Override public boolean available()
    fn available(&self) -> bool {
        return self.available;
    }

    /// java: @Override public void setAvailable(boolean available)
    fn set_available(&mut self, mut available: bool) {
        let mut changed: bool = (self.available != available);
        self.available = available;
        if changed {
            if !available {
                self.state_manager.lock().borrow_mut().sync();
            }
            ListOptionImpl::trigger_listener(&self.self_ref.upgrade().unwrap(), Event::AvailabilityChange, !available);
        }
    }

    /// java: @Override public void addEventListener(OptionEventListener<List<T>> listener)
    fn add_event_listener(&mut self, mut listener: compat::Shared<dyn OptionEventListener<Vec<T>>>) {
        self.listeners.push(listener.clone());
    }

    /// java: @Override @Deprecated public void addListener(BiConsumer<Option<List<T>>, List<T>> changedListener)
    fn add_listener(&mut self, changed_listener: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Vec<T>>>, Vec<T>)>) {
        // TODO(fabric2seer): `.pendingValue(..)` on a value of unknown type
        //   java: addEventListener((opt, event) -> changedListener.accept(opt, opt.pendingValue()));
    }

}

impl<T: Clone + 'static> ListOptionImpl<T> {
    /// java: @Override public @NotNull ImmutableList<ListOptionEntry<T>> options()
    pub fn options_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<dyn ListOptionEntry<T>>> {
        return __this.lock().borrow().entries.iter().cloned().collect::<Vec<_>>();
    }

    /// ListOptionImpl.options as the dispatch of OptionGroup.options
    pub fn options_dyn(__this: &compat::Shared<dyn OptionGroup>) -> Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>> {
        Self::options_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean collapsed()
    pub fn collapsed_impl(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.collapsed);
    }

    /// ListOptionImpl.collapsed as the dispatch of OptionGroup.collapsed
    pub fn collapsed_dyn(__this: &compat::Shared<dyn OptionGroup>) -> bool {
        Self::collapsed_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean isRoot()
    pub fn is_root_impl(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

    /// ListOptionImpl.isRoot as the dispatch of OptionGroup.isRoot
    pub fn is_root_dyn(__this: &compat::Shared<dyn OptionGroup>) -> bool {
        Self::is_root_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public @NotNull Component name()
    pub fn name_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return compat::read(__this, |__t| __t.name.clone());
    }

    /// ListOptionImpl.name as the dispatch of Option.name
    pub fn name_dyn(__this: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option>) -> MutableComponent {
        Self::name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public @NotNull OptionDescription description()
    pub fn description_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn OptionDescription> {
        return compat::read(__this, |__t| __t.description.clone());
    }

    /// ListOptionImpl.description as the dispatch of Option.description
    pub fn description_dyn(__this: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option>) -> compat::Shared<dyn OptionDescription> {
        Self::description_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public @NotNull Component tooltip()
    pub fn tooltip_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return ({ let __r = &__this.lock().borrow().description(); let __f = compat::dispatch(__r, |__t| __t.text()); __f(__r) });
    }

    /// ListOptionImpl.tooltip as the dispatch of Option.tooltip
    pub fn tooltip_dyn(__this: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option>) -> MutableComponent {
        Self::tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: private class EntryFactory
#[derive(Clone)]
pub struct EntryFactory {
    /// java: private final Function<ListOptionEntry<T>, Controller<T>> controllerFunction;
    pub controller_function: Arc<dyn Fn(compat::Shared<dyn ListOptionEntry<T>>) -> compat::Shared<dyn Controller<T>>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl EntryFactory {
    /// java: private EntryFactory(Function<ListOptionEntry<T>, Controller<T>> controllerFunction)
    pub fn new(controller_function: Arc<dyn Fn(compat::Shared<dyn ListOptionEntry<T>>) -> compat::Shared<dyn Controller<T>>>) -> compat::Shared<Self> {
        let __init_controller_function = controller_function.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            controller_function: __init_controller_function,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: public ListOptionEntry<T> create(T initialValue)
    pub fn create(__this: &compat::Shared<Self>, initial_value: &T) -> compat::Shared<dyn ListOptionEntry<T>> {
        // TODO(fabric2seer): `ListOptionImpl.this` (the enclosing instance of an inner class)
        //   java: return new ListOptionEntryImpl<>(ListOptionImpl.this, initialValue, controllerFunction);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: @ApiStatus.Internal public static final class BuilderImpl<T> implements Builder<T>
#[derive(Clone)]
pub struct BuilderImpl<T> {
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private OptionDescription description = OptionDescription.EMPTY;
    pub description: compat::Shared<dyn OptionDescription>,
    /// java: private Function<ListOptionEntry<T>, Controller<T>> controllerFunction;
    pub controller_function: Option<Arc<dyn Fn(compat::Shared<dyn ListOptionEntry<T>>) -> compat::Shared<dyn Controller<T>>>>,
    /// java: private final Set<OptionFlag> flags = new HashSet<>();
    pub flags: std::collections::HashSet<compat::Shared<dyn OptionFlag>>,
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
    pub listeners: Vec<compat::Shared<dyn OptionEventListener<Vec<T>>>>,
    /// java: private Binding<List<T>> binding;
    pub binding: Option<compat::Shared<dyn Binding<Vec<T>>>>,
    /// java: private StateManager<List<T>> stateManager;
    pub state_manager: Option<compat::Shared<dyn StateManager<Vec<T>>>>,
}

impl<T: Clone + 'static> BuilderImpl<T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            name: component::empty(),
            description: option_description::EMPTY.get_mut().clone(),
            controller_function: None,
            flags: std::collections::HashSet::new(),
            initial_value: None,
            collapsed: false,
            available: true,
            minimum_number_of_entries: 0,
            maximum_number_of_entries: i32::MAX,
            insert_entries_at_end: false,
            listeners: Vec::new(),
            binding: None,
            state_manager: None,
        };
        this
    }

}

impl<T: Clone + 'static> Builder<T> for BuilderImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder<T> name(@NotNull Component name)
    fn name(&mut self, mut name: MutableComponent) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(name, "`name` must not be null");
        self.name = name.clone();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> description(@NotNull OptionDescription description)
    fn description(&mut self, mut description: compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(description, "`description` must not be null");
        self.description = description.clone();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> initial(@NotNull Supplier<T> initialValue)
    fn initial(&mut self, initial_value: Arc<dyn Fn() -> T>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(initialValue, "`initialValue` cannot be empty");
        self.initial_value = Some(initial_value.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> initial(@NotNull T initialValue)
    fn initial_t(&mut self, initial_value: &T) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(initialValue, "`initialValue` cannot be empty");
        self.initial_value = Some(Arc::new(move || initial_value.clone()));
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> controller(@NotNull Function<Option<T>, ControllerBuilder<T>> controller)
    fn controller(&mut self, controller: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> compat::Shared<dyn ControllerBuilder<T>>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(controller, "`controller` cannot be null");
        self.controller_function = Some(Arc::new(move |opt| (controller)(opt.clone()).lock().borrow_mut().build()));
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> customController(@NotNull Function<ListOptionEntry<T>, Controller<T>> control)
    fn custom_controller(&mut self, control: Arc<dyn Fn(compat::Shared<dyn ListOptionEntry<T>>) -> compat::Shared<dyn Controller<T>>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(control, "`control` cannot be null");
        self.controller_function = Some(control.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> state(@NotNull StateManager<List<T>> stateManager)
    fn state(&mut self, mut state_manager: compat::Shared<dyn StateManager<Vec<T>>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(stateManager, "`stateManager` cannot be null");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(binding == null, "Cannot set state manager if binding is already set");
        self.state_manager = Some(state_manager.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> binding(@NotNull Binding<List<T>> binding)
    fn binding(&mut self, mut binding: compat::Shared<dyn Binding<Vec<T>>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(binding, "`binding` cannot be null");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(stateManager == null, "Cannot set binding if state manager is already set");
        self.binding = Some(binding.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> binding(@NotNull List<T> def, @NotNull Supplier<@NotNull List<T>> getter, @NotNull Consumer<@NotNull List<T>> setter)
    fn binding_3(&mut self, def: &Vec<T>, getter: Arc<dyn Fn() -> Vec<T>>, setter: Arc<dyn Fn(Vec<T>)>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(def, "`def` must not be null");
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(getter, "`getter` must not be null");
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(setter, "`setter` must not be null");
        self.binding = Some(binding::generic(def, getter.clone(), setter.clone()));
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> available(boolean available)
    fn available(&mut self, mut available: bool) -> compat::Shared<dyn Builder<T>> {
        self.available = available;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> minimumNumberOfEntries(int number)
    fn minimum_number_of_entries(&mut self, mut number: i32) -> compat::Shared<dyn Builder<T>> {
        self.minimum_number_of_entries = number;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> maximumNumberOfEntries(int number)
    fn maximum_number_of_entries(&mut self, mut number: i32) -> compat::Shared<dyn Builder<T>> {
        self.maximum_number_of_entries = number;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> insertEntriesAtEnd(boolean insertAtEnd)
    fn insert_entries_at_end(&mut self, mut insert_at_end: bool) -> compat::Shared<dyn Builder<T>> {
        self.insert_entries_at_end = insert_at_end;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> flag(@NotNull OptionFlag... flag)
    fn flag(&mut self, flag: &Vec<compat::Shared<dyn OptionFlag>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(flag, "`flag` must not be null");
        self.flags.extend(flag.to_vec().into_iter());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> flags(@NotNull Collection<OptionFlag> flags)
    fn flags(&mut self, flags: &Vec<compat::Shared<dyn OptionFlag>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(flags, "`flags` must not be null");
        self.flags.extend(flags.iter().cloned());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> collapsed(boolean collapsible)
    fn collapsed(&mut self, mut collapsible: bool) -> compat::Shared<dyn Builder<T>> {
        self.collapsed = collapsible;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> addListener(@NotNull OptionEventListener<List<T>> listener)
    fn add_listener(&mut self, mut listener: compat::Shared<dyn OptionEventListener<Vec<T>>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(listener, "`listener` must not be null");
        self.listeners.push(listener.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> addListeners(@NotNull Collection<@NotNull OptionEventListener<List<T>>> optionEventListeners)
    fn add_listeners(&mut self, option_event_listeners: &Vec<compat::Shared<dyn OptionEventListener<Vec<T>>>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(optionEventListeners, "`optionEventListeners` must not be null");
        self.listeners.extend(option_event_listeners.iter().cloned());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> listener(@NotNull BiConsumer<Option<List<T>>, List<T>> listener)
    fn listener(&mut self, listener: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Vec<T>>>, Vec<T>)>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(listener, "`listener` must not be null");
        // TODO(fabric2seer): `.pendingValue(..)` on a value of unknown type
        //   java: return this.addListener((opt, event) -> listener.accept(opt, opt.pendingValue()));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> listeners(@NotNull Collection<BiConsumer<Option<List<T>>, List<T>>> listeners)
    fn listeners(&mut self, listeners: &Vec<Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Vec<T>>>, Vec<T>)>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(listeners, "`listeners` must not be null");
        // TODO(fabric2seer): `.pendingValue(..)` on a value of unknown type
        //   java: this.addListeners(listeners.stream()
        //   java:                     .map(listener ->
        //   java:                             (OptionEventListener<List<T>>) (opt, event) ->
        //   java:                                     listener.accept(opt, opt.pendingValue())
        //   java:                     ).toList()
        //   java:             );
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption<T> build()
    fn build(&mut self) -> compat::Shared<dyn ListOption<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(controllerFunction, "`controller` must not be null");
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(initialValue, "`initialValue` must not be null");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(stateManager != null || binding != null, "Either a state manager or binding must be set");
        if self.state_manager.is_none() {
            self.state_manager = Some(state_manager::create_simple(self.binding.as_ref().unwrap()));
        }
        return (ListOptionImpl::new(self.name.clone(), self.description.clone(), self.state_manager.clone().unwrap(), self.initial_value.clone().unwrap(), self.controller_function.clone().unwrap(), self.flags.iter().cloned().collect::<std::collections::HashSet<_>>(), self.collapsed, self.available, self.minimum_number_of_entries, self.maximum_number_of_entries, self.insert_entries_at_end, self.listeners.clone()) as compat::Shared<dyn ListOption<compat::Object>>);
    }

}
