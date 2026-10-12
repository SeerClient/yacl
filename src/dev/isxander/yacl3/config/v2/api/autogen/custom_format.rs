//! Ported from dev.isxander.yacl3.config.v2.api.autogen.CustomFormat by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ClassRef;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface CustomFormat
///
/// Allows you to specify a custom `ValueFormatter` for a field.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct CustomFormat {
    /// java: Class<? extends ValueFormatter<?>> value();
    /// `new_instance` yields a `compat::RenderThread<Arc<dyn ValueFormatter<X>>>`.
    pub value: ClassRef,
}

impl CustomFormat {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of(value: ClassRef) -> CustomFormat {
        CustomFormat {
            value,
        }
    }

    /// java: Class<? extends ValueFormatter<?>> value();
    pub fn value(&self) -> ClassRef {
        self.value
    }
}
