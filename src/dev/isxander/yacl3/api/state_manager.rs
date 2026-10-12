//! Ported from dev.isxander.yacl3.api.StateManager.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::r#impl::immutable_state_manager::ImmutableStateManager;
use crate::dev::isxander::yacl3::r#impl::instant_state_manager::InstantStateManager;
use crate::dev::isxander::yacl3::r#impl::simple_state_manager::SimpleStateManager;
use crate::dev::isxander::yacl3::r#impl::x_mapped_state_manager::XMappedStateManager;

/// java: public interface StateManager<T>
pub trait StateManager<T: Clone + 'static>: compat::DynAny {
    /// java: void set(T value);
    fn set(&self, value: T);

    /// java: T get();
    fn get(&self) -> T;

    /// java: void apply();
    fn apply(&self);

    /// java: void resetToDefault(ResetAction action);
    fn reset_to_default(&self, action: ResetAction);

    /// java: void sync();
    fn sync(&self);

    /// java: boolean isSynced();
    fn is_synced(&self) -> bool;

    /// java: default boolean isAlwaysSynced()
    fn is_always_synced(&self) -> bool {
        false
    }

    /// java: boolean isDefault();
    fn is_default(&self) -> bool;

    /// java: void addListener(StateListener<T> stateListener);
    fn add_listener(&self, state_listener: Arc<dyn StateListener<T>>);

    /// `stateManager instanceof ProvidesBindingForDeprecation p ? p.getBinding() : null` (OptionImpl.binding,
    /// ListOptionImpl.binding): the binding of a state manager whose class implements `ProvidesBindingForDeprecation`.
    fn get_binding_for_deprecation(&self) -> std::option::Option<Arc<dyn Binding<T>>> {
        None
    }
}

/// java: default <U> StateManager<U> xmap(Function<T, U> to, Function<U, @Nullable T> from)
pub fn xmap<T: Clone + 'static, U: Clone + 'static>(
    this: &Arc<dyn StateManager<T>>,
    to: impl Fn(T) -> U + 'static,
    from: impl Fn(U) -> std::option::Option<T> + 'static,
) -> Arc<dyn StateManager<U>> {
    Arc::new(XMappedStateManager::new(this.clone(), Arc::new(to), Arc::new(from)))
}

/// java: static <T> StateManager<T> createSimple(Binding<T> binding)
pub fn create_simple<T: Clone + PartialEq + 'static>(binding: Arc<dyn Binding<T>>) -> Arc<dyn StateManager<T>> {
    Arc::new(SimpleStateManager::new(binding))
}

/// java: static <T> StateManager<T> createSimple(@NotNull T def, @NotNull Supplier<@NotNull T> getter, @NotNull Consumer<@NotNull T> setter)
pub fn create_simple_3<T: Clone + PartialEq + 'static>(
    def: T,
    getter: impl Fn() -> T + 'static,
    setter: impl Fn(T) + 'static,
) -> Arc<dyn StateManager<T>> {
    Arc::new(SimpleStateManager::new(binding::generic(def, getter, setter)))
}

/// java: static <T> StateManager<T> createInstant(Binding<T> binding)
pub fn create_instant<T: Clone + PartialEq + 'static>(binding: Arc<dyn Binding<T>>) -> Arc<dyn StateManager<T>> {
    Arc::new(InstantStateManager::new(binding))
}

/// java: static <T> StateManager<T> createInstant(@NotNull T def, @NotNull Supplier<@NotNull T> getter, @NotNull Consumer<@NotNull T> setter)
pub fn create_instant_3<T: Clone + PartialEq + 'static>(
    def: T,
    getter: impl Fn() -> T + 'static,
    setter: impl Fn(T) + 'static,
) -> Arc<dyn StateManager<T>> {
    Arc::new(InstantStateManager::new(binding::generic(def, getter, setter)))
}

/// java: static <T> StateManager<T> createImmutable(@NotNull T value)
pub fn create_immutable<T: Clone + 'static>(value: T) -> Arc<dyn StateManager<T>> {
    Arc::new(ImmutableStateManager::new(value))
}

/// java: enum ResetAction
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

/// java: interface StateListener<T> (a functional interface: any `Fn(T, T)` is one)
pub trait StateListener<T: Clone + 'static> {
    /// java: void onStateChange(T oldValue, T newValue);
    fn on_state_change(&self, old_value: T, new_value: T);
}

impl<T: Clone + 'static, F: Fn(T, T)> StateListener<T> for F {
    fn on_state_change(&self, old_value: T, new_value: T) {
        self(old_value, new_value)
    }
}

/// java: default StateListener<T> andThen(StateListener<T> after)
pub fn and_then<T: Clone + 'static>(this: Arc<dyn StateListener<T>>, after: Arc<dyn StateListener<T>>) -> Arc<dyn StateListener<T>> {
    Arc::new(move |old_value: T, new_value: T| {
        this.on_state_change(old_value.clone(), new_value.clone());
        after.on_state_change(old_value, new_value);
    })
}

/// java: static <T> StateListener<T> noop()
pub fn noop<T: Clone + 'static>() -> Arc<dyn StateListener<T>> {
    Arc::new(|_old_value: T, _new_value: T| {})
}
