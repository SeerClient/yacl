//! Ported from dev.isxander.yacl3.config.v2.api.autogen.TickBox by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface TickBox
///
/// An option factory: a `TickBoxControllerBuilder` controller. No elements.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct TickBox {}

impl TickBox {
    /// The marker annotation (it has no elements).
    pub fn of() -> TickBox {
        TickBox {}
    }
}
