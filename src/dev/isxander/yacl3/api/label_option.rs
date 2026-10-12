//! Ported from dev.isxander.yacl3.api.LabelOption.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::r#impl::label_option_impl::{BuilderImpl, LabelOptionImpl};
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface LabelOption extends Option<Component>
pub trait LabelOption: YOption<MutableComponent> {
    /// java: @NotNull Component label();
    fn label(&self) -> MutableComponent;
}

/// java: static LabelOption create(@NotNull Component label)
pub fn create(label: MutableComponent) -> Arc<dyn LabelOption> {
    LabelOptionImpl::new_1(label)
}

/// java: static Builder createBuilder()
pub fn create_builder() -> BuilderImpl {
    BuilderImpl::new()
}

/// java: interface Builder (setters by value, Java's `return this`)
pub trait Builder: Sized {
    /// java: Builder state(@NotNull StateManager<Component> stateManager);
    fn state(self, state_manager: Arc<dyn StateManager<MutableComponent>>) -> Self;

    /// java: Builder line(@NotNull Component line);
    fn line(self, line: MutableComponent) -> Self;

    /// java: Builder lines(@NotNull Collection<? extends Component> lines);
    fn lines(self, lines: Vec<MutableComponent>) -> Self;

    /// java: LabelOption build();
    fn build(self) -> Arc<dyn LabelOption>;
}
