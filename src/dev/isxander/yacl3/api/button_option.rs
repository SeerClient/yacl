//! Ported from dev.isxander.yacl3.api.ButtonOption.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::button_option_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: `BiConsumer<YACLScreen, ButtonOption>` - a button's action (the option's value type). A struct, not an alias:
/// `ButtonOption`'s own supertrait names this type, and an alias spelling `dyn ButtonOption` there is a cycle.
#[derive(Clone)]
pub struct ButtonAction(pub Arc<dyn Fn(&Arc<YACLScreen>, &dyn ButtonOption)>);

impl ButtonAction {
    /// `action.accept(screen, button)`
    pub fn accept(&self, screen: &Arc<YACLScreen>, button: &dyn ButtonOption) {
        (self.0)(screen, button)
    }
}

/// java: public interface ButtonOption extends Option<BiConsumer<YACLScreen, ButtonOption>>
pub trait ButtonOption: YOption<ButtonAction> {
    /// java: BiConsumer<YACLScreen, ButtonOption> action();
    fn action(&self) -> ButtonAction;
}

/// java: static Builder createBuilder()
pub fn create_builder() -> BuilderImpl {
    BuilderImpl::new()
}

/// java: interface Builder (setters by value, Java's `return this`)
pub trait Builder: Sized {
    /// java: Builder name(@NotNull Component name);
    fn name(self, name: MutableComponent) -> Self;

    /// java: Builder text(@NotNull Component text);
    fn text(self, text: MutableComponent) -> Self;

    /// java: Builder description(@NotNull OptionDescription description);
    fn description(self, description: Arc<dyn OptionDescription>) -> Self;

    /// java: Builder action(@NotNull BiConsumer<YACLScreen, ButtonOption> action);
    fn action(self, action: impl Fn(&Arc<YACLScreen>, &dyn ButtonOption) + 'static) -> Self;

    /// java: @Deprecated Builder action(@NotNull Consumer<YACLScreen> action);
    fn action_consumer(self, action: impl Fn(&Arc<YACLScreen>) + 'static) -> Self;

    /// java: Builder available(boolean available);
    fn available(self, available: bool) -> Self;

    /// java: ButtonOption build();
    fn build(self) -> Arc<dyn ButtonOption>;
}
