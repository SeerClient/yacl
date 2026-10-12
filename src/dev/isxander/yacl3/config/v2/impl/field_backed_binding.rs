//! Ported from dev.isxander.yacl3.config.v2.impl.FieldBackedBinding by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;

/// java: public record FieldBackedBinding<T>(FieldAccess<T> field, ReadOnlyFieldAccess<T> defaultField) implements Binding<T>
#[derive(Clone, Debug)]
pub struct FieldBackedBinding<T> {
    /// java: FieldAccess<T> field
    pub field: compat::Shared<dyn FieldAccess<T>>,
    /// java: ReadOnlyFieldAccess<T> defaultField
    pub default_field: compat::Shared<dyn ReadOnlyFieldAccess<T>>,
}

impl<T: Clone + 'static> FieldBackedBinding<T> {
    /// java: the canonical record constructor
    pub fn new(mut field: compat::Shared<dyn FieldAccess<T>>, mut default_field: compat::Shared<dyn ReadOnlyFieldAccess<T>>) -> Self {
        Self { field, default_field }
    }

    /// java: the record accessor `field()`
    pub fn field(&self) -> compat::Shared<dyn FieldAccess<T>> {
        self.field.clone()
    }

    /// java: the record accessor `defaultField()`
    pub fn default_field(&self) -> compat::Shared<dyn ReadOnlyFieldAccess<T>> {
        self.default_field.clone()
    }

}

impl<T: Clone + 'static> Binding<T> for FieldBackedBinding<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public T getValue()
    fn get_value(&self) -> T {
        return self.field.lock().borrow().get();
    }

    /// java: @Override public void setValue(T value)
    fn set_value(&mut self, mut value: T) {
        { let (__a0,) = (value.clone(),); self.field.lock().borrow_mut().set(__a0) };
    }

    /// java: @Override public T defaultValue()
    fn default_value(&self) -> T {
        return self.default_field.lock().borrow().get();
    }

}
