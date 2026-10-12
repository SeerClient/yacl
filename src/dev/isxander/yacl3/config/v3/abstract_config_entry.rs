//! Ported from dev.isxander.yacl3.config.v3.AbstractConfigEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntry;
use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntryBase;
use crate::dev::isxander::yacl3::config::v3::config_entry::ConfigEntry;

/// java: @ApiStatus.Experimental public abstract class AbstractConfigEntry<T> extends AbstractReadonlyConfigEntry<T> implements ConfigEntry<T>
#[derive(Clone)]
pub struct AbstractConfigEntryBase<T> {
    /// java: extends AbstractReadonlyConfigEntry
    pub base: AbstractReadonlyConfigEntryBase<T>,
    /// java: private T value;
    pub value: T,
    /// java: private final T defaultValue;
    pub default_value: T,
    /// java: private Function<T, T> setModifier;
    pub set_modifier: Arc<dyn Fn(T) -> T>,
}

impl<T: 'static> std::ops::Deref for AbstractConfigEntryBase<T> {
    type Target = AbstractReadonlyConfigEntryBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for AbstractConfigEntryBase<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> AbstractConfigEntryBase<T> {
    /// java: public AbstractConfigEntry(String fieldName, T defaultValue)
    pub fn new(mut field_name: String, mut default_value: T) -> Self {
        let __init_value = default_value.clone();
        let __init_default_value = default_value.clone();
        let __init_set_modifier = Arc::new(move |__x| __x);
        let mut this = Self {
            base: AbstractReadonlyConfigEntryBase::<T>::new(field_name.clone()),
            value: __init_value,
            default_value: __init_default_value,
            set_modifier: __init_set_modifier,
        };
        this
    }

}

/// java: the virtual methods of AbstractConfigEntry (implemented by AbstractConfigEntryBase and every subclass)
pub trait AbstractConfigEntry<T: Clone + 'static>: AbstractReadonlyConfigEntry + ConfigEntry {
    fn abstract_config_entry_base(&self) -> &AbstractConfigEntryBase<T>;
    fn abstract_config_entry_base_mut(&mut self) -> &mut AbstractConfigEntryBase<T>;

}

// AbstractConfigEntry.innerGet implements AbstractReadonlyConfigEntry.innerGet: it is emitted in each concrete subclass's `impl AbstractReadonlyConfigEntry` (Rust has no inherited impls)
// AbstractConfigEntry.modifyGet implements ReadonlyConfigEntry.modifyGet: it is emitted in each concrete subclass's `impl ReadonlyConfigEntry` (Rust has no inherited impls)
// AbstractConfigEntry.set implements ConfigEntry.set: it is emitted in each concrete subclass's `impl ConfigEntry` (Rust has no inherited impls)
// AbstractConfigEntry.defaultValue implements ConfigEntry.defaultValue: it is emitted in each concrete subclass's `impl ConfigEntry` (Rust has no inherited impls)
// AbstractConfigEntry.modifySet implements ConfigEntry.modifySet: it is emitted in each concrete subclass's `impl ConfigEntry` (Rust has no inherited impls)
