//! Ported from dev.isxander.yacl3.api.ListOption by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::list_option_entry::ListOptionEntry;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_event_listener::OptionEventListener;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::r#impl::list_option_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface ListOption<T> extends OptionGroup, Option<List<T>>
pub trait ListOption<T: Clone + 'static>: OptionGroup + crate::dev::isxander::yacl3::api::option::Option {
    /// java: @Override @NotNull ImmutableList<ListOptionEntry<T>> options();
    fn options(&self) -> Vec<compat::Shared<dyn ListOptionEntry<T>>>;

    /// java: @ApiStatus.Internal int numberOfEntries();
    fn number_of_entries(&self) -> i32;

    /// java: @ApiStatus.Internal int maximumNumberOfEntries();
    fn maximum_number_of_entries(&self) -> i32;

    /// java: @ApiStatus.Internal int minimumNumberOfEntries();
    fn minimum_number_of_entries(&self) -> i32;

    /// java: @ApiStatus.Internal ListOptionEntry<T> insertNewEntry();
    fn insert_new_entry(&mut self) -> compat::Shared<dyn ListOptionEntry<T>>;

    /// java: @ApiStatus.Internal void insertEntry(int index, ListOptionEntry<?> entry);
    fn insert_entry(&mut self, index: i32, entry: &compat::Shared<dyn ListOptionEntry<compat::Object>>);

    /// java: @ApiStatus.Internal int indexOf(ListOptionEntry<?> entry);
    fn index_of(&self, entry: &compat::Shared<dyn ListOptionEntry<compat::Object>>) -> i32;

    /// java: @ApiStatus.Internal void removeEntry(ListOptionEntry<?> entry);
    fn remove_entry(&mut self, entry: &compat::Shared<dyn ListOptionEntry<compat::Object>>);

    /// java: @ApiStatus.Internal void addRefreshListener(Runnable changedListener);
    fn add_refresh_listener(&mut self, changed_listener: Arc<dyn Fn()>);

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
    fn description(&mut self, description: compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> initial(@NotNull Supplier<T> initialValue);
    fn initial(&mut self, initial_value: Arc<dyn Fn() -> T>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> initial(@NotNull T initialValue);
    fn initial_t(&mut self, initial_value: &T) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> controller(@NotNull Function<Option<T>, ControllerBuilder<T>> controller);
    fn controller(&mut self, controller: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> compat::Shared<dyn ControllerBuilder<T>>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> customController(@NotNull Function<ListOptionEntry<T>, Controller<T>> control);
    fn custom_controller(&mut self, control: Arc<dyn Fn(compat::Shared<dyn ListOptionEntry<T>>) -> compat::Shared<dyn Controller<T>>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> state(@NotNull StateManager<List<T>> stateManager);
    fn state(&mut self, state_manager: compat::Shared<dyn StateManager<Vec<T>>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> binding(@NotNull Binding<List<T>> binding);
    fn binding(&mut self, binding: compat::Shared<dyn Binding<Vec<T>>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> binding(@NotNull List<T> def, @NotNull Supplier<@NotNull List<T>> getter, @NotNull Consumer<@NotNull List<T>> setter);
    fn binding_3(&mut self, def: &Vec<T>, getter: Arc<dyn Fn() -> Vec<T>>, setter: Arc<dyn Fn(Vec<T>)>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> available(boolean available);
    fn available(&mut self, available: bool) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> minimumNumberOfEntries(int number);
    fn minimum_number_of_entries(&mut self, number: i32) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> maximumNumberOfEntries(int number);
    fn maximum_number_of_entries(&mut self, number: i32) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> insertEntriesAtEnd(boolean insertAtEnd);
    fn insert_entries_at_end(&mut self, insert_at_end: bool) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> flag(@NotNull OptionFlag... flag);
    fn flag(&mut self, flag: &Vec<compat::Shared<dyn OptionFlag>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> flags(@NotNull Collection<OptionFlag> flags);
    fn flags(&mut self, flags: &Vec<compat::Shared<dyn OptionFlag>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> collapsed(boolean collapsible);
    fn collapsed(&mut self, collapsible: bool) -> compat::Shared<dyn Builder<T>>;

    /// java: ListOption.Builder<T> addListener(@NotNull OptionEventListener<List<T>> listener);
    fn add_listener(&mut self, listener: compat::Shared<dyn OptionEventListener<Vec<T>>>) -> compat::Shared<dyn Builder<T>>;

    /// java: ListOption.Builder<T> addListeners(@NotNull Collection<OptionEventListener<List<T>>> listeners);
    fn add_listeners(&mut self, listeners: &Vec<compat::Shared<dyn OptionEventListener<Vec<T>>>>) -> compat::Shared<dyn Builder<T>>;

    /// java: ListOption.Builder<T> listener(@NotNull BiConsumer<Option<List<T>>, List<T>> listener);
    fn listener(&mut self, listener: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Vec<T>>>, Vec<T>)>) -> compat::Shared<dyn Builder<T>>;

    /// java: ListOption.Builder<T> listeners(@NotNull Collection<BiConsumer<Option<List<T>>, List<T>>> listeners);
    fn listeners(&mut self, listeners: &Vec<Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Vec<T>>>, Vec<T>)>>) -> compat::Shared<dyn Builder<T>>;

    /// java: ListOption<T> build();
    fn build(&mut self) -> compat::Shared<dyn ListOption<T>>;

}
