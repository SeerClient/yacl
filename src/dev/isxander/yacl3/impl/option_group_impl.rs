//! Ported from dev.isxander.yacl3.impl.OptionGroupImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::list_option::ListOption;
use crate::dev::isxander::yacl3::api::option_addable::OptionAddable;
use crate::dev::isxander::yacl3::api::option_description;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_group::Builder;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: @ApiStatus.Internal public record OptionGroupImpl(@NotNull Component name, @NotNull OptionDescription description, ImmutableList<? extends Option<?>> options, boolean collapsed, boolean isRoot) implements OptionGroup
#[derive(Clone, Debug)]
pub struct OptionGroupImpl {
    /// java: Component name
    pub name: MutableComponent,
    /// java: OptionDescription description
    pub description: compat::Shared<dyn OptionDescription>,
    /// java: ImmutableList<? extends Option<?>> options
    pub options: Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>,
    /// java: boolean collapsed
    pub collapsed: bool,
    /// java: boolean isRoot
    pub is_root: bool,
}

impl OptionGroupImpl {
    /// java: the canonical record constructor
    pub fn new(mut name: MutableComponent, mut description: compat::Shared<dyn OptionDescription>, mut options: Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>, mut collapsed: bool, mut is_root: bool) -> Self {
        Self { name, description, options, collapsed, is_root }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: the record accessor `collapsed()`
    pub fn collapsed(&self) -> bool {
        self.collapsed
    }

    /// java: the record accessor `isRoot()`
    pub fn is_root(&self) -> bool {
        self.is_root
    }

}

impl OptionGroup for OptionGroupImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public OptionDescription description() (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn OptionDescription> {
        OptionGroupImpl::description_dyn
    }

    /// java: @Override public @NotNull Component tooltip() (dispatch: the body for this class)
    fn tooltip(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> MutableComponent {
        OptionGroupImpl::tooltip_dyn
    }

    /// java: @Override public @NotNull ImmutableList<? extends Option<?>> options() (dispatch: the body for this class)
    fn options(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>> {
        OptionGroupImpl::options_dyn
    }

}

impl OptionGroupImpl {
    /// java: @Override public OptionDescription description()
    pub fn description_impl(&self) -> compat::Shared<dyn OptionDescription> {
        return self.description.clone();
    }

    /// OptionGroupImpl.description as the dispatch of OptionGroup.description
    pub fn description_dyn(__this: &compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn OptionDescription> {
        compat::read_as::<Self, _, _>(__this, |__s| __s.description_impl())
    }

    /// java: @Override public @NotNull Component tooltip()
    pub fn tooltip_impl(&self) -> MutableComponent {
        return ({ let __f = compat::dispatch(&self.description, |__t| __t.text()); __f(&self.description) });
    }

    /// OptionGroupImpl.tooltip as the dispatch of OptionGroup.tooltip
    pub fn tooltip_dyn(__this: &compat::Shared<dyn OptionGroup>) -> MutableComponent {
        compat::read_as::<Self, _, _>(__this, |__s| __s.tooltip_impl())
    }

    /// java: @Override public @NotNull ImmutableList<? extends Option<?>> options()
    pub fn options_impl(&self) -> Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>> {
        return self.options.clone();
    }

    /// OptionGroupImpl.options as the dispatch of OptionGroup.options
    pub fn options_dyn(__this: &compat::Shared<dyn OptionGroup>) -> Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>> {
        compat::read_as::<Self, _, _>(__this, |__s| __s.options_impl())
    }

}

/// java: @ApiStatus.Internal public static final class BuilderImpl implements Builder
#[derive(Clone)]
pub struct BuilderImpl {
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private OptionDescription description = OptionDescription.EMPTY;
    pub description: compat::Shared<dyn OptionDescription>,
    /// java: private final List<Option<?>> options = new ArrayList<>();
    pub options: Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>,
    /// java: private boolean collapsed = false;
    pub collapsed: bool,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            name: component::empty(),
            description: option_description::EMPTY.get_mut().clone(),
            options: Vec::new(),
            collapsed: false,
        });
        __this
    }

}

impl Builder for BuilderImpl {
    /// java: @Override public Builder name(@NotNull Component name) (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        BuilderImpl::name_dyn
    }

    /// java: @Override public Builder description(@NotNull OptionDescription description) (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn Builder>, compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn Builder> {
        BuilderImpl::description_dyn
    }

    /// java: @Override public Builder collapsed(boolean collapsible) (dispatch: the body for this class)
    fn collapsed(&self) -> fn(&compat::Shared<dyn Builder>, bool) -> compat::Shared<dyn Builder> {
        BuilderImpl::collapsed_dyn
    }

    /// java: @Override public OptionGroup build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn OptionGroup> {
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
        //   java: Validate.notNull(name, "`name` must not be null");
        { let __v = name.clone(); compat::write(__this, |__t| __t.name = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.name as the dispatch of Builder.name
    pub fn name_dyn(__this: &compat::Shared<dyn Builder>, __p0: MutableComponent) -> compat::Shared<dyn Builder> {
        Self::name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder description(@NotNull OptionDescription description)
    pub fn description_impl(__this: &compat::Shared<Self>, mut description: compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(description, "`description` must not be null");
        { let __v = description.clone(); compat::write(__this, |__t| __t.description = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.description as the dispatch of Builder.description
    pub fn description_dyn(__this: &compat::Shared<dyn Builder>, __p0: compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn Builder> {
        Self::description_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder collapsed(boolean collapsible)
    pub fn collapsed_impl(__this: &compat::Shared<Self>, mut collapsible: bool) -> compat::Shared<dyn Builder> {
        { let __v = collapsible; compat::write(__this, |__t| __t.collapsed = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.collapsed as the dispatch of Builder.collapsed
    pub fn collapsed_dyn(__this: &compat::Shared<dyn Builder>, __p0: bool) -> compat::Shared<dyn Builder> {
        Self::collapsed_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public OptionGroup build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn OptionGroup> {
        // TODO(fabric2seer): static call Validate.notEmpty(..)
        //   java: Validate.notEmpty(options, "`options` must not be empty to build `OptionGroup`");
        return (compat::shared(OptionGroupImpl::new(compat::read(__this, |__t| __t.name.clone()), compat::read(__this, |__t| __t.description.clone()), __this.lock().borrow().options.iter().cloned().collect::<Vec<_>>(), compat::read(__this, |__t| __t.collapsed), false)) as compat::Shared<dyn OptionGroup>);
    }

    /// BuilderImpl.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn OptionGroup> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Builder option(@NotNull Option<?> option)
    pub fn option_impl(__this: &compat::Shared<Self>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(option, "`option` must not be null");
        if option.lock().borrow().as_any().is::<compat::Shared<dyn ListOption<compat::Object>>>() {
            panic!("UnsupportedOperationException: {}", "List options must not be added as an option but a group!");
        }
        let __h0 = option.clone();
        __this.lock().borrow_mut().options.push(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.option as the dispatch of OptionAddable.option
    pub fn option_dyn(__this: &compat::Shared<dyn OptionAddable>, __p0: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAddable> {
        Self::option_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder options(@NotNull Collection<? extends Option<?>> options)
    pub fn options_impl(__this: &compat::Shared<Self>, options: &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notEmpty(..)
        //   java: Validate.notEmpty(options, "`options` must not be empty");
        // TODO(fabric2seer): the condition: `.isInstance(..)` on a Class
        //   java: options.stream(..).anyMatch(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            panic!("UnsupportedOperationException: {}", "List options must not be added as an option but a group!");
        }
        let __h0 = options.iter().cloned();
        __this.lock().borrow_mut().options.extend(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.options as the dispatch of OptionAddable.options
    pub fn options_dyn(__this: &compat::Shared<dyn OptionAddable>, __p0: &Vec<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAddable> {
        Self::options_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
