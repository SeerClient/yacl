//! Ported from dev.isxander.yacl3.gui.controllers.slider.LongSliderController.

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

/// java: public static final Function<Long, Component> DEFAULT_FORMATTER = value ->
/// Component.literal(String.format("%,d", value).replaceAll("[\u00a0\u202F]", " "));
///
/// `String.format` without a locale formats in the JVM's default one; `compat::string_format` is the English
/// grouping (GAPS_Y2.md G9), whose separators are already plain.
pub fn default_formatter(value: i64) -> MutableComponent {
    component::literal(&compat::string_format("%,d", &[compat::FormatArg::Int(value)]).replace(['\u{a0}', '\u{202F}'], " "))
}

/// java: public class LongSliderController implements ISliderController<Long>
///
/// `ISliderController` for longs.
pub struct LongSliderController {
    /// java: private final Option<Long> option;
    option: Arc<dyn YOption<i64>>,
    /// java: private final long min, max, interval;
    min: i64,
    max: i64,
    interval: i64,
    /// java: private final ValueFormatter<Long> valueFormatter;
    value_formatter: Arc<dyn ValueFormatter<i64>>,
}

impl LongSliderController {
    /// java: public LongSliderController(Option<Long> option, long min, long max, long interval) - using the default value
    /// formatter `DEFAULT_FORMATTER`
    pub fn new(option: Arc<dyn YOption<i64>>, min: i64, max: i64, interval: i64) -> Self {
        Self::new_5(option, min, max, interval, Arc::new(default_formatter))
    }

    /// java: public LongSliderController(Option<Long> option, long min, long max, long interval,
    /// Function<Long, Component> valueFormatter)
    pub fn new_5(option: Arc<dyn YOption<i64>>, min: i64, max: i64, interval: i64, value_formatter: Arc<dyn ValueFormatter<i64>>) -> Self {
        // Validate.isTrue(..) throws IllegalArgumentException; Validate.notNull(valueFormatter) cannot fail here
        assert!(max > min, "`max` cannot be smaller than `min`");
        assert!(interval > 0, "`interval` must be more than 0");

        Self { option, min, max, interval, value_formatter }
    }

    /// java: public static LongSliderController createInternal(Option<Long> option, long min, long max, long interval,
    /// ValueFormatter<Long> formatter)
    pub fn create_internal(
        option: Arc<dyn YOption<i64>>,
        min: i64,
        max: i64,
        interval: i64,
        formatter: Arc<dyn ValueFormatter<i64>>,
    ) -> Arc<dyn Controller<i64>> {
        Arc::new(LongSliderController::new_5(option, min, max, interval, formatter))
    }
}

impl Controller<i64> for LongSliderController {
    /// java: @Override public Option<Long> option()
    fn option(&self) -> Arc<dyn YOption<i64>> {
        self.option.clone()
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        self.value_formatter.format(self.option.pending_value())
    }

    /// java: ISliderController.provideWidget (the interface default)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        i_slider_controller::provide_widget::<i64, Self>(self, screen, widget_dimension)
    }
}

impl ISliderController<i64> for LongSliderController {
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
        self.option.request_set(value as i64);
    }

    /// java: @Override public double pendingValue()
    fn pending_value(&self) -> f64 {
        self.option.pending_value() as f64
    }
}
