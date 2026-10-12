//! Ported from dev.isxander.yacl3.api.controller.ValueFormatter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface ValueFormatter<T>
pub trait ValueFormatter<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Component format(T value);
    fn format(&self, value: &T) -> MutableComponent;

}
