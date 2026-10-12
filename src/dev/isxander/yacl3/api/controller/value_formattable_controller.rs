//! Ported from dev.isxander.yacl3.api.controller.ValueFormattableController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface ValueFormattableController<T, B extends ValueFormattableController<T, B>> extends ControllerBuilder<T>
pub trait ValueFormattableController<T: Clone + 'static, B: Clone + 'static>: ControllerBuilder {
    /// java: B formatValue(ValueFormatter<T> formatter);
    fn format_value(&mut self, formatter: compat::Shared<dyn ValueFormatter<T>>) -> B;

    /// java: @Deprecated default B valueFormatter(Function<T, Component> formatter)
    fn value_formatter(&self, formatter: Arc<dyn Fn(T) -> MutableComponent>) -> B {
        return self.format_value(Arc::new(move |__a0| (formatter)(__a0.clone())));
    }

}
