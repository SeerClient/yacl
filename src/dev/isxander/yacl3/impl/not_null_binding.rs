//! Ported from dev.isxander.yacl3.impl.NotNullBinding.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;

/// java: public class NotNullBinding<T> implements Binding<T>
///
/// `Validate.notNull(..)` guards a Java reference; a Rust value of `T` is never null, so every check holds and the
/// class forwards to the binding it wraps.
pub struct NotNullBinding<T: Clone + 'static> {
    /// java: private final Binding<T> binding;
    pub binding: Arc<dyn Binding<T>>,
}

impl<T: Clone + 'static> NotNullBinding<T> {
    /// java: public NotNullBinding(Binding<T> binding)
    pub fn new(binding: Arc<dyn Binding<T>>) -> Self {
        Self { binding }
    }
}

impl<T: Clone + 'static> Binding<T> for NotNullBinding<T> {
    /// java: @Override public @NotNull T getValue()
    fn get_value(&self) -> T {
        self.binding.get_value()
    }

    /// java: @Override public void setValue(@NotNull T value)
    fn set_value(&self, value: T) {
        self.binding.set_value(value);
    }

    /// java: @Override public @NotNull T defaultValue()
    fn default_value(&self) -> T {
        self.binding.default_value()
    }
}
