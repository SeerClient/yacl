//! Ported from dev.isxander.yacl3.api.PlaceholderCategory.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::placeholder_category_impl::BuilderImpl;
use mcrust::net::minecraft::client::minecraft::Minecraft;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: `BiFunction<Minecraft, YACLScreen, Screen>` - the screen a placeholder tab opens.
pub type ScreenFunction = Arc<dyn Fn(&mut Minecraft, &Arc<YACLScreen>) -> compat::ScreenRef>;

/// java: public interface PlaceholderCategory extends ConfigCategory
pub trait PlaceholderCategory: ConfigCategory {
    /// java: BiFunction<Minecraft, YACLScreen, Screen> screen();
    fn screen(&self) -> ScreenFunction;
}

/// java: static Builder createBuilder()
pub fn create_builder() -> BuilderImpl {
    BuilderImpl::new()
}

/// java: interface Builder
pub trait Builder: Sized {
    /// java: Builder name(@NotNull Component name);
    fn name(self, name: MutableComponent) -> Self;

    /// java: Builder tooltip(@NotNull Component... tooltips);
    fn tooltip(self, tooltips: Vec<MutableComponent>) -> Self;

    /// java: Builder screen(@NotNull BiFunction<Minecraft, YACLScreen, Screen> screenFunction);
    fn screen(self, screen_function: impl Fn(&mut Minecraft, &Arc<YACLScreen>) -> compat::ScreenRef + 'static) -> Self;

    /// java: PlaceholderCategory build();
    fn build(self) -> Arc<dyn PlaceholderCategory>;
}
