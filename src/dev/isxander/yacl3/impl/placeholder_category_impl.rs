//! Ported from dev.isxander.yacl3.impl.PlaceholderCategoryImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::placeholder_category::Builder;
use crate::dev::isxander::yacl3::api::placeholder_category::PlaceholderCategory;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: @ApiStatus.Internal public record PlaceholderCategoryImpl(Component name, BiFunction<Minecraft, YACLScreen, Screen> screen, Component tooltip) implements PlaceholderCategory
#[derive(Clone, Debug)]
pub struct PlaceholderCategoryImpl {
    /// java: Component name
    pub name: MutableComponent,
    /// java: BiFunction<Minecraft, YACLScreen, Screen> screen
    pub screen: Arc<dyn Fn(mcrust::net::minecraft::client::minecraft::Minecraft, compat::Shared<YACLScreen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// java: Component tooltip
    pub tooltip: MutableComponent,
}

impl PlaceholderCategoryImpl {
    /// java: the canonical record constructor
    pub fn new(mut name: MutableComponent, mut screen: Arc<dyn Fn(mcrust::net::minecraft::client::minecraft::Minecraft, compat::Shared<YACLScreen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen>, mut tooltip: MutableComponent) -> Self {
        Self { name, screen, tooltip }
    }

    /// java: the record accessor `screen()`
    pub fn screen(&self) -> Arc<dyn Fn(mcrust::net::minecraft::client::minecraft::Minecraft, compat::Shared<YACLScreen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        self.screen.clone()
    }

}

impl PlaceholderCategory for PlaceholderCategoryImpl {
}

impl ConfigCategory for PlaceholderCategoryImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @NotNull ImmutableList<OptionGroup> groups() (dispatch: the body for this class)
    fn groups(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> Vec<compat::Shared<dyn OptionGroup>> {
        PlaceholderCategoryImpl::groups_dyn
    }

    /// java: @Override public @NotNull Component name() (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> MutableComponent {
        PlaceholderCategoryImpl::name_dyn
    }

    /// java: @Override public @NotNull Component tooltip() (dispatch: the body for this class)
    fn tooltip(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> MutableComponent {
        PlaceholderCategoryImpl::tooltip_dyn
    }

}

impl PlaceholderCategoryImpl {
    /// java: @Override public @NotNull ImmutableList<OptionGroup> groups()
    pub fn groups_impl(&self) -> Vec<compat::Shared<dyn OptionGroup>> {
        return vec![];
    }

    /// PlaceholderCategoryImpl.groups as the dispatch of ConfigCategory.groups
    pub fn groups_dyn(__this: &compat::Shared<dyn ConfigCategory>) -> Vec<compat::Shared<dyn OptionGroup>> {
        compat::read_as::<Self, _, _>(__this, |__s| __s.groups_impl())
    }

    /// java: @Override public @NotNull Component name()
    pub fn name_impl(&self) -> MutableComponent {
        return self.name.clone();
    }

    /// PlaceholderCategoryImpl.name as the dispatch of ConfigCategory.name
    pub fn name_dyn(__this: &compat::Shared<dyn ConfigCategory>) -> MutableComponent {
        compat::read_as::<Self, _, _>(__this, |__s| __s.name_impl())
    }

    /// java: @Override public @NotNull Component tooltip()
    pub fn tooltip_impl(&self) -> MutableComponent {
        return self.tooltip.clone();
    }

    /// PlaceholderCategoryImpl.tooltip as the dispatch of ConfigCategory.tooltip
    pub fn tooltip_dyn(__this: &compat::Shared<dyn ConfigCategory>) -> MutableComponent {
        compat::read_as::<Self, _, _>(__this, |__s| __s.tooltip_impl())
    }

}

/// java: @ApiStatus.Internal public static final class BuilderImpl implements Builder
#[derive(Clone)]
pub struct BuilderImpl {
    /// java: private Component name;
    pub name: Option<MutableComponent>,
    /// java: private final List<Component> tooltipLines = new ArrayList<>();
    pub tooltip_lines: Vec<MutableComponent>,
    /// java: private BiFunction<Minecraft, YACLScreen, Screen> screenFunction;
    pub screen_function: Option<Arc<dyn Fn(mcrust::net::minecraft::client::minecraft::Minecraft, compat::Shared<YACLScreen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen>>,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            name: None,
            tooltip_lines: Vec::new(),
            screen_function: None,
        });
        __this
    }

}

impl Builder for BuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder name(@NotNull Component name) (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        BuilderImpl::name_dyn
    }

    /// java: @Override public Builder tooltip(@NotNull Component... tooltips) (dispatch: the body for this class)
    fn tooltip(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        BuilderImpl::tooltip_dyn
    }

    /// java: @Override public Builder screen(@NotNull BiFunction<Minecraft, YACLScreen, Screen> screenFunction) (dispatch: the body for this class)
    fn screen(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn(mcrust::net::minecraft::client::minecraft::Minecraft, compat::Shared<YACLScreen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<dyn Builder> {
        BuilderImpl::screen_dyn
    }

    /// java: @Override public PlaceholderCategory build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn PlaceholderCategory> {
        BuilderImpl::build_dyn
    }

}

impl BuilderImpl {
    /// java: @Override public Builder name(@NotNull Component name)
    pub fn name_impl(__this: &compat::Shared<Self>, mut name: MutableComponent) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(name, "`name` cannot be null");
        { let __v = Some(name.clone()); compat::write(__this, |__t| __t.name = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.name as the dispatch of Builder.name
    pub fn name_dyn(__this: &compat::Shared<dyn Builder>, __p0: MutableComponent) -> compat::Shared<dyn Builder> {
        Self::name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder tooltip(@NotNull Component... tooltips)
    pub fn tooltip_impl(__this: &compat::Shared<Self>, tooltips: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notEmpty(..)
        //   java: Validate.notEmpty(tooltips, "`tooltips` cannot be empty");
        let __h0 = tooltips.to_vec().into_iter();
        __this.lock().borrow_mut().tooltip_lines.extend(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.tooltip as the dispatch of Builder.tooltip
    pub fn tooltip_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        Self::tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder screen(@NotNull BiFunction<Minecraft, YACLScreen, Screen> screenFunction)
    pub fn screen_impl(__this: &compat::Shared<Self>, screen_function: Arc<dyn Fn(mcrust::net::minecraft::client::minecraft::Minecraft, compat::Shared<YACLScreen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(screenFunction, "`screenFunction` cannot be null");
        { let __v = Some(screen_function.clone()); compat::write(__this, |__t| __t.screen_function = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.screen as the dispatch of Builder.screen
    pub fn screen_dyn(__this: &compat::Shared<dyn Builder>, __p0: Arc<dyn Fn(mcrust::net::minecraft::client::minecraft::Minecraft, compat::Shared<YACLScreen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<dyn Builder> {
        Self::screen_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public PlaceholderCategory build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn PlaceholderCategory> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(name, "`name` must not be null to build `ConfigCategory`");
        let mut concatenated_tooltip: MutableComponent = component::empty();
        let mut first: bool = true;
        for line in compat::read(__this, |__t| __t.tooltip_lines.clone()) {
            if !first {
                concatenated_tooltip = concatenated_tooltip.append_str("\n");
            }
            first = false;
            concatenated_tooltip = concatenated_tooltip.append(line.clone());
        }
        return (compat::shared(PlaceholderCategoryImpl::new(compat::read(__this, |__t| __t.name.clone()).unwrap(), compat::read(__this, |__t| __t.screen_function.clone()).unwrap(), concatenated_tooltip.clone())) as compat::Shared<dyn PlaceholderCategory>);
    }

    /// BuilderImpl.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn PlaceholderCategory> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
