//! Ported from dev.isxander.yacl3.config.v3.AbstractConfigEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntry;
use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntryBase;
use crate::dev::isxander::yacl3::config::v3::config_entry::ConfigEntry;

/// java: @ApiStatus.Experimental public abstract class AbstractConfigEntry<T> extends AbstractReadonlyConfigEntry<T> implements ConfigEntry<T>
/// - its fields, and the bodies of its methods every subclass's impls call (Rust has no inherited impls).
pub struct AbstractConfigEntryBase<T: 'static> {
    /// java: extends AbstractReadonlyConfigEntry
    pub base: AbstractReadonlyConfigEntryBase<T>,
    /// java: private T value;
    pub value: std::cell::RefCell<T>,
    /// java: private final T defaultValue;
    pub default_value: T,
    /// java: private Function<T, T> setModifier;
    pub set_modifier: std::cell::RefCell<Arc<dyn Fn(T) -> T>>,
}

impl<T: 'static> std::ops::Deref for AbstractConfigEntryBase<T> {
    type Target = AbstractReadonlyConfigEntryBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: Clone + 'static> AbstractConfigEntryBase<T> {
    /// java: public AbstractConfigEntry(String fieldName, T defaultValue)
    pub fn new(field_name: String, default_value: T) -> Self {
        Self {
            base: AbstractReadonlyConfigEntryBase::new(field_name),
            value: std::cell::RefCell::new(default_value.clone()),
            default_value,
            // java: UnaryOperator.identity()
            set_modifier: std::cell::RefCell::new(Arc::new(|x: T| x)),
        }
    }

    /// java: @Override protected T innerGet()
    pub fn inner_get(&self) -> T {
        self.value.borrow().clone()
    }

    /// java: @Override public void set(T value)
    pub fn set(&self, value: T) {
        let set_modifier = self.set_modifier.borrow().clone();
        let value = set_modifier(value);
        *self.value.borrow_mut() = value;
    }

    /// java: @Override public T defaultValue()
    pub fn default_value(&self) -> T {
        self.default_value.clone()
    }

    /// java: @Override public ConfigEntry<T> modifyGet(UnaryOperator<T> modifier) - `super.modifyGet(modifier)`
    /// before `return this`
    pub fn modify_get(&self, modifier: Arc<dyn Fn(T) -> T>) {
        self.base.modify_get(modifier);
    }

    /// java: @Override public ConfigEntry<T> modifySet(UnaryOperator<T> modifier) - the body before `return this`:
    /// `this.setModifier = this.setModifier.andThen(modifier);`
    pub fn modify_set(&self, modifier: Arc<dyn Fn(T) -> T>) {
        let prev = self.set_modifier.borrow().clone();
        *self.set_modifier.borrow_mut() = Arc::new(move |x: T| modifier(prev(x)));
    }
}

/// java: the virtual methods of AbstractConfigEntry (implemented by every subclass)
pub trait AbstractConfigEntry<T: Clone + 'static>: AbstractReadonlyConfigEntry<T> + ConfigEntry<T> {
    fn abstract_config_entry_base(&self) -> &AbstractConfigEntryBase<T>;
}
