//! Ported from dev.isxander.yacl3.config.v2.api.autogen.Dropdown by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface Dropdown
///
/// An option factory: a `DropdownStringControllerBuilder` controller.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct Dropdown {
    /// java: String[] values();
    pub values: Vec<String>,
    /// java: boolean allowEmptyValue() default false;
    pub allow_empty_value: bool,
    /// java: boolean allowAnyValue() default false;
    pub allow_any_value: bool,
}

impl Dropdown {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of(values: Vec<String>) -> Dropdown {
        Dropdown {
            values,
            allow_empty_value: false,
            allow_any_value: false,
        }
    }

    /// java: String[] values();
    pub fn values(&self) -> Vec<String> {
        self.values.clone()
    }

    /// java: boolean allowEmptyValue() default false;
    pub fn allow_empty_value(&self) -> bool {
        self.allow_empty_value
    }

    /// java: boolean allowAnyValue() default false;
    pub fn allow_any_value(&self) -> bool {
        self.allow_any_value
    }
}
