//! Ported from dev.isxander.yacl3.api.controller.ControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;

/// java: @FunctionalInterface public interface ControllerBuilder<T>
pub trait ControllerBuilder<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: @ApiStatus.Internal Controller<T> build();
    fn build(&mut self) -> compat::Shared<dyn Controller<T>>;

}
