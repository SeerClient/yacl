//! Ported from dev.isxander.yacl3.impl.InstantStateManager.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::state_manager::{self, ResetAction, StateListener, StateManager};
use crate::dev::isxander::yacl3::r#impl::provides_binding_for_deprecation::ProvidesBindingForDeprecation;
use std::cell::{Cell, RefCell};

/// java: public class InstantStateManager<T> implements StateManager<T>, ProvidesBindingForDeprecation<T>
pub struct InstantStateManager<T: Clone + 'static> {
    /// java: private final Binding<T> binding;
    pub binding: Arc<dyn Binding<T>>,
    /// java: private final T previousValue;
    pub previous_value: T,
    /// java: private T pendingValue;
    pub pending_value: RefCell<T>,
    /// java: private StateListener<T> stateListener;
    pub state_listener: RefCell<Arc<dyn StateListener<T>>>,
    /// java: private boolean requiresSave = false;
    pub requires_save: Cell<bool>,
}

impl<T: Clone + PartialEq + 'static> InstantStateManager<T> {
    /// java: public InstantStateManager(Binding<T> binding)
    pub fn new(binding: Arc<dyn Binding<T>>) -> Self {
        let previous_value = binding.get_value();
        let pending_value = binding.get_value();
        Self {
            binding,
            previous_value,
            pending_value: RefCell::new(pending_value),
            state_listener: RefCell::new(state_manager::noop()),
            requires_save: Cell::new(false),
        }
    }
}

impl<T: Clone + PartialEq + 'static> StateManager<T> for InstantStateManager<T> {
    /// java: @Override public void set(T value)
    fn set(&self, value: T) {
        let changed = *self.pending_value.borrow() != value;
        let previous_value = self.previous_value == value;
        self.binding.set_value(value.clone());
        *self.pending_value.borrow_mut() = value.clone();
        if previous_value {
            self.requires_save.set(false);
        }
        if changed && !previous_value {
            self.requires_save.set(true);
        }
        if changed {
            let listener = self.state_listener.borrow().clone();
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
        self.requires_save.set(false);
    }

    /// java: @Override public void resetToDefault(ResetAction action)
    fn reset_to_default(&self, _action: ResetAction) {
        self.set(self.binding.default_value());
    }

    /// java: @Override public void sync()
    fn sync(&self) {
        self.set(self.previous_value.clone());
    }

    /// java: @Override public boolean isSynced()
    fn is_synced(&self) -> bool {
        !self.requires_save.get()
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

impl<T: Clone + PartialEq + 'static> ProvidesBindingForDeprecation<T> for InstantStateManager<T> {
    /// java: @Override public Binding<T> getBinding()
    fn get_binding(&self) -> Arc<dyn Binding<T>> {
        self.binding.clone()
    }
}
