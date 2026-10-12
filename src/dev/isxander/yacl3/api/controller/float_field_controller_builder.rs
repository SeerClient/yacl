//! Ported from dev.isxander.yacl3.api.controller.FloatFieldControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::float_field_controller_builder_impl::FloatFieldControllerBuilderImpl;

/// java: public interface FloatFieldControllerBuilder extends NumberFieldControllerBuilder<Float, FloatFieldControllerBuilder>
pub trait FloatFieldControllerBuilder: NumberFieldControllerBuilder + compat::DynAny {
}

/// java: static FloatFieldControllerBuilder create(Option<Float> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f32>>) -> compat::Shared<dyn FloatFieldControllerBuilder> {
    return (FloatFieldControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn FloatFieldControllerBuilder>);
}
