//! Ported from dev.isxander.yacl3.api.OptionFlag by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::require_restart_screen::RequireRestartScreen;
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;

/// java: OptionFlag GAME_RESTART = client -> GuiUtils.setScreen(new RequireRestartScreen(GuiUtils.getCurrentScreen()));
pub static GAME_RESTART: compat::MainThread<compat::Shared<dyn OptionFlag>> = compat::MainThread::new(|| Arc::new(move |client| GuiUtils::set_screen_1(&RequireRestartScreen::new(GuiUtils::get_current_screen()))));
// TODO(fabric2seer): field `levelExtractor` of a value of unknown type
//   java: OptionFlag RELOAD_CHUNKS =
//   java:
//   java:             client -> client.levelExtractor.allChanged();
/// java: OptionFlag RELOAD_CHUNKS = client -> client.levelExtractor.allChanged();
pub static RELOAD_CHUNKS: compat::MainThread<compat::Shared<dyn OptionFlag>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `RELOAD_CHUNKS` - see the TODO above"));
/// java: @Deprecated OptionFlag WORLD_RENDER_UPDATE = RELOAD_CHUNKS;
pub static WORLD_RENDER_UPDATE: compat::MainThread<compat::Shared<dyn OptionFlag>> = compat::MainThread::new(|| RELOAD_CHUNKS.get_mut().clone());
// TODO(fabric2seer): `.delayTextureReload(..)` on a value of unknown type
//   java: OptionFlag ASSET_RELOAD = Minecraft::delayTextureReload;
/// java: OptionFlag ASSET_RELOAD = Minecraft::delayTextureReload;
pub static ASSET_RELOAD: compat::MainThread<compat::Shared<dyn OptionFlag>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ASSET_RELOAD` - see the TODO above"));

/// java: @FunctionalInterface public interface OptionFlag extends Consumer<Minecraft>
pub trait OptionFlag: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
}
