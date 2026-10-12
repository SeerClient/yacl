//! Ported from dev.isxander.yacl3.api.CustomTabProvider.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;

/// java: public interface CustomTabProvider (a `ConfigCategory` that also implements it answers
/// `ConfigCategory::as_custom_tab_provider`)
pub trait CustomTabProvider {
    /// java: Tab createTab(YACLScreen screen, ScreenRectangle tabArea);
    fn create_tab(&self, screen: &Arc<YACLScreen>, tab_area: ScreenRectangle) -> Arc<dyn compat::Tab>;
}
