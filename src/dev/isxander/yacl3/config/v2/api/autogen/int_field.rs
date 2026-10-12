//! Ported from dev.isxander.yacl3.config.v2.api.autogen.IntField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface IntField
///
/// A regular option factory: an `IntegerFieldControllerBuilder` controller.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct IntField {
    /// java: int min() default Integer.MIN_VALUE;
    pub min: i32,
    /// java: int max() default Integer.MAX_VALUE;
    pub max: i32,
    /// java: String format() default "%d";
    pub format: String,
}

impl IntField {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of() -> IntField {
        IntField {
            min: i32::MIN,
            max: i32::MAX,
            format: "%d".to_string(),
        }
    }

    /// java: int min() default Integer.MIN_VALUE;
    pub fn min(&self) -> i32 {
        self.min
    }

    /// java: int max() default Integer.MAX_VALUE;
    pub fn max(&self) -> i32 {
        self.max
    }

    /// java: String format() default "%d";
    pub fn format(&self) -> String {
        self.format.clone()
    }
}
