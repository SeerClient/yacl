//! Ported from dev.isxander.yacl3.impl.controller.FloatSliderControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::float_slider_controller_builder::FloatSliderControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::slider::float_slider_controller::{self, FloatSliderController};

/// java: public class FloatSliderControllerBuilderImpl extends AbstractControllerBuilderImpl<Float> implements FloatSliderControllerBuilder
pub struct FloatSliderControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<f32>>,
    /// java: private f32 min, max;
    pub min: f32,
    pub max: f32,
    /// java: private f32 step;
    pub step: f32,
    /// java: private ValueFormatter<Float> formatter = FloatSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: Arc<dyn ValueFormatter<f32>>,
}

impl FloatSliderControllerBuilderImpl {
    /// java: public FloatSliderControllerBuilderImpl(Option<Float> option)
    pub fn new(option: Arc<dyn YOption<f32>>) -> Self {
        Self { option, min: Default::default(), max: Default::default(), step: Default::default(), formatter: Arc::new(float_slider_controller::default_formatter) }
    }
}

impl SliderControllerBuilder<f32> for FloatSliderControllerBuilderImpl {
    /// java: @Override public FloatSliderControllerBuilder range(Float min, Float max)
    fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// java: @Override public FloatSliderControllerBuilder step(Float step)
    fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }
}

impl ValueFormattableController<f32> for FloatSliderControllerBuilderImpl {
    /// java: @Override public FloatSliderControllerBuilder formatValue(ValueFormatter<Float> formatter)
    fn format_value(mut self, formatter: Arc<dyn ValueFormatter<f32>>) -> Self {
        self.formatter = formatter;
        self
    }
}

impl ControllerBuilder<f32> for FloatSliderControllerBuilderImpl {
    /// java: @Override public Controller<Float> build()
    fn build(&self) -> Arc<dyn Controller<f32>> {
        FloatSliderController::create_internal(self.option.clone(), self.min, self.max, self.step, self.formatter.clone())
    }
}

impl FloatSliderControllerBuilder for FloatSliderControllerBuilderImpl {}
