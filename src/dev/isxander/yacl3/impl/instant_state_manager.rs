//! Ported from dev.isxander.yacl3.impl.InstantStateManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::state_manager;
use crate::dev::isxander::yacl3::api::state_manager::ResetAction;
use crate::dev::isxander::yacl3::api::state_manager::StateListener;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::r#impl::provides_binding_for_deprecation::ProvidesBindingForDeprecation;

/// java: public class InstantStateManager<T> implements StateManager<T>, ProvidesBindingForDeprecation<T>
#[derive(Clone)]
pub struct InstantStateManager<T> {
    /// java: private final Binding<T> binding;
    pub binding: compat::Shared<dyn Binding<T>>,
    /// java: private final T previousValue;
    pub previous_value: T,
    /// java: private T pendingValue;
    pub pending_value: T,
    /// java: private StateListener<T> stateListener;
    pub state_listener: compat::Shared<dyn StateListener<T>>,
    /// java: private boolean requiresSave = false;
    pub requires_save: bool,
}

impl<T: Clone + 'static> InstantStateManager<T> {
    /// java: public InstantStateManager(Binding<T> binding)
    pub fn new(mut binding: compat::Shared<dyn Binding<T>>) -> Self {
        let __init_binding = binding.clone();
        let __init_previous_value = binding.lock().borrow().get_value();
        let __init_pending_value = binding.lock().borrow().get_value();
        let __init_state_listener = state_manager::noop();
        let mut this = Self {
            binding: __init_binding,
            previous_value: __init_previous_value,
            pending_value: __init_pending_value,
            state_listener: __init_state_listener,
            requires_save: false,
        };
        this
    }

}

impl<T: Clone + 'static> StateManager<T> for InstantStateManager<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void set(T value)
    fn set(&mut self, mut value: T) {
        let mut changed: bool = !(self.pending_value == value);
        let mut previous_value: bool = (self.previous_value == value);
        { let (__a0,) = (value.clone(),); self.binding.lock().borrow_mut().set_value(__a0) };
        self.pending_value = value.clone();
        if previous_value {
            self.requires_save = false;
        }
        if changed && !previous_value {
            self.requires_save = true;
        }
        if changed {
            let __h0 = &self.pending_value;
            self.state_listener.lock().borrow().on_state_change(__h0, &value);
        }
    }

    /// java: @Override public T get()
    fn get(&self) -> T {
        return self.pending_value.clone();
    }

    /// java: @Override public void apply()
    fn apply(&mut self) {
        self.requires_save = false;
    }

    /// java: @Override public void resetToDefault(ResetAction action)
    fn reset_to_default(&mut self, mut action: ResetAction) {
        { let __s0 = self.binding.lock().borrow().default_value(); self.set(__s0) };
    }

    /// java: @Override public void sync()
    fn sync(&mut self) {
        { let __s0 = self.previous_value.clone(); self.set(__s0) };
    }

    /// java: @Override public boolean isSynced()
    fn is_synced(&self) -> bool {
        return !self.requires_save;
    }

    /// java: @Override public boolean isDefault()
    fn is_default(&self) -> bool {
        return (self.binding.lock().borrow().default_value() == self.pending_value);
    }

    /// java: @Override public void addListener(StateListener<T> stateListener)
    fn add_listener(&mut self, state_listener: &compat::Shared<dyn StateListener<T>>) {
        self.state_listener = self.state_listener.lock().borrow().and_then(state_listener);
    }

}

impl<T: Clone + 'static> ProvidesBindingForDeprecation<T> for InstantStateManager<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Binding<T> getBinding()
    fn get_binding(&self) -> compat::Shared<dyn Binding<T>> {
        return self.binding.clone();
    }

}
