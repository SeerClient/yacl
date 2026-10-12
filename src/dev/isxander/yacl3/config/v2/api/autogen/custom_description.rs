//! Ported from dev.isxander.yacl3.config.v2.api.autogen.CustomDescription by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface CustomDescription
///
/// Extra description lines (translation keys) for an auto-generated option.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct CustomDescription {
    /// java: String[] value() default "";
    pub value: Vec<String>,
}

impl CustomDescription {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of() -> CustomDescription {
        CustomDescription {
            value: vec!["".to_string()],
        }
    }

    /// java: String[] value() default "";
    pub fn value(&self) -> Vec<String> {
        self.value.clone()
    }
}
