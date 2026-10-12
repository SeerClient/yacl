//! Ported from dev.isxander.yacl3.config.v2.api.FieldAccess by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;

/// java: public interface FieldAccess<T> extends ReadOnlyFieldAccess<T>
pub trait FieldAccess<T: Clone + 'static>: ReadOnlyFieldAccess {
    /// java: void set(T value);
    fn set(&mut self, value: T);

}
