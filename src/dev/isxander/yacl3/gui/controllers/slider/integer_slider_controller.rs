//! Ported from dev.isxander.yacl3.gui.controllers.slider.IntegerSliderController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::{self, ISliderController};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public static final Function<Integer, Component> DEFAULT_FORMATTER = value ->
/// Component.literal(String.format("%,d", value).replaceAll("[\u00a0\u202F]", " "));
///
/// `String.format` without a locale formats in the JVM's default one; `compat::string_format` is the English
/// grouping (GAPS_Y2.md G9), whose separators are already plain.
pub fn default_formatter(value: i32) -> MutableComponent {
    component::literal(&compat::string_format("%,d", &[compat::FormatArg::Int(value as i64)]).replace(['\u{a0}', '\u{202F}'], " "))
}

/// java: public class IntegerSliderController implements ISliderController<Integer>
///
/// `ISliderController` for integers.
pub struct IntegerSliderController {
    /// java: private final Option<Integer> option;
    option: Arc<dyn YOption<i32>>,
    /// java: private final int min, max, interval;
    min: i32,
    max: i32,
    interval: i32,
    /// java: private final ValueFormatter<Integer> valueFormatter;
    value_formatter: Arc<dyn ValueFormatter<i32>>,
}

impl IntegerSliderController {
    /// java: public IntegerSliderController(Option<Integer> option, int min, int max, int interval) - using the default value
    /// formatter `DEFAULT_FORMATTER`
    pub fn new(option: Arc<dyn YOption<i32>>, min: i32, max: i32, interval: i32) -> Self {
        Self::new_5(option, min, max, interval, Arc::new(default_formatter))
    }

    /// java: public IntegerSliderController(Option<Integer> option, int min, int max, int interval,
    /// Function<Integer, Component> valueFormatter)
    pub fn new_5(option: Arc<dyn YOption<i32>>, min: i32, max: i32, interval: i32, value_formatter: Arc<dyn ValueFormatter<i32>>) -> Self {
        // Validate.isTrue(..) throws IllegalArgumentException; Validate.notNull(valueFormatter) cannot fail here
        assert!(max > min, "`max` cannot be smaller than `min`");
        assert!(interval > 0, "`interval` must be more than 0");

        Self { option, min, max, interval, value_formatter }
    }

    /// java: public static IntegerSliderController createInternal(Option<Integer> option, int min, int max, int interval,
    /// ValueFormatter<Integer> formatter)
    pub fn create_internal(
        option: Arc<dyn YOption<i32>>,
        min: i32,
        max: i32,
        interval: i32,
        formatter: Arc<dyn ValueFormatter<i32>>,
    ) -> Arc<dyn Controller<i32>> {
        Arc::new(IntegerSliderController::new_5(option, min, max, interval, formatter))
    }
}

impl Controller<i32> for IntegerSliderController {
    /// java: @Override public Option<Integer> option()
    fn option(&self) -> Arc<dyn YOption<i32>> {
        self.option.clone()
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        self.value_formatter.format(self.option.pending_value())
    }

    /// java: ISliderController.provideWidget (the interface default)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        i_slider_controller::provide_widget::<i32, Self>(self, screen, widget_dimension)
    }
}

impl ISliderController<i32> for IntegerSliderController {
    /// java: @Override public double min()
    fn min(&self) -> f64 {
        self.min as f64
    }

    /// java: @Override public double max()
    fn max(&self) -> f64 {
        self.max as f64
    }

    /// java: @Override public double interval()
    fn interval(&self) -> f64 {
        self.interval as f64
    }

    /// java: @Override public void setPendingValue(double value)
    fn set_pending_value(&self, value: f64) {
        self.option.request_set(value as i32);
    }

    /// java: @Override public double pendingValue()
    fn pending_value(&self) -> f64 {
        self.option.pending_value() as f64
    }
}
