//! Ported from dev.isxander.yacl3.api.controller.DoubleSliderControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::double_slider_controller_builder_impl::DoubleSliderControllerBuilderImpl;

/// java: public interface DoubleSliderControllerBuilder extends SliderControllerBuilder<Double, DoubleSliderControllerBuilder>
pub trait DoubleSliderControllerBuilder: SliderControllerBuilder<f64> {}

/// java: static DoubleSliderControllerBuilder create(Option<Double> option)
pub fn create(option: Arc<dyn YOption<f64>>) -> DoubleSliderControllerBuilderImpl {
    DoubleSliderControllerBuilderImpl::new(option)
}
