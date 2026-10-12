//! Ported from dev.isxander.yacl3.impl.LabelOptionImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::label_option::{Builder, LabelOption};
use crate::dev::isxander::yacl3::api::option::{AnyOption, Option as YOption};
use crate::dev::isxander::yacl3::api::option_description::{self, OptionDescription};
use crate::dev::isxander::yacl3::api::option_event_listener::{Event, OptionEventListener};
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::state_manager::{self, StateManager};
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::label_controller::LabelController;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::option_impl::OptionImpl;
use crate::dev::isxander::yacl3::r#impl::self_contained_binding::SelfContainedBinding;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class LabelOptionImpl extends OptionImpl<Component> implements LabelOption
pub struct LabelOptionImpl {
    /// java: extends OptionImpl<Component>
    pub base: OptionImpl<MutableComponent>,
}

impl LabelOptionImpl {
    /// java: public LabelOptionImpl(@NotNull StateManager<Component> stateManager, @NotNull Collection<OptionEventListener<Component>> optionEventListeners)
    pub fn new(
        state_manager: Arc<dyn StateManager<MutableComponent>>,
        option_event_listeners: Vec<Arc<dyn OptionEventListener<MutableComponent>>>,
    ) -> Arc<LabelOptionImpl> {
        // java: super(Component.literal("Label Option"), OptionDescription::of, LabelController::new, stateManager, true,
        //       ImmutableSet.of(), optionEventListeners);
        let base = OptionImpl::fields(
            component::literal("Label Option"),
            Arc::new(|label: MutableComponent| option_description::of(vec![label])),
            state_manager,
            true,
            Vec::new(),
            option_event_listeners,
        );
        let option = Arc::new(LabelOptionImpl { base });
        let this: Arc<dyn YOption<MutableComponent>> = option.clone();
        option.base.construct(
            this,
            Arc::new(|option: Arc<dyn YOption<MutableComponent>>| -> Arc<dyn Controller<MutableComponent>> { Arc::new(LabelController::new(option)) }),
        );
        option
    }

    /// java: public LabelOptionImpl(Component label)
    pub fn new_1(label: MutableComponent) -> Arc<LabelOptionImpl> {
        LabelOptionImpl::new(state_manager::create_immutable(label), Vec::new())
    }
}

impl AnyOption for LabelOptionImpl {
    fn name(&self) -> MutableComponent {
        self.base.name()
    }

    fn description(&self) -> Arc<dyn OptionDescription> {
        self.base.description()
    }

    fn tooltip(&self) -> MutableComponent {
        self.base.tooltip()
    }

    fn available(&self) -> bool {
        self.base.available()
    }

    /// java: @Override public void setAvailable(boolean available)
    fn set_available(&self, _available: bool) {
        panic!("UnsupportedOperationException: Cannot change availability of label option");
    }

    fn flags(&self) -> Vec<Arc<dyn OptionFlag>> {
        self.base.flags()
    }

    fn changed(&self) -> bool {
        self.base.changed()
    }

    fn apply_value(&self) -> bool {
        self.base.apply_value()
    }

    fn forget_pending_value(&self) {
        self.base.forget_pending_value()
    }

    fn request_set_default(&self) {
        self.base.request_set_default()
    }

    fn is_pending_value_default(&self) -> bool {
        self.base.is_pending_value_default()
    }

    fn provide_controller_widget(&self, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        self.base.provide_controller_widget(screen, widget_dimension)
    }

    fn add_any_event_listener(&self, listener: Arc<dyn Fn(&dyn AnyOption, Event)>) {
        self.base.add_any_event_listener(listener)
    }

    fn add_any_listener(&self, changed_listener: Arc<dyn Fn(&dyn AnyOption)>) {
        self.base.add_any_listener(changed_listener)
    }
}

impl YOption<MutableComponent> for LabelOptionImpl {
    fn controller(&self) -> Arc<dyn Controller<MutableComponent>> {
        self.base.controller()
    }

    fn state_manager(&self) -> Arc<dyn StateManager<MutableComponent>> {
        self.base.state_manager()
    }

    fn binding(&self) -> Arc<dyn Binding<MutableComponent>> {
        self.base.binding()
    }

    fn pending_value(&self) -> MutableComponent {
        self.base.pending_value()
    }

    fn request_set(&self, value: MutableComponent) {
        self.base.request_set(value)
    }

    fn add_event_listener(&self, listener: Arc<dyn OptionEventListener<MutableComponent>>) {
        self.base.add_event_listener(listener)
    }

    fn add_listener(&self, changed_listener: Arc<dyn Fn(&dyn YOption<MutableComponent>, MutableComponent)>) {
        self.base.add_listener(changed_listener)
    }
}

impl LabelOption for LabelOptionImpl {
    /// java: @Override public @NotNull Component label()
    fn label(&self) -> MutableComponent {
        self.base.state_manager.get()
    }
}

/// java: @ApiStatus.Internal public static final class BuilderImpl implements LabelOption.Builder
pub struct BuilderImpl {
    /// java: private StateManager<Component> stateManager;
    pub state_manager: Option<Arc<dyn StateManager<MutableComponent>>>,
    /// java: private final List<Component> lines = new ArrayList<>();
    pub lines: Vec<MutableComponent>,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { state_manager: None, lines: Vec::new() }
    }
}

impl Default for BuilderImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder for BuilderImpl {
    /// java: @Override public LabelOption.Builder state(@NotNull StateManager<Component> stateManager)
    fn state(mut self, state_manager: Arc<dyn StateManager<MutableComponent>>) -> Self {
        assert!(self.lines.is_empty(), "Cannot set state manager if lines have already been defined");
        self.state_manager = Some(state_manager);
        self
    }

    /// java: @Override public LabelOption.Builder line(@NotNull Component line)
    fn line(mut self, line: MutableComponent) -> Self {
        assert!(
            self.state_manager.is_none(),
            ".line() is a helper to create a state manager for you at build. If you have defined a custom state manager, do not use .line()"
        );
        self.lines.push(line);
        self
    }

    /// java: @Override public LabelOption.Builder lines(@NotNull Collection<? extends Component> lines)
    fn lines(mut self, lines: Vec<MutableComponent>) -> Self {
        assert!(
            self.state_manager.is_none(),
            ".lines() is a helper to create a state manager for you at build. If you have defined a custom state manager, do not use .lines()"
        );
        self.lines.extend(lines);
        self
    }

    /// java: @Override public LabelOption build()
    fn build(mut self) -> Arc<dyn LabelOption> {
        assert!(self.state_manager.is_some() || !self.lines.is_empty(), "Cannot build label option without a state manager or lines");
        if !self.lines.is_empty() {
            let mut text = component::empty();
            let mut iterator = self.lines.into_iter().peekable();
            while let Some(line) = iterator.next() {
                text = text.append(line);
                if iterator.peek().is_some() {
                    text = text.append_str("\n");
                }
            }
            self.state_manager = Some(state_manager::create_simple(Arc::new(SelfContainedBinding::new_1(text))));
        }
        LabelOptionImpl::new(self.state_manager.expect("checked above"), Vec::new())
    }
}
