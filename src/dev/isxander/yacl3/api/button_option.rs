//! Ported from dev.isxander.yacl3.api.ButtonOption by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::button_option_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface ButtonOption extends Option<BiConsumer<YACLScreen, ButtonOption>>
pub trait ButtonOption: crate::dev::isxander::yacl3::api::option::Option + compat::DynAny {
    /// java: BiConsumer<YACLScreen, ButtonOption> action(); (dispatch: the body for this class)
    fn action(&self) -> fn(&compat::Shared<dyn ButtonOption>) -> Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>;

}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (BuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder
pub trait Builder: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Builder name(@NotNull Component name); (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder text(@NotNull Component text); (dispatch: the body for this class)
    fn text(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder description(@NotNull OptionDescription description); (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn Builder>, compat::Shared<dyn OptionDescription>) -> compat::Shared<dyn Builder>;

    /// java: Builder action(@NotNull BiConsumer<YACLScreen, ButtonOption> action); (dispatch: the body for this class)
    fn action(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn(compat::Shared<YACLScreen>, compat::Shared<dyn ButtonOption>)>) -> compat::Shared<dyn Builder>;

    /// java: @Deprecated Builder action(@NotNull Consumer<YACLScreen> action); (dispatch: the body for this class)
    fn action_consumer(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn(compat::Shared<YACLScreen>)>) -> compat::Shared<dyn Builder>;

    /// java: Builder available(boolean available); (dispatch: the body for this class)
    fn available(&self) -> fn(&compat::Shared<dyn Builder>, bool) -> compat::Shared<dyn Builder>;

    /// java: ButtonOption build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn ButtonOption>;

}
