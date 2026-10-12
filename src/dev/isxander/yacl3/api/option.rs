//! Ported from dev.isxander.yacl3.api.Option by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_event_listener::OptionEventListener;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::r#impl::option_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface Option<T>
pub trait Option<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: @NotNull Component name();
    fn name(&self) -> MutableComponent;

    /// java: @NotNull OptionDescription description();
    fn description(&self) -> compat::Shared<dyn OptionDescription>;

    /// java: @Deprecated @NotNull Component tooltip();
    fn tooltip(&self) -> MutableComponent;

    /// java: @NotNull Controller<T> controller();
    fn controller(&self) -> compat::Shared<dyn Controller<T>>;

    /// java: @NotNull StateManager<T> stateManager();
    fn state_manager(&self) -> compat::Shared<dyn StateManager<T>>;

    /// java: @Deprecated @NotNull Binding<T> binding();
    fn binding(&self) -> compat::Shared<dyn Binding<T>>;

    /// java: boolean available();
    fn available(&self) -> bool;

    /// java: void setAvailable(boolean available);
    fn set_available(&mut self, available: bool);

    /// java: @NotNull ImmutableSet<OptionFlag> flags();
    fn flags(&self) -> std::collections::HashSet<compat::Shared<dyn OptionFlag>>;

    /// java: boolean changed();
    fn changed(&self) -> bool;

    /// java: @NotNull T pendingValue();
    fn pending_value(&self) -> T;

    /// java: void requestSet(@NotNull T value);
    fn request_set(&mut self, value: T);

    /// java: boolean applyValue();
    fn apply_value(&self) -> bool;

    /// java: void forgetPendingValue();
    fn forget_pending_value(&mut self);

    /// java: void requestSetDefault();
    fn request_set_default(&mut self);

    /// java: boolean isPendingValueDefault();
    fn is_pending_value_default(&self) -> bool;

    /// java: default boolean canResetToDefault()
    fn can_reset_to_default(&self) -> bool {
        return true;
    }

    /// java: void addEventListener(OptionEventListener<T> listener);
    fn add_event_listener(&mut self, listener: compat::Shared<dyn OptionEventListener<T>>);

    /// java: @Deprecated void addListener(BiConsumer<Option<T>, T> changedListener);
    fn add_listener(&mut self, changed_listener: Arc<dyn Fn(compat::Shared<dyn Option<T>>, T)>);

}

/// java: static <T> Builder<T> createBuilder()
pub fn create_builder<T: 'static>() -> compat::Shared<dyn Builder<T>> {
    return (compat::shared(BuilderImpl::new()) as compat::Shared<dyn Builder<compat::Object>>);
}

/// java: @Deprecated static <T> Builder<T> createBuilder(Class<T> typeClass)
pub fn create_builder_1<T: 'static>(type_class: &compat::ClassOf<T>) -> compat::Shared<dyn Builder<T>> {
    return create_builder();
}

/// java: interface Builder<T>
pub trait Builder<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Builder<T> name(@NotNull Component name);
    fn name(&mut self, name: MutableComponent) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> description(@NotNull OptionDescription description);
    fn description(&mut self, description: &compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> description(@NotNull Function<T, OptionDescription> descriptionFunction);
    fn description_function(&mut self, description_function: Arc<dyn Fn(T) -> compat::Shared<dyn OptionDescription>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> controller(@NotNull Function<Option<T>, ControllerBuilder<T>> controllerBuilder);
    fn controller(&mut self, controller_builder: Arc<dyn Fn(compat::Shared<dyn Option<T>>) -> compat::Shared<dyn ControllerBuilder<T>>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> customController(@NotNull Function<Option<T>, Controller<T>> control);
    fn custom_controller(&mut self, control: Arc<dyn Fn(compat::Shared<dyn Option<T>>) -> compat::Shared<dyn Controller<T>>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> stateManager(@NotNull StateManager<T> stateManager);
    fn state_manager(&mut self, state_manager: compat::Shared<dyn StateManager<T>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> binding(@NotNull Binding<T> binding);
    fn binding(&mut self, binding: compat::Shared<dyn Binding<T>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> binding(@NotNull T def, @NotNull Supplier<@NotNull T> getter, @NotNull Consumer<@NotNull T> setter);
    fn binding_3(&mut self, def: &T, getter: Arc<dyn Fn() -> T>, setter: Arc<dyn Fn(T)>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> available(boolean available);
    fn available(&mut self, available: bool) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> flag(@NotNull OptionFlag... flag);
    fn flag(&mut self, flag: &Vec<compat::Shared<dyn OptionFlag>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> flags(@NotNull Collection<? extends OptionFlag> flags);
    fn flags(&mut self, flags: &Vec<compat::Shared<dyn OptionFlag>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> addListener(@NotNull OptionEventListener<T> listener);
    fn add_listener(&mut self, listener: compat::Shared<dyn OptionEventListener<T>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> addListeners(@NotNull Collection<OptionEventListener<T>> listeners);
    fn add_listeners(&mut self, listeners: &Vec<compat::Shared<dyn OptionEventListener<T>>>) -> compat::Shared<dyn Builder<T>>;

    /// java: @Deprecated Builder<T> instant(boolean instant);
    fn instant(&mut self, instant: bool) -> compat::Shared<dyn Builder<T>>;

    /// java: @Deprecated Builder<T> listener(@NotNull BiConsumer<Option<T>, T> listener);
    fn listener(&mut self, listener: Arc<dyn Fn(compat::Shared<dyn Option<T>>, T)>) -> compat::Shared<dyn Builder<T>>;

    /// java: @Deprecated Builder<T> listeners(@NotNull Collection<BiConsumer<Option<T>, T>> listeners);
    fn listeners(&mut self, listeners: &Vec<Arc<dyn Fn(compat::Shared<dyn Option<T>>, T)>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Option<T> build();
    fn build(&mut self) -> compat::Shared<dyn Option<T>>;

}
