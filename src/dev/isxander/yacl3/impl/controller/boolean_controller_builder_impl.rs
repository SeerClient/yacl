//! Ported from dev.isxander.yacl3.impl.controller.BooleanControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::boolean_controller_builder::BooleanControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::boolean_controller::{self, BooleanController};

/// java: public class BooleanControllerBuilderImpl extends AbstractControllerBuilderImpl<Boolean> implements BooleanControllerBuilder
pub struct BooleanControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<bool>>,
    /// java: private boolean coloured = false;
    pub coloured: bool,
    /// java: private ValueFormatter<Boolean> formatter = BooleanController.ON_OFF_FORMATTER::apply;
    pub formatter: Arc<dyn ValueFormatter<bool>>,
}

impl BooleanControllerBuilderImpl {
    /// java: public BooleanControllerBuilderImpl(Option<Boolean> option)
    pub fn new(option: Arc<dyn YOption<bool>>) -> Self {
        Self { option, coloured: false, formatter: Arc::new(boolean_controller::on_off_formatter) }
    }
}

impl BooleanControllerBuilder for BooleanControllerBuilderImpl {
    /// java: @Override public BooleanControllerBuilder coloured(boolean coloured)
    fn coloured(mut self, coloured: bool) -> Self {
        self.coloured = coloured;
        self
    }

    /// java: @Override public BooleanControllerBuilder onOffFormatter()
    fn on_off_formatter(mut self) -> Self {
        self.formatter = Arc::new(boolean_controller::on_off_formatter);
        self
    }

    /// java: @Override public BooleanControllerBuilder yesNoFormatter()
    fn yes_no_formatter(mut self) -> Self {
        self.formatter = Arc::new(boolean_controller::yes_no_formatter);
        self
    }

    /// java: @Override public BooleanControllerBuilder trueFalseFormatter()
    fn true_false_formatter(mut self) -> Self {
        self.formatter = Arc::new(boolean_controller::true_false_formatter);
        self
    }
}

impl ValueFormattableController<bool> for BooleanControllerBuilderImpl {
    /// java: @Override public BooleanControllerBuilder formatValue(ValueFormatter<Boolean> formatter)
    fn format_value(mut self, formatter: Arc<dyn ValueFormatter<bool>>) -> Self {
        self.formatter = formatter;
        self
    }
}

impl ControllerBuilder<bool> for BooleanControllerBuilderImpl {
    /// java: @Override public Controller<Boolean> build()
    fn build(&self) -> Arc<dyn Controller<bool>> {
        BooleanController::create_internal(self.option.clone(), self.formatter.clone(), self.coloured)
    }
}
