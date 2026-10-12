//! Ported from dev.isxander.yacl3.api.OptionFlag.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::require_restart_screen::RequireRestartScreen;
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;
use mcrust::net::minecraft::client::minecraft::Minecraft;

/// java: @FunctionalInterface public interface OptionFlag extends Consumer<Minecraft>
///
/// Any `Fn(&mut Minecraft)` is one. A flag is held as `Arc<dyn OptionFlag>`; Java's `ImmutableSet<OptionFlag>`
/// is a `Vec` without duplicates by IDENTITY (`Arc::ptr_eq`), as a set of lambdas is (`add_flag`).
pub trait OptionFlag {
    /// java: void accept(Minecraft client);
    fn accept(&self, client: &mut Minecraft);
}

impl<F: Fn(&mut Minecraft)> OptionFlag for F {
    fn accept(&self, client: &mut Minecraft) {
        self(client)
    }
}

thread_local! {
    static GAME_RESTART: Arc<dyn OptionFlag> = Arc::new(|_client: &mut Minecraft| {
        GuiUtils::set_screen_1(compat::ScreenRef::new_keeping_parent(RequireRestartScreen::new(GuiUtils::get_current_screen())));
    });
    static RELOAD_CHUNKS: Arc<dyn OptionFlag> = Arc::new(|client: &mut Minecraft| {
        // java: client.levelExtractor.allChanged() - the port's LevelExtractor.allChanged is GameRenderer.all_changed,
        // handed the level (`LevelExtractor.allChanged:410-411` reads it).
        let level = mcrust::mcrust::modding::game::level();
        client.game_renderer.all_changed(level.as_deref());
    });
    static ASSET_RELOAD: Arc<dyn OptionFlag> = Arc::new(|client: &mut Minecraft| {
        // java: Minecraft::delayTextureReload - `this.pendingReload = ..`, which the loop's `runTick` drains.
        client.pending_reload = true;
    });
}

/// java: OptionFlag GAME_RESTART = client -> GuiUtils.setScreen(new RequireRestartScreen(GuiUtils.getCurrentScreen()));
#[allow(non_snake_case)]
pub fn GAME_RESTART() -> Arc<dyn OptionFlag> {
    GAME_RESTART.with(|f| f.clone())
}

/// java: OptionFlag RELOAD_CHUNKS = client -> client.levelExtractor.allChanged();
#[allow(non_snake_case)]
pub fn RELOAD_CHUNKS() -> Arc<dyn OptionFlag> {
    RELOAD_CHUNKS.with(|f| f.clone())
}

/// java: @Deprecated OptionFlag WORLD_RENDER_UPDATE = RELOAD_CHUNKS;
#[allow(non_snake_case)]
pub fn WORLD_RENDER_UPDATE() -> Arc<dyn OptionFlag> {
    RELOAD_CHUNKS()
}

/// java: OptionFlag ASSET_RELOAD = Minecraft::delayTextureReload;
#[allow(non_snake_case)]
pub fn ASSET_RELOAD() -> Arc<dyn OptionFlag> {
    ASSET_RELOAD.with(|f| f.clone())
}

/// `set.add(flag)` on Java's `HashSet<OptionFlag>` / `ImmutableSet.copyOf(..)`: a flag object is added once.
pub fn add_flag(flags: &mut Vec<Arc<dyn OptionFlag>>, flag: Arc<dyn OptionFlag>) {
    if !flags.iter().any(|f| Arc::ptr_eq(f, &flag)) {
        flags.push(flag);
    }
}
