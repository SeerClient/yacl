//! Ported from dev.isxander.yacl3.impl.OptionImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::option::{self as yoption, AnyOption, Builder, Option as YOption};
use crate::dev::isxander::yacl3::api::option_description::{self, OptionDescription};
use crate::dev::isxander::yacl3::api::option_event_listener::{Event, OptionEventListener};
use crate::dev::isxander::yacl3::api::option_flag::{self, OptionFlag};
use crate::dev::isxander::yacl3::api::state_manager::{self, ResetAction, StateManager};
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::YACLConstants;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::cell::{Cell, RefCell};
use std::sync::Weak;

/// `OptionImpl.listeners` + `currentListenerDepth` + `triggerListener` (also `ListOptionImpl`'s): the listener list
/// and the object handed to every listener as the option (Java's `this` - the outermost object, so a subclass such as
/// `LabelOptionImpl` is what a listener receives).
pub struct OptionListeners<T: Clone + 'static> {
    listeners: RefCell<Vec<Arc<dyn OptionEventListener<T>>>>,
    current_listener_depth: Cell<i32>,
    this: RefCell<Weak<dyn YOption<T>>>,
}

impl<T: Clone + 'static> OptionListeners<T> {
    pub fn new(listeners: Vec<Arc<dyn OptionEventListener<T>>>, this: Weak<dyn YOption<T>>) -> OptionListeners<T> {
        OptionListeners { listeners: RefCell::new(listeners), current_listener_depth: Cell::new(0), this: RefCell::new(this) }
    }

    /// The option object every listener is handed (`this`), recorded once it is behind its reference.
    pub fn set_this(&self, this: Weak<dyn YOption<T>>) {
        *self.this.borrow_mut() = this;
    }

    /// `listeners.add(listener)`
    pub fn add(&self, listener: Arc<dyn OptionEventListener<T>>) {
        self.listeners.borrow_mut().push(listener);
    }

    /// java: private void triggerListener(OptionEventListener.Event event, boolean allowDepth)
    pub fn trigger_listener(&self, event: Event, allow_depth: bool) {
        if allow_depth || self.current_listener_depth.get() == 0 {
            assert!(
                self.current_listener_depth.get() <= 10,
                "Listener depth exceeded 10! Possible cyclic listener pattern: a listener triggered an event that triggered the initial event etc etc."
            );
            self.current_listener_depth.set(self.current_listener_depth.get() + 1);
            let this = self.this.borrow().upgrade();
            if let Some(this) = this {
                // the list as it is now: a listener that adds a listener does not see it run this time
                let listeners = self.listeners.borrow().clone();
                for listener in listeners {
                    listener.on_event(&*this, event);
                }
            }
            self.current_listener_depth.set(self.current_listener_depth.get() - 1);
        }
    }
}

/// `this` before the object is behind its reference (an empty `Weak<dyn Option<T>>`).
pub fn no_this<T: Clone + 'static>() -> Weak<dyn YOption<T>> {
    Weak::<OptionImpl<T>>::new()
}

/// java: @ApiStatus.Internal public class OptionImpl<T> implements Option<T>
pub struct OptionImpl<T: Clone + 'static> {
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private OptionDescription description;
    pub description: Rc<RefCell<Option<Arc<dyn OptionDescription>>>>,
    /// java: private final Controller<T> controller;
    pub controller: RefCell<Option<Arc<dyn Controller<T>>>>,
    /// java: private boolean available;
    pub available: Cell<bool>,
    /// java: private final ImmutableSet<OptionFlag> flags;
    pub flags: Vec<Arc<dyn OptionFlag>>,
    /// java: private final StateManager<T> stateManager;
    pub state_manager: Arc<dyn StateManager<T>>,
    /// java: private final List<OptionEventListener<T>> listeners; private int currentListenerDepth;
    pub listeners: Rc<OptionListeners<T>>,
    description_function: Arc<dyn Fn(T) -> Arc<dyn OptionDescription>>,
}

impl<T: Clone + 'static> OptionImpl<T> {
    /// java: public OptionImpl(@NotNull Component name, @NotNull Function<T, OptionDescription> descriptionFunction,
    /// @NotNull Function<Option<T>, Controller<T>> controlGetter, @NotNull StateManager<T> stateManager,
    /// boolean available, ImmutableSet<OptionFlag> flags, @NotNull Collection<OptionEventListener<T>> listeners)
    pub fn new(
        name: MutableComponent,
        description_function: Arc<dyn Fn(T) -> Arc<dyn OptionDescription>>,
        control_getter: Arc<dyn Fn(Arc<dyn YOption<T>>) -> Arc<dyn Controller<T>>>,
        state_manager: Arc<dyn StateManager<T>>,
        available: bool,
        flags: Vec<Arc<dyn OptionFlag>>,
        listeners: Vec<Arc<dyn OptionEventListener<T>>>,
    ) -> Arc<OptionImpl<T>> {
        let option = Arc::new(OptionImpl::fields(name, description_function, state_manager, available, flags, listeners));
        let this: Arc<dyn YOption<T>> = option.clone();
        option.construct(this, control_getter);
        option
    }

    /// The constructor's field initialisers (`:31-35`): the object before it is behind its reference.
    pub fn fields(
        name: MutableComponent,
        description_function: Arc<dyn Fn(T) -> Arc<dyn OptionDescription>>,
        state_manager: Arc<dyn StateManager<T>>,
        available: bool,
        flags: Vec<Arc<dyn OptionFlag>>,
        listeners: Vec<Arc<dyn OptionEventListener<T>>>,
    ) -> OptionImpl<T> {
        OptionImpl {
            name,
            description: Rc::new(RefCell::new(None)),
            controller: RefCell::new(None),
            available: Cell::new(available),
            flags,
            state_manager,
            listeners: Rc::new(OptionListeners::new(listeners, no_this::<T>())),
            description_function,
        }
    }

    /// The constructor's statements that hand `this` on (`:36-41`), with `this` the outermost object (a subclass's).
    pub fn construct(&self, this: Arc<dyn YOption<T>>, control_getter: Arc<dyn Fn(Arc<dyn YOption<T>>) -> Arc<dyn Controller<T>>>) {
        self.listeners.set_this(Arc::downgrade(&this));
        // java: this.controller = controlGetter.apply(this);
        let controller = control_getter(this);
        *self.controller.borrow_mut() = Some(controller);
        // java: this.stateManager.addListener((oldValue, newValue) -> triggerListener(STATE_CHANGE, false));
        let listeners = Rc::downgrade(&self.listeners);
        self.state_manager.add_listener(Arc::new(move |_old_value: T, _new_value: T| {
            if let Some(listeners) = listeners.upgrade() {
                listeners.trigger_listener(Event::StateChange, false);
            }
        }));
        // java: addEventListener((opt, event) -> description = descriptionFunction.apply(opt.pendingValue()));
        let description = Rc::downgrade(&self.description);
        let description_function = self.description_function.clone();
        self.listeners.add(Arc::new(move |opt: &dyn YOption<T>, _event: Event| {
            let next = description_function(opt.pending_value());
            if let Some(description) = description.upgrade() {
                *description.borrow_mut() = Some(next);
            }
        }));
        // java: triggerListener(OptionEventListener.Event.INITIAL, false);
        self.listeners.trigger_listener(Event::Initial, false);
    }
}

impl<T: Clone + 'static> AnyOption for OptionImpl<T> {
    /// java: @Override public @NotNull Component name()
    fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: @Override public @NotNull OptionDescription description()
    fn description(&self) -> Arc<dyn OptionDescription> {
        self.description.borrow().clone().unwrap_or_else(option_description::EMPTY)
    }

    /// java: @Override public @NotNull Component tooltip()
    fn tooltip(&self) -> MutableComponent {
        self.description().text()
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
            self.listeners.trigger_listener(Event::AvailabilityChange, !available);
        }
    }

    /// java: @Override public @NotNull ImmutableSet<OptionFlag> flags()
    fn flags(&self) -> Vec<Arc<dyn OptionFlag>> {
        self.flags.clone()
    }

    /// java: @Override public boolean changed()
    fn changed(&self) -> bool {
        !self.state_manager.is_synced()
    }

    /// java: @Override public boolean applyValue()
    fn apply_value(&self) -> bool {
        if self.changed() {
            self.state_manager.apply();
            return true;
        }
        false
    }

    /// java: @Override public void forgetPendingValue()
    fn forget_pending_value(&self) {
        self.state_manager.sync();
    }

    /// java: @Override public void requestSetDefault()
    fn request_set_default(&self) {
        self.state_manager.reset_to_default(ResetAction::ByOption);
    }

    /// java: @Override public boolean isPendingValueDefault()
    fn is_pending_value_default(&self) -> bool {
        self.state_manager.is_default()
    }

    fn provide_controller_widget(&self, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        self.controller().provide_widget(screen, widget_dimension)
    }

    fn add_any_event_listener(&self, listener: Arc<dyn Fn(&dyn AnyOption, Event)>) {
        self.add_event_listener(yoption::any_event_listener(listener));
    }

    fn add_any_listener(&self, changed_listener: Arc<dyn Fn(&dyn AnyOption)>) {
        self.add_event_listener(yoption::any_listener(changed_listener));
    }
}

impl<T: Clone + 'static> YOption<T> for OptionImpl<T> {
    /// java: @Override public @NotNull Controller<T> controller()
    fn controller(&self) -> Arc<dyn Controller<T>> {
        self.controller.borrow().clone().expect("OptionImpl.controller() before the constructor set it")
    }

    /// java: @Override public @NotNull StateManager<T> stateManager()
    fn state_manager(&self) -> Arc<dyn StateManager<T>> {
        self.state_manager.clone()
    }

    /// java: @Override @Deprecated public @NotNull Binding<T> binding()
    fn binding(&self) -> Arc<dyn Binding<T>> {
        match self.state_manager.get_binding_for_deprecation() {
            Some(binding) => binding,
            None => panic!(
                "UnsupportedOperationException: Binding is not available for this option - using a new state manager which does not directly expose the binding as it may not have one."
            ),
        }
    }

    /// java: @Override public @NotNull T pendingValue()
    fn pending_value(&self) -> T {
        self.state_manager.get()
    }

    /// java: @Override public void requestSet(@NotNull T value)
    fn request_set(&self, value: T) {
        self.state_manager.set(value);
    }

    /// java: @Override public void addEventListener(OptionEventListener<T> listener)
    fn add_event_listener(&self, listener: Arc<dyn OptionEventListener<T>>) {
        self.listeners.add(listener);
    }

    /// java: @Override @Deprecated public void addListener(BiConsumer<Option<T>, T> changedListener)
    fn add_listener(&self, changed_listener: Arc<dyn Fn(&dyn YOption<T>, T)>) {
        self.add_event_listener(yoption::listener_as_event_listener(changed_listener));
    }
}

/// java: @ApiStatus.Internal public static class BuilderImpl<T> implements Builder<T>
pub struct BuilderImpl<T: Clone + 'static> {
    /// java: private Component name = Component.literal("Name not specified!").withStyle(ChatFormatting.RED);
    pub name: MutableComponent,
    /// java: private Function<T, OptionDescription> descriptionFunction = pending -> OptionDescription.EMPTY;
    pub description_function: Arc<dyn Fn(T) -> Arc<dyn OptionDescription>>,
    /// java: private Function<Option<T>, Controller<T>> controlGetter;
    pub control_getter: Option<Arc<dyn Fn(Arc<dyn YOption<T>>) -> Arc<dyn Controller<T>>>>,
    /// java: private boolean available = true;
    pub available: bool,
    /// java: private final Set<OptionFlag> flags = new HashSet<>();
    pub flags: Vec<Arc<dyn OptionFlag>>,
    /// java: private final List<OptionEventListener<T>> listeners = new ArrayList<>();
    pub listeners: Vec<Arc<dyn OptionEventListener<T>>>,
    /// java: private @Nullable Binding<T> binding;
    pub binding: Option<Arc<dyn Binding<T>>>,
    /// java: private boolean instantDeprecated = false;
    pub instant_deprecated: bool,
    /// java: private @Nullable StateManager<T> stateManager;
    pub state_manager: Option<Arc<dyn StateManager<T>>>,
}

impl<T: Clone + PartialEq + 'static> BuilderImpl<T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {
            name: component::literal("Name not specified!").with_style_format(ChatFormatting::Red),
            description_function: Arc::new(|_pending: T| option_description::EMPTY()),
            control_getter: None,
            available: true,
            flags: Vec::new(),
            listeners: Vec::new(),
            binding: None,
            instant_deprecated: false,
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
    /// java: @Override public Builder<T> name(@NotNull Component name)
    fn name(mut self, name: MutableComponent) -> Self {
        self.name = name;
        self
    }

    /// java: @Override public Builder<T> description(@NotNull OptionDescription description)
    fn description(self, description: Arc<dyn OptionDescription>) -> Self {
        self.description_function(move |_opt: T| description.clone())
    }

    /// java: @Override public Builder<T> description(@NotNull Function<T, OptionDescription> descriptionFunction)
    fn description_function(mut self, description_function: impl Fn(T) -> Arc<dyn OptionDescription> + 'static) -> Self {
        self.description_function = Arc::new(description_function);
        self
    }

    /// java: @Override public Builder<T> controller(@NotNull Function<Option<T>, ControllerBuilder<T>> controllerBuilder)
    fn controller<B: ControllerBuilder<T> + 'static>(self, controller_builder: impl Fn(Arc<dyn YOption<T>>) -> B + 'static) -> Self {
        self.custom_controller(move |opt: Arc<dyn YOption<T>>| controller_builder(opt).build())
    }

    /// java: @Override public Builder<T> customController(@NotNull Function<Option<T>, Controller<T>> control)
    fn custom_controller(mut self, control: impl Fn(Arc<dyn YOption<T>>) -> Arc<dyn Controller<T>> + 'static) -> Self {
        self.control_getter = Some(Arc::new(control));
        self
    }

    /// java: @Override public Builder<T> stateManager(@NotNull StateManager<T> stateManager)
    fn state_manager(mut self, state_manager: Arc<dyn StateManager<T>>) -> Self {
        assert!(self.binding.is_none(), "Cannot set state manager when binding is set");
        assert!(!self.instant_deprecated, "Cannot set state manager when instant is set");
        self.state_manager = Some(state_manager);
        self
    }

    /// java: @Override public Builder<T> binding(@NotNull Binding<T> binding)
    fn binding(mut self, binding: Arc<dyn Binding<T>>) -> Self {
        assert!(self.state_manager.is_none(), "Cannot set binding when state manager is set");
        self.binding = Some(binding);
        self
    }

    /// java: @Override public Builder<T> binding(@NotNull T def, @NotNull Supplier<@NotNull T> getter, @NotNull Consumer<@NotNull T> setter)
    fn binding_3(self, def: T, getter: impl Fn() -> T + 'static, setter: impl Fn(T) + 'static) -> Self {
        self.binding(binding::generic(def, getter, setter))
    }

    /// java: @Override public Builder<T> available(boolean available)
    fn available(mut self, available: bool) -> Self {
        self.available = available;
        self
    }

    /// java: @Override public Builder<T> flag(@NotNull OptionFlag... flag)
    fn flag(mut self, flag: Vec<Arc<dyn OptionFlag>>) -> Self {
        for f in flag {
            option_flag::add_flag(&mut self.flags, f);
        }
        self
    }

    /// java: @Override public Builder<T> flags(@NotNull Collection<? extends OptionFlag> flags)
    fn flags(mut self, flags: Vec<Arc<dyn OptionFlag>>) -> Self {
        for f in flags {
            option_flag::add_flag(&mut self.flags, f);
        }
        self
    }

    /// java: @Override public Builder<T> addListener(@NotNull OptionEventListener<T> listener)
    fn add_listener(mut self, listener: Arc<dyn OptionEventListener<T>>) -> Self {
        self.listeners.push(listener);
        self
    }

    /// java: @Override public Builder<T> addListeners(@NotNull Collection<@NotNull OptionEventListener<T>> optionEventListeners)
    fn add_listeners(mut self, listeners: Vec<Arc<dyn OptionEventListener<T>>>) -> Self {
        self.listeners.extend(listeners);
        self
    }

    /// java: @Override @Deprecated public Builder<T> instant(boolean instant)
    fn instant(mut self, instant: bool) -> Self {
        assert!(self.state_manager.is_none(), "Cannot set instant when state manager is set");
        YACLConstants::logger().error(
            "Option.Builder#instant is deprecated behaviour. Please use a custom state manager instead: `.state(StateManager.createInstant(Binding))`",
        );
        self.instant_deprecated = instant;
        self
    }

    /// java: @Override public Builder<T> listener(@NotNull BiConsumer<Option<T>, T> listener)
    fn listener(self, listener: Arc<dyn Fn(&dyn YOption<T>, T)>) -> Self {
        self.add_listener(yoption::listener_as_event_listener(listener))
    }

    /// java: @Override public Builder<T> listeners(@NotNull Collection<BiConsumer<Option<T>, T>> listeners)
    fn listeners(self, listeners: Vec<Arc<dyn Fn(&dyn YOption<T>, T)>>) -> Self {
        let mapped: Vec<Arc<dyn OptionEventListener<T>>> = listeners.into_iter().map(yoption::listener_as_event_listener).collect();
        self.add_listeners(mapped)
    }

    /// java: @Override public Option<T> build()
    fn build(mut self) -> Arc<dyn YOption<T>> {
        let control_getter = self.control_getter.clone().expect("`control` must not be null when building `Option`");
        if self.instant_deprecated {
            let Some(binding) = self.binding.clone() else {
                panic!("IllegalStateException: Cannot build option with instant when binding is not set");
            };
            assert!(self.flags.is_empty(), "instant application does not support option flags");
            self.state_manager = Some(state_manager::create_instant(binding));
        } else if let Some(binding) = self.binding.clone() {
            self.state_manager = Some(state_manager::create_simple(binding));
        }
        let state_manager = self.state_manager.clone().expect(
            "State manager must be set, either by using .binding() to create a simple manager or .state() to create an advanced one",
        );
        assert!(
            !state_manager.is_always_synced() || self.flags.is_empty(),
            "Always synced state managers do not support option flags."
        );
        OptionImpl::new(self.name, self.description_function, control_getter, state_manager, self.available, self.flags, self.listeners)
    }
}
