//! Ported from dev.isxander.yacl3.api.controller.StringControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::string_controller_builder_impl::StringControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::string_controller_builder_impl::StringControllerBuilderImplBase;

/// java: public interface StringControllerBuilder extends ControllerBuilder<String>
pub trait StringControllerBuilder: ControllerBuilder + compat::DynAny {
}

/// java: static StringControllerBuilder create(Option<String> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<String>>) -> compat::Shared<dyn StringControllerBuilder> {
    return ((StringControllerBuilderImplBase::new(option.clone()) as compat::Shared<dyn StringControllerBuilderImpl>) as compat::Shared<dyn StringControllerBuilder>);
}
