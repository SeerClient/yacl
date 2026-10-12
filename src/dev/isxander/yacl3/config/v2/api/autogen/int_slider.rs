//! Ported from dev.isxander.yacl3.config.v2.api.autogen.IntSlider by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface IntSlider
///
/// A regular option factory: an `IntegerSliderControllerBuilder` controller.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct IntSlider {
    /// java: int min();
    pub min: i32,
    /// java: int max();
    pub max: i32,
    /// java: int step();
    pub step: i32,
    /// java: String format() default "%d";
    pub format: String,
}

impl IntSlider {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of(min: i32, max: i32, step: i32) -> IntSlider {
        IntSlider {
            min,
            max,
            step,
            format: "%d".to_string(),
        }
    }

    /// java: int min();
    pub fn min(&self) -> i32 {
        self.min
    }

    /// java: int max();
    pub fn max(&self) -> i32 {
        self.max
    }

    /// java: int step();
    pub fn step(&self) -> i32 {
        self.step
    }

    /// java: String format() default "%d";
    pub fn format(&self) -> String {
        self.format.clone()
    }
}
