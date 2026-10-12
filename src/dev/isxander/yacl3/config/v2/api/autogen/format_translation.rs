//! Ported from dev.isxander.yacl3.config.v2.api.autogen.FormatTranslation by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface FormatTranslation
///
/// A custom value formatter in the form of a translation key (one argument: the value's `toString`).
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct FormatTranslation {
    /// java: String value() default "";
    pub value: String,
}

impl FormatTranslation {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of() -> FormatTranslation {
        FormatTranslation {
            value: "".to_string(),
        }
    }

    /// java: String value() default "";
    pub fn value(&self) -> String {
        self.value.clone()
    }
}
