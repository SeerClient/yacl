//! Ported from dev.isxander.yacl3.impl.OptionGroupImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::option_addable::OptionAddable;
use crate::dev::isxander::yacl3::api::option_description::{self, OptionDescription};
use crate::dev::isxander::yacl3::api::option_group::{Builder, OptionGroup};
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: @ApiStatus.Internal public record OptionGroupImpl(@NotNull Component name, @NotNull OptionDescription description,
/// ImmutableList<? extends Option<?>> options, boolean collapsed, boolean isRoot) implements OptionGroup
pub struct OptionGroupImpl {
    pub name: MutableComponent,
    pub description: Arc<dyn OptionDescription>,
    pub options: Vec<Arc<dyn AnyOption>>,
    pub collapsed: bool,
    pub is_root: bool,
}

impl OptionGroupImpl {
    /// java: the canonical record constructor
    pub fn new(name: MutableComponent, description: Arc<dyn OptionDescription>, options: Vec<Arc<dyn AnyOption>>, collapsed: bool, is_root: bool) -> Self {
        Self { name, description, options, collapsed, is_root }
    }
}

impl OptionGroup for OptionGroupImpl {
    /// java: the record accessor `name()`
    fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: @Override public OptionDescription description()
    fn description(&self) -> Arc<dyn OptionDescription> {
        self.description.clone()
    }

    /// java: @Override public @NotNull Component tooltip()
    fn tooltip(&self) -> MutableComponent {
        self.description.text()
    }

    /// java: @Override public @NotNull ImmutableList<? extends Option<?>> options()
    fn options(&self) -> Vec<Arc<dyn AnyOption>> {
        self.options.clone()
    }

    /// java: the record accessor `collapsed()`
    fn collapsed(&self) -> bool {
        self.collapsed
    }

    /// java: the record accessor `isRoot()`
    fn is_root(&self) -> bool {
        self.is_root
    }
}

/// java: @ApiStatus.Internal public static final class BuilderImpl implements Builder
pub struct BuilderImpl {
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private OptionDescription description = OptionDescription.EMPTY;
    pub description: Arc<dyn OptionDescription>,
    /// java: private final List<Option<?>> options = new ArrayList<>();
    pub options: Vec<Arc<dyn AnyOption>>,
    /// java: private boolean collapsed = false;
    pub collapsed: bool,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { name: component::empty(), description: option_description::EMPTY(), options: Vec::new(), collapsed: false }
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
        if option.as_list_option().is_some() {
            panic!("UnsupportedOperationException: List options must not be added as an option but a group!");
        }
        self.options.push(option);
        self
    }

    /// java: @Override public Builder options(@NotNull Collection<? extends Option<?>> options)
    fn options(mut self, options: Vec<Arc<dyn AnyOption>>) -> Self {
        assert!(!options.is_empty(), "`options` must not be empty");
        if options.iter().any(|o| o.as_list_option().is_some()) {
            panic!("UnsupportedOperationException: List options must not be added as an option but a group!");
        }
        self.options.extend(options);
        self
    }
}

impl Builder for BuilderImpl {
    /// java: @Override public Builder name(@NotNull Component name)
    fn name(mut self, name: MutableComponent) -> Self {
        self.name = name;
        self
    }

    /// java: @Override public Builder description(@NotNull OptionDescription description)
    fn description(mut self, description: Arc<dyn OptionDescription>) -> Self {
        self.description = description;
        self
    }

    /// java: @Override public Builder collapsed(boolean collapsible)
    fn collapsed(mut self, collapsible: bool) -> Self {
        self.collapsed = collapsible;
        self
    }

    /// java: @Override public OptionGroup build()
    fn build(self) -> Arc<dyn OptionGroup> {
        assert!(!self.options.is_empty(), "`options` must not be empty to build `OptionGroup`");
        Arc::new(OptionGroupImpl::new(self.name, self.description, self.options, self.collapsed, false))
    }
}
