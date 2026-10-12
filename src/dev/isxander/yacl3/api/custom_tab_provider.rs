//! Ported from dev.isxander.yacl3.api.CustomTabProvider by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::client::gui::components::tabs::tab::Tab;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;

/// java: public interface CustomTabProvider
/// a functional interface: `createTab(YACLScreen screen, ScreenRectangle tabArea)` - a lambda / method reference is this closure type
pub type CustomTabProvider = Arc<dyn Fn(compat::Shared<YACLScreen>, ScreenRectangle) -> Box<dyn Tab>>;
