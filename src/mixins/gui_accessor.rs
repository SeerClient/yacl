//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.GuiAccessor (the 26.2 branch).
//! @Mixin(net.minecraft.client.gui.Gui)
//!
//! `@Accessor("screen") void yacl$setScreen(Screen screen)` - a raw write of `Gui.screen`, skipping
//! `Gui.setScreen`'s `removed()` / `init` / `added()`. YACL's one caller is `PopupControllerScreen.onClose`, which puts
//! the YACL screen the popup was drawn over back without re-initialising it.
//!
//! The port keeps `Gui.screen` in the client loop's screen stack, which a mod reaches only through `Gui.setScreen`
//! requests (`mcrust::modding::game::set_screen*`). The write is made as that request, and it is marked: the screen it
//! puts back skips the `init(w, h)` the request runs on it (`YACLScreenAdapter::reposition_elements` asks
//! `consume_raw_screen_write`). rust: port gap (GAPS_Y2.md G6) - the port still calls `added()` on it (YACL's screens do
//! not override it).

#[allow(unused_imports)]
use crate::prelude::*;
use std::cell::Cell;

thread_local! {
    /// A raw `Gui.screen` write is in flight: the screen it puts up runs no `init`.
    static RAW_SCREEN_WRITE: Cell<bool> = const { Cell::new(false) };
}

/// @Accessor yacl$setScreen -> net.minecraft.client.gui.Gui.screen
pub fn yacl_set_screen(screen: compat::ScreenRef) {
    RAW_SCREEN_WRITE.with(|flag| flag.set(true));
    screen.set();
}

/// Whether the screen going up now was put there by `yacl_set_screen` (once: the mark is cleared).
pub fn consume_raw_screen_write() -> bool {
    RAW_SCREEN_WRITE.with(|flag| flag.replace(false))
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this. (An accessor registers nothing.)
pub fn register() {}
