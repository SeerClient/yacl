//! Ported from dev.isxander.yacl3.api.controller.TickBoxControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::tick_box_controller_builder_impl::TickBoxControllerBuilderImpl;

/// java: public interface TickBoxControllerBuilder extends ControllerBuilder<Boolean>
pub trait TickBoxControllerBuilder: ControllerBuilder<bool> {}

/// java: static TickBoxControllerBuilder create(Option<Boolean> option)
pub fn create(option: Arc<dyn YOption<bool>>) -> TickBoxControllerBuilderImpl {
    TickBoxControllerBuilderImpl::new(option)
}
