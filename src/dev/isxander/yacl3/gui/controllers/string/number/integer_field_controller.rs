//! Ported from dev.isxander.yacl3.gui.controllers.string.number.IntegerFieldController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController;
use crate::dev::isxander::yacl3::gui::controllers::slider::integer_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::controllers::string::number::number_field_controller::{self, NumberFieldController, NUMBER_FORMAT};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class IntegerFieldController extends NumberFieldController<Integer>
pub struct IntegerFieldController {
    /// java: extends NumberFieldController<Integer>
    pub base: NumberFieldController<i32>,
    /// java: private final int min, max;
    min: i32,
    max: i32,
}

impl IntegerFieldController {
    /// java: public IntegerFieldController(Option<Integer> option, int min, int max, Function<Integer, Component> formatter)
    pub fn new(option: Arc<dyn YOption<i32>>, min: i32, max: i32, formatter: Arc<dyn ValueFormatter<i32>>) -> Self {
        Self { base: NumberFieldController::new(option, formatter), min, max }
    }

    /// java: public IntegerFieldController(Option<Integer> option, int min, int max) - `IntegerSliderController.DEFAULT_FORMATTER`
    pub fn new_3(option: Arc<dyn YOption<i32>>, min: i32, max: i32) -> Self {
        Self::new(option, min, max, Arc::new(integer_slider_controller::default_formatter))
    }

    /// java: public IntegerFieldController(Option<Integer> option, Function<Integer, Component> formatter) -
    /// `this(option, -Integer.MAX_VALUE, Integer.MAX_VALUE, formatter)`
    pub fn new_formatter(option: Arc<dyn YOption<i32>>, formatter: Arc<dyn ValueFormatter<i32>>) -> Self {
        Self::new(option, -i32::MAX, i32::MAX, formatter)
    }

    /// java: public IntegerFieldController(Option<Integer> option) -
    /// `this(option, -Integer.MAX_VALUE, Integer.MAX_VALUE, IntegerSliderController.DEFAULT_FORMATTER)`
    pub fn new_1(option: Arc<dyn YOption<i32>>) -> Self {
        Self::new(option, -i32::MAX, i32::MAX, Arc::new(integer_slider_controller::default_formatter))
    }

    /// java: @ApiStatus.Internal public static IntegerFieldController createInternal(Option<Integer> option, int min, int max,
    /// ValueFormatter<Integer> formatter)
    pub fn create_internal(
        option: Arc<dyn YOption<i32>>,
        min: i32,
        max: i32,
        formatter: Arc<dyn ValueFormatter<i32>>,
    ) -> Arc<dyn Controller<i32>> {
        Arc::new(IntegerFieldController::new(option, min, max, formatter))
    }
}

impl Controller<i32> for IntegerFieldController {
    /// java: NumberFieldController.option()
    fn option(&self) -> Arc<dyn YOption<i32>> {
        self.base.option()
    }

    /// java: NumberFieldController.formatValue()
    fn format_value(&self) -> MutableComponent {
        self.base.format_value()
    }

    /// java: NumberFieldController.provideWidget(..) - a `StringControllerElement` that applies on unfocus
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        number_field_controller::provide_widget::<i32, Self>(self, screen, widget_dimension)
    }
}

impl ISliderController<i32> for IntegerFieldController {
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
        self.base.option().request_set(value as i32);
    }

    /// java: @Override public double pendingValue()
    fn pending_value(&self) -> f64 {
        self.base.option().pending_value() as f64
    }
}

impl IStringController<i32> for IntegerFieldController {
    /// java: @Override public String getString() - `NUMBER_FORMAT.format(option().pendingValue())`
    fn get_string(&self) -> String {
        NUMBER_FORMAT.format(self.base.option().pending_value() as f64)
    }

    /// java: NumberFieldController.setFromString(String value)
    fn set_from_string(&self, value: &str) {
        number_field_controller::set_from_string::<i32, Self>(self, value)
    }

    /// java: NumberFieldController.isInputValid(String input)
    fn is_input_valid(&self, input: &str) -> bool {
        number_field_controller::is_input_valid(input)
    }
}
