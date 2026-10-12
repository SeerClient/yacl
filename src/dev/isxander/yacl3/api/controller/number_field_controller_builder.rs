//! Ported from dev.isxander.yacl3.api.controller.NumberFieldControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;

/// java: public interface NumberFieldControllerBuilder<T extends Number, B extends NumberFieldControllerBuilder<T, B>> extends ValueFormattableController<T, B>
/// (`B` is the builder's own type: `Self`)
pub trait NumberFieldControllerBuilder<T: Clone + 'static>: ValueFormattableController<T> {
    /// java: B min(T min);
    fn min(self, min: T) -> Self;

    /// java: B max(T max);
    fn max(self, max: T) -> Self;

    /// java: B range(T min, T max);
    fn range(self, min: T, max: T) -> Self;
}
