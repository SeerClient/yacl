//! Ported from dev.isxander.yacl3.api.controller.IntegerFieldControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::integer_field_controller_builder_impl::IntegerFieldControllerBuilderImpl;

/// java: public interface IntegerFieldControllerBuilder extends NumberFieldControllerBuilder<Integer, IntegerFieldControllerBuilder>
pub trait IntegerFieldControllerBuilder: NumberFieldControllerBuilder + compat::DynAny {
}

/// java: static IntegerFieldControllerBuilder create(Option<Integer> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i32>>) -> compat::Shared<dyn IntegerFieldControllerBuilder> {
    return (IntegerFieldControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn IntegerFieldControllerBuilder>);
}
