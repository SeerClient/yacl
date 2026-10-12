//! Ported from dev.isxander.yacl3.impl.controller.LongFieldControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::long_field_controller_builder::LongFieldControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::slider::long_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::number::long_field_controller::LongFieldController;

/// java: public class LongFieldControllerBuilderImpl extends AbstractControllerBuilderImpl<Long> implements LongFieldControllerBuilder
pub struct LongFieldControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<i64>>,
    /// java: private i64 min = Long.MIN_VALUE;
    pub min: i64,
    /// java: private i64 max = Long.MAX_VALUE;
    pub max: i64,
    /// java: private ValueFormatter<Long> formatter = LongSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: Arc<dyn ValueFormatter<i64>>,
}

impl LongFieldControllerBuilderImpl {
    /// java: public LongFieldControllerBuilderImpl(Option<Long> option)
    pub fn new(option: Arc<dyn YOption<i64>>) -> Self {
        Self { option, min: i64::MIN, max: i64::MAX, formatter: Arc::new(long_slider_controller::default_formatter) }
    }
}

impl NumberFieldControllerBuilder<i64> for LongFieldControllerBuilderImpl {
    /// java: @Override public LongFieldControllerBuilder min(Long min)
    fn min(mut self, min: i64) -> Self {
        self.min = min;
        self
    }

    /// java: @Override public LongFieldControllerBuilder max(Long max)
    fn max(mut self, max: i64) -> Self {
        self.max = max;
        self
    }

    /// java: @Override public LongFieldControllerBuilder range(Long min, Long max)
    fn range(mut self, min: i64, max: i64) -> Self {
        self.min = min;
        self.max = max;
        self
    }
}

impl ValueFormattableController<i64> for LongFieldControllerBuilderImpl {
    /// java: @Override public LongFieldControllerBuilder formatValue(ValueFormatter<Long> formatter)
    fn format_value(mut self, formatter: Arc<dyn ValueFormatter<i64>>) -> Self {
        self.formatter = formatter;
        self
    }
}

impl ControllerBuilder<i64> for LongFieldControllerBuilderImpl {
    /// java: @Override public Controller<Long> build()
    fn build(&self) -> Arc<dyn Controller<i64>> {
        LongFieldController::create_internal(self.option.clone(), self.min, self.max, self.formatter.clone())
    }
}

impl LongFieldControllerBuilder for LongFieldControllerBuilderImpl {}
