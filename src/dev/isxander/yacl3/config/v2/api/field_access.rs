//! Ported from dev.isxander.yacl3.config.v2.api.FieldAccess by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;

/// java: public interface FieldAccess<T> extends ReadOnlyFieldAccess<T>
///
/// A writable field instance access. `set` takes `&self`: it writes the config object the access points at, not
/// the access itself.
pub trait FieldAccess<T: Clone + 'static>: ReadOnlyFieldAccess<T> {
    /// java: void set(T value);
    fn set(&self, value: T);
}
