//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.MinecraftMixin (the 26.2 branch).
//! @Mixin(net.minecraft.client.Minecraft)
//!
//! `@Inject(method = "exitWorldAndClose", at = @At(value = "INVOKE", target = "Minecraft.close()V"))`: before the
//! call of `close()` inside `exitWorldAndClose`. The port runs `Minecraft.close` (hooked) right where
//! `exitWorldAndClose` calls it (`minecraft.rs`, after `run()` returns) and from nowhere else, so the call site is
//! the HEAD of `close` - the same instant.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer_manager::ImageRendererManager;
use mcrust::mcrust::modding::hook::{self, At, CallbackInfo};

/// @Inject closeImages
fn close_images(_ci: &mut CallbackInfo<'_>) {
    ImageRendererManager::close_all();
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    if let Err(error) = hook::inject("net.minecraft.client.Minecraft#close", At::Head, close_images) {
        crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER
            .warn(&format!("YACL: could not inject MinecraftMixin.closeImages: {error:?}"));
    }
}
