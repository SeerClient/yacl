//! Ported from dev.isxander.yacl3.impl.controller.TickBoxControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::tick_box_controller_builder::TickBoxControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::tick_box_controller::TickBoxController;

/// java: public class TickBoxControllerBuilderImpl extends AbstractControllerBuilderImpl<Boolean> implements TickBoxControllerBuilder
pub struct TickBoxControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<bool>>,
}

impl TickBoxControllerBuilderImpl {
    /// java: public TickBoxControllerBuilderImpl(Option<Boolean> option)
    pub fn new(option: Arc<dyn YOption<bool>>) -> Self {
        Self { option }
    }
}

impl ControllerBuilder<bool> for TickBoxControllerBuilderImpl {
    /// java: @Override public Controller<Boolean> build()
    fn build(&self) -> Arc<dyn Controller<bool>> {
        Arc::new(TickBoxController::new(self.option.clone()))
    }
}

impl TickBoxControllerBuilder for TickBoxControllerBuilderImpl {}
