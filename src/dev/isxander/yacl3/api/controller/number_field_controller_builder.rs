//! Ported from dev.isxander.yacl3.api.controller.NumberFieldControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;

/// java: public interface NumberFieldControllerBuilder<T extends Number, B extends NumberFieldControllerBuilder<T, B>> extends ValueFormattableController<T, B>
pub trait NumberFieldControllerBuilder<T: Clone + 'static, B: Clone + 'static>: ValueFormattableController {
    /// java: B min(T min);
    fn min(&mut self, min: &T) -> B;

    /// java: B max(T max);
    fn max(&mut self, max: &T) -> B;

    /// java: B range(T min, T max);
    fn range(&mut self, min: &T, max: &T) -> B;

}
