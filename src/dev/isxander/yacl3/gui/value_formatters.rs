//! Ported from dev.isxander.yacl3.gui.ValueFormatters.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public final class ValueFormatters
pub struct ValueFormatters;

impl ValueFormatters {
    /// java: public static ValueFormatter<Float> percent(int decimalPlaces)
    pub fn percent(decimal_places: i32) -> Arc<dyn ValueFormatter<f32>> {
        Arc::new(PercentFormatter::new_canonical(decimal_places))
    }
}

/// java: public record PercentFormatter(int decimalPlaces) implements ValueFormatter<Float>
#[derive(Clone, Debug, PartialEq)]
pub struct PercentFormatter {
    /// java: int decimalPlaces
    pub decimal_places: i32,
}

impl PercentFormatter {
    /// java: public PercentFormatter() - `this(1)`
    pub fn new() -> Self {
        Self::new_canonical(1)
    }

    /// java: the canonical record constructor
    pub fn new_canonical(decimal_places: i32) -> Self {
        Self { decimal_places }
    }

    /// java: the record accessor `decimalPlaces()`
    pub fn decimal_places(&self) -> i32 {
        self.decimal_places
    }
}

impl Default for PercentFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl ValueFormatter<f32> for PercentFormatter {
    /// java: @Override public Component format(Float value)
    ///
    /// `value * 100` is float arithmetic; `Formatter` prints a `Float` through `(double) value`.
    fn format(&self, value: f32) -> MutableComponent {
        let format = format!("%.{}f%%", self.decimal_places);
        component::literal(&compat::string_format(&format, &[compat::FormatArg::Float((value * 100.0) as f64)]))
    }
}
