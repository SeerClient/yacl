//! Ported from dev.isxander.yacl3.api.controller.LongSliderControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::long_slider_controller_builder_impl::LongSliderControllerBuilderImpl;

/// java: public interface LongSliderControllerBuilder extends SliderControllerBuilder<Long, LongSliderControllerBuilder>
pub trait LongSliderControllerBuilder: SliderControllerBuilder<i64> {}

/// java: static LongSliderControllerBuilder create(Option<Long> option)
pub fn create(option: Arc<dyn YOption<i64>>) -> LongSliderControllerBuilderImpl {
    LongSliderControllerBuilderImpl::new(option)
}
