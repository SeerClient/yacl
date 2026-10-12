//! Ported from dev.isxander.yacl3.gui.ValueFormatters by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public final class ValueFormatters
#[derive(Clone)]
pub struct ValueFormatters {}

impl ValueFormatters {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static ValueFormatter<Float> percent(int decimalPlaces)
    pub fn percent(mut decimal_places: i32) -> compat::Shared<dyn ValueFormatter<f32>> {
        return (compat::shared(PercentFormatter::new_canonical(decimal_places)) as compat::Shared<dyn ValueFormatter<compat::Object>>);
    }

}

/// java: public record PercentFormatter(int decimalPlaces) implements ValueFormatter<Float>
#[derive(Clone, Debug, PartialEq)]
pub struct PercentFormatter {
    /// java: int decimalPlaces
    pub decimal_places: i32,
}

impl PercentFormatter {
    /// java: public PercentFormatter()
    pub fn new() -> Self {
        let mut this = Self::new_canonical(1);
        this
    }

    /// java: the canonical record constructor
    pub fn new_canonical(mut decimal_places: i32) -> Self {
        Self { decimal_places }
    }

    /// java: the record accessor `decimalPlaces()`
    pub fn decimal_places(&self) -> i32 {
        self.decimal_places
    }

}

impl ValueFormatter<f32> for PercentFormatter {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Component format(Float value)
    fn format(&self, value: &f32) -> MutableComponent {
        // TODO(fabric2seer): String.format with a non-literal format
        //   java: return Component.literal(String.format("%." + decimalPlaces + "f%%", value * 100));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
