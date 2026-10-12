//! Ported from dev.isxander.yacl3.impl.ImmutableStateManager.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::state_manager::{ResetAction, StateListener, StateManager};

/// java: public class ImmutableStateManager<T> implements StateManager<T>
pub struct ImmutableStateManager<T> {
    /// java: private final T value;
    pub value: T,
}

impl<T: Clone + 'static> ImmutableStateManager<T> {
    /// java: public ImmutableStateManager(T value)
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: Clone + 'static> StateManager<T> for ImmutableStateManager<T> {
    /// java: @Override public void set(T value)
    fn set(&self, _value: T) {
        panic!("UnsupportedOperationException: Cannot set value of immutable state manager");
    }

    /// java: @Override public T get()
    fn get(&self) -> T {
        self.value.clone()
    }

    /// java: @Override public void apply()
    fn apply(&self) {
        // no-op
    }

    /// java: @Override public void resetToDefault(ResetAction action)
    fn reset_to_default(&self, _action: ResetAction) {
        // always default
    }

    /// java: @Override public void sync()
    fn sync(&self) {
        // always synced
    }

    /// java: @Override public boolean isSynced()
    fn is_synced(&self) -> bool {
        true
    }

    /// java: @Override public boolean isAlwaysSynced()
    fn is_always_synced(&self) -> bool {
        true
    }

    /// java: @Override public boolean isDefault()
    fn is_default(&self) -> bool {
        true
    }

    /// java: @Override public void addListener(StateListener<T> stateListener)
    fn add_listener(&self, _state_listener: Arc<dyn StateListener<T>>) {
        // as the values never change, listeners are not needed and would never be called
    }
}
