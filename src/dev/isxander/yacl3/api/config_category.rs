//! Ported from dev.isxander.yacl3.api.ConfigCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option_addable::OptionAddable;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::r#impl::config_category_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface ConfigCategory
pub trait ConfigCategory: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: @NotNull Component name(); (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> MutableComponent;

    /// java: @NotNull ImmutableList<OptionGroup> groups(); (dispatch: the body for this class)
    fn groups(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> Vec<compat::Shared<dyn OptionGroup>>;

    /// java: @NotNull Component tooltip(); (dispatch: the body for this class)
    fn tooltip(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> MutableComponent;

}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (BuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder extends OptionAddable
pub trait Builder: OptionAddable {
    /// java: Builder name(@NotNull Component name); (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder group(@NotNull OptionGroup group); (dispatch: the body for this class)
    fn group(&self) -> fn(&compat::Shared<dyn Builder>, compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn Builder>;

    /// java: default Builder group(@NotNull Supplier<@NotNull OptionGroup> groupSupplier) (dispatch: the body for this class)
    fn group_supplier(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn() -> compat::Shared<dyn OptionGroup>>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::group_supplier_body
    }

    /// java: default Builder groupIf(boolean condition, @NotNull OptionGroup group) (dispatch: the body for this class)
    fn group_if(&self) -> fn(&compat::Shared<dyn Builder>, bool, &compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::group_if_body
    }

    /// java: default Builder groupIf(boolean condition, @NotNull Supplier<@NotNull OptionGroup> groupSupplier) (dispatch: the body for this class)
    fn group_if_boolean_supplier(&self) -> fn(&compat::Shared<dyn Builder>, bool, Arc<dyn Fn() -> compat::Shared<dyn OptionGroup>>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::group_if_boolean_supplier_body
    }

    /// java: Builder groups(@NotNull Collection<OptionGroup> groups); (dispatch: the body for this class)
    fn groups(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<compat::Shared<dyn OptionGroup>>) -> compat::Shared<dyn Builder>;

    /// java: default Builder groupsIf(boolean condition, @NotNull Collection<OptionGroup> groups) (dispatch: the body for this class)
    fn groups_if(&self) -> fn(&compat::Shared<dyn Builder>, bool, &Vec<compat::Shared<dyn OptionGroup>>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::groups_if_body
    }

    /// java: OptionGroup.Builder rootGroupBuilder(); (dispatch: the body for this class)
    fn root_group_builder(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>;

    /// java: Builder tooltip(@NotNull Component... tooltips); (dispatch: the body for this class)
    fn tooltip(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder>;

    /// java: ConfigCategory build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn ConfigCategory>;

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

    /// java: default Builder group(@NotNull Supplier<@NotNull OptionGroup> groupSupplier)
    pub fn group_supplier_body(__this: &compat::Shared<dyn Builder>, group_supplier: Arc<dyn Fn() -> compat::Shared<dyn OptionGroup>>) -> compat::Shared<dyn Builder> {
        return ({ let __f = compat::dispatch(__this, |__t| __t.group()); __f(__this, (group_supplier)()) });
    }

    /// java: default Builder groupIf(boolean condition, @NotNull OptionGroup group)
    pub fn group_if_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, group: &compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn Builder> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.group()); __f(__this, group.clone()) }) } else { __this.clone() });
    }

    /// java: default Builder groupIf(boolean condition, @NotNull Supplier<@NotNull OptionGroup> groupSupplier)
    pub fn group_if_boolean_supplier_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, group_supplier: Arc<dyn Fn() -> compat::Shared<dyn OptionGroup>>) -> compat::Shared<dyn Builder> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.group_supplier()); __f(__this, group_supplier.clone()) }) } else { __this.clone() });
    }

    /// java: default Builder groupsIf(boolean condition, @NotNull Collection<OptionGroup> groups)
    pub fn groups_if_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, groups: &Vec<compat::Shared<dyn OptionGroup>>) -> compat::Shared<dyn Builder> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.groups()); __f(__this, groups) }) } else { __this.clone() });
    }

}
