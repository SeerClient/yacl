//! Ported from dev.isxander.yacl3.api.controller.TickBoxControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::tick_box_controller_builder_impl::TickBoxControllerBuilderImpl;

/// java: public interface TickBoxControllerBuilder extends ControllerBuilder<Boolean>
pub trait TickBoxControllerBuilder: ControllerBuilder + compat::DynAny {
}

/// java: static TickBoxControllerBuilder create(Option<Boolean> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>>) -> compat::Shared<dyn TickBoxControllerBuilder> {
    return (TickBoxControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn TickBoxControllerBuilder>);
}
