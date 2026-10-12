//! Ported from dev.isxander.yacl3.impl.HiddenNameListOptionEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::list_option::ListOption;
use crate::dev::isxander::yacl3::api::list_option_entry::ListOptionEntry;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_event_listener::OptionEventListener;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public class HiddenNameListOptionEntry<T> implements ListOptionEntry<T>
#[derive(Clone)]
pub struct HiddenNameListOptionEntry<T> {
    /// java: private final ListOptionEntry<T> option;
    pub option: compat::Shared<dyn ListOptionEntry<T>>,
}

impl<T: Clone + 'static> HiddenNameListOptionEntry<T> {
    /// java: public HiddenNameListOptionEntry(ListOptionEntry<T> option)
    pub fn new(mut option: compat::Shared<dyn ListOptionEntry<T>>) -> Self {
        let __init_option = option.clone();
        let mut this = Self {
            option: __init_option,
        };
        this
    }

}

impl<T: Clone + 'static> ListOptionEntry<T> for HiddenNameListOptionEntry<T> {
    /// java: @Override public ListOption<T> parentGroup()
    fn parent_group(&self) -> compat::Shared<dyn ListOption<T>> {
        return self.option.lock().borrow().parent_group();
    }

}

impl<T: Clone + 'static> crate::dev::isxander::yacl3::api::option::Option<compat::Object> for HiddenNameListOptionEntry<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @NotNull Component name()
    fn name(&self) -> MutableComponent {
        return component::empty();
    }

    /// java: @Override public @NotNull OptionDescription description()
    fn description(&self) -> compat::Shared<dyn OptionDescription> {
        return self.option.lock().borrow().description();
    }

    /// java: @Override @Deprecated public @NotNull Component tooltip()
    fn tooltip(&self) -> MutableComponent {
        return self.option.lock().borrow().tooltip();
    }

    /// java: @Override public @NotNull StateManager<T> stateManager()
    fn state_manager(&self) -> compat::Shared<dyn StateManager<T>> {
        return self.option.lock().borrow().state_manager();
    }

    /// java: @Override public @NotNull Controller<T> controller()
    fn controller(&self) -> compat::Shared<dyn Controller<T>> {
        return self.option.lock().borrow().controller();
    }

    /// java: @Override public @NotNull Binding<T> binding()
    fn binding(&self) -> compat::Shared<dyn Binding<T>> {
        return self.option.lock().borrow().binding();
    }

    /// java: @Override public boolean available()
    fn available(&self) -> bool {
        return self.option.lock().borrow().available();
    }

    /// java: @Override public void setAvailable(boolean available)
    fn set_available(&mut self, mut available: bool) {
        self.option.lock().borrow_mut().set_available(available);
    }

    /// java: @Override public @NotNull ImmutableSet<OptionFlag> flags()
    fn flags(&self) -> std::collections::HashSet<compat::Shared<dyn OptionFlag>> {
        return self.option.lock().borrow().flags();
    }

    /// java: @Override public boolean changed()
    fn changed(&self) -> bool {
        return self.option.lock().borrow().changed();
    }

    /// java: @Override public @NotNull T pendingValue()
    fn pending_value(&self) -> T {
        return self.option.lock().borrow().pending_value();
    }

    /// java: @Override public void requestSet(@NotNull T value)
    fn request_set(&mut self, mut value: T) {
        { let (__a0,) = (value.clone(),); self.option.lock().borrow_mut().request_set(__a0) };
    }

    /// java: @Override public boolean applyValue()
    fn apply_value(&self) -> bool {
        return self.option.lock().borrow().apply_value();
    }

    /// java: @Override public void forgetPendingValue()
    fn forget_pending_value(&mut self) {
        self.option.lock().borrow_mut().forget_pending_value();
    }

    /// java: @Override public void requestSetDefault()
    fn request_set_default(&mut self) {
        self.option.lock().borrow_mut().request_set_default();
    }

    /// java: @Override public boolean isPendingValueDefault()
    fn is_pending_value_default(&self) -> bool {
        return self.option.lock().borrow().is_pending_value_default();
    }

    /// java: @Override public boolean canResetToDefault()
    fn can_reset_to_default(&self) -> bool {
        return self.option.lock().borrow().can_reset_to_default();
    }

    /// java: @Override @Deprecated public void addListener(BiConsumer<Option<T>, T> changedListener)
    fn add_listener(&mut self, changed_listener: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, T)>) {
        { let (__a0,) = (changed_listener.clone(),); self.option.lock().borrow_mut().add_listener(__a0) };
    }

    /// java: @Override public void addEventListener(OptionEventListener<T> listener)
    fn add_event_listener(&mut self, mut listener: compat::Shared<dyn OptionEventListener<T>>) {
        { let (__a0,) = (listener.clone(),); self.option.lock().borrow_mut().add_event_listener(__a0) };
    }

}
