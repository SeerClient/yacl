//! Ported from dev.isxander.yacl3.config.v2.api.autogen.Label by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface Label
///
/// An option factory creating a `LabelOption`; the field must be a `Component`. No elements.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Label {}

impl Label {
    /// The marker annotation (it has no elements).
    pub fn of() -> Label {
        Label {}
    }
}
