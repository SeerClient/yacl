//! Ported from dev.isxander.yacl3.impl.PlaceholderCategoryImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::placeholder_category::{Builder, PlaceholderCategory, ScreenFunction};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::client::minecraft::Minecraft;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: @ApiStatus.Internal public record PlaceholderCategoryImpl(Component name, BiFunction<Minecraft, YACLScreen, Screen> screen,
/// Component tooltip) implements PlaceholderCategory
pub struct PlaceholderCategoryImpl {
    pub name: MutableComponent,
    pub screen: ScreenFunction,
    pub tooltip: MutableComponent,
}

impl PlaceholderCategoryImpl {
    /// java: the canonical record constructor
    pub fn new(name: MutableComponent, screen: ScreenFunction, tooltip: MutableComponent) -> Self {
        Self { name, screen, tooltip }
    }
}

impl ConfigCategory for PlaceholderCategoryImpl {
    /// java: @Override public @NotNull Component name()
    fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: @Override public @NotNull ImmutableList<OptionGroup> groups()
    fn groups(&self) -> Vec<Arc<dyn OptionGroup>> {
        Vec::new()
    }

    /// java: @Override public @NotNull Component tooltip()
    fn tooltip(&self) -> MutableComponent {
        self.tooltip.clone()
    }

    fn as_placeholder_category(&self) -> Option<&dyn PlaceholderCategory> {
        Some(self)
    }
}

impl PlaceholderCategory for PlaceholderCategoryImpl {
    /// java: the record accessor `screen()`
    fn screen(&self) -> ScreenFunction {
        self.screen.clone()
    }
}

/// java: @ApiStatus.Internal public static final class BuilderImpl implements Builder
pub struct BuilderImpl {
    /// java: private Component name;
    pub name: Option<MutableComponent>,
    /// java: private final List<Component> tooltipLines = new ArrayList<>();
    pub tooltip_lines: Vec<MutableComponent>,
    /// java: private BiFunction<Minecraft, YACLScreen, Screen> screenFunction;
    pub screen_function: Option<ScreenFunction>,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { name: None, tooltip_lines: Vec::new(), screen_function: None }
    }
}

impl Default for BuilderImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder for BuilderImpl {
    /// java: @Override public Builder name(@NotNull Component name)
    fn name(mut self, name: MutableComponent) -> Self {
        self.name = Some(name);
        self
    }

    /// java: @Override public Builder tooltip(@NotNull Component... tooltips)
    fn tooltip(mut self, tooltips: Vec<MutableComponent>) -> Self {
        assert!(!tooltips.is_empty(), "`tooltips` cannot be empty");
        self.tooltip_lines.extend(tooltips);
        self
    }

    /// java: @Override public Builder screen(@NotNull BiFunction<Minecraft, YACLScreen, Screen> screenFunction)
    fn screen(mut self, screen_function: impl Fn(&mut Minecraft, &Arc<YACLScreen>) -> compat::ScreenRef + 'static) -> Self {
        self.screen_function = Some(Arc::new(screen_function));
        self
    }

    /// java: @Override public PlaceholderCategory build()
    fn build(self) -> Arc<dyn PlaceholderCategory> {
        let name = self.name.expect("`name` must not be null to build `ConfigCategory`");
        let mut concatenated_tooltip = component::empty();
        let mut first = true;
        for line in self.tooltip_lines {
            if !first {
                concatenated_tooltip = concatenated_tooltip.append_str("\n");
            }
            first = false;
            concatenated_tooltip = concatenated_tooltip.append(line);
        }
        // java passes the (possibly null) screen function on; a placeholder tab with none cannot open anything
        let screen = self.screen_function.unwrap_or_else(|| Arc::new(|_client: &mut Minecraft, _screen: &Arc<YACLScreen>| compat::ScreenRef::Null));
        Arc::new(PlaceholderCategoryImpl::new(name, screen, concatenated_tooltip))
    }
}
