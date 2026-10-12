//! Ported from dev.isxander.yacl3.config.v2.api.autogen.FloatField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface FloatField
///
/// A regular option factory: a `FloatFieldControllerBuilder` controller.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct FloatField {
    /// java: float min() default -Float.MAX_VALUE;
    pub min: f32,
    /// java: float max() default Float.MAX_VALUE;
    pub max: f32,
    /// java: String format() default "%.1f";
    pub format: String,
}

impl FloatField {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of() -> FloatField {
        FloatField {
            min: -f32::MAX,
            max: f32::MAX,
            format: "%.1f".to_string(),
        }
    }

    /// java: float min() default -Float.MAX_VALUE;
    pub fn min(&self) -> f32 {
        self.min
    }

    /// java: float max() default Float.MAX_VALUE;
    pub fn max(&self) -> f32 {
        self.max
    }

    /// java: String format() default "%.1f";
    pub fn format(&self) -> String {
        self.format.clone()
    }
}
