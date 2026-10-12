//! Ported from dev.isxander.yacl3.config.v2.api.autogen.LongField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface LongField
///
/// A regular option factory: a `LongFieldControllerBuilder` controller.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct LongField {
    /// java: long min() default Long.MIN_VALUE;
    pub min: i64,
    /// java: long max() default Long.MAX_VALUE;
    pub max: i64,
    /// java: String format() default "%d";
    pub format: String,
}

impl LongField {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of() -> LongField {
        LongField {
            min: i64::MIN,
            max: i64::MAX,
            format: "%d".to_string(),
        }
    }

    /// java: long min() default Long.MIN_VALUE;
    pub fn min(&self) -> i64 {
        self.min
    }

    /// java: long max() default Long.MAX_VALUE;
    pub fn max(&self) -> i64 {
        self.max
    }

    /// java: String format() default "%d";
    pub fn format(&self) -> String {
        self.format.clone()
    }
}
