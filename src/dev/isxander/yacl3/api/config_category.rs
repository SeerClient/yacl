//! Ported from dev.isxander.yacl3.api.ConfigCategory.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::custom_tab_provider::CustomTabProvider;
use crate::dev::isxander::yacl3::api::option_addable::OptionAddable;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::placeholder_category::PlaceholderCategory;
use crate::dev::isxander::yacl3::r#impl::config_category_impl::{BuilderImpl, RootGroupBuilder};
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface ConfigCategory
pub trait ConfigCategory: compat::DynAny {
    /// java: @NotNull Component name();
    fn name(&self) -> MutableComponent;

    /// java: @NotNull ImmutableList<OptionGroup> groups();
    fn groups(&self) -> Vec<Arc<dyn OptionGroup>>;

    /// java: @NotNull Component tooltip();
    fn tooltip(&self) -> MutableComponent;

    /// `category instanceof CustomTabProvider tabProvider ? tabProvider : null` (YACLScreen.init)
    fn as_custom_tab_provider(&self) -> Option<&dyn CustomTabProvider> {
        None
    }

    /// `category instanceof PlaceholderCategory placeholder ? placeholder : null` (YACLScreen.init)
    fn as_placeholder_category(&self) -> Option<&dyn PlaceholderCategory> {
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

    /// java: Builder group(@NotNull OptionGroup group);
    fn group(self, group: Arc<dyn OptionGroup>) -> Self;

    /// java: default Builder group(@NotNull Supplier<@NotNull OptionGroup> groupSupplier)
    fn group_supplier(self, group_supplier: impl FnOnce() -> Arc<dyn OptionGroup>) -> Self {
        self.group(group_supplier())
    }

    /// java: default Builder groupIf(boolean condition, @NotNull OptionGroup group)
    fn group_if(self, condition: bool, group: Arc<dyn OptionGroup>) -> Self {
        if condition { self.group(group) } else { self }
    }

    /// java: default Builder groupIf(boolean condition, @NotNull Supplier<@NotNull OptionGroup> groupSupplier)
    fn group_if_boolean_supplier(self, condition: bool, group_supplier: impl FnOnce() -> Arc<dyn OptionGroup>) -> Self {
        if condition { self.group_supplier(group_supplier) } else { self }
    }

    /// java: Builder groups(@NotNull Collection<OptionGroup> groups);
    fn groups(self, groups: Vec<Arc<dyn OptionGroup>>) -> Self;

    /// java: default Builder groupsIf(boolean condition, @NotNull Collection<OptionGroup> groups)
    fn groups_if(self, condition: bool, groups: Vec<Arc<dyn OptionGroup>>) -> Self {
        if condition { self.groups(groups) } else { self }
    }

    /// java: OptionGroup.Builder rootGroupBuilder();
    /// The root group's builder adds to THIS builder (Java's inner class over `BuilderImpl.this`), so it borrows it.
    fn root_group_builder(&mut self) -> RootGroupBuilder<'_>;

    /// java: Builder tooltip(@NotNull Component... tooltips);
    fn tooltip(self, tooltips: Vec<MutableComponent>) -> Self;

    /// java: ConfigCategory build();
    fn build(self) -> Arc<dyn ConfigCategory>;
}
