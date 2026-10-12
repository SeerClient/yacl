//! Ported from dev.isxander.yacl3.impl.NotNullBinding by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;

/// java: public class NotNullBinding<T> implements Binding<T>
#[derive(Clone)]
pub struct NotNullBinding<T> {
    /// java: private final Binding<T> binding;
    pub binding: compat::Shared<dyn Binding<T>>,
}

impl<T: Clone + 'static> NotNullBinding<T> {
    /// java: public NotNullBinding(Binding<T> binding)
    pub fn new(mut binding: compat::Shared<dyn Binding<T>>) -> Self {
        let __init_binding = binding.clone();
        let mut this = Self {
            binding: __init_binding,
        };
        this
    }

}

impl<T: Clone + 'static> Binding<T> for NotNullBinding<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @NotNull T getValue()
    fn get_value(&self) -> T {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: return Validate.notNull(binding.getValue(), "Binding's value must not be null, please use Optionals if you want null behaviour.");
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void setValue(@NotNull T value)
    fn set_value(&mut self, mut value: T) {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(value, "Binding's value must not be set to null, please use Optionals if you want null behaviour.");
        { let (__a0,) = (value.clone(),); self.binding.lock().borrow_mut().set_value(__a0) };
    }

    /// java: @Override public @NotNull T defaultValue()
    fn default_value(&self) -> T {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: return Validate.notNull(binding.defaultValue(), "Binding's default value must not be null, please use Optionals if you want null behaviour.");
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
