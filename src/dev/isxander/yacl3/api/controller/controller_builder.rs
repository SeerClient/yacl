//! Ported from dev.isxander.yacl3.api.controller.ControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;

/// java: @FunctionalInterface public interface ControllerBuilder<T>
///
/// The YACL builders implement it; a Java lambda `() -> new MyController(option)` is `controller_builder::of(..)`, and
/// an erased `ControllerBuilder<T>` reference is `Arc<dyn ControllerBuilder<T>>` (which is one too).
pub trait ControllerBuilder<T: Clone + 'static> {
    /// java: @ApiStatus.Internal Controller<T> build();
    fn build(&self) -> Arc<dyn Controller<T>>;
}

impl<T: Clone + 'static> ControllerBuilder<T> for Arc<dyn ControllerBuilder<T>> {
    fn build(&self) -> Arc<dyn Controller<T>> {
        (**self).build()
    }
}

impl<T: Clone + 'static> ControllerBuilder<T> for Box<dyn ControllerBuilder<T>> {
    fn build(&self) -> Arc<dyn Controller<T>> {
        (**self).build()
    }
}

/// A Java lambda implementing `ControllerBuilder<T>` (`() -> new MyController(option)`).
pub struct FnControllerBuilder<T: Clone + 'static> {
    build: Arc<dyn Fn() -> Arc<dyn Controller<T>>>,
}

impl<T: Clone + 'static> ControllerBuilder<T> for FnControllerBuilder<T> {
    fn build(&self) -> Arc<dyn Controller<T>> {
        (self.build)()
    }
}

/// `(ControllerBuilder<T>) () -> ..`
pub fn of<T: Clone + 'static>(build: impl Fn() -> Arc<dyn Controller<T>> + 'static) -> FnControllerBuilder<T> {
    FnControllerBuilder { build: Arc::new(build) }
}
