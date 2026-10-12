//! Ported from dev.isxander.yacl3.api.controller.DoubleFieldControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::double_field_controller_builder_impl::DoubleFieldControllerBuilderImpl;

/// java: public interface DoubleFieldControllerBuilder extends NumberFieldControllerBuilder<Double, DoubleFieldControllerBuilder>
pub trait DoubleFieldControllerBuilder: NumberFieldControllerBuilder + compat::DynAny {
}

/// java: static DoubleFieldControllerBuilder create(Option<Double> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f64>>) -> compat::Shared<dyn DoubleFieldControllerBuilder> {
    return (DoubleFieldControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn DoubleFieldControllerBuilder>);
}
