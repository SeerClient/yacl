//! Ported from dev.isxander.yacl3.api.controller.LongSliderControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::long_slider_controller_builder_impl::LongSliderControllerBuilderImpl;

/// java: public interface LongSliderControllerBuilder extends SliderControllerBuilder<Long, LongSliderControllerBuilder>
pub trait LongSliderControllerBuilder: SliderControllerBuilder + compat::DynAny {
}

/// java: static LongSliderControllerBuilder create(Option<Long> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i64>>) -> compat::Shared<dyn LongSliderControllerBuilder> {
    return (LongSliderControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn LongSliderControllerBuilder>);
}
