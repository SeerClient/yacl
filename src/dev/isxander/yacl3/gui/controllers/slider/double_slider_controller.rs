//! Ported from dev.isxander.yacl3.gui.controllers.slider.DoubleSliderController.

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

/// java: public static final Function<Double, Component> DEFAULT_FORMATTER = value ->
/// Component.literal(String.format("%,.2f", value).replaceAll("[\u00a0\u202F]", " "));
///
/// `String.format` without a locale formats in the JVM's default one; `compat::string_format` is the English
/// grouping (GAPS_Y2.md G9), whose separators are already plain.
pub fn default_formatter(value: f64) -> MutableComponent {
    component::literal(&compat::string_format("%,.2f", &[compat::FormatArg::Float(value)]).replace(['\u{a0}', '\u{202F}'], " "))
}

/// java: public class DoubleSliderController implements ISliderController<Double>
///
/// `ISliderController` for doubles.
pub struct DoubleSliderController {
    /// java: private final Option<Double> option;
    option: Arc<dyn YOption<f64>>,
    /// java: private final double min, max, interval;
    min: f64,
    max: f64,
    interval: f64,
    /// java: private final ValueFormatter<Double> valueFormatter;
    value_formatter: Arc<dyn ValueFormatter<f64>>,
}

impl DoubleSliderController {
    /// java: public DoubleSliderController(Option<Double> option, double min, double max, double interval) - using the default value
    /// formatter `DEFAULT_FORMATTER`
    pub fn new(option: Arc<dyn YOption<f64>>, min: f64, max: f64, interval: f64) -> Self {
        Self::new_5(option, min, max, interval, Arc::new(default_formatter))
    }

    /// java: public DoubleSliderController(Option<Double> option, double min, double max, double interval,
    /// Function<Double, Component> valueFormatter)
    pub fn new_5(option: Arc<dyn YOption<f64>>, min: f64, max: f64, interval: f64, value_formatter: Arc<dyn ValueFormatter<f64>>) -> Self {
        // Validate.isTrue(..) throws IllegalArgumentException; Validate.notNull(valueFormatter) cannot fail here
        assert!(max > min, "`max` cannot be smaller than `min`");
        assert!(interval > 0.0, "`interval` must be more than 0");

        Self { option, min, max, interval, value_formatter }
    }

    /// java: public static DoubleSliderController createInternal(Option<Double> option, double min, double max, double interval,
    /// ValueFormatter<Double> formatter)
    pub fn create_internal(
        option: Arc<dyn YOption<f64>>,
        min: f64,
        max: f64,
        interval: f64,
        formatter: Arc<dyn ValueFormatter<f64>>,
    ) -> Arc<dyn Controller<f64>> {
        Arc::new(DoubleSliderController::new_5(option, min, max, interval, formatter))
    }
}

impl Controller<f64> for DoubleSliderController {
    /// java: @Override public Option<Double> option()
    fn option(&self) -> Arc<dyn YOption<f64>> {
        self.option.clone()
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        self.value_formatter.format(self.option.pending_value())
    }

    /// java: ISliderController.provideWidget (the interface default)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        i_slider_controller::provide_widget::<f64, Self>(self, screen, widget_dimension)
    }
}

impl ISliderController<f64> for DoubleSliderController {
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
        self.option.request_set(value);
    }

    /// java: @Override public double pendingValue()
    fn pending_value(&self) -> f64 {
        self.option.pending_value() as f64
    }
}
