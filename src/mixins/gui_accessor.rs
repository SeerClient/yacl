//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.GuiAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.Gui)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor yacl$setScreen -> net.minecraft.client.gui.Gui.screen
pub fn yacl_set_screen(obj: &mcrust::net::minecraft::client::gui::gui::Gui) -> () {
    obj.screen
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
