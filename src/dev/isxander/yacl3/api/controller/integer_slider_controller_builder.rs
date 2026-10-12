//! Ported from dev.isxander.yacl3.api.controller.IntegerSliderControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::integer_slider_controller_builder_impl::IntegerSliderControllerBuilderImpl;

/// java: public interface IntegerSliderControllerBuilder extends SliderControllerBuilder<Integer, IntegerSliderControllerBuilder>
pub trait IntegerSliderControllerBuilder: SliderControllerBuilder<i32> {}

/// java: static IntegerSliderControllerBuilder create(Option<Integer> option)
pub fn create(option: Arc<dyn YOption<i32>>) -> IntegerSliderControllerBuilderImpl {
    IntegerSliderControllerBuilderImpl::new(option)
}
