//! Ported from dev.isxander.yacl3.config.v3.AbstractReadonlyConfigEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry;

/// java: @ApiStatus.Experimental public abstract class AbstractReadonlyConfigEntry<T> implements ReadonlyConfigEntry<T>
/// - its fields, and the bodies of its methods every subclass's `impl ReadonlyConfigEntry` calls (Rust has no
/// inherited impls).
pub struct AbstractReadonlyConfigEntryBase<T: 'static> {
    /// java: private final String fieldName;
    pub field_name: String,
    /// java: private Function<T, T> getModifier;
    pub get_modifier: std::cell::RefCell<Arc<dyn Fn(T) -> T>>,
}

impl<T: Clone + 'static> AbstractReadonlyConfigEntryBase<T> {
    /// java: public AbstractReadonlyConfigEntry(String fieldName)
    pub fn new(field_name: String) -> Self {
        Self {
            field_name,
            // java: UnaryOperator.identity()
            get_modifier: std::cell::RefCell::new(Arc::new(|x: T| x)),
        }
    }

    /// java: @Override public String fieldName()
    pub fn field_name(&self) -> String {
        self.field_name.clone()
    }

    /// java: @Override public T get() - `this.getModifier.apply(this.innerGet())`, handed `innerGet()`
    pub fn get(&self, inner_get: T) -> T {
        let get_modifier = self.get_modifier.borrow().clone();
        get_modifier(inner_get)
    }

    /// java: @Override public ReadonlyConfigEntry<T> modifyGet(UnaryOperator<T> modifier) - the body before
    /// `return this`: `this.getModifier = this.getModifier.andThen(modifier);`
    pub fn modify_get(&self, modifier: Arc<dyn Fn(T) -> T>) {
        let prev = self.get_modifier.borrow().clone();
        *self.get_modifier.borrow_mut() = Arc::new(move |x: T| modifier(prev(x)));
    }
}

/// java: the virtual methods of AbstractReadonlyConfigEntry (implemented by every subclass)
pub trait AbstractReadonlyConfigEntry<T: Clone + 'static>: ReadonlyConfigEntry<T> {
    fn abstract_readonly_config_entry_base(&self) -> &AbstractReadonlyConfigEntryBase<T>;

    /// java: protected abstract T innerGet();
    fn inner_get(&self) -> T;
}
