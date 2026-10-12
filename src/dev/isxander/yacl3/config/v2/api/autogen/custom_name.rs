//! Ported from dev.isxander.yacl3.config.v2.api.autogen.CustomName by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface CustomName
///
/// Overrides the name of an auto-generated option.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct CustomName {
    /// java: String value() default "";
    pub value: String,
}

impl CustomName {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of() -> CustomName {
        CustomName {
            value: "".to_string(),
        }
    }

    /// java: String value() default "";
    pub fn value(&self) -> String {
        self.value.clone()
    }
}
