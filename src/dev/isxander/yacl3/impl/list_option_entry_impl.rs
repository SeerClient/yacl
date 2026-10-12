//! Ported from dev.isxander.yacl3.impl.ListOptionEntryImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::list_option::ListOption;
use crate::dev::isxander::yacl3::api::list_option_entry::ListOptionEntry;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_event_listener::OptionEventListener;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::list_entry_widget::ListEntryWidget;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::hidden_name_list_option_entry::HiddenNameListOptionEntry;
use crate::dev::isxander::yacl3::r#impl::list_option_impl::ListOptionImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: @ApiStatus.Internal public final class ListOptionEntryImpl<T> implements ListOptionEntry<T>
#[derive(Clone)]
pub struct ListOptionEntryImpl<T> {
    /// java: private final ListOptionImpl<T> group;
    pub group: compat::Shared<ListOptionImpl<T>>,
    /// java: private T value;
    pub value: T,
    /// java: private final Binding<T> binding;
    pub binding: compat::Shared<dyn Binding<T>>,
    /// java: private final Controller<T> controller;
    pub controller: Option<compat::Shared<dyn Controller<T>>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl<T: Clone + 'static> ListOptionEntryImpl<T> {
    /// java: ListOptionEntryImpl(ListOptionImpl<T> group, T initialValue, @NotNull Function<ListOptionEntry<T>, Controller<T>> controlGetter)
    pub fn new(mut group: compat::Shared<ListOptionImpl<T>>, mut initial_value: T, control_getter: Arc<dyn Fn(compat::Shared<dyn ListOptionEntry<T>>) -> compat::Shared<dyn Controller<T>>>) -> compat::Shared<Self> {
        let __init_group = group.clone();
        let __init_value = initial_value.clone();
        let __init_binding = (compat::shared(EntryBinding::new()) as compat::Shared<dyn Binding<compat::Object>>);
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            group: __init_group,
            value: __init_value,
            binding: __init_binding,
            controller: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some((compat::shared(EntryController::new((control_getter)(HiddenNameListOptionEntry::new((__this.clone() as compat::Shared<dyn ListOptionEntry<compat::Object>>))), __this.clone())) as compat::Shared<dyn Controller<compat::Object>>)); compat::write(&__this, |__t| __t.controller = __v); }
        __this
    }

}

impl<T: Clone + 'static> ListOptionEntry<T> for ListOptionEntryImpl<T> {
    /// java: @Override public ListOption<T> parentGroup()
    fn parent_group(&self) -> compat::Shared<dyn ListOption<T>> {
        return (self.group.clone() as compat::Shared<dyn ListOption<compat::Object>>);
    }

}

impl<T: Clone + 'static> crate::dev::isxander::yacl3::api::option::Option<compat::Object> for ListOptionEntryImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @NotNull Component name()
    fn name(&self) -> MutableComponent {
        return self.group.lock().borrow().name();
    }

    /// java: @Override public @NotNull OptionDescription description()
    fn description(&self) -> compat::Shared<dyn OptionDescription> {
        return self.group.lock().borrow().description();
    }

    /// java: @Override public @NotNull Component tooltip()
    fn tooltip(&self) -> MutableComponent {
        return self.group.lock().borrow().tooltip();
    }

    /// java: @Override public @NotNull Controller<T> controller()
    fn controller(&self) -> compat::Shared<dyn Controller<T>> {
        return self.controller.clone().unwrap();
    }

    /// java: @Override public @NotNull StateManager<T> stateManager()
    fn state_manager(&self) -> compat::Shared<dyn StateManager<T>> {
        panic!("UnsupportedOperationException: {}", "ListOptionEntryImpl does not support state managers");
    }

    /// java: @Override public @NotNull Binding<T> binding()
    fn binding(&self) -> compat::Shared<dyn Binding<T>> {
        return self.binding.clone();
    }

    /// java: @Override public boolean available()
    fn available(&self) -> bool {
        return self.parent_group().lock().borrow().available();
    }

    /// java: @Override public void setAvailable(boolean available)
    fn set_available(&mut self, mut available: bool) {
    }

    /// java: @Override public boolean changed()
    fn changed(&self) -> bool {
        return false;
    }

    /// java: @Override public @NotNull T pendingValue()
    fn pending_value(&self) -> T {
        return self.value.clone();
    }

    /// java: @Override public void requestSet(@NotNull T value)
    fn request_set(&mut self, mut value: T) {
        { let (__a0,) = (value.clone(),); self.binding.lock().borrow_mut().set_value(__a0) };
    }

    /// java: @Override public boolean applyValue()
    fn apply_value(&self) -> bool {
        return false;
    }

    /// java: @Override public void forgetPendingValue()
    fn forget_pending_value(&mut self) {
    }

    /// java: @Override public void requestSetDefault()
    fn request_set_default(&mut self) {
    }

    /// java: @Override public boolean isPendingValueDefault()
    fn is_pending_value_default(&self) -> bool {
        return false;
    }

    /// java: @Override public boolean canResetToDefault()
    fn can_reset_to_default(&self) -> bool {
        return false;
    }

    /// java: @Override public void addEventListener(OptionEventListener<T> listener)
    fn add_event_listener(&mut self, mut listener: compat::Shared<dyn OptionEventListener<T>>) {
    }

    /// java: @Override public void addListener(BiConsumer<Option<T>, T> changedListener)
    fn add_listener(&mut self, changed_listener: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, T)>) {
    }

    /// java: @Override default @NotNull ImmutableSet<OptionFlag> flags()
    fn flags(&self) -> std::collections::HashSet<compat::Shared<dyn OptionFlag>> {
        return self.parent_group().lock().borrow().flags();
    }

}

/// java: @ApiStatus.Internal public record EntryController<T>(Controller<T> controller, ListOptionEntryImpl<T> entry) implements Controller<T>
#[derive(Clone, Debug)]
pub struct EntryController<T> {
    /// java: Controller<T> controller
    pub controller: compat::Shared<dyn Controller<T>>,
    /// java: ListOptionEntryImpl<T> entry
    pub entry: compat::Shared<ListOptionEntryImpl<T>>,
}

impl<T: Clone + 'static> EntryController<T> {
    /// java: the canonical record constructor
    pub fn new(mut controller: compat::Shared<dyn Controller<T>>, mut entry: compat::Shared<ListOptionEntryImpl<T>>) -> Self {
        Self { controller, entry }
    }

    /// java: the record accessor `controller()`
    pub fn controller(&self) -> compat::Shared<dyn Controller<T>> {
        self.controller.clone()
    }

    /// java: the record accessor `entry()`
    pub fn entry(&self) -> compat::Shared<ListOptionEntryImpl<T>> {
        self.entry.clone()
    }

}

impl<T: Clone + 'static> Controller<T> for EntryController<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> option()
    fn option(&self) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>> {
        return self.controller.lock().borrow().option();
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        return self.controller.lock().borrow().format_value();
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        return (ListEntryWidget::new(screen.clone(), (self.entry.clone() as compat::Shared<dyn ListOptionEntry<compat::Object>>), self.controller.lock().borrow().provide_widget(screen, widget_dimension)) as compat::Shared<dyn AbstractWidget>);
    }

}

/// java: private class EntryBinding implements Binding<T>
#[derive(Clone)]
pub struct EntryBinding {}

impl EntryBinding {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

impl Binding<T> for EntryBinding {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void setValue(T newValue)
    fn set_value(&mut self, mut new_value: T) {
        // TODO(fabric2seer): `value` is a field of the enclosing instance (ListOptionEntryImpl.this) - an inner class has no outer reference in Rust
        //   java: value = newValue;
        // TODO(fabric2seer): `group` is a field of the enclosing instance (ListOptionEntryImpl.this) - an inner class has no outer reference in Rust
        //   java: group.triggerListener(OptionEventListener.Event.OTHER, true);
    }

    /// java: @Override public T getValue()
    fn get_value(&self) -> T {
        // TODO(fabric2seer): `value` is a field of the enclosing instance (ListOptionEntryImpl.this) - an inner class has no outer reference in Rust
        //   java: return value;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public T defaultValue()
    fn default_value(&self) -> T {
        panic!("UnsupportedOperationException");
    }

}
