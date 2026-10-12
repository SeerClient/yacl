//! Ported from dev.isxander.yacl3.api.controller.LongFieldControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::long_field_controller_builder_impl::LongFieldControllerBuilderImpl;

/// java: public interface LongFieldControllerBuilder extends NumberFieldControllerBuilder<Long, LongFieldControllerBuilder>
pub trait LongFieldControllerBuilder: NumberFieldControllerBuilder<i64> {}

/// java: static LongFieldControllerBuilder create(Option<Long> option)
pub fn create(option: Arc<dyn YOption<i64>>) -> LongFieldControllerBuilderImpl {
    LongFieldControllerBuilderImpl::new(option)
}
