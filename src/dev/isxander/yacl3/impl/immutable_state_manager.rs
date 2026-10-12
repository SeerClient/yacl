//! Ported from dev.isxander.yacl3.impl.ImmutableStateManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::state_manager::ResetAction;
use crate::dev::isxander::yacl3::api::state_manager::StateListener;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;

/// java: public class ImmutableStateManager<T> implements StateManager<T>
#[derive(Clone)]
pub struct ImmutableStateManager<T> {
    /// java: private final T value;
    pub value: T,
}

impl<T: Clone + 'static> ImmutableStateManager<T> {
    /// java: public ImmutableStateManager(T value)
    pub fn new(mut value: T) -> Self {
        let __init_value = value.clone();
        let mut this = Self {
            value: __init_value,
        };
        this
    }

}

impl<T: Clone + 'static> StateManager<T> for ImmutableStateManager<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void set(T value)
    fn set(&mut self, mut value: T) {
        panic!("UnsupportedOperationException: {}", "Cannot set value of immutable state manager");
    }

    /// java: @Override public T get()
    fn get(&self) -> T {
        return self.value.clone();
    }

    /// java: @Override public void apply()
    fn apply(&mut self) {
    }

    /// java: @Override public void resetToDefault(ResetAction action)
    fn reset_to_default(&mut self, mut action: ResetAction) {
    }

    /// java: @Override public void sync()
    fn sync(&mut self) {
    }

    /// java: @Override public boolean isSynced()
    fn is_synced(&self) -> bool {
        return true;
    }

    /// java: @Override public boolean isAlwaysSynced()
    fn is_always_synced(&self) -> bool {
        return true;
    }

    /// java: @Override public boolean isDefault()
    fn is_default(&self) -> bool {
        return true;
    }

    /// java: @Override public void addListener(StateListener<T> stateListener)
    fn add_listener(&mut self, state_listener: &compat::Shared<dyn StateListener<T>>) {
    }

}
