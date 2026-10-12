//! Ported from dev.isxander.yacl3.impl.controller.IntegerFieldControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::integer_field_controller_builder::IntegerFieldControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::slider::integer_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::number::integer_field_controller::IntegerFieldController;

/// java: public class IntegerFieldControllerBuilderImpl extends AbstractControllerBuilderImpl<Integer> implements IntegerFieldControllerBuilder
pub struct IntegerFieldControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<i32>>,
    /// java: private i32 min = Integer.MIN_VALUE;
    pub min: i32,
    /// java: private i32 max = Integer.MAX_VALUE;
    pub max: i32,
    /// java: private ValueFormatter<Integer> formatter = IntegerSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: Arc<dyn ValueFormatter<i32>>,
}

impl IntegerFieldControllerBuilderImpl {
    /// java: public IntegerFieldControllerBuilderImpl(Option<Integer> option)
    pub fn new(option: Arc<dyn YOption<i32>>) -> Self {
        Self { option, min: i32::MIN, max: i32::MAX, formatter: Arc::new(integer_slider_controller::default_formatter) }
    }
}

impl NumberFieldControllerBuilder<i32> for IntegerFieldControllerBuilderImpl {
    /// java: @Override public IntegerFieldControllerBuilder min(Integer min)
    fn min(mut self, min: i32) -> Self {
        self.min = min;
        self
    }

    /// java: @Override public IntegerFieldControllerBuilder max(Integer max)
    fn max(mut self, max: i32) -> Self {
        self.max = max;
        self
    }

    /// java: @Override public IntegerFieldControllerBuilder range(Integer min, Integer max)
    fn range(mut self, min: i32, max: i32) -> Self {
        self.min = min;
        self.max = max;
        self
    }
}

impl ValueFormattableController<i32> for IntegerFieldControllerBuilderImpl {
    /// java: @Override public IntegerFieldControllerBuilder formatValue(ValueFormatter<Integer> formatter)
    fn format_value(mut self, formatter: Arc<dyn ValueFormatter<i32>>) -> Self {
        self.formatter = formatter;
        self
    }
}

impl ControllerBuilder<i32> for IntegerFieldControllerBuilderImpl {
    /// java: @Override public Controller<Integer> build()
    fn build(&self) -> Arc<dyn Controller<i32>> {
        IntegerFieldController::create_internal(self.option.clone(), self.min, self.max, self.formatter.clone())
    }
}

impl IntegerFieldControllerBuilder for IntegerFieldControllerBuilderImpl {}
