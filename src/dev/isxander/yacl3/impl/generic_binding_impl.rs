//! Ported from dev.isxander.yacl3.impl.GenericBindingImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;

/// java: public final class GenericBindingImpl<T> implements Binding<T>
pub struct GenericBindingImpl<T> {
    /// java: private final T def;
    pub def: T,
    /// java: private final Supplier<T> getter;
    pub getter: Arc<dyn Fn() -> T>,
    /// java: private final Consumer<T> setter;
    pub setter: Arc<dyn Fn(T)>,
}

impl<T: Clone + 'static> GenericBindingImpl<T> {
    /// java: public GenericBindingImpl(T def, Supplier<T> getter, Consumer<T> setting)
    pub fn new(def: T, getter: Arc<dyn Fn() -> T>, setting: Arc<dyn Fn(T)>) -> Self {
        Self { def, getter, setter: setting }
    }
}

impl<T: Clone + 'static> Binding<T> for GenericBindingImpl<T> {
    /// java: @Override public void setValue(T value)
    fn set_value(&self, value: T) {
        (self.setter)(value);
    }

    /// java: @Override public T getValue()
    fn get_value(&self) -> T {
        (self.getter)()
    }

    /// java: @Override public T defaultValue()
    fn default_value(&self) -> T {
        self.def.clone()
    }
}
