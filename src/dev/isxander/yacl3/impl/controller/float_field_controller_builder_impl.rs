//! Ported from dev.isxander.yacl3.impl.controller.FloatFieldControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::float_field_controller_builder::FloatFieldControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::slider::float_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::number::float_field_controller::FloatFieldController;

/// java: public class FloatFieldControllerBuilderImpl extends AbstractControllerBuilderImpl<Float> implements FloatFieldControllerBuilder
pub struct FloatFieldControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<f32>>,
    /// java: private f32 min = -Float.MAX_VALUE;
    pub min: f32,
    /// java: private f32 max = Float.MAX_VALUE;
    pub max: f32,
    /// java: private ValueFormatter<Float> formatter = FloatSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: Arc<dyn ValueFormatter<f32>>,
}

impl FloatFieldControllerBuilderImpl {
    /// java: public FloatFieldControllerBuilderImpl(Option<Float> option)
    pub fn new(option: Arc<dyn YOption<f32>>) -> Self {
        Self { option, min: -f32::MAX, max: f32::MAX, formatter: Arc::new(float_slider_controller::default_formatter) }
    }
}

impl NumberFieldControllerBuilder<f32> for FloatFieldControllerBuilderImpl {
    /// java: @Override public FloatFieldControllerBuilder min(Float min)
    fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// java: @Override public FloatFieldControllerBuilder max(Float max)
    fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    /// java: @Override public FloatFieldControllerBuilder range(Float min, Float max)
    fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }
}

impl ValueFormattableController<f32> for FloatFieldControllerBuilderImpl {
    /// java: @Override public FloatFieldControllerBuilder formatValue(ValueFormatter<Float> formatter)
    fn format_value(mut self, formatter: Arc<dyn ValueFormatter<f32>>) -> Self {
        self.formatter = formatter;
        self
    }
}

impl ControllerBuilder<f32> for FloatFieldControllerBuilderImpl {
    /// java: @Override public Controller<Float> build()
    fn build(&self) -> Arc<dyn Controller<f32>> {
        FloatFieldController::create_internal(self.option.clone(), self.min, self.max, self.formatter.clone())
    }
}

impl FloatFieldControllerBuilder for FloatFieldControllerBuilderImpl {}
