//! Ported from dev.isxander.yacl3.impl.OptionImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::option::Builder;
use crate::dev::isxander::yacl3::api::option_description;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_event_listener::Event;
use crate::dev::isxander::yacl3::api::option_event_listener::OptionEventListener;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::state_manager;
use crate::dev::isxander::yacl3::api::state_manager::ResetAction;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::r#impl::provides_binding_for_deprecation::ProvidesBindingForDeprecation;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: @ApiStatus.Internal public class OptionImpl<T> implements Option<T>
#[derive(Clone)]
pub struct OptionImplBase<T> {
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private OptionDescription description;
    pub description: Option<compat::Shared<dyn OptionDescription>>,
    /// java: private final Controller<T> controller;
    pub controller: Option<compat::Shared<dyn Controller<T>>>,
    /// java: private boolean available;
    pub available: bool,
    /// java: private final ImmutableSet<OptionFlag> flags;
    pub flags: std::collections::HashSet<compat::Shared<dyn OptionFlag>>,
    /// java: private final StateManager<T> stateManager;
    pub state_manager: compat::Shared<dyn StateManager<T>>,
    /// java: private final List<OptionEventListener<T>> listeners;
    pub listeners: Vec<compat::Shared<dyn OptionEventListener<T>>>,
    /// java: private int currentListenerDepth;
    pub current_listener_depth: i32,
}

impl<T: Clone + 'static> OptionImplBase<T> {
    /// java: public OptionImpl( @NotNull Component name, @NotNull Function<T, OptionDescription> descriptionFunction, @NotNull Function<Option<T>, Controller<T>> controlGetter, @NotNull StateManager<T> stateManage
    pub fn new(mut name: MutableComponent, description_function: Arc<dyn Fn(T) -> compat::Shared<dyn OptionDescription>>, control_getter: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> compat::Shared<dyn Controller<T>>>, mut state_manager: compat::Shared<dyn StateManager<T>>, mut available: bool, mut flags: std::collections::HashSet<compat::Shared<dyn OptionFlag>>, mut listeners: Vec<compat::Shared<dyn OptionEventListener<T>>>) -> Self {
        let __init_name = name.clone();
        let __init_available = available;
        let __init_flags = flags.clone();
        let __init_listeners = listeners.iter().cloned().collect::<Vec<_>>();
        let __init_state_manager = state_manager.clone();
        let mut this = Self {
            name: __init_name,
            description: None,
            controller: None,
            available: __init_available,
            flags: __init_flags,
            state_manager: __init_state_manager,
            listeners: __init_listeners,
            current_listener_depth: 0,
        };
        this.controller = Some((control_getter)(this.clone()));
        { let (__a0,) = (&Arc::new(move |old_value, new_value| this.trigger_listener(Event::StateChange, false)),); this.state_manager.lock().borrow_mut().add_listener(__a0) };
        // TODO(fabric2seer): `.pendingValue(..)` on a value of unknown type
        //   java: addEventListener((opt, event) -> description = descriptionFunction.apply(opt.pendingValue()));
        this.trigger_listener(Event::Initial, false);
        this
    }

}

/// java: the virtual methods of OptionImpl (implemented by OptionImplBase and every subclass)
pub trait OptionImpl<T: Clone + 'static>: crate::dev::isxander::yacl3::api::option::Option {
    fn option_impl_base(&self) -> &OptionImplBase<T>;
    fn option_impl_base_mut(&mut self) -> &mut OptionImplBase<T>;

    /// java: private void triggerListener(OptionEventListener.Event event, boolean allowDepth)
    fn trigger_listener(&mut self, mut event: Event, mut allow_depth: bool) {
        if allow_depth || self.option_impl_base().current_listener_depth == 0 {
            // TODO(fabric2seer): static call Validate.isTrue(..)
            //   java: Validate.isTrue(
            //   java:                     currentListenerDepth <= 10,
            //   java:                     "Listener depth exceeded 10! Possible cyclic listener pattern: a listener triggered an event that triggered the initial event etc etc."
            //   java:             );
            self.option_impl_base_mut().current_listener_depth += 1;
            for listener in self.option_impl_base().listeners.iter() {
                // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
                //   java: listener.onEvent(this, event);
            }
            self.option_impl_base_mut().current_listener_depth -= 1;
        }
    }

}

impl<T: Clone + 'static> OptionImpl<T> for OptionImplBase<T> {
    fn option_impl_base(&self) -> &OptionImplBase<T> {
        self
    }
    fn option_impl_base_mut(&mut self) -> &mut OptionImplBase<T> {
        self
    }

}

impl<T: Clone + 'static> crate::dev::isxander::yacl3::api::option::Option<T> for OptionImplBase<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @NotNull Component name()
    fn name(&self) -> MutableComponent {
        return self.name.clone();
    }

    /// java: @Override public @NotNull OptionDescription description()
    fn description(&self) -> compat::Shared<dyn OptionDescription> {
        return self.description.clone().unwrap();
    }

    /// java: @Override public @NotNull Component tooltip()
    fn tooltip(&self) -> MutableComponent {
        return ({ let __r = self.description.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.text()); __f(__r) });
    }

    /// java: @Override public @NotNull Controller<T> controller()
    fn controller(&self) -> compat::Shared<dyn Controller<T>> {
        return self.controller.clone().unwrap();
    }

    /// java: @Override public @NotNull StateManager<T> stateManager()
    fn state_manager(&self) -> compat::Shared<dyn StateManager<T>> {
        return self.state_manager.clone();
    }

    /// java: @Override @Deprecated public @NotNull Binding<T> binding()
    fn binding(&self) -> compat::Shared<dyn Binding<T>> {
        if self.state_manager.lock().borrow().as_any().is::<compat::Shared<dyn ProvidesBindingForDeprecation<compat::Object>>>() {
            return (self.state_manager.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn ProvidesBindingForDeprecation<T>>>().unwrap().lock().borrow()).get_binding();
        }
        panic!("UnsupportedOperationException: {}", "Binding is not available for this option - using a new state manager which does not directly expose the binding as it may not have one.");
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
            self.trigger_listener(Event::AvailabilityChange, !available);
        }
    }

    /// java: @Override public @NotNull ImmutableSet<OptionFlag> flags()
    fn flags(&self) -> std::collections::HashSet<compat::Shared<dyn OptionFlag>> {
        return self.flags.clone();
    }

    /// java: @Override public boolean changed()
    fn changed(&self) -> bool {
        return !(self.state_manager.lock().borrow().is_synced());
    }

    /// java: @Override public @NotNull T pendingValue()
    fn pending_value(&self) -> T {
        return self.state_manager.lock().borrow().get();
    }

    /// java: @Override public void requestSet(@NotNull T value)
    fn request_set(&mut self, mut value: T) {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(value, "`value` cannot be null");
        { let (__a0,) = (value.clone(),); self.state_manager.lock().borrow_mut().set(__a0) };
    }

    /// java: @Override public boolean applyValue()
    fn apply_value(&self) -> bool {
        if self.changed() {
            self.state_manager.lock().borrow_mut().apply();
            return true;
        }
        return false;
    }

    /// java: @Override public void forgetPendingValue()
    fn forget_pending_value(&mut self) {
        self.state_manager.lock().borrow_mut().sync();
    }

    /// java: @Override public void requestSetDefault()
    fn request_set_default(&mut self) {
        self.state_manager.lock().borrow_mut().reset_to_default(ResetAction::ByOption);
    }

    /// java: @Override public boolean isPendingValueDefault()
    fn is_pending_value_default(&self) -> bool {
        return self.state_manager.lock().borrow().is_default();
    }

    /// java: @Override public void addEventListener(OptionEventListener<T> listener)
    fn add_event_listener(&mut self, mut listener: compat::Shared<dyn OptionEventListener<T>>) {
        self.listeners.push(listener.clone());
    }

    /// java: @Override @Deprecated public void addListener(BiConsumer<Option<T>, T> changedListener)
    fn add_listener(&mut self, changed_listener: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, T)>) {
        // TODO(fabric2seer): `.pendingValue(..)` on a value of unknown type
        //   java: addEventListener((opt, event) -> changedListener.accept(opt, opt.pendingValue()));
    }

}

/// java: @ApiStatus.Internal public static class BuilderImpl<T> implements Builder<T>
#[derive(Clone)]
pub struct BuilderImpl<T> {
    /// java: private Component name = Component.literal("Name not specified!").withStyle(ChatFormatting.RED);
    pub name: MutableComponent,
    /// java: private Function<T, OptionDescription> descriptionFunction = pending -> OptionDescription.EMPTY;
    pub description_function: Arc<dyn Fn(T) -> compat::Shared<dyn OptionDescription>>,
    /// java: private Function<Option<T>, Controller<T>> controlGetter;
    pub control_getter: Option<Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> compat::Shared<dyn Controller<T>>>>,
    /// java: private boolean available = true;
    pub available: bool,
    /// java: private final Set<OptionFlag> flags = new HashSet<>();
    pub flags: std::collections::HashSet<compat::Shared<dyn OptionFlag>>,
    /// java: private final List<OptionEventListener<T>> listeners = new ArrayList<>();
    pub listeners: Vec<compat::Shared<dyn OptionEventListener<T>>>,
    /// java: private @Nullable Binding<T> binding;
    pub binding: Option<compat::Shared<dyn Binding<T>>>,
    /// java: private boolean instantDeprecated = false;
    pub instant_deprecated: bool,
    /// java: private @Nullable StateManager<T> stateManager;
    pub state_manager: Option<compat::Shared<dyn StateManager<T>>>,
}

impl<T: Clone + 'static> BuilderImpl<T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            name: (component::literal("Name not specified!")).with_style_format(ChatFormatting::Red),
            description_function: Arc::new(move |pending| option_description::EMPTY.get_mut().clone()),
            control_getter: None,
            available: true,
            flags: std::collections::HashSet::new(),
            listeners: Vec::new(),
            binding: None,
            instant_deprecated: false,
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
        //   java: Validate.notNull(name, "`name` cannot be null");
        self.name = name.clone();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> description(@NotNull OptionDescription description)
    fn description(&mut self, description: &compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn Builder<T>> {
        return self.description(&Arc::new(move |opt| description.clone()));
    }

    /// java: @Override public Builder<T> description(@NotNull Function<T, OptionDescription> descriptionFunction)
    fn description_function(&mut self, description_function: Arc<dyn Fn(T) -> compat::Shared<dyn OptionDescription>>) -> compat::Shared<dyn Builder<T>> {
        self.description_function = description_function.clone();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> controller(@NotNull Function<Option<T>, ControllerBuilder<T>> controllerBuilder)
    fn controller(&mut self, controller_builder: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> compat::Shared<dyn ControllerBuilder<T>>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(controllerBuilder, "`controllerBuilder` cannot be null");
        return self.custom_controller(Arc::new(move |opt| (controller_builder)(opt.clone()).lock().borrow_mut().build()));
    }

    /// java: @Override public Builder<T> customController(@NotNull Function<Option<T>, Controller<T>> control)
    fn custom_controller(&mut self, control: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> compat::Shared<dyn Controller<T>>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(control, "`control` cannot be null");
        self.control_getter = Some(control.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> stateManager(@NotNull StateManager<T> stateManager)
    fn state_manager(&mut self, mut state_manager: compat::Shared<dyn StateManager<T>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(stateManager, "`stateManager` cannot be null");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(binding == null, "Cannot set state manager when binding is set");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(!instantDeprecated, "Cannot set state manager when instant is set");
        self.state_manager = Some(state_manager.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> binding(@NotNull Binding<T> binding)
    fn binding(&mut self, mut binding: compat::Shared<dyn Binding<T>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(binding, "`binding` cannot be null");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(stateManager == null, "Cannot set binding when state manager is set");
        self.binding = Some(binding.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> binding(@NotNull T def, @NotNull Supplier<@NotNull T> getter, @NotNull Consumer<@NotNull T> setter)
    fn binding_3(&mut self, def: &T, getter: Arc<dyn Fn() -> T>, setter: Arc<dyn Fn(T)>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(def, "`def` must not be null");
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(getter, "`getter` must not be null");
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(setter, "`setter` must not be null");
        return self.binding(binding::generic(def, getter.clone(), setter.clone()));
    }

    /// java: @Override public Builder<T> available(boolean available)
    fn available(&mut self, mut available: bool) -> compat::Shared<dyn Builder<T>> {
        self.available = available;
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

    /// java: @Override public Builder<T> flags(@NotNull Collection<? extends OptionFlag> flags)
    fn flags(&mut self, flags: &Vec<compat::Shared<dyn OptionFlag>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(flags, "`flags` must not be null");
        self.flags.extend(flags.iter().cloned());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override @Deprecated public Builder<T> instant(boolean instant)
    fn instant(&mut self, mut instant: bool) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(stateManager == null, "Cannot set instant when state manager is set");
        logger().error("Option.Builder#instant is deprecated behaviour. Please use a custom state manager instead: `.state(StateManager.createInstant(Binding))`");
        self.instant_deprecated = instant;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> addListener(@NotNull OptionEventListener<T> listener)
    fn add_listener(&mut self, mut listener: compat::Shared<dyn OptionEventListener<T>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(listener, "`listener` must not be null");
        self.listeners.push(listener.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> addListeners(@NotNull Collection<@NotNull OptionEventListener<T>> optionEventListeners)
    fn add_listeners(&mut self, option_event_listeners: &Vec<compat::Shared<dyn OptionEventListener<T>>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(optionEventListeners, "`optionEventListeners` must not be null");
        self.listeners.extend(option_event_listeners.iter().cloned());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> listener(@NotNull BiConsumer<Option<T>, T> listener)
    fn listener(&mut self, listener: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, T)>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(listener, "`listener` must not be null");
        // TODO(fabric2seer): `.pendingValue(..)` on a value of unknown type
        //   java: return this.addListener((opt, event) -> listener.accept(opt, opt.pendingValue()));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> listeners(@NotNull Collection<BiConsumer<Option<T>, T>> listeners)
    fn listeners(&mut self, listeners: &Vec<Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, T)>>) -> compat::Shared<dyn Builder<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(listeners, "`listeners` must not be null");
        // TODO(fabric2seer): `.pendingValue(..)` on a value of unknown type
        //   java: this.addListeners(listeners.stream()
        //   java:                     .map(listener ->
        //   java:                             (OptionEventListener<T>) (opt, event) ->
        //   java:                                     listener.accept(opt, opt.pendingValue())
        //   java:                     ).toList()
        //   java:             );
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Option<T> build()
    fn build(&mut self) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(controlGetter, "`control` must not be null when building `Option`");
        if self.instant_deprecated {
            if self.binding.is_none() {
                panic!("IllegalStateException: {}", "Cannot build option with instant when binding is not set");
            }
            // TODO(fabric2seer): static call Validate.isTrue(..)
            //   java: Validate.isTrue(flags.isEmpty(), "instant application does not support option flags");
            self.state_manager = Some(state_manager::create_instant(self.binding.as_ref().unwrap()));
        } else if self.binding.is_some() {
            self.state_manager = Some(state_manager::create_simple(self.binding.as_ref().unwrap()));
        }
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(stateManager, "State manager must be set, either by using .binding() to create a simple manager or .state() to create an advanced one");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(!stateManager.isAlwaysSynced() || flags.isEmpty(), "Always synced state managers do not support option flags.");
        return ((compat::shared(OptionImplBase::new(self.name.clone(), self.description_function.clone(), self.control_getter.clone().unwrap(), self.state_manager.clone().unwrap(), self.available, self.flags.iter().cloned().collect::<std::collections::HashSet<_>>(), self.listeners.clone())) as compat::Shared<dyn OptionImpl<compat::Object>>) as compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>);
    }

}
