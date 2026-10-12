//! Ported from dev.isxander.yacl3.api.StateManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::r#impl::immutable_state_manager::ImmutableStateManager;
use crate::dev::isxander::yacl3::r#impl::instant_state_manager::InstantStateManager;
use crate::dev::isxander::yacl3::r#impl::simple_state_manager::SimpleStateManager;
use crate::dev::isxander::yacl3::r#impl::x_mapped_state_manager::XMappedStateManager;

/// java: public interface StateManager<T>
pub trait StateManager<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void set(T value);
    fn set(&mut self, value: T);

    /// java: T get();
    fn get(&self) -> T;

    /// java: void apply();
    fn apply(&mut self);

    /// java: void resetToDefault(ResetAction action);
    fn reset_to_default(&mut self, action: ResetAction);

    /// java: void sync();
    fn sync(&mut self);

    /// java: boolean isSynced();
    fn is_synced(&self) -> bool;

    /// java: default boolean isAlwaysSynced()
    fn is_always_synced(&self) -> bool {
        return false;
    }

    /// java: boolean isDefault();
    fn is_default(&self) -> bool;

    /// java: void addListener(StateListener<T> stateListener);
    fn add_listener(&mut self, state_listener: &compat::Shared<dyn StateListener<T>>);

    /// java: default <U> StateManager<U> xmap(Function<T, U> to, Function<U, @Nullable T> from)
    fn xmap<U: 'static>(&self, to: Arc<dyn Fn(T) -> U>, from: Arc<dyn Fn(U) -> Option<T>>) -> compat::Shared<dyn StateManager<U>> where Self: Sized {
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return new XMappedStateManager<>(this, to, from);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: static <T> StateManager<T> createSimple(Binding<T> binding)
pub fn create_simple<T: 'static>(binding: &compat::Shared<dyn Binding<T>>) -> compat::Shared<dyn StateManager<T>> {
    return (compat::shared(SimpleStateManager::new(binding.clone())) as compat::Shared<dyn StateManager<compat::Object>>);
}

/// java: static <T> StateManager<T> createSimple(@NotNull T def, @NotNull Supplier<@NotNull T> getter, @NotNull Consumer<@NotNull T> setter)
pub fn create_simple_3<T: 'static>(def: &T, getter: Arc<dyn Fn() -> T>, setter: Arc<dyn Fn(T)>) -> compat::Shared<dyn StateManager<T>> {
    return (compat::shared(SimpleStateManager::new(binding::generic(def, getter.clone(), setter.clone()))) as compat::Shared<dyn StateManager<compat::Object>>);
}

/// java: static <T> StateManager<T> createInstant(Binding<T> binding)
pub fn create_instant<T: 'static>(binding: &compat::Shared<dyn Binding<T>>) -> compat::Shared<dyn StateManager<T>> {
    return (compat::shared(InstantStateManager::new(binding.clone())) as compat::Shared<dyn StateManager<compat::Object>>);
}

/// java: static <T> StateManager<T> createInstant(@NotNull T def, @NotNull Supplier<@NotNull T> getter, @NotNull Consumer<@NotNull T> setter)
pub fn create_instant_3<T: 'static>(def: &T, getter: Arc<dyn Fn() -> T>, setter: Arc<dyn Fn(T)>) -> compat::Shared<dyn StateManager<T>> {
    return (compat::shared(InstantStateManager::new(binding::generic(def, getter.clone(), setter.clone()))) as compat::Shared<dyn StateManager<compat::Object>>);
}

/// java: static <T> StateManager<T> createImmutable(@NotNull T value)
pub fn create_immutable<T: Clone + 'static>(value: &T) -> compat::Shared<dyn StateManager<T>> {
    return (compat::shared(ImmutableStateManager::new(value.clone())) as compat::Shared<dyn StateManager<compat::Object>>);
}

/// java: enum ResetAction
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ResetAction {
    ByOption,
    ByGlobal,
}

impl ResetAction {
    pub const VALUES: [ResetAction; 2] = [ResetAction::ByOption, ResetAction::ByGlobal];

    /// java: values()
    pub fn values() -> Vec<ResetAction> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            ResetAction::ByOption => "BY_OPTION",
            ResetAction::ByGlobal => "BY_GLOBAL",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> ResetAction {
        match name {
            "BY_OPTION" => ResetAction::ByOption,
            "BY_GLOBAL" => ResetAction::ByGlobal,
            _ => panic!("IllegalArgumentException: No enum constant ResetAction.{name}"),
        }
    }

}

impl std::fmt::Display for ResetAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// java: interface StateListener<T>
pub trait StateListener<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void onStateChange(T oldValue, T newValue);
    fn on_state_change(&self, old_value: &T, new_value: &T);

    /// java: default StateListener<T> andThen(StateListener<T> after)
    fn and_then(&self, after: &compat::Shared<dyn StateListener<T>>) -> compat::Shared<dyn StateListener<T>> {
        return Arc::new(move |old_value, new_value| {
            self.on_state_change(&old_value, &new_value);
            after.lock().borrow().on_state_change(&old_value, &new_value);
        });
    }

}

/// java: static <T> StateListener<T> noop()
pub fn noop<T: 'static>() -> compat::Shared<dyn StateListener<T>> {
    return Arc::new(move |old_value, new_value| {

        });
}
