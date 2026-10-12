//! Ported from dev.isxander.yacl3.impl.controller.DoubleSliderControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::double_slider_controller_builder::DoubleSliderControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::slider::double_slider_controller::{self, DoubleSliderController};

/// java: public class DoubleSliderControllerBuilderImpl extends AbstractControllerBuilderImpl<Double> implements DoubleSliderControllerBuilder
pub struct DoubleSliderControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<f64>>,
    /// java: private f64 min, max;
    pub min: f64,
    pub max: f64,
    /// java: private f64 step;
    pub step: f64,
    /// java: private ValueFormatter<Double> formatter = DoubleSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: Arc<dyn ValueFormatter<f64>>,
}

impl DoubleSliderControllerBuilderImpl {
    /// java: public DoubleSliderControllerBuilderImpl(Option<Double> option)
    pub fn new(option: Arc<dyn YOption<f64>>) -> Self {
        Self { option, min: Default::default(), max: Default::default(), step: Default::default(), formatter: Arc::new(double_slider_controller::default_formatter) }
    }
}

impl SliderControllerBuilder<f64> for DoubleSliderControllerBuilderImpl {
    /// java: @Override public DoubleSliderControllerBuilder range(Double min, Double max)
    fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// java: @Override public DoubleSliderControllerBuilder step(Double step)
    fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }
}

impl ValueFormattableController<f64> for DoubleSliderControllerBuilderImpl {
    /// java: @Override public DoubleSliderControllerBuilder formatValue(ValueFormatter<Double> formatter)
    fn format_value(mut self, formatter: Arc<dyn ValueFormatter<f64>>) -> Self {
        self.formatter = formatter;
        self
    }
}

impl ControllerBuilder<f64> for DoubleSliderControllerBuilderImpl {
    /// java: @Override public Controller<Double> build()
    fn build(&self) -> Arc<dyn Controller<f64>> {
        DoubleSliderController::create_internal(self.option.clone(), self.min, self.max, self.step, self.formatter.clone())
    }
}

impl DoubleSliderControllerBuilder for DoubleSliderControllerBuilderImpl {}
