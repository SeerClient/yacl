//! Ported from dev.isxander.yacl3.config.v2.api.autogen.AutoGen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface AutoGen
///
/// A field annotated with this generates an option in the auto-generated GUI.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct AutoGen {
    /// java: String category();
    pub category: String,
    /// java: String group() default "";
    pub group: String,
}

impl AutoGen {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of(category: String) -> AutoGen {
        AutoGen {
            category,
            group: "".to_string(),
        }
    }

    /// java: String category();
    pub fn category(&self) -> String {
        self.category.clone()
    }

    /// java: String group() default "";
    pub fn group(&self) -> String {
        self.group.clone()
    }
}
