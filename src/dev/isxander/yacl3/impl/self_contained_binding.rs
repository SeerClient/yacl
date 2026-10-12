//! Ported from dev.isxander.yacl3.impl.SelfContainedBinding.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;

/// java: public class SelfContainedBinding<T> implements Binding<T>
pub struct SelfContainedBinding<T> {
    /// java: private T value;
    pub value: std::cell::RefCell<T>,
    /// java: private final T defaultValue;
    pub default_value: T,
}

impl<T: Clone + 'static> SelfContainedBinding<T> {
    /// java: public SelfContainedBinding(T value, T defaultValue)
    pub fn new(value: T, default_value: T) -> Self {
        Self { value: std::cell::RefCell::new(value), default_value }
    }

    /// java: public SelfContainedBinding(T value)
    pub fn new_1(value: T) -> Self {
        Self::new(value.clone(), value)
    }
}

impl<T: Clone + 'static> Binding<T> for SelfContainedBinding<T> {
    /// java: @Override public void setValue(T value)
    fn set_value(&self, value: T) {
        *self.value.borrow_mut() = value;
    }

    /// java: @Override public T getValue()
    fn get_value(&self) -> T {
        self.value.borrow().clone()
    }

    /// java: @Override public T defaultValue()
    fn default_value(&self) -> T {
        self.default_value.clone()
    }
}
