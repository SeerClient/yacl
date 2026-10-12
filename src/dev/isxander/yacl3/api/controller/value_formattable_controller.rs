//! Ported from dev.isxander.yacl3.api.controller.ValueFormattableController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface ValueFormattableController<T, B extends ValueFormattableController<T, B>> extends ControllerBuilder<T>
///
/// `B` is the builder's own type (Java's self type), so it is `Self` here; setters take the builder by value.
pub trait ValueFormattableController<T: Clone + 'static>: ControllerBuilder<T> + Sized {
    /// java: B formatValue(ValueFormatter<T> formatter);
    fn format_value(self, formatter: Arc<dyn ValueFormatter<T>>) -> Self;

    /// java: @Deprecated default B valueFormatter(Function<T, Component> formatter)
    fn value_formatter(self, formatter: impl Fn(T) -> MutableComponent + 'static) -> Self {
        self.format_value(Arc::new(formatter))
    }
}
