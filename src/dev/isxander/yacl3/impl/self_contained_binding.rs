//! Ported from dev.isxander.yacl3.impl.SelfContainedBinding by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;

/// java: public class SelfContainedBinding<T> implements Binding<T>
#[derive(Clone)]
pub struct SelfContainedBinding<T> {
    /// java: private T value;
    pub value: Option<T>,
    /// java: private final T defaultValue;
    pub default_value: Option<T>,
}

impl<T: Clone + 'static> SelfContainedBinding<T> {
    /// java: public SelfContainedBinding(T value, T defaultValue)
    pub fn new(mut value: T, mut default_value: T) -> Self {
        let __init_value = Some(value.clone());
        let __init_default_value = Some(default_value.clone());
        let mut this = Self {
            value: __init_value,
            default_value: __init_default_value,
        };
        this
    }

    /// java: public SelfContainedBinding(T value)
    pub fn new_1(mut value: T) -> Self {
        let mut this = Self::new(value.clone(), value.clone());
        this
    }

}

impl<T: Clone + 'static> Binding<T> for SelfContainedBinding<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void setValue(T value)
    fn set_value(&mut self, mut value: T) {
        self.value = Some(value.clone());
    }

    /// java: @Override public T getValue()
    fn get_value(&self) -> T {
        return self.value.clone().unwrap();
    }

    /// java: @Override public T defaultValue()
    fn default_value(&self) -> T {
        return self.default_value.clone().unwrap();
    }

}
