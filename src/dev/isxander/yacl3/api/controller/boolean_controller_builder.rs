//! Ported from dev.isxander.yacl3.api.controller.BooleanControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::r#impl::controller::boolean_controller_builder_impl::BooleanControllerBuilderImpl;

/// java: public interface BooleanControllerBuilder extends ValueFormattableController<Boolean, BooleanControllerBuilder>
pub trait BooleanControllerBuilder: ValueFormattableController + compat::DynAny {
    /// java: BooleanControllerBuilder coloured(boolean coloured); (dispatch: the body for this class)
    fn coloured(&self) -> fn(&compat::Shared<dyn BooleanControllerBuilder>, bool) -> compat::Shared<dyn BooleanControllerBuilder>;

    /// java: BooleanControllerBuilder onOffFormatter(); (dispatch: the body for this class)
    fn on_off_formatter(&self) -> fn(&compat::Shared<dyn BooleanControllerBuilder>) -> compat::Shared<dyn BooleanControllerBuilder>;

    /// java: BooleanControllerBuilder yesNoFormatter(); (dispatch: the body for this class)
    fn yes_no_formatter(&self) -> fn(&compat::Shared<dyn BooleanControllerBuilder>) -> compat::Shared<dyn BooleanControllerBuilder>;

    /// java: BooleanControllerBuilder trueFalseFormatter(); (dispatch: the body for this class)
    fn true_false_formatter(&self) -> fn(&compat::Shared<dyn BooleanControllerBuilder>) -> compat::Shared<dyn BooleanControllerBuilder>;

}

/// java: static BooleanControllerBuilder create(Option<Boolean> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>>) -> compat::Shared<dyn BooleanControllerBuilder> {
    return (BooleanControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn BooleanControllerBuilder>);
}
