//! Ported from dev.isxander.yacl3.config.v3.ConfigEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry;

/// java: @ApiStatus.Experimental public interface ConfigEntry<T> extends ReadonlyConfigEntry<T>
pub trait ConfigEntry<T: Clone + 'static>: ReadonlyConfigEntry {
    /// java: void set(T value);
    fn set(&mut self, value: &T);

    /// java: T defaultValue();
    fn default_value(&self) -> T;

    /// java: @Override ConfigEntry<T> modifyGet(UnaryOperator<T> modifier);
    fn modify_get(&mut self, modifier: Arc<dyn Fn(T) -> T>) -> compat::Shared<dyn ConfigEntry<T>>;

    /// java: @Override default ConfigEntry<T> onGet(Consumer<T> consumer)
    fn on_get(&self, consumer: Arc<dyn Fn(T)>) -> compat::Shared<dyn ConfigEntry<T>> {
        return self.modify_get(Arc::new(move |v| {
            (consumer)(v.clone());
            return v;
        }));
    }

    /// java: ConfigEntry<T> modifySet(UnaryOperator<T> modifier);
    fn modify_set(&mut self, modifier: Arc<dyn Fn(T) -> T>) -> compat::Shared<dyn ConfigEntry<T>>;

    /// java: default ConfigEntry<T> onSet(Consumer<T> consumer)
    fn on_set(&self, consumer: Arc<dyn Fn(T)>) -> compat::Shared<dyn ConfigEntry<T>> {
        return self.modify_set(Arc::new(move |v| {
            (consumer)(v.clone());
            return v;
        }));
    }

    /// java: default Binding<T> asBinding()
    fn as_binding(&self) -> compat::Shared<dyn Binding<T>> {
        return binding::generic(&self.default_value(), Arc::new(move || self.get()), Arc::new(move |__a0| { self.set(&__a0); }));
    }

}
