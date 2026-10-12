//! Ported from dev.isxander.yacl3.config.v2.api.SerialEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface SerialEntry
///
/// Marks a field as serializable (also allowed on a class: only `required` and `nullable` there).
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct SerialEntry {
    /// java: String value() default "";
    pub value: String,
    /// java: String comment() default "";
    pub comment: String,
    /// java: boolean required() default true;
    pub required: bool,
    /// java: boolean nullable() default false;
    pub nullable: bool,
}

impl SerialEntry {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of() -> SerialEntry {
        SerialEntry {
            value: "".to_string(),
            comment: "".to_string(),
            required: true,
            nullable: false,
        }
    }

    /// java: String value() default "";
    pub fn value(&self) -> String {
        self.value.clone()
    }

    /// java: String comment() default "";
    pub fn comment(&self) -> String {
        self.comment.clone()
    }

    /// java: boolean required() default true;
    pub fn required(&self) -> bool {
        self.required
    }

    /// java: boolean nullable() default false;
    pub fn nullable(&self) -> bool {
        self.nullable
    }
}
