//! Ported from dev.isxander.yacl3.api.controller.DoubleFieldControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::double_field_controller_builder_impl::DoubleFieldControllerBuilderImpl;

/// java: public interface DoubleFieldControllerBuilder extends NumberFieldControllerBuilder<Double, DoubleFieldControllerBuilder>
pub trait DoubleFieldControllerBuilder: NumberFieldControllerBuilder<f64> {}

/// java: static DoubleFieldControllerBuilder create(Option<Double> option)
pub fn create(option: Arc<dyn YOption<f64>>) -> DoubleFieldControllerBuilderImpl {
    DoubleFieldControllerBuilderImpl::new(option)
}
