//! Ported from dev.isxander.yacl3.impl.HiddenNameListOptionEntry.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::list_option::ListOption;
use crate::dev::isxander::yacl3::api::list_option_entry::ListOptionEntry;
use crate::dev::isxander::yacl3::api::option::{AnyOption, Option as YOption};
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::api::option_event_listener::{Event, OptionEventListener};
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::sync::Weak;

/// java: public class HiddenNameListOptionEntry<T> implements ListOptionEntry<T>
///
/// It is handed to the entry's controller, which the entry holds - so it reaches the entry weakly.
pub struct HiddenNameListOptionEntry<T: Clone + 'static> {
    /// java: private final ListOptionEntry<T> option;
    pub option: Weak<dyn ListOptionEntry<T>>,
}

impl<T: Clone + 'static> HiddenNameListOptionEntry<T> {
    /// java: public HiddenNameListOptionEntry(ListOptionEntry<T> option)
    pub fn new(option: Weak<dyn ListOptionEntry<T>>) -> Self {
        Self { option }
    }

    fn option(&self) -> Arc<dyn ListOptionEntry<T>> {
        self.option.upgrade().expect("a list entry's controller outlived the entry")
    }
}

impl<T: Clone + 'static> AnyOption for HiddenNameListOptionEntry<T> {
    /// java: @Override public @NotNull Component name()
    fn name(&self) -> MutableComponent {
        component::empty()
    }

    /// java: @Override public @NotNull OptionDescription description()
    fn description(&self) -> Arc<dyn OptionDescription> {
        self.option().description()
    }

    /// java: @Override @Deprecated public @NotNull Component tooltip()
    fn tooltip(&self) -> MutableComponent {
        self.option().tooltip()
    }

    /// java: @Override public boolean available()
    fn available(&self) -> bool {
        self.option().available()
    }

    /// java: @Override public void setAvailable(boolean available)
    fn set_available(&self, available: bool) {
        self.option().set_available(available);
    }

    /// java: @Override public @NotNull ImmutableSet<OptionFlag> flags()
    fn flags(&self) -> Vec<Arc<dyn OptionFlag>> {
        self.option().flags()
    }

    /// java: @Override public boolean changed()
    fn changed(&self) -> bool {
        self.option().changed()
    }

    /// java: @Override public boolean applyValue()
    fn apply_value(&self) -> bool {
        self.option().apply_value()
    }

    /// java: @Override public void forgetPendingValue()
    fn forget_pending_value(&self) {
        self.option().forget_pending_value();
    }

    /// java: @Override public void requestSetDefault()
    fn request_set_default(&self) {
        self.option().request_set_default();
    }

    /// java: @Override public boolean isPendingValueDefault()
    fn is_pending_value_default(&self) -> bool {
        self.option().is_pending_value_default()
    }

    /// java: @Override public boolean canResetToDefault()
    fn can_reset_to_default(&self) -> bool {
        self.option().can_reset_to_default()
    }

    fn provide_controller_widget(&self, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        self.controller().provide_widget(screen, widget_dimension)
    }

    fn add_any_event_listener(&self, listener: Arc<dyn Fn(&dyn AnyOption, Event)>) {
        self.option().add_any_event_listener(listener);
    }

    fn add_any_listener(&self, changed_listener: Arc<dyn Fn(&dyn AnyOption)>) {
        self.option().add_any_listener(changed_listener);
    }
}

impl<T: Clone + 'static> YOption<T> for HiddenNameListOptionEntry<T> {
    /// java: @Override public @NotNull Controller<T> controller()
    fn controller(&self) -> Arc<dyn Controller<T>> {
        self.option().controller()
    }

    /// java: @Override public @NotNull StateManager<T> stateManager()
    fn state_manager(&self) -> Arc<dyn StateManager<T>> {
        self.option().state_manager()
    }

    /// java: @Override public @NotNull Binding<T> binding()
    fn binding(&self) -> Arc<dyn Binding<T>> {
        self.option().binding()
    }

    /// java: @Override public @NotNull T pendingValue()
    fn pending_value(&self) -> T {
        self.option().pending_value()
    }

    /// java: @Override public void requestSet(@NotNull T value)
    fn request_set(&self, value: T) {
        self.option().request_set(value);
    }

    /// java: @Override public void addEventListener(OptionEventListener<T> listener)
    fn add_event_listener(&self, listener: Arc<dyn OptionEventListener<T>>) {
        self.option().add_event_listener(listener);
    }

    /// java: @Override @Deprecated public void addListener(BiConsumer<Option<T>, T> changedListener)
    fn add_listener(&self, changed_listener: Arc<dyn Fn(&dyn YOption<T>, T)>) {
        self.option().add_listener(changed_listener);
    }
}

impl<T: Clone + 'static> ListOptionEntry<T> for HiddenNameListOptionEntry<T> {
    /// java: @Override public ListOption<T> parentGroup()
    fn parent_group(&self) -> Arc<dyn ListOption<T>> {
        self.option().parent_group()
    }
}
