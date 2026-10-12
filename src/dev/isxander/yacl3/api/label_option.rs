//! Ported from dev.isxander.yacl3.api.LabelOption by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::state_manager::StateManager;
use crate::dev::isxander::yacl3::r#impl::label_option_impl::BuilderImpl;
use crate::dev::isxander::yacl3::r#impl::label_option_impl::LabelOptionImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface LabelOption extends Option<Component>
pub trait LabelOption: crate::dev::isxander::yacl3::api::option::Option + compat::DynAny {
    /// java: @NotNull Component label(); (dispatch: the body for this class)
    fn label(&self) -> fn(&compat::Shared<dyn LabelOption>) -> MutableComponent;

}

/// java: static LabelOption create(@NotNull Component label)
pub fn create(label: &MutableComponent) -> compat::Shared<dyn LabelOption> {
    return (LabelOptionImpl::new_1(label.clone()) as compat::Shared<dyn LabelOption>);
}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (BuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder
pub trait Builder: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Builder state(@NotNull StateManager<Component> stateManager); (dispatch: the body for this class)
    fn state(&self) -> fn(&compat::Shared<dyn Builder>, compat::Shared<dyn StateManager<MutableComponent>>) -> compat::Shared<dyn Builder>;

    /// java: Builder line(@NotNull Component line); (dispatch: the body for this class)
    fn line(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder lines(@NotNull Collection<? extends Component> lines); (dispatch: the body for this class)
    fn lines(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder>;

    /// java: LabelOption build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn LabelOption>;

}
