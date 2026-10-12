//! Ported from dev.isxander.yacl3.impl.controller.TickBoxControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::tick_box_controller_builder::TickBoxControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::tick_box_controller::TickBoxController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;

/// java: public class TickBoxControllerBuilderImpl extends AbstractControllerBuilderImpl<Boolean> implements TickBoxControllerBuilder
#[derive(Clone)]
pub struct TickBoxControllerBuilderImpl {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<bool>,
}

impl std::ops::Deref for TickBoxControllerBuilderImpl {
    type Target = AbstractControllerBuilderImplBase<bool>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TickBoxControllerBuilderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TickBoxControllerBuilderImpl {
    /// java: public TickBoxControllerBuilderImpl(Option<Boolean> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractControllerBuilderImplBase::<bool>::new(option.clone()),
        });
        __this
    }

}

impl AbstractControllerBuilderImpl<bool> for TickBoxControllerBuilderImpl {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<bool> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<bool> {
        &mut self.base
    }

}

impl ControllerBuilder<compat::Object> for TickBoxControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<Boolean> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<bool>> {
        return (compat::shared(TickBoxController::new(self.option.clone())) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl TickBoxControllerBuilder for TickBoxControllerBuilderImpl {
}
