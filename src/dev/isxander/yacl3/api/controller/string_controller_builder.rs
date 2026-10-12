//! Ported from dev.isxander.yacl3.api.controller.StringControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::string_controller_builder_impl::StringControllerBuilderImpl;

/// java: public interface StringControllerBuilder extends ControllerBuilder<String>
pub trait StringControllerBuilder: ControllerBuilder<String> {}

/// java: static StringControllerBuilder create(Option<String> option)
pub fn create(option: Arc<dyn YOption<String>>) -> StringControllerBuilderImpl {
    StringControllerBuilderImpl::new(option)
}
