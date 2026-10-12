//! Ported from dev.isxander.yacl3.api.controller.LongFieldControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::long_field_controller_builder_impl::LongFieldControllerBuilderImpl;

/// java: public interface LongFieldControllerBuilder extends NumberFieldControllerBuilder<Long, LongFieldControllerBuilder>
pub trait LongFieldControllerBuilder: NumberFieldControllerBuilder + compat::DynAny {
}

/// java: static LongFieldControllerBuilder create(Option<Long> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i64>>) -> compat::Shared<dyn LongFieldControllerBuilder> {
    return (LongFieldControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn LongFieldControllerBuilder>);
}
