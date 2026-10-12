//! Ported from dev.isxander.yacl3.impl.controller.LongSliderControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::long_slider_controller_builder::LongSliderControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::slider::long_slider_controller::{self, LongSliderController};

/// java: public class LongSliderControllerBuilderImpl extends AbstractControllerBuilderImpl<Long> implements LongSliderControllerBuilder
pub struct LongSliderControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<i64>>,
    /// java: private i64 min, max;
    pub min: i64,
    pub max: i64,
    /// java: private i64 step;
    pub step: i64,
    /// java: private ValueFormatter<Long> formatter = LongSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: Arc<dyn ValueFormatter<i64>>,
}

impl LongSliderControllerBuilderImpl {
    /// java: public LongSliderControllerBuilderImpl(Option<Long> option)
    pub fn new(option: Arc<dyn YOption<i64>>) -> Self {
        Self { option, min: Default::default(), max: Default::default(), step: Default::default(), formatter: Arc::new(long_slider_controller::default_formatter) }
    }
}

impl SliderControllerBuilder<i64> for LongSliderControllerBuilderImpl {
    /// java: @Override public LongSliderControllerBuilder range(Long min, Long max)
    fn range(mut self, min: i64, max: i64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// java: @Override public LongSliderControllerBuilder step(Long step)
    fn step(mut self, step: i64) -> Self {
        self.step = step;
        self
    }
}

impl ValueFormattableController<i64> for LongSliderControllerBuilderImpl {
    /// java: @Override public LongSliderControllerBuilder formatValue(ValueFormatter<Long> formatter)
    fn format_value(mut self, formatter: Arc<dyn ValueFormatter<i64>>) -> Self {
        self.formatter = formatter;
        self
    }
}

impl ControllerBuilder<i64> for LongSliderControllerBuilderImpl {
    /// java: @Override public Controller<Long> build()
    fn build(&self) -> Arc<dyn Controller<i64>> {
        LongSliderController::create_internal(self.option.clone(), self.min, self.max, self.step, self.formatter.clone())
    }
}

impl LongSliderControllerBuilder for LongSliderControllerBuilderImpl {}
