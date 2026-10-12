//! Ported from dev.isxander.yacl3.impl.controller.IntegerSliderControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::integer_slider_controller_builder::IntegerSliderControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::slider::integer_slider_controller::{self, IntegerSliderController};

/// java: public class IntegerSliderControllerBuilderImpl extends AbstractControllerBuilderImpl<Integer> implements IntegerSliderControllerBuilder
pub struct IntegerSliderControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<i32>>,
    /// java: private i32 min, max;
    pub min: i32,
    pub max: i32,
    /// java: private i32 step;
    pub step: i32,
    /// java: private ValueFormatter<Integer> formatter = IntegerSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: Arc<dyn ValueFormatter<i32>>,
}

impl IntegerSliderControllerBuilderImpl {
    /// java: public IntegerSliderControllerBuilderImpl(Option<Integer> option)
    pub fn new(option: Arc<dyn YOption<i32>>) -> Self {
        Self { option, min: Default::default(), max: Default::default(), step: Default::default(), formatter: Arc::new(integer_slider_controller::default_formatter) }
    }
}

impl SliderControllerBuilder<i32> for IntegerSliderControllerBuilderImpl {
    /// java: @Override public IntegerSliderControllerBuilder range(Integer min, Integer max)
    fn range(mut self, min: i32, max: i32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// java: @Override public IntegerSliderControllerBuilder step(Integer step)
    fn step(mut self, step: i32) -> Self {
        self.step = step;
        self
    }
}

impl ValueFormattableController<i32> for IntegerSliderControllerBuilderImpl {
    /// java: @Override public IntegerSliderControllerBuilder formatValue(ValueFormatter<Integer> formatter)
    fn format_value(mut self, formatter: Arc<dyn ValueFormatter<i32>>) -> Self {
        self.formatter = formatter;
        self
    }
}

impl ControllerBuilder<i32> for IntegerSliderControllerBuilderImpl {
    /// java: @Override public Controller<Integer> build()
    fn build(&self) -> Arc<dyn Controller<i32>> {
        IntegerSliderController::create_internal(self.option.clone(), self.min, self.max, self.step, self.formatter.clone())
    }
}

impl IntegerSliderControllerBuilder for IntegerSliderControllerBuilderImpl {}
