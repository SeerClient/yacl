//! Ported from dev.isxander.yacl3.config.v2.api.autogen.DoubleSlider by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface DoubleSlider
///
/// A regular option factory: a `DoubleSliderControllerBuilder` controller.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct DoubleSlider {
    /// java: double min();
    pub min: f64,
    /// java: double max();
    pub max: f64,
    /// java: double step();
    pub step: f64,
    /// java: String format() default "%.2f";
    pub format: String,
}

impl DoubleSlider {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of(min: f64, max: f64, step: f64) -> DoubleSlider {
        DoubleSlider {
            min,
            max,
            step,
            format: "%.2f".to_string(),
        }
    }

    /// java: double min();
    pub fn min(&self) -> f64 {
        self.min
    }

    /// java: double max();
    pub fn max(&self) -> f64 {
        self.max
    }

    /// java: double step();
    pub fn step(&self) -> f64 {
        self.step
    }

    /// java: String format() default "%.2f";
    pub fn format(&self) -> String {
        self.format.clone()
    }
}
