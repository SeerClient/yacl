//! Ported from dev.isxander.yacl3.config.v3.AbstractReadonlyConfigEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry;

/// java: @ApiStatus.Experimental public abstract class AbstractReadonlyConfigEntry<T> implements ReadonlyConfigEntry<T>
#[derive(Clone)]
pub struct AbstractReadonlyConfigEntryBase<T> {
    /// java: private final String fieldName;
    pub field_name: String,
    /// java: private Function<T, T> getModifier;
    pub get_modifier: Arc<dyn Fn(T) -> T>,
}

impl<T: Clone + 'static> AbstractReadonlyConfigEntryBase<T> {
    /// java: public AbstractReadonlyConfigEntry(String fieldName)
    pub fn new(mut field_name: String) -> Self {
        let __init_field_name = field_name.clone();
        let __init_get_modifier = Arc::new(move |__x| __x);
        let mut this = Self {
            field_name: __init_field_name,
            get_modifier: __init_get_modifier,
        };
        this
    }

}

/// java: the virtual methods of AbstractReadonlyConfigEntry (implemented by AbstractReadonlyConfigEntryBase and every subclass)
pub trait AbstractReadonlyConfigEntry<T: Clone + 'static>: ReadonlyConfigEntry {
    fn abstract_readonly_config_entry_base(&self) -> &AbstractReadonlyConfigEntryBase<T>;
    fn abstract_readonly_config_entry_base_mut(&mut self) -> &mut AbstractReadonlyConfigEntryBase<T>;

    /// java: protected abstract T innerGet();
    fn inner_get(&self) -> T;

}

// AbstractReadonlyConfigEntry.fieldName implements ReadonlyConfigEntry.fieldName: it is emitted in each concrete subclass's `impl ReadonlyConfigEntry` (Rust has no inherited impls)
// AbstractReadonlyConfigEntry.get implements ReadonlyConfigEntry.get: it is emitted in each concrete subclass's `impl ReadonlyConfigEntry` (Rust has no inherited impls)
// AbstractReadonlyConfigEntry.modifyGet implements ReadonlyConfigEntry.modifyGet: it is emitted in each concrete subclass's `impl ReadonlyConfigEntry` (Rust has no inherited impls)
