//! Ported from dev.isxander.yacl3.config.ConfigEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface ConfigEntry
///
/// @Deprecated - use `SerialEntry` (v2). A marker annotation with no elements.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ConfigEntry {}

impl ConfigEntry {
    /// The marker annotation (it has no elements).
    pub fn of() -> ConfigEntry {
        ConfigEntry {}
    }
}
