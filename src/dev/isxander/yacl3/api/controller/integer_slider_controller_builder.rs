//! Ported from dev.isxander.yacl3.api.controller.IntegerSliderControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::integer_slider_controller_builder_impl::IntegerSliderControllerBuilderImpl;

/// java: public interface IntegerSliderControllerBuilder extends SliderControllerBuilder<Integer, IntegerSliderControllerBuilder>
pub trait IntegerSliderControllerBuilder: SliderControllerBuilder + compat::DynAny {
}

/// java: static IntegerSliderControllerBuilder create(Option<Integer> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i32>>) -> compat::Shared<dyn IntegerSliderControllerBuilder> {
    return (IntegerSliderControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn IntegerSliderControllerBuilder>);
}
