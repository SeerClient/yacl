//! Ported from dev.isxander.yacl3.config.v2.api.autogen.FloatSlider by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface FloatSlider
///
/// A regular option factory: a `FloatSliderControllerBuilder` controller.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct FloatSlider {
    /// java: float min();
    pub min: f32,
    /// java: float max();
    pub max: f32,
    /// java: float step();
    pub step: f32,
    /// java: String format() default "%.1f";
    pub format: String,
}

impl FloatSlider {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of(min: f32, max: f32, step: f32) -> FloatSlider {
        FloatSlider {
            min,
            max,
            step,
            format: "%.1f".to_string(),
        }
    }

    /// java: float min();
    pub fn min(&self) -> f32 {
        self.min
    }

    /// java: float max();
    pub fn max(&self) -> f32 {
        self.max
    }

    /// java: float step();
    pub fn step(&self) -> f32 {
        self.step
    }

    /// java: String format() default "%.1f";
    pub fn format(&self) -> String {
        self.format.clone()
    }
}
