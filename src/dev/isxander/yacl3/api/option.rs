//! Ported from dev.isxander.yacl3.api.Option.
//!
//! `Option<T>` is the typed interface; `AnyOption` is Java's `Option<?>` - every method that does not mention `T`,
//! plus the erased forms of the typed ones the GUI calls through a wildcard (`option.controller().provideWidget(..)`,
//! `option.addListener((opt, val) -> ..)` where `val` is unused, `instanceof ListOption<?>`). `Option<T>: AnyOption`,
//! so an `Arc<dyn Option<T>>` coerces to `Arc<dyn AnyOption>` (trait upcasting) wherever Java widens to `Option<?>`.
//!
//! Inside this module `Option` is the YACL interface; `std::option::Option` is spelled out.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::list_option::AnyListOption;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_event_listener::{Event, OptionEventListener};
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::option_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: `Option<?>` - the members of `Option<T>` that do not mention `T`.
pub trait AnyOption: compat::DynAny {
    /// java: @NotNull Component name();
    fn name(&self) -> MutableComponent;

    /// java: @NotNull OptionDescription description();
    fn description(&self) -> Arc<dyn OptionDescription>;

    /// java: @Deprecated @NotNull Component tooltip();
    fn tooltip(&self) -> MutableComponent;

    /// java: boolean available();
    fn available(&self) -> bool;

    /// java: void setAvailable(boolean available);
    fn set_available(&self, available: bool);

    /// java: @NotNull ImmutableSet<OptionFlag> flags();
    fn flags(&self) -> Vec<Arc<dyn OptionFlag>>;

    /// java: boolean changed();
    fn changed(&self) -> bool;

    /// java: boolean applyValue();
    fn apply_value(&self) -> bool;

    /// java: void forgetPendingValue();
    fn forget_pending_value(&self);

    /// java: void requestSetDefault();
    fn request_set_default(&self);

    /// java: boolean isPendingValueDefault();
    fn is_pending_value_default(&self) -> bool;

    /// java: default boolean canResetToDefault()
    fn can_reset_to_default(&self) -> bool {
        true
    }

    /// `option.controller().provideWidget(screen, widgetDimension)` on an `Option<?>` (OptionListWidget).
    fn provide_controller_widget(&self, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget>;

    /// `option.addEventListener((opt, event) -> ..)` on an `Option<?>` (YACLScreen): `opt` as `Option<?>`.
    fn add_any_event_listener(&self, listener: Arc<dyn Fn(&dyn AnyOption, Event)>);

    /// `option.addListener((opt, val) -> ..)` on an `Option<?>` whose body does not read `val` (OptionListWidget's
    /// reset buttons, ControllerWidget's tooltip): `addEventListener((opt, event) -> changedListener.accept(opt,
    /// opt.pendingValue()))` with `opt` as `Option<?>`.
    fn add_any_listener(&self, changed_listener: Arc<dyn Fn(&dyn AnyOption)>);

    /// `option instanceof ListOption<?> listOption ? listOption : null`.
    fn as_list_option(&self) -> std::option::Option<Arc<dyn AnyListOption>> {
        None
    }
}

/// java: public interface Option<T>
pub trait Option<T: Clone + 'static>: AnyOption {
    /// java: @NotNull Controller<T> controller();
    fn controller(&self) -> Arc<dyn Controller<T>>;

    /// java: @NotNull StateManager<T> stateManager();
    fn state_manager(&self) -> Arc<dyn StateManager<T>>;

    /// java: @Deprecated @NotNull Binding<T> binding();
    fn binding(&self) -> Arc<dyn Binding<T>>;

    /// java: @NotNull T pendingValue();
    fn pending_value(&self) -> T;

    /// java: void requestSet(@NotNull T value);
    fn request_set(&self, value: T);

    /// java: void addEventListener(OptionEventListener<T> listener);
    fn add_event_listener(&self, listener: Arc<dyn OptionEventListener<T>>);

    /// java: @Deprecated void addListener(BiConsumer<Option<T>, T> changedListener);
    fn add_listener(&self, changed_listener: Arc<dyn Fn(&dyn Option<T>, T)>);
}

/// `addListener(changedListener)`'s body every implementation shares:
/// `addEventListener((opt, event) -> changedListener.accept(opt, opt.pendingValue()))`.
pub fn listener_as_event_listener<T: Clone + 'static>(changed_listener: Arc<dyn Fn(&dyn Option<T>, T)>) -> Arc<dyn OptionEventListener<T>> {
    Arc::new(move |opt: &dyn Option<T>, _event: Event| changed_listener(opt, opt.pending_value()))
}

/// `add_any_event_listener`'s body for a typed option: the erased listener, handed the option as `Option<?>`.
pub fn any_event_listener<T: Clone + 'static>(listener: Arc<dyn Fn(&dyn AnyOption, Event)>) -> Arc<dyn OptionEventListener<T>> {
    Arc::new(move |opt: &dyn Option<T>, event: Event| listener(opt as &dyn AnyOption, event))
}

/// `add_any_listener`'s body for a typed option.
pub fn any_listener<T: Clone + 'static>(changed_listener: Arc<dyn Fn(&dyn AnyOption)>) -> Arc<dyn OptionEventListener<T>> {
    Arc::new(move |opt: &dyn Option<T>, _event: Event| changed_listener(opt as &dyn AnyOption))
}

/// java: static <T> Builder<T> createBuilder()
pub fn create_builder<T: Clone + PartialEq + 'static>() -> BuilderImpl<T> {
    BuilderImpl::new()
}

/// java: @Deprecated static <T> Builder<T> createBuilder(Class<T> typeClass)
pub fn create_builder_1<T: Clone + PartialEq + 'static>(_type_class: &compat::ClassOf<T>) -> BuilderImpl<T> {
    create_builder()
}

/// java: interface Builder<T>
///
/// Every setter takes the builder BY VALUE and answers it (Java's `return this`), so a chain reads as Java's and a
/// statement is `builder = builder.name(..)`.
pub trait Builder<T: Clone + 'static>: Sized {
    /// java: Builder<T> name(@NotNull Component name);
    fn name(self, name: MutableComponent) -> Self;

    /// java: Builder<T> description(@NotNull OptionDescription description);
    fn description(self, description: Arc<dyn OptionDescription>) -> Self;

    /// java: Builder<T> description(@NotNull Function<T, OptionDescription> descriptionFunction);
    fn description_function(self, description_function: impl Fn(T) -> Arc<dyn OptionDescription> + 'static) -> Self;

    /// java: Builder<T> controller(@NotNull Function<Option<T>, ControllerBuilder<T>> controllerBuilder);
    fn controller<B: ControllerBuilder<T> + 'static>(self, controller_builder: impl Fn(Arc<dyn Option<T>>) -> B + 'static) -> Self;

    /// java: Builder<T> customController(@NotNull Function<Option<T>, Controller<T>> control);
    fn custom_controller(self, control: impl Fn(Arc<dyn Option<T>>) -> Arc<dyn Controller<T>> + 'static) -> Self;

    /// java: Builder<T> stateManager(@NotNull StateManager<T> stateManager);
    fn state_manager(self, state_manager: Arc<dyn StateManager<T>>) -> Self;

    /// java: Builder<T> binding(@NotNull Binding<T> binding);
    fn binding(self, binding: Arc<dyn Binding<T>>) -> Self;

    /// java: Builder<T> binding(@NotNull T def, @NotNull Supplier<@NotNull T> getter, @NotNull Consumer<@NotNull T> setter);
    fn binding_3(self, def: T, getter: impl Fn() -> T + 'static, setter: impl Fn(T) + 'static) -> Self;

    /// java: Builder<T> available(boolean available);
    fn available(self, available: bool) -> Self;

    /// java: Builder<T> flag(@NotNull OptionFlag... flag);
    fn flag(self, flag: Vec<Arc<dyn OptionFlag>>) -> Self;

    /// java: Builder<T> flags(@NotNull Collection<? extends OptionFlag> flags);
    fn flags(self, flags: Vec<Arc<dyn OptionFlag>>) -> Self;

    /// java: Builder<T> addListener(@NotNull OptionEventListener<T> listener);
    fn add_listener(self, listener: Arc<dyn OptionEventListener<T>>) -> Self;

    /// java: Builder<T> addListeners(@NotNull Collection<OptionEventListener<T>> listeners);
    fn add_listeners(self, listeners: Vec<Arc<dyn OptionEventListener<T>>>) -> Self;

    /// java: @Deprecated Builder<T> instant(boolean instant);
    fn instant(self, instant: bool) -> Self;

    /// java: @Deprecated Builder<T> listener(@NotNull BiConsumer<Option<T>, T> listener);
    fn listener(self, listener: Arc<dyn Fn(&dyn Option<T>, T)>) -> Self;

    /// java: @Deprecated Builder<T> listeners(@NotNull Collection<BiConsumer<Option<T>, T>> listeners);
    fn listeners(self, listeners: Vec<Arc<dyn Fn(&dyn Option<T>, T)>>) -> Self;

    /// java: Option<T> build();
    fn build(self) -> Arc<dyn Option<T>>;
}
