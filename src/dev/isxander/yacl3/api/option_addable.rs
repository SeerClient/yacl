//! Ported from dev.isxander.yacl3.api.OptionAddable by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface OptionAddable
pub trait OptionAddable: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: OptionAddable option(@NotNull Option<?> option); (dispatch: the body for this class)
    fn option(&self) -> fn(&compat::Shared<dyn OptionAddable>, &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAddable>;

    /// java: default OptionAddable option(@NotNull Supplier<@NotNull Option<?>> optionSupplier) (dispatch: the body for this class)
    fn option_supplier(&self) -> fn(&compat::Shared<dyn OptionAddable>, Arc<dyn Fn() -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        <dyn OptionAddable>::option_supplier_body
    }

    /// java: default OptionAddable optionIf(boolean condition, @NotNull Option<?> option) (dispatch: the body for this class)
    fn option_if(&self) -> fn(&compat::Shared<dyn OptionAddable>, bool, &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAddable> {
        <dyn OptionAddable>::option_if_body
    }

    /// java: default OptionAddable optionIf(boolean condition, @NotNull Supplier<@NotNull Option<?>> optionSupplier) (dispatch: the body for this class)
    fn option_if_boolean_supplier(&self) -> fn(&compat::Shared<dyn OptionAddable>, bool, Arc<dyn Fn() -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        <dyn OptionAddable>::option_if_boolean_supplier_body
    }

    /// java: OptionAddable options(@NotNull Collection<? extends Option<?>> options); (dispatch: the body for this class)
    fn options(&self) -> fn(&compat::Shared<dyn OptionAddable>, &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable>;

    /// java: default OptionAddable optionsIf(boolean condition, @NotNull Collection<? extends Option<?>> options) (dispatch: the body for this class)
    fn options_if(&self) -> fn(&compat::Shared<dyn OptionAddable>, bool, &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        <dyn OptionAddable>::options_if_body
    }

}

impl dyn OptionAddable {
    /// java: default OptionAddable option(@NotNull Supplier<@NotNull Option<?>> optionSupplier)
    pub fn option_supplier_body(__this: &compat::Shared<dyn OptionAddable>, option_supplier: Arc<dyn Fn() -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        return ({ let __f = compat::dispatch(__this, |__t| __t.option()); __f(__this, &(option_supplier)()) });
    }

    /// java: default OptionAddable optionIf(boolean condition, @NotNull Option<?> option)
    pub fn option_if_body(__this: &compat::Shared<dyn OptionAddable>, mut condition: bool, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAddable> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.option()); __f(__this, option) }) } else { __this.clone() });
    }

    /// java: default OptionAddable optionIf(boolean condition, @NotNull Supplier<@NotNull Option<?>> optionSupplier)
    pub fn option_if_boolean_supplier_body(__this: &compat::Shared<dyn OptionAddable>, mut condition: bool, option_supplier: Arc<dyn Fn() -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.option_supplier()); __f(__this, option_supplier.clone()) }) } else { __this.clone() });
    }

    /// java: default OptionAddable optionsIf(boolean condition, @NotNull Collection<? extends Option<?>> options)
    pub fn options_if_body(__this: &compat::Shared<dyn OptionAddable>, mut condition: bool, options: &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.options()); __f(__this, options) }) } else { __this.clone() });
    }

}
