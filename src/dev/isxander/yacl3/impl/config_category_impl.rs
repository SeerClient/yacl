//! Ported from dev.isxander.yacl3.impl.ConfigCategoryImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::Builder;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::api::list_option::ListOption;
use crate::dev::isxander::yacl3::api::option_addable::OptionAddable;
use crate::dev::isxander::yacl3::api::option_description;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::r#impl::option_group_impl::OptionGroupImpl;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: @ApiStatus.Internal public record ConfigCategoryImpl(Component name, ImmutableList<OptionGroup> groups, Component tooltip) implements ConfigCategory
#[derive(Clone, Debug)]
pub struct ConfigCategoryImpl {
    /// java: Component name
    pub name: MutableComponent,
    /// java: ImmutableList<OptionGroup> groups
    pub groups: Vec<compat::Shared<dyn OptionGroup>>,
    /// java: Component tooltip
    pub tooltip: MutableComponent,
}

impl ConfigCategoryImpl {
    /// java: the canonical record constructor
    pub fn new(mut name: MutableComponent, mut groups: Vec<compat::Shared<dyn OptionGroup>>, mut tooltip: MutableComponent) -> Self {
        Self { name, groups, tooltip }
    }

}

impl ConfigCategory for ConfigCategoryImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @NotNull Component name() (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> MutableComponent {
        ConfigCategoryImpl::name_dyn
    }

    /// java: @Override public @NotNull ImmutableList<OptionGroup> groups() (dispatch: the body for this class)
    fn groups(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> Vec<compat::Shared<dyn OptionGroup>> {
        ConfigCategoryImpl::groups_dyn
    }

    /// java: @Override public @NotNull Component tooltip() (dispatch: the body for this class)
    fn tooltip(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> MutableComponent {
        ConfigCategoryImpl::tooltip_dyn
    }

}

impl ConfigCategoryImpl {
    /// java: @Override public @NotNull Component name()
    pub fn name_impl(&self) -> MutableComponent {
        return self.name.clone();
    }

    /// ConfigCategoryImpl.name as the dispatch of ConfigCategory.name
    pub fn name_dyn(__this: &compat::Shared<dyn ConfigCategory>) -> MutableComponent {
        compat::read_as::<Self, _, _>(__this, |__s| __s.name_impl())
    }

    /// java: @Override public @NotNull ImmutableList<OptionGroup> groups()
    pub fn groups_impl(&self) -> Vec<compat::Shared<dyn OptionGroup>> {
        return self.groups.clone();
    }

    /// ConfigCategoryImpl.groups as the dispatch of ConfigCategory.groups
    pub fn groups_dyn(__this: &compat::Shared<dyn ConfigCategory>) -> Vec<compat::Shared<dyn OptionGroup>> {
        compat::read_as::<Self, _, _>(__this, |__s| __s.groups_impl())
    }

    /// java: @Override public @NotNull Component tooltip()
    pub fn tooltip_impl(&self) -> MutableComponent {
        return self.tooltip.clone();
    }

    /// ConfigCategoryImpl.tooltip as the dispatch of ConfigCategory.tooltip
    pub fn tooltip_dyn(__this: &compat::Shared<dyn ConfigCategory>) -> MutableComponent {
        compat::read_as::<Self, _, _>(__this, |__s| __s.tooltip_impl())
    }

}

/// java: @ApiStatus.Internal public static final class BuilderImpl implements Builder
#[derive(Clone)]
pub struct BuilderImpl {
    /// java: private Component name;
    pub name: Option<MutableComponent>,
    /// java: private final List<Option<?>> rootOptions = new ArrayList<>();
    pub root_options: Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>,
    /// java: private final RootGroupBuilder rootGroupBuilder = new RootGroupBuilder();
    pub root_group_builder: compat::Shared<RootGroupBuilder>,
    /// java: private final List<OptionGroup> groups = new ArrayList<>();
    pub groups: Vec<compat::Shared<dyn OptionGroup>>,
    /// java: private final List<Component> tooltipLines = new ArrayList<>();
    pub tooltip_lines: Vec<MutableComponent>,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            name: None,
            root_options: Vec::new(),
            root_group_builder: RootGroupBuilder::new(),
            groups: Vec::new(),
            tooltip_lines: Vec::new(),
        });
        __this
    }

}

impl Builder for BuilderImpl {
    /// java: @Override public Builder name(@NotNull Component name) (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        BuilderImpl::name_dyn
    }

    /// java: @Override public Builder group(@NotNull OptionGroup group) (dispatch: the body for this class)
    fn group(&self) -> fn(&compat::Shared<dyn Builder>, compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn Builder> {
        BuilderImpl::group_dyn
    }

    /// java: @Override public Builder groups(@NotNull Collection<OptionGroup> groups) (dispatch: the body for this class)
    fn groups(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<compat::Shared<dyn OptionGroup>>) -> compat::Shared<dyn Builder> {
        BuilderImpl::groups_dyn
    }

    /// java: @Override public Builder tooltip(@NotNull Component... tooltips) (dispatch: the body for this class)
    fn tooltip(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        BuilderImpl::tooltip_dyn
    }

    /// java: @Override public OptionGroup.Builder rootGroupBuilder() (dispatch: the body for this class)
    fn root_group_builder(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        BuilderImpl::root_group_builder_dyn
    }

    /// java: @Override public ConfigCategory build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn ConfigCategory> {
        BuilderImpl::build_dyn
    }

}

impl OptionAddable for BuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder option(@NotNull Option<?> option) (dispatch: the body for this class)
    fn option(&self) -> fn(&compat::Shared<dyn OptionAddable>, &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAddable> {
        BuilderImpl::option_dyn
    }

    /// java: @Override public Builder options(@NotNull Collection<? extends Option<?>> options) (dispatch: the body for this class)
    fn options(&self) -> fn(&compat::Shared<dyn OptionAddable>, &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        BuilderImpl::options_dyn
    }

    /// java: @Override default Builder option(@NotNull Supplier<@NotNull Option<?>> optionSupplier) (dispatch: the body for this class)
    fn option_supplier(&self) -> fn(&compat::Shared<dyn OptionAddable>, Arc<dyn Fn() -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        <dyn Builder>::option_supplier_from::<Self>
    }

    /// java: @Override default Builder optionIf(boolean condition, @NotNull Option<?> option) (dispatch: the body for this class)
    fn option_if(&self) -> fn(&compat::Shared<dyn OptionAddable>, bool, &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAddable> {
        <dyn Builder>::option_if_from::<Self>
    }

    /// java: @Override default Builder optionIf(boolean condition, @NotNull Supplier<@NotNull Option<?>> optionSupplier) (dispatch: the body for this class)
    fn option_if_boolean_supplier(&self) -> fn(&compat::Shared<dyn OptionAddable>, bool, Arc<dyn Fn() -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        <dyn Builder>::option_if_boolean_supplier_from::<Self>
    }

    /// java: @Override default Builder optionsIf(boolean condition, @NotNull Collection<? extends Option<?>> options) (dispatch: the body for this class)
    fn options_if(&self) -> fn(&compat::Shared<dyn OptionAddable>, bool, &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        <dyn Builder>::options_if_from::<Self>
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

    /// java: @Override public Builder group(@NotNull OptionGroup group)
    pub fn group_impl(__this: &compat::Shared<Self>, mut group: compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(group, "`group` must not be null");
        let __h0 = group.clone();
        __this.lock().borrow_mut().groups.push(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.group as the dispatch of Builder.group
    pub fn group_dyn(__this: &compat::Shared<dyn Builder>, __p0: compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn Builder> {
        Self::group_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder groups(@NotNull Collection<OptionGroup> groups)
    pub fn groups_impl(__this: &compat::Shared<Self>, groups: &Vec<compat::Shared<dyn OptionGroup>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notEmpty(..)
        //   java: Validate.notEmpty(groups, "`groups` must not be empty");
        let __h0 = groups.iter().cloned();
        __this.lock().borrow_mut().groups.extend(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.groups as the dispatch of Builder.groups
    pub fn groups_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<compat::Shared<dyn OptionGroup>>) -> compat::Shared<dyn Builder> {
        Self::groups_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
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

    /// java: @Override public OptionGroup.Builder rootGroupBuilder()
    pub fn root_group_builder_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        return (compat::read(__this, |__t| __t.root_group_builder.clone()) as compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>);
    }

    /// BuilderImpl.rootGroupBuilder as the dispatch of Builder.rootGroupBuilder
    pub fn root_group_builder_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        Self::root_group_builder_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public ConfigCategory build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn ConfigCategory> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(name, "`name` must not be null to build `ConfigCategory`");
        let mut combined_groups: Vec<compat::Shared<dyn OptionGroup>> = Vec::new();
        combined_groups.push((compat::shared(OptionGroupImpl::new(mcrust::net::minecraft::network::chat::common_components::empty(), option_description::EMPTY.get_mut().clone(), __this.lock().borrow().root_options.iter().cloned().collect::<Vec<_>>(), false, true)) as compat::Shared<dyn OptionGroup>));
        combined_groups.extend(compat::read(__this, |__t| __t.groups.clone()).into_iter());
        // TODO(fabric2seer): static call Validate.notEmpty(..)
        //   java: Validate.notEmpty(combinedGroups, "at least one option must be added to build `ConfigCategory`");
        let mut concatenated_tooltip: MutableComponent = component::empty();
        let mut first: bool = true;
        for line in compat::read(__this, |__t| __t.tooltip_lines.clone()) {
            if *line.get_contents() == *mcrust::net::minecraft::network::chat::common_components::empty().get_contents() {
                continue;
            }
            if !first {
                concatenated_tooltip = concatenated_tooltip.append_str("\n");
            }
            first = false;
            concatenated_tooltip = concatenated_tooltip.append(line.clone());
        }
        return (compat::shared(ConfigCategoryImpl::new(compat::read(__this, |__t| __t.name.clone()).unwrap(), combined_groups.iter().cloned().collect::<Vec<_>>(), concatenated_tooltip.clone())) as compat::Shared<dyn ConfigCategory>);
    }

    /// BuilderImpl.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn ConfigCategory> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Builder option(@NotNull Option<?> option)
    pub fn option_impl(__this: &compat::Shared<Self>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(option, "`option` must not be null");
        if let Some(list_option) = option.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn ListOption<compat::Object>>>() {
            logger().warn("Adding list option as an option is not supported! Rerouting to group!");
            return ({ let __f = compat::dispatch(__this, |__t| __t.group()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Builder>), (list_option.clone() as compat::Shared<dyn OptionGroup>)) });
        }
        let __h0 = option.clone();
        __this.lock().borrow_mut().root_options.push(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.option as the dispatch of OptionAddable.option
    pub fn option_dyn(__this: &compat::Shared<dyn OptionAddable>, __p0: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAddable> {
        Self::option_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder options(@NotNull Collection<? extends Option<?>> options)
    pub fn options_impl(__this: &compat::Shared<Self>, options: &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(options, "`options` must not be null");
        // TODO(fabric2seer): the condition: `.isInstance(..)` on a Class
        //   java: options.stream(..).anyMatch(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            panic!("UnsupportedOperationException: {}", "List options must not be added as an option but a group!");
        }
        let __h0 = options.iter().cloned();
        __this.lock().borrow_mut().root_options.extend(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.options as the dispatch of OptionAddable.options
    pub fn options_dyn(__this: &compat::Shared<dyn OptionAddable>, __p0: &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        Self::options_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

/// java: private class RootGroupBuilder implements OptionGroup.Builder
#[derive(Clone)]
pub struct RootGroupBuilder {}

impl RootGroupBuilder {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

}

impl crate::dev::isxander::yacl3::api::option_group::Builder for RootGroupBuilder {
    /// java: @Override public OptionGroup.Builder name(@NotNull Component name) (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>, MutableComponent) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        RootGroupBuilder::name_dyn
    }

    /// java: @Override public OptionGroup.Builder description(@NotNull OptionDescription description) (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>, compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        RootGroupBuilder::description_dyn
    }

    /// java: @Override public OptionGroup.Builder collapsed(boolean collapsible) (dispatch: the body for this class)
    fn collapsed(&self) -> fn(&compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>, bool) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        RootGroupBuilder::collapsed_dyn
    }

    /// java: @Override public OptionGroup build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>) -> compat::Shared<dyn OptionGroup> {
        RootGroupBuilder::build_dyn
    }

}

impl OptionAddable for RootGroupBuilder {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public OptionGroup.Builder option(@NotNull Option<?> option) (dispatch: the body for this class)
    fn option(&self) -> fn(&compat::Shared<dyn OptionAddable>, &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAddable> {
        RootGroupBuilder::option_dyn
    }

    /// java: @Override public OptionGroup.Builder options(@NotNull Collection<? extends Option<?>> options) (dispatch: the body for this class)
    fn options(&self) -> fn(&compat::Shared<dyn OptionAddable>, &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        RootGroupBuilder::options_dyn
    }

    /// java: @Override default Builder option(@NotNull Supplier<@NotNull Option<?>> optionSupplier) (dispatch: the body for this class)
    fn option_supplier(&self) -> fn(&compat::Shared<dyn OptionAddable>, Arc<dyn Fn() -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        <dyn crate::dev::isxander::yacl3::api::option_group::Builder>::option_supplier_from::<Self>
    }

    /// java: @Override default Builder optionIf(boolean condition, @NotNull Option<?> option) (dispatch: the body for this class)
    fn option_if(&self) -> fn(&compat::Shared<dyn OptionAddable>, bool, &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAddable> {
        <dyn crate::dev::isxander::yacl3::api::option_group::Builder>::option_if_from::<Self>
    }

    /// java: @Override default Builder optionIf(boolean condition, @NotNull Supplier<@NotNull Option<?>> optionSupplier) (dispatch: the body for this class)
    fn option_if_boolean_supplier(&self) -> fn(&compat::Shared<dyn OptionAddable>, bool, Arc<dyn Fn() -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        <dyn crate::dev::isxander::yacl3::api::option_group::Builder>::option_if_boolean_supplier_from::<Self>
    }

    /// java: @Override default Builder optionsIf(boolean condition, @NotNull Collection<? extends Option<?>> options) (dispatch: the body for this class)
    fn options_if(&self) -> fn(&compat::Shared<dyn OptionAddable>, bool, &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        <dyn crate::dev::isxander::yacl3::api::option_group::Builder>::options_if_from::<Self>
    }

}

impl RootGroupBuilder {
    /// java: @Override public OptionGroup.Builder name(@NotNull Component name)
    pub fn name_impl(__this: &compat::Shared<Self>, mut name: MutableComponent) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        panic!("UnsupportedOperationException: {}", "Cannot set name of root group!");
    }

    /// RootGroupBuilder.name as the dispatch of Builder.name
    pub fn name_dyn(__this: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>, __p0: MutableComponent) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        Self::name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public OptionGroup.Builder description(@NotNull OptionDescription description)
    pub fn description_impl(__this: &compat::Shared<Self>, mut description: compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        panic!("UnsupportedOperationException: {}", "Cannot set name of root group!");
    }

    /// RootGroupBuilder.description as the dispatch of Builder.description
    pub fn description_dyn(__this: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>, __p0: compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        Self::description_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public OptionGroup.Builder collapsed(boolean collapsible)
    pub fn collapsed_impl(__this: &compat::Shared<Self>, mut collapsible: bool) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        panic!("UnsupportedOperationException: {}", "Cannot set collapsible of root group!");
    }

    /// RootGroupBuilder.collapsed as the dispatch of Builder.collapsed
    pub fn collapsed_dyn(__this: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>, __p0: bool) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        Self::collapsed_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public OptionGroup build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn OptionGroup> {
        panic!("UnsupportedOperationException: {}", "Cannot build root group!");
    }

    /// RootGroupBuilder.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>) -> compat::Shared<dyn OptionGroup> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public OptionGroup.Builder option(@NotNull Option<?> option)
    pub fn option_impl(__this: &compat::Shared<Self>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        // TODO(fabric2seer): `BuilderImpl.this` (the enclosing instance of an inner class)
        //   java: BuilderImpl.this.option(option);
        return (__this.clone() as compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>);
    }

    /// RootGroupBuilder.option as the dispatch of OptionAddable.option
    pub fn option_dyn(__this: &compat::Shared<dyn OptionAddable>, __p0: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAddable> {
        Self::option_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public OptionGroup.Builder options(@NotNull Collection<? extends Option<?>> options)
    pub fn options_impl(__this: &compat::Shared<Self>, options: &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> {
        // TODO(fabric2seer): `BuilderImpl.this` (the enclosing instance of an inner class)
        //   java: BuilderImpl.this.options(options);
        return (__this.clone() as compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>);
    }

    /// RootGroupBuilder.options as the dispatch of OptionAddable.options
    pub fn options_dyn(__this: &compat::Shared<dyn OptionAddable>, __p0: &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        Self::options_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
