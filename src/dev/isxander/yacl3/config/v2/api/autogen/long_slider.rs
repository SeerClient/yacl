//! Ported from dev.isxander.yacl3.config.v2.api.autogen.LongSlider by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface LongSlider
///
/// A regular option factory: a `LongSliderControllerBuilder` controller.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct LongSlider {
    /// java: long min();
    pub min: i64,
    /// java: long max();
    pub max: i64,
    /// java: long step();
    pub step: i64,
    /// java: String format() default "%d";
    pub format: String,
}

impl LongSlider {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of(min: i64, max: i64, step: i64) -> LongSlider {
        LongSlider {
            min,
            max,
            step,
            format: "%d".to_string(),
        }
    }

    /// java: long min();
    pub fn min(&self) -> i64 {
        self.min
    }

    /// java: long max();
    pub fn max(&self) -> i64 {
        self.max
    }

    /// java: long step();
    pub fn step(&self) -> i64 {
        self.step
    }

    /// java: String format() default "%d";
    pub fn format(&self) -> String {
        self.format.clone()
    }
}
