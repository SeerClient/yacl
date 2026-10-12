//! Ported from dev.isxander.yacl3.api.controller.DoubleSliderControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::double_slider_controller_builder_impl::DoubleSliderControllerBuilderImpl;

/// java: public interface DoubleSliderControllerBuilder extends SliderControllerBuilder<Double, DoubleSliderControllerBuilder>
pub trait DoubleSliderControllerBuilder: SliderControllerBuilder + compat::DynAny {
}

/// java: static DoubleSliderControllerBuilder create(Option<Double> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f64>>) -> compat::Shared<dyn DoubleSliderControllerBuilder> {
    return (DoubleSliderControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn DoubleSliderControllerBuilder>);
}
