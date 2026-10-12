//! Ported from dev.isxander.yacl3.api.controller.FloatSliderControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::float_slider_controller_builder_impl::FloatSliderControllerBuilderImpl;

/// java: public interface FloatSliderControllerBuilder extends SliderControllerBuilder<Float, FloatSliderControllerBuilder>
pub trait FloatSliderControllerBuilder: SliderControllerBuilder + compat::DynAny {
}

/// java: static FloatSliderControllerBuilder create(Option<Float> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f32>>) -> compat::Shared<dyn FloatSliderControllerBuilder> {
    return (FloatSliderControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn FloatSliderControllerBuilder>);
}
