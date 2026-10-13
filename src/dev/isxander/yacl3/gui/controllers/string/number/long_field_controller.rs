//! Ported from dev.isxander.yacl3.gui.controllers.string.number.LongFieldController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController;
use crate::dev::isxander::yacl3::gui::controllers::slider::long_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::controllers::string::number::number_field_controller::{self, NumberFieldController, NUMBER_FORMAT};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class LongFieldController extends NumberFieldController<Long>
pub struct LongFieldController {
    /// java: extends NumberFieldController<Long>
    pub base: NumberFieldController<i64>,
    /// java: private final long min, max;
    min: i64,
    max: i64,
}

impl LongFieldController {
    /// java: public LongFieldController(Option<Long> option, long min, long max, Function<Long, Component> formatter)
    pub fn new(option: Arc<dyn YOption<i64>>, min: i64, max: i64, formatter: Arc<dyn ValueFormatter<i64>>) -> Self {
        Self { base: NumberFieldController::new(option, formatter), min, max }
    }

    /// java: public LongFieldController(Option<Long> option, long min, long max) - `LongSliderController.DEFAULT_FORMATTER`
    pub fn new_3(option: Arc<dyn YOption<i64>>, min: i64, max: i64) -> Self {
        Self::new(option, min, max, Arc::new(long_slider_controller::default_formatter))
    }

    /// java: public LongFieldController(Option<Long> option, Function<Long, Component> formatter) -
    /// `this(option, -Long.MAX_VALUE, Long.MAX_VALUE, formatter)`
    pub fn new_formatter(option: Arc<dyn YOption<i64>>, formatter: Arc<dyn ValueFormatter<i64>>) -> Self {
        Self::new(option, -i64::MAX, i64::MAX, formatter)
    }

    /// java: public LongFieldController(Option<Long> option) -
    /// `this(option, -Long.MAX_VALUE, Long.MAX_VALUE, LongSliderController.DEFAULT_FORMATTER)`
    pub fn new_1(option: Arc<dyn YOption<i64>>) -> Self {
        Self::new(option, -i64::MAX, i64::MAX, Arc::new(long_slider_controller::default_formatter))
    }

    /// java: @ApiStatus.Internal public static LongFieldController createInternal(Option<Long> option, long min, long max,
    /// ValueFormatter<Long> formatter)
    pub fn create_internal(
        option: Arc<dyn YOption<i64>>,
        min: i64,
        max: i64,
        formatter: Arc<dyn ValueFormatter<i64>>,
    ) -> Arc<dyn Controller<i64>> {
        Arc::new(LongFieldController::new(option, min, max, formatter))
    }
}

impl Controller<i64> for LongFieldController {
    /// java: NumberFieldController.option()
    fn option(&self) -> Arc<dyn YOption<i64>> {
        self.base.option()
    }

    /// java: NumberFieldController.formatValue()
    fn format_value(&self) -> MutableComponent {
        self.base.format_value()
    }

    /// java: NumberFieldController.provideWidget(..) - a `StringControllerElement` that applies on unfocus
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        number_field_controller::provide_widget::<i64, Self>(self, screen, widget_dimension)
    }
}

impl ISliderController<i64> for LongFieldController {
    /// java: @Override public double min()
    fn min(&self) -> f64 {
        self.min as f64
    }

    /// java: @Override public double max()
    fn max(&self) -> f64 {
        self.max as f64
    }

    /// java: NumberFieldController.interval()
    fn interval(&self) -> f64 {
        self.base.interval()
    }

    /// java: @Override public void setPendingValue(double value)
    fn set_pending_value(&self, value: f64) {
        self.base.option().request_set(value as i64);
    }

    /// java: @Override public double pendingValue()
    fn pending_value(&self) -> f64 {
        self.base.option().pending_value() as f64
    }
}

impl IStringController<i64> for LongFieldController {
    /// java: @Override public String getString() - `NUMBER_FORMAT.format(option().pendingValue())`
    fn get_string(&self) -> String {
        NUMBER_FORMAT.format(self.base.option().pending_value() as f64)
    }

    /// java: NumberFieldController.setFromString(String value)
    fn set_from_string(&self, value: &str) {
        number_field_controller::set_from_string::<i64, Self>(self, value)
    }

    /// java: NumberFieldController.isInputValid(String input)
    fn is_input_valid(&self, input: &str) -> bool {
        number_field_controller::is_input_valid(input)
    }
}
