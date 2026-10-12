//! Ported from dev.isxander.yacl3.api.controller.FloatFieldControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::float_field_controller_builder_impl::FloatFieldControllerBuilderImpl;

/// java: public interface FloatFieldControllerBuilder extends NumberFieldControllerBuilder<Float, FloatFieldControllerBuilder>
pub trait FloatFieldControllerBuilder: NumberFieldControllerBuilder<f32> {}

/// java: static FloatFieldControllerBuilder create(Option<Float> option)
pub fn create(option: Arc<dyn YOption<f32>>) -> FloatFieldControllerBuilderImpl {
    FloatFieldControllerBuilderImpl::new(option)
}
