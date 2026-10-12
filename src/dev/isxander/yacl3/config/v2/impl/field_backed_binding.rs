//! Ported from dev.isxander.yacl3.config.v2.impl.FieldBackedBinding by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;

/// java: public record FieldBackedBinding<T>(FieldAccess<T> field, ReadOnlyFieldAccess<T> defaultField) implements Binding<T>
pub struct FieldBackedBinding<T: 'static> {
    /// java: FieldAccess<T> field
    pub field: Arc<dyn FieldAccess<T>>,
    /// java: ReadOnlyFieldAccess<T> defaultField
    pub default_field: Arc<dyn ReadOnlyFieldAccess<T>>,
}

impl<T: 'static> Clone for FieldBackedBinding<T> {
    fn clone(&self) -> Self {
        Self { field: self.field.clone(), default_field: self.default_field.clone() }
    }
}

impl<T: Clone + 'static> FieldBackedBinding<T> {
    /// java: the canonical record constructor
    pub fn new(field: Arc<dyn FieldAccess<T>>, default_field: Arc<dyn ReadOnlyFieldAccess<T>>) -> Self {
        Self { field, default_field }
    }

    /// java: the record accessor `field()`
    pub fn field(&self) -> Arc<dyn FieldAccess<T>> {
        self.field.clone()
    }

    /// java: the record accessor `defaultField()`
    pub fn default_field(&self) -> Arc<dyn ReadOnlyFieldAccess<T>> {
        self.default_field.clone()
    }
}

impl<T: Clone + 'static> Binding<T> for FieldBackedBinding<T> {
    /// java: @Override public void setValue(T value)
    fn set_value(&self, value: T) {
        self.field.set(value);
    }

    /// java: @Override public T getValue()
    fn get_value(&self) -> T {
        self.field.get()
    }

    /// java: @Override public T defaultValue()
    fn default_value(&self) -> T {
        self.default_field.get()
    }
}
