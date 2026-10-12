//! Ported from dev.isxander.yacl3.config.v2.api.autogen.MasterTickBox by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface MasterTickBox
///
/// An option factory like `TickBox` that controls other options' availability.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct MasterTickBox {
    /// java: String[] value();
    pub value: Vec<String>,
    /// java: boolean invert() default false;
    pub invert: bool,
}

impl MasterTickBox {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of(value: Vec<String>) -> MasterTickBox {
        MasterTickBox {
            value,
            invert: false,
        }
    }

    /// java: String[] value();
    pub fn value(&self) -> Vec<String> {
        self.value.clone()
    }

    /// java: boolean invert() default false;
    pub fn invert(&self) -> bool {
        self.invert
    }
}
