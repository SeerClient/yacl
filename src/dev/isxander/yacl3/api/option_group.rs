//! Ported from dev.isxander.yacl3.api.OptionGroup.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::list_option::AnyListOption;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::option_addable::OptionAddable;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::r#impl::option_group_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface OptionGroup
pub trait OptionGroup: compat::DynAny {
    /// java: Component name();
    fn name(&self) -> MutableComponent;

    /// java: OptionDescription description();
    fn description(&self) -> Arc<dyn OptionDescription>;

    /// java: @Deprecated Component tooltip();
    fn tooltip(&self) -> MutableComponent;

    /// java: @NotNull ImmutableList<? extends Option<?>> options();
    fn options(&self) -> Vec<Arc<dyn AnyOption>>;

    /// java: boolean collapsed();
    fn collapsed(&self) -> bool;

    /// java: boolean isRoot();
    fn is_root(&self) -> bool;

    /// `group instanceof ListOption<?> listOption ? listOption : null`.
    fn as_list_option(&self) -> std::option::Option<Arc<dyn AnyListOption>> {
        None
    }
}

/// java: static Builder createBuilder()
pub fn create_builder() -> BuilderImpl {
    BuilderImpl::new()
}

/// java: interface Builder extends OptionAddable
pub trait Builder: OptionAddable {
    /// java: Builder name(@NotNull Component name);
    fn name(self, name: MutableComponent) -> Self;

    /// java: Builder description(@NotNull OptionDescription description);
    fn description(self, description: Arc<dyn OptionDescription>) -> Self;

    /// java: Builder collapsed(boolean collapsible);
    fn collapsed(self, collapsible: bool) -> Self;

    /// java: OptionGroup build();
    fn build(self) -> Arc<dyn OptionGroup>;
}
