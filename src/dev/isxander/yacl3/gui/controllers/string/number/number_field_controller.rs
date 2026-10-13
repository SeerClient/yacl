//! Ported from dev.isxander.yacl3.gui.controllers.string.number.NumberFieldController.
//!
//! `public abstract class NumberFieldController<T extends Number> implements ISliderController<T>, IStringController<T>`:
//! a subclass holds a `NumberFieldController<T>` (the class's fields) and implements `NumberFieldControllerClass`
//! (the abstract members it supplies); the class's bodies are the functions below, which each subclass's `Controller`
//! / `ISliderController` / `IStringController` impls call.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElement;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::util::mth;
use std::sync::LazyLock;

/// java: protected static final NumberFormat NUMBER_FORMAT = NumberFormat.getInstance();
pub static NUMBER_FORMAT: LazyLock<DecimalFormat> = LazyLock::new(compat::number_format_instance);

/// java: private static final DecimalFormatSymbols DECIMAL_FORMAT_SYMBOLS = DecimalFormatSymbols.getInstance();
/// - only its grouping separator is read.
fn decimal_format_symbols_grouping_separator() -> char {
    mcrust::java::text::decimal_format_symbols::get_instance(mcrust::java::text::decimal_format_symbols::Locale::Us).grouping_separator
}

/// java: the fields of `NumberFieldController<T extends Number>`
pub struct NumberFieldController<T: Clone + 'static> {
    /// java: private final Option<T> option;
    option: Arc<dyn YOption<T>>,
    /// java: private final ValueFormatter<T> displayFormatter;
    display_formatter: Arc<dyn ValueFormatter<T>>,
}

impl<T: Clone + 'static> NumberFieldController<T> {
    /// java: public NumberFieldController(Option<T> option, Function<T, Component> displayFormatter)
    pub fn new(option: Arc<dyn YOption<T>>, display_formatter: Arc<dyn ValueFormatter<T>>) -> Self {
        Self { option, display_formatter }
    }

    /// java: @Override public Option<T> option()
    pub fn option(&self) -> Arc<dyn YOption<T>> {
        self.option.clone()
    }

    /// java: @Override public Component formatValue()
    pub fn format_value(&self) -> MutableComponent {
        self.display_formatter.format(self.option.pending_value())
    }

    /// java: @Override public double interval()
    pub fn interval(&self) -> f64 {
        -1.0
    }
}

/// java: @Override public void setFromString(String value)
pub fn set_from_string<T: Clone + 'static, C: ISliderController<T> + ?Sized>(this: &C, value: &str) {
    // try { .. } catch (ParseException ignore)
    let transformed = transform_input(value);
    match compat::number_format_parse(&transformed, 0) {
        (Some(number), _) => this.set_pending_value(mth::clamp_f64(
            number,
            <C as ISliderController<T>>::min(this),
            <C as ISliderController<T>>::max(this),
        )),
        (None, _) => LOGGER.warn(&format!("Failed to parse number: {value}")),
    }
}

/// java: @Override public boolean isInputValid(String input)
pub fn is_input_valid(input: &str) -> bool {
    let input = transform_input(input);
    let (_, index) = compat::number_format_parse(&input, 0);
    index == input.chars().count()
}

/// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
pub fn provide_widget<T: Clone + 'static, C: IStringController<T> + ?Sized + 'static>(
    this: Arc<C>,
    screen: &Arc<YACLScreen>,
    widget_dimension: Arc<dyn Dimension<i32>>,
) -> Arc<dyn AbstractWidget> {
    Arc::new(StringControllerElement::new(this, screen, widget_dimension, false))
}

/// java: protected String transformInput(String input)
pub fn transform_input(input: &str) -> String {
    let mut input = input.to_string();
    if input.is_empty() {
        input = "0".to_string();
    }
    if input == "-" {
        input = "-0".to_string();
    }
    input.replace(decimal_format_symbols_grouping_separator(), "")
}
