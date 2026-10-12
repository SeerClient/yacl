//! Ported from dev.isxander.yacl3.config.v2.api.autogen.ColorField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface ColorField
///
/// An option factory: a regular option with a `ColorControllerBuilder` controller.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct ColorField {
    /// java: boolean allowAlpha() default false;
    pub allow_alpha: bool,
}

impl ColorField {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of() -> ColorField {
        ColorField {
            allow_alpha: false,
        }
    }

    /// java: boolean allowAlpha() default false;
    pub fn allow_alpha(&self) -> bool {
        self.allow_alpha
    }
}
