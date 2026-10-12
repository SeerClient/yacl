//! Ported from dev.isxander.yacl3.api.OptionAddable.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::AnyOption;

/// java: public interface OptionAddable
///
/// Builders take themselves by value and answer themselves (Java's `return this`). An `Option<?>` argument is
/// `Arc<dyn AnyOption>`; a typed `Arc<dyn Option<T>>` coerces to it at the call.
pub trait OptionAddable: Sized {
    /// java: OptionAddable option(@NotNull Option<?> option);
    fn option(self, option: Arc<dyn AnyOption>) -> Self;

    /// java: default OptionAddable option(@NotNull Supplier<@NotNull Option<?>> optionSupplier)
    fn option_supplier(self, option_supplier: impl FnOnce() -> Arc<dyn AnyOption>) -> Self {
        self.option(option_supplier())
    }

    /// java: default OptionAddable optionIf(boolean condition, @NotNull Option<?> option)
    fn option_if(self, condition: bool, option: Arc<dyn AnyOption>) -> Self {
        if condition { self.option(option) } else { self }
    }

    /// java: default OptionAddable optionIf(boolean condition, @NotNull Supplier<@NotNull Option<?>> optionSupplier)
    fn option_if_boolean_supplier(self, condition: bool, option_supplier: impl FnOnce() -> Arc<dyn AnyOption>) -> Self {
        if condition { self.option_supplier(option_supplier) } else { self }
    }

    /// java: OptionAddable options(@NotNull Collection<? extends Option<?>> options);
    fn options(self, options: Vec<Arc<dyn AnyOption>>) -> Self;

    /// java: default OptionAddable optionsIf(boolean condition, @NotNull Collection<? extends Option<?>> options)
    fn options_if(self, condition: bool, options: Vec<Arc<dyn AnyOption>>) -> Self {
        if condition { self.options(options) } else { self }
    }
}
