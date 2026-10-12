//! Ported from dev.isxander.yacl3.api.controller.IntegerFieldControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::integer_field_controller_builder_impl::IntegerFieldControllerBuilderImpl;

/// java: public interface IntegerFieldControllerBuilder extends NumberFieldControllerBuilder<Integer, IntegerFieldControllerBuilder>
pub trait IntegerFieldControllerBuilder: NumberFieldControllerBuilder<i32> {}

/// java: static IntegerFieldControllerBuilder create(Option<Integer> option)
pub fn create(option: Arc<dyn YOption<i32>>) -> IntegerFieldControllerBuilderImpl {
    IntegerFieldControllerBuilderImpl::new(option)
}
