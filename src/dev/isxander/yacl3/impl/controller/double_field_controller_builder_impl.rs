//! Ported from dev.isxander.yacl3.impl.controller.DoubleFieldControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::double_field_controller_builder::DoubleFieldControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::slider::double_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::number::double_field_controller::DoubleFieldController;

/// java: public class DoubleFieldControllerBuilderImpl extends AbstractControllerBuilderImpl<Double> implements DoubleFieldControllerBuilder
pub struct DoubleFieldControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<f64>>,
    /// java: private f64 min = -Double.MAX_VALUE;
    pub min: f64,
    /// java: private f64 max = Double.MAX_VALUE;
    pub max: f64,
    /// java: private ValueFormatter<Double> formatter = DoubleSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: Arc<dyn ValueFormatter<f64>>,
}

impl DoubleFieldControllerBuilderImpl {
    /// java: public DoubleFieldControllerBuilderImpl(Option<Double> option)
    pub fn new(option: Arc<dyn YOption<f64>>) -> Self {
        Self { option, min: -f64::MAX, max: f64::MAX, formatter: Arc::new(double_slider_controller::default_formatter) }
    }
}

impl NumberFieldControllerBuilder<f64> for DoubleFieldControllerBuilderImpl {
    /// java: @Override public DoubleFieldControllerBuilder min(Double min)
    fn min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    /// java: @Override public DoubleFieldControllerBuilder max(Double max)
    fn max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    /// java: @Override public DoubleFieldControllerBuilder range(Double min, Double max)
    fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }
}

impl ValueFormattableController<f64> for DoubleFieldControllerBuilderImpl {
    /// java: @Override public DoubleFieldControllerBuilder formatValue(ValueFormatter<Double> formatter)
    fn format_value(mut self, formatter: Arc<dyn ValueFormatter<f64>>) -> Self {
        self.formatter = formatter;
        self
    }
}

impl ControllerBuilder<f64> for DoubleFieldControllerBuilderImpl {
    /// java: @Override public Controller<Double> build()
    fn build(&self) -> Arc<dyn Controller<f64>> {
        DoubleFieldController::create_internal(self.option.clone(), self.min, self.max, self.formatter.clone())
    }
}

impl DoubleFieldControllerBuilder for DoubleFieldControllerBuilderImpl {}
