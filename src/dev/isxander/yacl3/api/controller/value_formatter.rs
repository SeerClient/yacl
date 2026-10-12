//! Ported from dev.isxander.yacl3.api.controller.ValueFormatter.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface ValueFormatter<T> (any `Fn(T) -> Component` is one)
pub trait ValueFormatter<T: Clone + 'static> {
    /// java: Component format(T value);
    fn format(&self, value: T) -> MutableComponent;
}

impl<T: Clone + 'static, F: Fn(T) -> MutableComponent> ValueFormatter<T> for F {
    fn format(&self, value: T) -> MutableComponent {
        self(value)
    }
}
