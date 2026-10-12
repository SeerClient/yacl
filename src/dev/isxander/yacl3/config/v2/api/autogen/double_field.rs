//! Ported from dev.isxander.yacl3.config.v2.api.autogen.DoubleField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface DoubleField
///
/// A regular option factory: a `DoubleFieldControllerBuilder` controller.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct DoubleField {
    /// java: double min() default -Double.MAX_VALUE;
    pub min: f64,
    /// java: double max() default Double.MAX_VALUE;
    pub max: f64,
    /// java: String format() default "%.2f";
    pub format: String,
}

impl DoubleField {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of() -> DoubleField {
        DoubleField {
            min: -f64::MAX,
            max: f64::MAX,
            format: "%.2f".to_string(),
        }
    }

    /// java: double min() default -Double.MAX_VALUE;
    pub fn min(&self) -> f64 {
        self.min
    }

    /// java: double max() default Double.MAX_VALUE;
    pub fn max(&self) -> f64 {
        self.max
    }

    /// java: String format() default "%.2f";
    pub fn format(&self) -> String {
        self.format.clone()
    }
}
