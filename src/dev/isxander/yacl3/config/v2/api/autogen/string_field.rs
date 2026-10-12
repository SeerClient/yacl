//! Ported from dev.isxander.yacl3.config.v2.api.autogen.StringField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface StringField
///
/// A regular option factory: a `StringControllerBuilder` controller. No elements.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct StringField {}

impl StringField {
    /// The marker annotation (it has no elements).
    pub fn of() -> StringField {
        StringField {}
    }
}
