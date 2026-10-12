//! Ported from dev.isxander.yacl3.config.v3.ConfigEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry;

/// java: @ApiStatus.Experimental public interface ConfigEntry<T> extends ReadonlyConfigEntry<T>
///
/// Java overrides `modifyGet` / `onGet` here with a covariant return (`ConfigEntry<T>`). Rust has no covariant
/// override and both traits are in scope everywhere (the prelude), so the overrides are `modify_get_entry` /
/// `on_get_entry`; `modify_get` / `on_get` (the `ReadonlyConfigEntry` ones) answer the same object.
pub trait ConfigEntry<T: Clone + 'static>: ReadonlyConfigEntry<T> {
    /// java: void set(T value);
    fn set(&self, value: T);

    /// java: T defaultValue();
    fn default_value(&self) -> T;

    /// java: @Override ConfigEntry<T> modifyGet(UnaryOperator<T> modifier);
    fn modify_get_entry(&self, modifier: Arc<dyn Fn(T) -> T>) -> Arc<dyn ConfigEntry<T>>;

    /// java: @Override default ConfigEntry<T> onGet(Consumer<T> consumer)
    fn on_get_entry(&self, consumer: Arc<dyn Fn(T)>) -> Arc<dyn ConfigEntry<T>> {
        self.modify_get_entry(Arc::new(move |v: T| {
            consumer(v.clone());
            v
        }))
    }

    /// java: ConfigEntry<T> modifySet(UnaryOperator<T> modifier);
    fn modify_set(&self, modifier: Arc<dyn Fn(T) -> T>) -> Arc<dyn ConfigEntry<T>>;

    /// java: default ConfigEntry<T> onSet(Consumer<T> consumer)
    fn on_set(&self, consumer: Arc<dyn Fn(T)>) -> Arc<dyn ConfigEntry<T>> {
        self.modify_set(Arc::new(move |v: T| {
            consumer(v.clone());
            v
        }))
    }
}

/// java: default Binding<T> asBinding()
///
/// `this` is the entry's own reference: the binding's getter and setter keep it (`this::get`, `this::set`).
pub fn as_binding<T: Clone + 'static>(this: &Arc<dyn ConfigEntry<T>>) -> Arc<dyn Binding<T>> {
    let getter = this.clone();
    let setter = this.clone();
    binding::generic(this.default_value(), move || getter.get(), move |v: T| setter.set(v))
}
