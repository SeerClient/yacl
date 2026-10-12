//! Ported from dev.isxander.yacl3.impl.ButtonOptionImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::button_option::Builder;
use crate::dev::isxander::yacl3::api::button_option::ButtonOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::option_description;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_event_listener::OptionEventListener;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::state_manager;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::gui::controllers::action_controller::ActionController;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: @ApiStatus.Internal public final class ButtonOptionImpl implements ButtonOption
#[derive(Clone)]
pub struct ButtonOptionImpl {
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final OptionDescription description;
    pub description: compat::Shared<dyn OptionDescription>,
    /// java: private final StateManager<BiConsumer<YACLScreen, ButtonOption>> stateManager;
    pub state_manager: compat::Shared<dyn StateManager<Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>>>,
    /// java: private boolean available;
    pub available: bool,
    /// java: private final Controller<BiConsumer<YACLScreen, ButtonOption>> controller;
    pub controller: Option<compat::Shared<dyn Controller<Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>>>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl ButtonOptionImpl {
    /// java: public ButtonOptionImpl( @NotNull Component name, @Nullable OptionDescription description, @NotNull BiConsumer<YACLScreen, ButtonOption> action, @Nullable Component text, boolean available )
    pub fn new(mut name: MutableComponent, mut description: Option<compat::Shared<dyn OptionDescription>>, action: Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>, mut text: Option<MutableComponent>, mut available: bool) -> compat::Shared<Self> {
        let __init_name = name.clone();
        let __init_description = description.clone().unwrap();
        let __init_state_manager = state_manager::create_immutable(&action);
        let __init_available = available;
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            name: __init_name,
            description: __init_description,
            state_manager: __init_state_manager,
            available: __init_available,
            controller: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(((if text.is_some() { (ActionController::new_2((__this.clone() as compat::Shared<dyn ButtonOption>), text.clone().unwrap()) as compat::Shared<dyn Controller<compat::Object>>) } else { (ActionController::new((__this.clone() as compat::Shared<dyn ButtonOption>)) as compat::Shared<dyn Controller<compat::Object>>) }) as compat::Shared<dyn Controller<compat::Object>>)); compat::write(&__this, |__t| __t.controller = __v); }
        __this
    }

}

impl ButtonOption for ButtonOptionImpl {
    /// java: @Override public BiConsumer<YACLScreen, ButtonOption> action() (dispatch: the body for this class)
    fn action(&self) -> fn(&compat::Shared<dyn ButtonOption>) -> Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)> {
        ButtonOptionImpl::action_dyn
    }

}

impl crate::dev::isxander::yacl3::api::option::Option<compat::Object> for ButtonOptionImpl {
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

    /// java: @Override public boolean available()
    fn available(&self) -> bool {
        return self.available;
    }

    /// java: @Override public void setAvailable(boolean available)
    fn set_available(&mut self, mut available: bool) {
        self.available = available;
    }

    /// java: @Override public @NotNull Controller<BiConsumer<YACLScreen, ButtonOption>> controller()
    fn controller(&self) -> compat::Shared<dyn Controller<Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>>> {
        return self.controller.clone().unwrap();
    }

    /// java: @Override public @NotNull StateManager<BiConsumer<YACLScreen, ButtonOption>> stateManager()
    fn state_manager(&self) -> compat::Shared<dyn StateManager<Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>>> {
        return self.state_manager.clone();
    }

    /// java: @Override public @NotNull Binding<BiConsumer<YACLScreen, ButtonOption>> binding()
    fn binding(&self) -> compat::Shared<dyn Binding<Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>>> {
        return (compat::shared(EmptyBinderImpl::new()) as compat::Shared<dyn Binding<compat::Object>>);
    }

    /// java: @Override public @NotNull ImmutableSet<OptionFlag> flags()
    fn flags(&self) -> std::collections::HashSet<compat::Shared<dyn OptionFlag>> {
        return std::collections::HashSet::from([]);
    }

    /// java: @Override public boolean changed()
    fn changed(&self) -> bool {
        return false;
    }

    /// java: @Override public @NotNull BiConsumer<YACLScreen, ButtonOption> pendingValue()
    fn pending_value(&self) -> Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public void requestSet(@NotNull BiConsumer<YACLScreen, ButtonOption> value)
    fn request_set(&mut self, mut value: Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>) {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public boolean applyValue()
    fn apply_value(&self) -> bool {
        return false;
    }

    /// java: @Override public void forgetPendingValue()
    fn forget_pending_value(&mut self) {
    }

    /// java: @Override public void requestSetDefault()
    fn request_set_default(&mut self) {
    }

    /// java: @Override public boolean isPendingValueDefault()
    fn is_pending_value_default(&self) -> bool {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public void addEventListener(OptionEventListener<BiConsumer<YACLScreen, ButtonOption>> listener)
    fn add_event_listener(&mut self, mut listener: compat::Shared<dyn OptionEventListener<Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>>>) {
    }

    /// java: @Override public void addListener(BiConsumer<Option<BiConsumer<YACLScreen, ButtonOption>>, BiConsumer<YACLScreen, ButtonOption>> changedListener)
    fn add_listener(&mut self, changed_listener: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>>>, Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>)>) {
    }

}

impl ButtonOptionImpl {
    /// java: @Override public BiConsumer<YACLScreen, ButtonOption> action()
    pub fn action_impl(__this: &compat::Shared<Self>) -> Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)> {
        return __this.lock().borrow().state_manager().lock().borrow().get();
    }

    /// ButtonOptionImpl.action as the dispatch of ButtonOption.action
    pub fn action_dyn(__this: &compat::Shared<dyn ButtonOption>) -> Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)> {
        Self::action_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: private static class EmptyBinderImpl implements Binding<BiConsumer<YACLScreen, ButtonOption>>
#[derive(Clone)]
pub struct EmptyBinderImpl {}

impl EmptyBinderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

impl Binding<Arc<dyn Fn(compat::Shared<crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen>, compat::Shared<dyn crate::dev::isxander::yacl3::api::button_option::ButtonOption>)>> for EmptyBinderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void setValue(BiConsumer<YACLScreen, ButtonOption> value)
    fn set_value(&mut self, mut value: Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>) {
    }

    /// java: @Override public BiConsumer<YACLScreen, ButtonOption> getValue()
    fn get_value(&self) -> Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public BiConsumer<YACLScreen, ButtonOption> defaultValue()
    fn default_value(&self) -> Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)> {
        panic!("UnsupportedOperationException");
    }

}

/// java: @ApiStatus.Internal public static final class BuilderImpl implements Builder
#[derive(Clone)]
pub struct BuilderImpl {
    /// java: private Component name;
    pub name: Option<MutableComponent>,
    /// java: private Component text = null;
    pub text: Option<MutableComponent>,
    /// java: private OptionDescription description = OptionDescription.EMPTY;
    pub description: compat::Shared<dyn OptionDescription>,
    /// java: private boolean available = true;
    pub available: bool,
    /// java: private BiConsumer<YACLScreen, ButtonOption> action;
    pub action: Option<Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>>,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            name: None,
            text: None,
            description: option_description::EMPTY.get_mut().clone(),
            available: true,
            action: None,
        });
        __this
    }

}

impl Builder for BuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder name(@NotNull Component name) (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        BuilderImpl::name_dyn
    }

    /// java: @Override public Builder text(@NotNull Component text) (dispatch: the body for this class)
    fn text(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        BuilderImpl::text_dyn
    }

    /// java: @Override public Builder description(@NotNull OptionDescription description) (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn Builder>, compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn Builder> {
        BuilderImpl::description_dyn
    }

    /// java: @Override public Builder action(@NotNull BiConsumer<YACLScreen, ButtonOption> action) (dispatch: the body for this class)
    fn action(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>) -> compat::Shared<dyn Builder> {
        BuilderImpl::action_dyn
    }

    /// java: @Override @Deprecated public Builder action(@NotNull Consumer<YACLScreen> action) (dispatch: the body for this class)
    fn action_consumer(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn(compat::Shared<YACLScreen>)>) -> compat::Shared<dyn Builder> {
        BuilderImpl::action_consumer_dyn
    }

    /// java: @Override public Builder available(boolean available) (dispatch: the body for this class)
    fn available(&self) -> fn(&compat::Shared<dyn Builder>, bool) -> compat::Shared<dyn Builder> {
        BuilderImpl::available_dyn
    }

    /// java: @Override public ButtonOption build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn ButtonOption> {
        BuilderImpl::build_dyn
    }

}

impl BuilderImpl {
    /// java: @Override public Builder name(@NotNull Component name)
    pub fn name_impl(__this: &compat::Shared<Self>, mut name: MutableComponent) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(name, "`name` cannot be null");
        { let __v = Some(name.clone()); compat::write(__this, |__t| __t.name = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.name as the dispatch of Builder.name
    pub fn name_dyn(__this: &compat::Shared<dyn Builder>, __p0: MutableComponent) -> compat::Shared<dyn Builder> {
        Self::name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder text(@NotNull Component text)
    pub fn text_impl(__this: &compat::Shared<Self>, mut text: MutableComponent) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(text, "`text` cannot be null");
        { let __v = Some(text.clone()); compat::write(__this, |__t| __t.text = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.text as the dispatch of Builder.text
    pub fn text_dyn(__this: &compat::Shared<dyn Builder>, __p0: MutableComponent) -> compat::Shared<dyn Builder> {
        Self::text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder description(@NotNull OptionDescription description)
    pub fn description_impl(__this: &compat::Shared<Self>, mut description: compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(description, "`description` cannot be null");
        { let __v = description.clone(); compat::write(__this, |__t| __t.description = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.description as the dispatch of Builder.description
    pub fn description_dyn(__this: &compat::Shared<dyn Builder>, __p0: compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn Builder> {
        Self::description_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder action(@NotNull BiConsumer<YACLScreen, ButtonOption> action)
    pub fn action_impl(__this: &compat::Shared<Self>, action: Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(action, "`action` cannot be null");
        { let __v = Some(action.clone()); compat::write(__this, |__t| __t.action = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.action as the dispatch of Builder.action
    pub fn action_dyn(__this: &compat::Shared<dyn Builder>, __p0: Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>) -> compat::Shared<dyn Builder> {
        Self::action_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override @Deprecated public Builder action(@NotNull Consumer<YACLScreen> action)
    pub fn action_consumer_impl(__this: &compat::Shared<Self>, action: Arc<dyn Fn(compat::Shared<YACLScreen>)>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(action, "`action` cannot be null");
        { let __v = Some(Arc::new(move |screen, button| { (action)(screen.clone()); })); compat::write(__this, |__t| __t.action = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.action as the dispatch of Builder.action
    pub fn action_consumer_dyn(__this: &compat::Shared<dyn Builder>, __p0: Arc<dyn Fn(compat::Shared<YACLScreen>)>) -> compat::Shared<dyn Builder> {
        Self::action_consumer_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder available(boolean available)
    pub fn available_impl(__this: &compat::Shared<Self>, mut available: bool) -> compat::Shared<dyn Builder> {
        { let __v = available; compat::write(__this, |__t| __t.available = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.available as the dispatch of Builder.available
    pub fn available_dyn(__this: &compat::Shared<dyn Builder>, __p0: bool) -> compat::Shared<dyn Builder> {
        Self::available_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public ButtonOption build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn ButtonOption> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(name, "`name` must not be null when building `ButtonOption`");
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(action, "`action` must not be null when building `ButtonOption`");
        return (ButtonOptionImpl::new(compat::read(__this, |__t| __t.name.clone()).unwrap(), Some(compat::read(__this, |__t| __t.description.clone())), compat::read(__this, |__t| __t.action.clone()).clone().unwrap(), compat::read(__this, |__t| __t.text.clone()), compat::read(__this, |__t| __t.available)) as compat::Shared<dyn ButtonOption>);
    }

    /// BuilderImpl.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn ButtonOption> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
