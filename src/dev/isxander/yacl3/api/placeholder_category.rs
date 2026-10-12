//! Ported from dev.isxander.yacl3.api.PlaceholderCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::placeholder_category_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface PlaceholderCategory extends ConfigCategory
pub trait PlaceholderCategory: ConfigCategory {
    /// java: BiFunction<Minecraft, YACLScreen, Screen> screen(); (dispatch: the body for this class)
    fn screen(&self) -> fn(&compat::Shared<dyn PlaceholderCategory>) -> Arc<dyn Fn(mcrust::net::minecraft::client::minecraft::Minecraft, compat::Shared<YACLScreen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen>;

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

    /// java: Builder tooltip(@NotNull Component... tooltips); (dispatch: the body for this class)
    fn tooltip(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder>;

    /// java: Builder screen(@NotNull BiFunction<Minecraft, YACLScreen, Screen> screenFunction); (dispatch: the body for this class)
    fn screen(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn(mcrust::net::minecraft::client::minecraft::Minecraft, compat::Shared<YACLScreen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<dyn Builder>;

    /// java: PlaceholderCategory build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn PlaceholderCategory>;

}
