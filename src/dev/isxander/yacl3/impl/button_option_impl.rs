//! Ported from dev.isxander.yacl3.impl.ButtonOptionImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::button_option::{ButtonAction, ButtonOption, Builder};
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::option::{AnyOption, Option as YOption};
use crate::dev::isxander::yacl3::api::option_description::{self, OptionDescription};
use crate::dev::isxander::yacl3::api::option_event_listener::{Event, OptionEventListener};
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::state_manager::{self, StateManager};
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::action_controller::ActionController;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::cell::{Cell, RefCell};

/// java: @ApiStatus.Internal public final class ButtonOptionImpl implements ButtonOption
pub struct ButtonOptionImpl {
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final OptionDescription description;
    pub description: Arc<dyn OptionDescription>,
    /// java: private final StateManager<BiConsumer<YACLScreen, ButtonOption>> stateManager;
    pub state_manager: Arc<dyn StateManager<ButtonAction>>,
    /// java: private boolean available;
    pub available: Cell<bool>,
    /// java: private final Controller<BiConsumer<YACLScreen, ButtonOption>> controller;
    pub controller: RefCell<Option<Arc<dyn Controller<ButtonAction>>>>,
}

impl ButtonOptionImpl {
    /// java: public ButtonOptionImpl(@NotNull Component name, @Nullable OptionDescription description,
    /// @NotNull BiConsumer<YACLScreen, ButtonOption> action, @Nullable Component text, boolean available)
    pub fn new(
        name: MutableComponent,
        description: Arc<dyn OptionDescription>,
        action: ButtonAction,
        text: Option<MutableComponent>,
        available: bool,
    ) -> Arc<ButtonOptionImpl> {
        let option = Arc::new(ButtonOptionImpl {
            name,
            description,
            state_manager: state_manager::create_immutable(action),
            available: Cell::new(available),
            controller: RefCell::new(None),
        });
        // java: this.controller = text != null ? new ActionController(this, text) : new ActionController(this);
        let this: Arc<dyn ButtonOption> = option.clone();
        let controller: Arc<dyn Controller<ButtonAction>> = match text {
            Some(text) => Arc::new(ActionController::new_1(this, text)),
            None => Arc::new(ActionController::new(this)),
        };
        *option.controller.borrow_mut() = Some(controller);
        option
    }
}

impl AnyOption for ButtonOptionImpl {
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

    /// java: @Override public boolean available()
    fn available(&self) -> bool {
        self.available.get()
    }

    /// java: @Override public void setAvailable(boolean available)
    fn set_available(&self, available: bool) {
        self.available.set(available);
    }

    /// java: @Override public @NotNull ImmutableSet<OptionFlag> flags()
    fn flags(&self) -> Vec<Arc<dyn OptionFlag>> {
        Vec::new()
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
        panic!("UnsupportedOperationException");
    }

    fn provide_controller_widget(&self, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        self.controller().provide_widget(screen, widget_dimension)
    }

    /// java: @Override public void addEventListener(..) - empty
    fn add_any_event_listener(&self, _listener: Arc<dyn Fn(&dyn AnyOption, Event)>) {}

    /// java: @Override public void addListener(..) - empty
    fn add_any_listener(&self, _changed_listener: Arc<dyn Fn(&dyn AnyOption)>) {}
}

impl YOption<ButtonAction> for ButtonOptionImpl {
    /// java: @Override public @NotNull Controller<BiConsumer<YACLScreen, ButtonOption>> controller()
    fn controller(&self) -> Arc<dyn Controller<ButtonAction>> {
        self.controller.borrow().clone().expect("ButtonOptionImpl.controller() before the constructor set it")
    }

    /// java: @Override public @NotNull StateManager<BiConsumer<YACLScreen, ButtonOption>> stateManager()
    fn state_manager(&self) -> Arc<dyn StateManager<ButtonAction>> {
        self.state_manager.clone()
    }

    /// java: @Override public @NotNull Binding<BiConsumer<YACLScreen, ButtonOption>> binding()
    fn binding(&self) -> Arc<dyn Binding<ButtonAction>> {
        Arc::new(EmptyBinderImpl)
    }

    /// java: @Override public @NotNull BiConsumer<YACLScreen, ButtonOption> pendingValue()
    fn pending_value(&self) -> ButtonAction {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public void requestSet(@NotNull BiConsumer<YACLScreen, ButtonOption> value)
    fn request_set(&self, _value: ButtonAction) {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public void addEventListener(OptionEventListener<..> listener) - empty
    fn add_event_listener(&self, _listener: Arc<dyn OptionEventListener<ButtonAction>>) {}

    /// java: @Override public void addListener(BiConsumer<..> changedListener) - empty
    fn add_listener(&self, _changed_listener: Arc<dyn Fn(&dyn YOption<ButtonAction>, ButtonAction)>) {}
}

impl ButtonOption for ButtonOptionImpl {
    /// java: @Override public BiConsumer<YACLScreen, ButtonOption> action()
    fn action(&self) -> ButtonAction {
        self.state_manager().get()
    }
}

/// java: private static class EmptyBinderImpl implements Binding<BiConsumer<YACLScreen, ButtonOption>>
pub struct EmptyBinderImpl;

impl Binding<ButtonAction> for EmptyBinderImpl {
    fn set_value(&self, _value: ButtonAction) {}

    fn get_value(&self) -> ButtonAction {
        panic!("UnsupportedOperationException");
    }

    fn default_value(&self) -> ButtonAction {
        panic!("UnsupportedOperationException");
    }
}

/// java: @ApiStatus.Internal public static final class BuilderImpl implements Builder
pub struct BuilderImpl {
    /// java: private Component name;
    pub name: Option<MutableComponent>,
    /// java: private Component text = null;
    pub text: Option<MutableComponent>,
    /// java: private OptionDescription description = OptionDescription.EMPTY;
    pub description: Arc<dyn OptionDescription>,
    /// java: private boolean available = true;
    pub available: bool,
    /// java: private BiConsumer<YACLScreen, ButtonOption> action;
    pub action: Option<ButtonAction>,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { name: None, text: None, description: option_description::EMPTY(), available: true, action: None }
    }
}

impl Default for BuilderImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder for BuilderImpl {
    /// java: @Override public Builder name(@NotNull Component name)
    fn name(mut self, name: MutableComponent) -> Self {
        self.name = Some(name);
        self
    }

    /// java: @Override public Builder text(@NotNull Component text)
    fn text(mut self, text: MutableComponent) -> Self {
        self.text = Some(text);
        self
    }

    /// java: @Override public Builder description(@NotNull OptionDescription description)
    fn description(mut self, description: Arc<dyn OptionDescription>) -> Self {
        self.description = description;
        self
    }

    /// java: @Override public Builder action(@NotNull BiConsumer<YACLScreen, ButtonOption> action)
    fn action(mut self, action: impl Fn(&Arc<YACLScreen>, &dyn ButtonOption) + 'static) -> Self {
        self.action = Some(Arc::new(action));
        self
    }

    /// java: @Override @Deprecated public Builder action(@NotNull Consumer<YACLScreen> action)
    fn action_consumer(mut self, action: impl Fn(&Arc<YACLScreen>) + 'static) -> Self {
        self.action = Some(Arc::new(move |screen: &Arc<YACLScreen>, _button: &dyn ButtonOption| action(screen)));
        self
    }

    /// java: @Override public Builder available(boolean available)
    fn available(mut self, available: bool) -> Self {
        self.available = available;
        self
    }

    /// java: @Override public ButtonOption build()
    fn build(self) -> Arc<dyn ButtonOption> {
        let name = self.name.expect("`name` must not be null when building `ButtonOption`");
        let action = self.action.expect("`action` must not be null when building `ButtonOption`");
        ButtonOptionImpl::new(name, self.description, action, self.text, self.available)
    }
}
