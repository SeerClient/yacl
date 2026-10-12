//! Ported from dev.isxander.yacl3.impl.SimpleStateManager.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::state_manager::{self, ResetAction, StateListener, StateManager};
use crate::dev::isxander::yacl3::r#impl::provides_binding_for_deprecation::ProvidesBindingForDeprecation;
use std::cell::RefCell;

/// java: public class SimpleStateManager<T> implements StateManager<T>, ProvidesBindingForDeprecation<T>
///
/// Every field read is a borrow for one statement: `set` runs the state listener (the option's event listeners,
/// which read `get()` back) with nothing borrowed.
pub struct SimpleStateManager<T: Clone + 'static> {
    /// java: private T pendingValue;
    pub pending_value: RefCell<T>,
    /// java: private final Binding<T> binding;
    pub binding: Arc<dyn Binding<T>>,
    /// java: private StateListener<T> stateListener;
    pub state_listener: RefCell<Arc<dyn StateListener<T>>>,
}

impl<T: Clone + PartialEq + 'static> SimpleStateManager<T> {
    /// java: public SimpleStateManager(Binding<T> binding)
    pub fn new(binding: Arc<dyn Binding<T>>) -> Self {
        let pending_value = binding.get_value();
        Self {
            pending_value: RefCell::new(pending_value),
            binding,
            state_listener: RefCell::new(state_manager::noop()),
        }
    }
}

impl<T: Clone + PartialEq + 'static> StateManager<T> for SimpleStateManager<T> {
    /// java: @Override public void set(T value)
    fn set(&self, value: T) {
        let changed = *self.pending_value.borrow() != value;
        *self.pending_value.borrow_mut() = value.clone();
        if changed {
            let listener = self.state_listener.borrow().clone();
            // java: stateListener.onStateChange(this.pendingValue, value) - both are the new value
            let pending = self.pending_value.borrow().clone();
            listener.on_state_change(pending, value);
        }
    }

    /// java: @Override public T get()
    fn get(&self) -> T {
        self.pending_value.borrow().clone()
    }

    /// java: @Override public void apply()
    fn apply(&self) {
        let pending = self.pending_value.borrow().clone();
        self.binding.set_value(pending);
    }

    /// java: @Override public void resetToDefault(ResetAction action)
    fn reset_to_default(&self, _action: ResetAction) {
        self.set(self.binding.default_value());
    }

    /// java: @Override public void sync()
    fn sync(&self) {
        self.set(self.binding.get_value());
    }

    /// java: @Override public boolean isSynced()
    fn is_synced(&self) -> bool {
        let value = self.binding.get_value();
        value == *self.pending_value.borrow()
    }

    /// java: @Override public boolean isDefault()
    fn is_default(&self) -> bool {
        let value = self.binding.default_value();
        value == *self.pending_value.borrow()
    }

    /// java: @Override public void addListener(StateListener<T> stateListener)
    fn add_listener(&self, state_listener: Arc<dyn StateListener<T>>) {
        let current = self.state_listener.borrow().clone();
        *self.state_listener.borrow_mut() = state_manager::and_then(current, state_listener);
    }

    fn get_binding_for_deprecation(&self) -> std::option::Option<Arc<dyn Binding<T>>> {
        Some(self.get_binding())
    }
}

impl<T: Clone + PartialEq + 'static> ProvidesBindingForDeprecation<T> for SimpleStateManager<T> {
    /// java: @Override public Binding<T> getBinding()
    fn get_binding(&self) -> Arc<dyn Binding<T>> {
        self.binding.clone()
    }
}
