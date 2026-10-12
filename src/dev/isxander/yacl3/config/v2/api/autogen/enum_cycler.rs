//! Ported from dev.isxander.yacl3.config.v2.api.autogen.EnumCycler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface EnumCycler
///
/// An option factory: a regular option with a `CyclingListControllerBuilder` controller. If the enum implements
/// [`CyclableEnum`], its allowed values are used; otherwise `allowedOrdinals`.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq, Default)]
pub struct EnumCycler {
    /// java: int[] allowedOrdinals() default {};
    pub allowed_ordinals: Vec<i32>,
}

impl EnumCycler {
    /// The annotation as written with no elements: every element takes its declared default.
    pub fn of() -> EnumCycler {
        EnumCycler { allowed_ordinals: Vec::new() }
    }

    /// java: int[] allowedOrdinals() default {};
    pub fn allowed_ordinals(&self) -> Vec<i32> {
        self.allowed_ordinals.clone()
    }
}

/// java: interface CyclableEnum<T extends Enum<T>>
///
/// An enum implements it to choose the values `EnumCyclerImpl` cycles through. YACL reaches it on a field of an
/// enum type it does not know through `TypeClass::cyclable` (generated: `|v| <E as CyclableEnum<E>>::allowed_values`).
pub trait CyclableEnum<T: Clone + 'static> {
    /// java: T[] allowedValues();
    fn allowed_values(&self) -> Vec<T>;
}
