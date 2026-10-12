//! Ported from dev.isxander.yacl3.api.OptionGroup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option_addable::OptionAddable;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::r#impl::option_group_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface OptionGroup
pub trait OptionGroup: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Component name(); (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> MutableComponent;

    /// java: OptionDescription description(); (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn OptionDescription>;

    /// java: @Deprecated Component tooltip(); (dispatch: the body for this class)
    fn tooltip(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> MutableComponent;

    /// java: @NotNull ImmutableList<? extends Option<?>> options(); (dispatch: the body for this class)
    fn options(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>;

    /// java: boolean collapsed(); (dispatch: the body for this class)
    fn collapsed(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> bool;

    /// java: boolean isRoot(); (dispatch: the body for this class)
    fn is_root(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> bool;

}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (BuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder extends OptionAddable
pub trait Builder: OptionAddable {
    /// java: Builder name(@NotNull Component name); (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder description(@NotNull OptionDescription description); (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn Builder>, compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn Builder>;

    /// java: Builder collapsed(boolean collapsible); (dispatch: the body for this class)
    fn collapsed(&self) -> fn(&compat::Shared<dyn Builder>, bool) -> compat::Shared<dyn Builder>;

    /// java: OptionGroup build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn OptionGroup>;

}

impl dyn Builder {
    /// java: @Override default Builder option(@NotNull Supplier<@NotNull Option<?>> optionSupplier)
    pub fn option_supplier_body(__this: &compat::Shared<dyn Builder>, option_supplier: Arc<dyn Fn() -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static field OptionAddable.super
        //   java: OptionAddable.super.option(optionSupplier);
        return __this.clone();
    }

    /// Builder.option as the dispatch of OptionAddable.option for the concrete class `__C`
    pub fn option_supplier_from<__C: Builder + 'static>(__this: &compat::Shared<dyn OptionAddable>, __p0: Arc<dyn Fn() -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        <dyn Builder>::option_supplier_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn Builder>), __p0)
    }

    /// java: @Override default Builder optionIf(boolean condition, @NotNull Option<?> option)
    pub fn option_if_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static field OptionAddable.super
        //   java: OptionAddable.super.optionIf(condition, option);
        return __this.clone();
    }

    /// Builder.optionIf as the dispatch of OptionAddable.optionIf for the concrete class `__C`
    pub fn option_if_from<__C: Builder + 'static>(__this: &compat::Shared<dyn OptionAddable>, __p0: bool, __p1: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAddable> {
        <dyn Builder>::option_if_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn Builder>), __p0, __p1)
    }

    /// java: @Override default Builder optionIf(boolean condition, @NotNull Supplier<@NotNull Option<?>> optionSupplier)
    pub fn option_if_boolean_supplier_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, option_supplier: Arc<dyn Fn() -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static field OptionAddable.super
        //   java: OptionAddable.super.optionIf(condition, optionSupplier);
        return __this.clone();
    }

    /// Builder.optionIf as the dispatch of OptionAddable.optionIf for the concrete class `__C`
    pub fn option_if_boolean_supplier_from<__C: Builder + 'static>(__this: &compat::Shared<dyn OptionAddable>, __p0: bool, __p1: Arc<dyn Fn() -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        <dyn Builder>::option_if_boolean_supplier_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn Builder>), __p0, __p1)
    }

    /// java: @Override default Builder optionsIf(boolean condition, @NotNull Collection<? extends Option<?>> options)
    pub fn options_if_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, options: &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static field OptionAddable.super
        //   java: OptionAddable.super.optionsIf(condition, options);
        return __this.clone();
    }

    /// Builder.optionsIf as the dispatch of OptionAddable.optionsIf for the concrete class `__C`
    pub fn options_if_from<__C: Builder + 'static>(__this: &compat::Shared<dyn OptionAddable>, __p0: bool, __p1: &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        <dyn Builder>::options_if_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn Builder>), __p0, __p1)
    }

}
