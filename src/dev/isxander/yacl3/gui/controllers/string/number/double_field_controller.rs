//! Ported from dev.isxander.yacl3.gui.controllers.string.number.DoubleFieldController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController;
use crate::dev::isxander::yacl3::gui::controllers::slider::double_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::controllers::string::number::number_field_controller::{self, NumberFieldController, NUMBER_FORMAT};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class DoubleFieldController extends NumberFieldController<Double>
pub struct DoubleFieldController {
    /// java: extends NumberFieldController<Double>
    pub base: NumberFieldController<f64>,
    /// java: private final double min, max;
    min: f64,
    max: f64,
}

impl DoubleFieldController {
    /// java: public DoubleFieldController(Option<Double> option, double min, double max, Function<Double, Component> formatter)
    pub fn new(option: Arc<dyn YOption<f64>>, min: f64, max: f64, formatter: Arc<dyn ValueFormatter<f64>>) -> Self {
        Self { base: NumberFieldController::new(option, formatter), min, max }
    }

    /// java: public DoubleFieldController(Option<Double> option, double min, double max) - `DoubleSliderController.DEFAULT_FORMATTER`
    pub fn new_3(option: Arc<dyn YOption<f64>>, min: f64, max: f64) -> Self {
        Self::new(option, min, max, Arc::new(double_slider_controller::default_formatter))
    }

    /// java: public DoubleFieldController(Option<Double> option, Function<Double, Component> formatter) -
    /// `this(option, -Double.MAX_VALUE, Double.MAX_VALUE, formatter)`
    pub fn new_formatter(option: Arc<dyn YOption<f64>>, formatter: Arc<dyn ValueFormatter<f64>>) -> Self {
        Self::new(option, -f64::MAX, f64::MAX, formatter)
    }

    /// java: public DoubleFieldController(Option<Double> option) -
    /// `this(option, -Double.MAX_VALUE, Double.MAX_VALUE, DoubleSliderController.DEFAULT_FORMATTER)`
    pub fn new_1(option: Arc<dyn YOption<f64>>) -> Self {
        Self::new(option, -f64::MAX, f64::MAX, Arc::new(double_slider_controller::default_formatter))
    }

    /// java: @ApiStatus.Internal public static DoubleFieldController createInternal(Option<Double> option, double min, double max,
    /// ValueFormatter<Double> formatter)
    pub fn create_internal(
        option: Arc<dyn YOption<f64>>,
        min: f64,
        max: f64,
        formatter: Arc<dyn ValueFormatter<f64>>,
    ) -> Arc<dyn Controller<f64>> {
        Arc::new(DoubleFieldController::new(option, min, max, formatter))
    }
}

impl Controller<f64> for DoubleFieldController {
    /// java: NumberFieldController.option()
    fn option(&self) -> Arc<dyn YOption<f64>> {
        self.base.option()
    }

    /// java: NumberFieldController.formatValue()
    fn format_value(&self) -> MutableComponent {
        self.base.format_value()
    }

    /// java: NumberFieldController.provideWidget(..) - a `StringControllerElement` that applies on unfocus
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        number_field_controller::provide_widget::<f64, Self>(self, screen, widget_dimension)
    }
}

impl ISliderController<f64> for DoubleFieldController {
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
        self.base.option().request_set(value);
    }

    /// java: @Override public double pendingValue()
    fn pending_value(&self) -> f64 {
        self.base.option().pending_value() as f64
    }
}

impl IStringController<f64> for DoubleFieldController {
    /// java: @Override public String getString() - `NUMBER_FORMAT.format(option().pendingValue())`
    fn get_string(&self) -> String {
        NUMBER_FORMAT.format(self.base.option().pending_value() as f64)
    }

    /// java: NumberFieldController.setFromString(String value)
    fn set_from_string(&self, value: &str) {
        number_field_controller::set_from_string::<f64, Self>(self, value)
    }

    /// java: NumberFieldController.isInputValid(String input)
    fn is_input_valid(&self, input: &str) -> bool {
        number_field_controller::is_input_valid(input)
    }
}
