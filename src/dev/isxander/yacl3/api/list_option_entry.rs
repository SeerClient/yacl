//! Ported from dev.isxander.yacl3.api.ListOptionEntry.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::list_option::ListOption;
use crate::dev::isxander::yacl3::api::option::Option as YOption;

/// java: public interface ListOptionEntry<T> extends Option<T>
///
/// Its two default overrides (`flags()` and `available()` answering the parent group's) are written by each
/// implementation (Rust has no default override of a supertrait's method); `parent_flags` / `parent_available` are
/// their bodies.
pub trait ListOptionEntry<T: Clone + 'static>: YOption<T> {
    /// java: ListOption<T> parentGroup();
    fn parent_group(&self) -> Arc<dyn ListOption<T>>;
}

/// java: @Override default @NotNull ImmutableSet<OptionFlag> flags() - `parentGroup().flags()`
pub fn parent_flags<T: Clone + 'static>(this: &dyn ListOptionEntry<T>) -> Vec<Arc<dyn crate::dev::isxander::yacl3::api::option_flag::OptionFlag>> {
    crate::dev::isxander::yacl3::api::option::AnyOption::flags(&*this.parent_group())
}

/// java: @Override default boolean available() - `parentGroup().available()`
pub fn parent_available<T: Clone + 'static>(this: &dyn ListOptionEntry<T>) -> bool {
    crate::dev::isxander::yacl3::api::option::AnyOption::available(&*this.parent_group())
}
