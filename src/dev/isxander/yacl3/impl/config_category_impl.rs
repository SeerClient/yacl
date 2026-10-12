//! Ported from dev.isxander.yacl3.impl.ConfigCategoryImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::{Builder, ConfigCategory};
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::option_addable::OptionAddable;
use crate::dev::isxander::yacl3::api::option_description::{self, OptionDescription};
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::r#impl::option_group_impl::OptionGroupImpl;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::YACLConstants;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: @ApiStatus.Internal public record ConfigCategoryImpl(Component name, ImmutableList<OptionGroup> groups, Component tooltip) implements ConfigCategory
pub struct ConfigCategoryImpl {
    pub name: MutableComponent,
    pub groups: Vec<Arc<dyn OptionGroup>>,
    pub tooltip: MutableComponent,
}

impl ConfigCategoryImpl {
    /// java: the canonical record constructor
    pub fn new(name: MutableComponent, groups: Vec<Arc<dyn OptionGroup>>, tooltip: MutableComponent) -> Self {
        Self { name, groups, tooltip }
    }
}

impl ConfigCategory for ConfigCategoryImpl {
    /// java: @Override public @NotNull Component name()
    fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: @Override public @NotNull ImmutableList<OptionGroup> groups()
    fn groups(&self) -> Vec<Arc<dyn OptionGroup>> {
        self.groups.clone()
    }

    /// java: @Override public @NotNull Component tooltip()
    fn tooltip(&self) -> MutableComponent {
        self.tooltip.clone()
    }
}

/// java: @ApiStatus.Internal public static final class BuilderImpl implements Builder
pub struct BuilderImpl {
    /// java: private Component name;
    pub name: Option<MutableComponent>,
    /// java: private final List<Option<?>> rootOptions = new ArrayList<>();
    pub root_options: Vec<Arc<dyn AnyOption>>,
    /// java: private final List<OptionGroup> groups = new ArrayList<>();
    pub groups: Vec<Arc<dyn OptionGroup>>,
    /// java: private final List<Component> tooltipLines = new ArrayList<>();
    pub tooltip_lines: Vec<MutableComponent>,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { name: None, root_options: Vec::new(), groups: Vec::new(), tooltip_lines: Vec::new() }
    }

    /// `option(option)`'s body on the builder in place (the root group builder reaches it through a borrow).
    fn add_option(&mut self, option: Arc<dyn AnyOption>) {
        if let Some(list_option) = option.as_list_option() {
            YACLConstants::logger().warn("Adding list option as an option is not supported! Rerouting to group!");
            let group: Arc<dyn OptionGroup> = list_option;
            self.groups.push(group);
            return;
        }
        self.root_options.push(option);
    }

    /// `options(options)`'s body on the builder in place.
    fn add_options(&mut self, options: Vec<Arc<dyn AnyOption>>) {
        if options.iter().any(|o| o.as_list_option().is_some()) {
            panic!("UnsupportedOperationException: List options must not be added as an option but a group!");
        }
        self.root_options.extend(options);
    }
}

impl Default for BuilderImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl OptionAddable for BuilderImpl {
    /// java: @Override public Builder option(@NotNull Option<?> option)
    fn option(mut self, option: Arc<dyn AnyOption>) -> Self {
        self.add_option(option);
        self
    }

    /// java: @Override public Builder options(@NotNull Collection<? extends Option<?>> options)
    fn options(mut self, options: Vec<Arc<dyn AnyOption>>) -> Self {
        self.add_options(options);
        self
    }
}

impl Builder for BuilderImpl {
    /// java: @Override public Builder name(@NotNull Component name)
    fn name(mut self, name: MutableComponent) -> Self {
        self.name = Some(name);
        self
    }

    /// java: @Override public Builder group(@NotNull OptionGroup group)
    fn group(mut self, group: Arc<dyn OptionGroup>) -> Self {
        self.groups.push(group);
        self
    }

    /// java: @Override public Builder groups(@NotNull Collection<OptionGroup> groups)
    fn groups(mut self, groups: Vec<Arc<dyn OptionGroup>>) -> Self {
        assert!(!groups.is_empty(), "`groups` must not be empty");
        self.groups.extend(groups);
        self
    }

    /// java: @Override public Builder tooltip(@NotNull Component... tooltips)
    fn tooltip(mut self, tooltips: Vec<MutableComponent>) -> Self {
        assert!(!tooltips.is_empty(), "`tooltips` cannot be empty");
        self.tooltip_lines.extend(tooltips);
        self
    }

    /// java: @Override public OptionGroup.Builder rootGroupBuilder()
    fn root_group_builder(&mut self) -> RootGroupBuilder<'_> {
        RootGroupBuilder { outer: self }
    }

    /// java: @Override public ConfigCategory build()
    fn build(self) -> Arc<dyn ConfigCategory> {
        let name = self.name.expect("`name` must not be null to build `ConfigCategory`");
        let mut combined_groups: Vec<Arc<dyn OptionGroup>> = Vec::new();
        combined_groups.push(Arc::new(OptionGroupImpl::new(component::empty(), option_description::EMPTY(), self.root_options, false, true)));
        combined_groups.extend(self.groups);
        assert!(!combined_groups.is_empty(), "at least one option must be added to build `ConfigCategory`");
        let mut concatenated_tooltip = component::empty();
        let mut first = true;
        let empty_contents = component::empty();
        for line in self.tooltip_lines {
            if line.get_contents() == empty_contents.get_contents() {
                continue;
            }
            if !first {
                concatenated_tooltip = concatenated_tooltip.append_str("\n");
            }
            first = false;
            concatenated_tooltip = concatenated_tooltip.append(line);
        }
        Arc::new(ConfigCategoryImpl::new(name, combined_groups, concatenated_tooltip))
    }
}

/// java: private class RootGroupBuilder implements OptionGroup.Builder (an inner class over `BuilderImpl.this`)
pub struct RootGroupBuilder<'a> {
    outer: &'a mut BuilderImpl,
}

impl OptionAddable for RootGroupBuilder<'_> {
    /// java: @Override public OptionGroup.Builder option(@NotNull Option<?> option)
    fn option(self, option: Arc<dyn AnyOption>) -> Self {
        self.outer.add_option(option);
        self
    }

    /// java: @Override public OptionGroup.Builder options(@NotNull Collection<? extends Option<?>> options)
    fn options(self, options: Vec<Arc<dyn AnyOption>>) -> Self {
        self.outer.add_options(options);
        self
    }
}

impl crate::dev::isxander::yacl3::api::option_group::Builder for RootGroupBuilder<'_> {
    /// java: @Override public OptionGroup.Builder name(@NotNull Component name)
    fn name(self, _name: MutableComponent) -> Self {
        panic!("UnsupportedOperationException: Cannot set name of root group!");
    }

    /// java: @Override public OptionGroup.Builder description(@NotNull OptionDescription description)
    fn description(self, _description: Arc<dyn OptionDescription>) -> Self {
        panic!("UnsupportedOperationException: Cannot set name of root group!");
    }

    /// java: @Override public OptionGroup.Builder collapsed(boolean collapsible)
    fn collapsed(self, _collapsible: bool) -> Self {
        panic!("UnsupportedOperationException: Cannot set collapsible of root group!");
    }

    /// java: @Override public OptionGroup build()
    fn build(self) -> Arc<dyn OptionGroup> {
        panic!("UnsupportedOperationException: Cannot build root group!");
    }
}
