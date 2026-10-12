//! Ported from dev.isxander.yacl3.api.controller.SliderControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;

/// java: public interface SliderControllerBuilder<T extends Number, B extends SliderControllerBuilder<T, B>> extends ValueFormattableController<T, B>
/// (`B` is the builder's own type: `Self`)
pub trait SliderControllerBuilder<T: Clone + 'static>: ValueFormattableController<T> {
    /// java: B range(T min, T max);
    fn range(self, min: T, max: T) -> Self;

    /// java: B step(T step);
    fn step(self, step: T) -> Self;
}
