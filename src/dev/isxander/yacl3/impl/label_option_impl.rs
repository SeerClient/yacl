//! Ported from dev.isxander.yacl3.impl.LabelOptionImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::label_option::Builder;
use crate::dev::isxander::yacl3::api::label_option::LabelOption;
use crate::dev::isxander::yacl3::api::option_description;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_event_listener::OptionEventListener;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::state_manager;
use crate::dev::isxander::yacl3::api::state_manager::ResetAction;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::gui::controllers::label_controller::LabelController;
use crate::dev::isxander::yacl3::r#impl::option_impl::OptionImpl;
use crate::dev::isxander::yacl3::r#impl::option_impl::OptionImplBase;
use crate::dev::isxander::yacl3::r#impl::provides_binding_for_deprecation::ProvidesBindingForDeprecation;
use crate::dev::isxander::yacl3::r#impl::self_contained_binding::SelfContainedBinding;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class LabelOptionImpl extends OptionImpl<Component> implements LabelOption
#[derive(Clone)]
pub struct LabelOptionImpl {
    /// java: extends OptionImpl
    pub base: OptionImplBase<MutableComponent>,
}

impl std::ops::Deref for LabelOptionImpl {
    type Target = OptionImplBase<MutableComponent>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LabelOptionImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LabelOptionImpl {
    /// java: public LabelOptionImpl( @NotNull StateManager<Component> stateManager, @NotNull Collection<OptionEventListener<Component>> optionEventListeners )
    pub fn new(mut state_manager: compat::Shared<dyn StateManager<MutableComponent>>, mut option_event_listeners: Vec<compat::Shared<dyn OptionEventListener<MutableComponent>>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: OptionImplBase::<MutableComponent>::new(component::literal("Label Option"), Arc::new(move |__a0| option_description::of(&vec![__a0.clone()])), Arc::new(move |__a0| LabelController::new(__a0.clone())), state_manager.clone(), true, std::collections::HashSet::from([]), option_event_listeners.clone()),
        });
        __this
    }

    /// java: public LabelOptionImpl(Component label)
    pub fn new_1(mut label: MutableComponent) -> compat::Shared<Self> {
        let __this = Self::new(state_manager::create_immutable(&label), std::collections::HashSet::from([]));
        __this
    }

}

impl OptionImpl<mcrust::net::minecraft::network::chat::mutable_component::MutableComponent> for LabelOptionImpl {
    fn option_impl_base(&self) -> &OptionImplBase<mcrust::net::minecraft::network::chat::mutable_component::MutableComponent> {
        &self.base
    }
    fn option_impl_base_mut(&mut self) -> &mut OptionImplBase<mcrust::net::minecraft::network::chat::mutable_component::MutableComponent> {
        &mut self.base
    }

}

impl crate::dev::isxander::yacl3::api::option::Option<compat::Object> for LabelOptionImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void setAvailable(boolean available)
    fn set_available(&mut self, mut available: bool) {
        panic!("UnsupportedOperationException: {}", "Cannot change availability of label option");
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

impl LabelOption for LabelOptionImpl {
    /// java: @Override public @NotNull Component label() (dispatch: the body for this class)
    fn label(&self) -> fn(&compat::Shared<dyn LabelOption>) -> MutableComponent {
        LabelOptionImpl::label_dyn
    }

}

impl LabelOptionImpl {
    /// java: @Override public @NotNull Component label()
    pub fn label_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return __this.lock().borrow().state_manager().lock().borrow().get();
    }

    /// LabelOptionImpl.label as the dispatch of LabelOption.label
    pub fn label_dyn(__this: &compat::Shared<dyn LabelOption>) -> MutableComponent {
        Self::label_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: @ApiStatus.Internal public static final class BuilderImpl implements LabelOption.Builder
#[derive(Clone)]
pub struct BuilderImpl {
    /// java: private StateManager<Component> stateManager;
    pub state_manager: Option<compat::Shared<dyn StateManager<MutableComponent>>>,
    /// java: private final List<Component> lines = new ArrayList<>();
    pub lines: Vec<MutableComponent>,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            state_manager: None,
            lines: Vec::new(),
        });
        __this
    }

}

impl Builder for BuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public LabelOption.Builder state(@NotNull StateManager<Component> stateManager) (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn Builder>, compat::Shared<dyn StateManager<MutableComponent>>) -> compat::Shared<dyn Builder> {
        BuilderImpl::state_dyn
    }

    /// java: @Override public LabelOption.Builder line(@NotNull Component line) (dispatch: the body for this class)
    fn line(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        BuilderImpl::line_dyn
    }

    /// java: @Override public LabelOption.Builder lines(@NotNull Collection<? extends Component> lines) (dispatch: the body for this class)
    fn lines(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        BuilderImpl::lines_dyn
    }

    /// java: @Override public LabelOption build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn LabelOption> {
        BuilderImpl::build_dyn
    }

}

impl BuilderImpl {
    /// java: @Override public LabelOption.Builder state(@NotNull StateManager<Component> stateManager)
    pub fn state_impl(__this: &compat::Shared<Self>, mut state_manager: compat::Shared<dyn StateManager<MutableComponent>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(stateManager, "`stateManager` must not be null");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(this.lines.isEmpty(), "Cannot set state manager if lines have already been defined");
        { let __v = Some(state_manager.clone()); compat::write(__this, |__t| __t.state_manager = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.state as the dispatch of Builder.state
    pub fn state_dyn(__this: &compat::Shared<dyn Builder>, __p0: compat::Shared<dyn StateManager<MutableComponent>>) -> compat::Shared<dyn Builder> {
        Self::state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public LabelOption.Builder line(@NotNull Component line)
    pub fn line_impl(__this: &compat::Shared<Self>, mut line: MutableComponent) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(stateManager == null, ".line() is a helper to create a state manager for you at build. If you have defined a custom state manager, do not use .line()");
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(line, "`line` must not be null");
        let __h0 = line.clone();
        __this.lock().borrow_mut().lines.push(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.line as the dispatch of Builder.line
    pub fn line_dyn(__this: &compat::Shared<dyn Builder>, __p0: MutableComponent) -> compat::Shared<dyn Builder> {
        Self::line_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public LabelOption.Builder lines(@NotNull Collection<? extends Component> lines)
    pub fn lines_impl(__this: &compat::Shared<Self>, lines: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(stateManager == null, ".lines() is a helper to create a state manager for you at build. If you have defined a custom state manager, do not use .lines()");
        let __h0 = lines.iter().cloned();
        __this.lock().borrow_mut().lines.extend(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.lines as the dispatch of Builder.lines
    pub fn lines_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        Self::lines_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public LabelOption build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn LabelOption> {
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(stateManager != null || !lines.isEmpty(), "Cannot build label option without a state manager or lines");
        if !(__this.lock().borrow().lines.is_empty()) {
            let mut text: MutableComponent = component::empty();
            let mut iterator = compat::read(__this, |__t| __t.lines.clone()).into_iter();
            // TODO(fabric2seer): the condition: Iterator.hasNext - use a `for` loop / `while let Some(x) = it.next()`
            //   java: iterator.hasNext(..)
            while todo!("fabric2seer: the condition (TODO above)") {
                text = text.append(iterator.next().unwrap());
                // TODO(fabric2seer): the condition: Iterator.hasNext - use a `for` loop / `while let Some(x) = it.next()`
                //   java: iterator.hasNext(..)
                if todo!("fabric2seer: the condition (TODO above)") {
                    text = text.append_str("\n");
                }
            }
            { let __v = Some(state_manager::create_simple(&(compat::shared(SelfContainedBinding::new_1(text.clone())) as compat::Shared<dyn Binding<compat::Object>>))); compat::write(__this, |__t| __t.state_manager = __v); }
        }
        return (LabelOptionImpl::new(compat::read(__this, |__t| __t.state_manager.clone()).unwrap(), std::collections::HashSet::from([])) as compat::Shared<dyn LabelOption>);
    }

    /// BuilderImpl.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn LabelOption> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
