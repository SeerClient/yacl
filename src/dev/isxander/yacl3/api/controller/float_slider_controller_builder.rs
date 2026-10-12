//! Ported from dev.isxander.yacl3.api.controller.FloatSliderControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::float_slider_controller_builder_impl::FloatSliderControllerBuilderImpl;

/// java: public interface FloatSliderControllerBuilder extends SliderControllerBuilder<Float, FloatSliderControllerBuilder>
pub trait FloatSliderControllerBuilder: SliderControllerBuilder<f32> {}

/// java: static FloatSliderControllerBuilder create(Option<Float> option)
pub fn create(option: Arc<dyn YOption<f32>>) -> FloatSliderControllerBuilderImpl {
    FloatSliderControllerBuilderImpl::new(option)
}
