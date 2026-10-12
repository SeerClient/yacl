//! Ported from dev.isxander.yacl3.api.controller.SliderControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;

/// java: public interface SliderControllerBuilder<T extends Number, B extends SliderControllerBuilder<T, B>> extends ValueFormattableController<T, B>
pub trait SliderControllerBuilder<T: Clone + 'static, B: Clone + 'static>: ValueFormattableController {
    /// java: B range(T min, T max);
    fn range(&mut self, min: &T, max: &T) -> B;

    /// java: B step(T step);
    fn step(&mut self, step: &T) -> B;

}
