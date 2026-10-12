//! Ported from dev.isxander.yacl3.api.ListOption.
//!
//! `ListOption<T> extends OptionGroup, Option<List<T>>`; `AnyListOption` is Java's `ListOption<?>` (the members the
//! GUI calls through a wildcard). A `ListOption<T>` reference coerces to `Arc<dyn OptionGroup>` (to be added to a
//! category) and to `Arc<dyn AnyOption>`.
//!
//! `name()`, `description()` and `tooltip()` are declared by both `OptionGroup` and `Option`, as in Java; through a
//! `dyn ListOption<T>` name the one meant (`OptionGroup::name(&*list)`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::list_option_entry::ListOptionEntry;
use crate::dev::isxander::yacl3::api::option::{AnyOption, Option as YOption};
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_event_listener::OptionEventListener;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::r#impl::list_option_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: `ListOption<?>` - what the GUI reaches through a wildcard list option.
pub trait AnyListOption: AnyOption + OptionGroup {
    /// java: @ApiStatus.Internal int numberOfEntries();
    fn number_of_entries(&self) -> i32;

    /// java: @ApiStatus.Internal int maximumNumberOfEntries();
    fn maximum_number_of_entries(&self) -> i32;

    /// java: @ApiStatus.Internal int minimumNumberOfEntries();
    fn minimum_number_of_entries(&self) -> i32;

    /// java: @ApiStatus.Internal ListOptionEntry<T> insertNewEntry(); (the entry as `Option<?>`)
    fn insert_new_entry_any(&self) -> Arc<dyn AnyOption>;

    /// java: @ApiStatus.Internal void addRefreshListener(Runnable changedListener);
    fn add_refresh_listener(&self, changed_listener: Arc<dyn Fn()>);
}

/// java: public interface ListOption<T> extends OptionGroup, Option<List<T>>
pub trait ListOption<T: Clone + 'static>: AnyListOption + YOption<Vec<T>> {
    /// java: @Override @NotNull ImmutableList<ListOptionEntry<T>> options(); (typed; `OptionGroup::options` is the
    /// same list as `Option<?>`s)
    fn list_entries(&self) -> Vec<Arc<dyn ListOptionEntry<T>>>;

    /// java: @ApiStatus.Internal ListOptionEntry<T> insertNewEntry();
    fn insert_new_entry(&self) -> Arc<dyn ListOptionEntry<T>>;

    /// java: @ApiStatus.Internal void insertEntry(int index, ListOptionEntry<?> entry);
    fn insert_entry(&self, index: i32, entry: Arc<dyn ListOptionEntry<T>>);

    /// java: @ApiStatus.Internal int indexOf(ListOptionEntry<?> entry);
    fn index_of(&self, entry: &dyn ListOptionEntry<T>) -> i32;

    /// java: @ApiStatus.Internal void removeEntry(ListOptionEntry<?> entry);
    fn remove_entry(&self, entry: &dyn ListOptionEntry<T>);
}

/// java: static <T> Builder<T> createBuilder()
pub fn create_builder<T: Clone + PartialEq + 'static>() -> BuilderImpl<T> {
    BuilderImpl::new()
}

/// java: @Deprecated static <T> Builder<T> createBuilder(Class<T> typeClass)
pub fn create_builder_1<T: Clone + PartialEq + 'static>(_type_class: &compat::ClassOf<T>) -> BuilderImpl<T> {
    create_builder()
}

/// java: interface Builder<T> (setters by value, Java's `return this`)
pub trait Builder<T: Clone + 'static>: Sized {
    /// java: Builder<T> name(@NotNull Component name);
    fn name(self, name: MutableComponent) -> Self;

    /// java: Builder<T> description(@NotNull OptionDescription description);
    fn description(self, description: Arc<dyn OptionDescription>) -> Self;

    /// java: Builder<T> initial(@NotNull Supplier<T> initialValue);
    fn initial(self, initial_value: impl Fn() -> T + 'static) -> Self;

    /// java: Builder<T> initial(@NotNull T initialValue);
    fn initial_t(self, initial_value: T) -> Self;

    /// java: Builder<T> controller(@NotNull Function<Option<T>, ControllerBuilder<T>> controller);
    fn controller<B: ControllerBuilder<T> + 'static>(self, controller: impl Fn(Arc<dyn YOption<T>>) -> B + 'static) -> Self;

    /// java: Builder<T> customController(@NotNull Function<ListOptionEntry<T>, Controller<T>> control);
    fn custom_controller(self, control: impl Fn(Arc<dyn ListOptionEntry<T>>) -> Arc<dyn Controller<T>> + 'static) -> Self;

    /// java: Builder<T> state(@NotNull StateManager<List<T>> stateManager);
    fn state(self, state_manager: Arc<dyn StateManager<Vec<T>>>) -> Self;

    /// java: Builder<T> binding(@NotNull Binding<List<T>> binding);
    fn binding(self, binding: Arc<dyn Binding<Vec<T>>>) -> Self;

    /// java: Builder<T> binding(@NotNull List<T> def, @NotNull Supplier<@NotNull List<T>> getter, @NotNull Consumer<@NotNull List<T>> setter);
    fn binding_3(self, def: Vec<T>, getter: impl Fn() -> Vec<T> + 'static, setter: impl Fn(Vec<T>) + 'static) -> Self;

    /// java: Builder<T> available(boolean available);
    fn available(self, available: bool) -> Self;

    /// java: Builder<T> minimumNumberOfEntries(int number);
    fn minimum_number_of_entries(self, number: i32) -> Self;

    /// java: Builder<T> maximumNumberOfEntries(int number);
    fn maximum_number_of_entries(self, number: i32) -> Self;

    /// java: Builder<T> insertEntriesAtEnd(boolean insertAtEnd);
    fn insert_entries_at_end(self, insert_at_end: bool) -> Self;

    /// java: Builder<T> flag(@NotNull OptionFlag... flag);
    fn flag(self, flag: Vec<Arc<dyn OptionFlag>>) -> Self;

    /// java: Builder<T> flags(@NotNull Collection<OptionFlag> flags);
    fn flags(self, flags: Vec<Arc<dyn OptionFlag>>) -> Self;

    /// java: Builder<T> collapsed(boolean collapsible);
    fn collapsed(self, collapsible: bool) -> Self;

    /// java: ListOption.Builder<T> addListener(@NotNull OptionEventListener<List<T>> listener);
    fn add_listener(self, listener: Arc<dyn OptionEventListener<Vec<T>>>) -> Self;

    /// java: ListOption.Builder<T> addListeners(@NotNull Collection<OptionEventListener<List<T>>> listeners);
    fn add_listeners(self, listeners: Vec<Arc<dyn OptionEventListener<Vec<T>>>>) -> Self;

    /// java: ListOption.Builder<T> listener(@NotNull BiConsumer<Option<List<T>>, List<T>> listener);
    fn listener(self, listener: Arc<dyn Fn(&dyn YOption<Vec<T>>, Vec<T>)>) -> Self;

    /// java: ListOption.Builder<T> listeners(@NotNull Collection<BiConsumer<Option<List<T>>, List<T>>> listeners);
    fn listeners(self, listeners: Vec<Arc<dyn Fn(&dyn YOption<Vec<T>>, Vec<T>)>>) -> Self;

    /// java: ListOption<T> build();
    fn build(self) -> Arc<dyn ListOption<T>>;
}
