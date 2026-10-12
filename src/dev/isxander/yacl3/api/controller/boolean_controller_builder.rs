//! Ported from dev.isxander.yacl3.api.controller.BooleanControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::r#impl::controller::boolean_controller_builder_impl::BooleanControllerBuilderImpl;

/// java: public interface BooleanControllerBuilder extends ValueFormattableController<Boolean, BooleanControllerBuilder>
pub trait BooleanControllerBuilder: ValueFormattableController<bool> {
    /// java: BooleanControllerBuilder coloured(boolean coloured);
    fn coloured(self, coloured: bool) -> Self;

    /// java: BooleanControllerBuilder onOffFormatter();
    fn on_off_formatter(self) -> Self;

    /// java: BooleanControllerBuilder yesNoFormatter();
    fn yes_no_formatter(self) -> Self;

    /// java: BooleanControllerBuilder trueFalseFormatter();
    fn true_false_formatter(self) -> Self;
}

/// java: static BooleanControllerBuilder create(Option<Boolean> option)
pub fn create(option: Arc<dyn YOption<bool>>) -> BooleanControllerBuilderImpl {
    BooleanControllerBuilderImpl::new(option)
}
