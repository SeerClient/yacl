//! Ported from dev.isxander.yacl3.impl.GenericBindingImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;

/// java: public final class GenericBindingImpl<T> implements Binding<T>
#[derive(Clone)]
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
    pub fn new(mut def: T, getter: Arc<dyn Fn() -> T>, setting: Arc<dyn Fn(T)>) -> Self {
        let __init_def = def.clone();
        let __init_getter = getter.clone();
        let __init_setter = setting.clone();
        let mut this = Self {
            def: __init_def,
            getter: __init_getter,
            setter: __init_setter,
        };
        this
    }

}

impl<T: Clone + 'static> Binding<T> for GenericBindingImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void setValue(T value)
    fn set_value(&mut self, mut value: T) {
        (self.setter)(value.clone());
    }

    /// java: @Override public T getValue()
    fn get_value(&self) -> T {
        return (self.getter)();
    }

    /// java: @Override public T defaultValue()
    fn default_value(&self) -> T {
        return self.def.clone();
    }

}
