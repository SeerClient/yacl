//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.MinecraftMixin by fabric2seer.
//! @Mixin(net.minecraft.client.Minecraft)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Inject closeImages
///   java: @Inject(
/// target: net.minecraft.client.Minecraft#close  [exists]
/// within: net.minecraft.client.Minecraft#exitWorldAndClose
/// Rust: net/minecraft/client/minecraft.rs:7313 fn close(regional_compliancies: &crate::net::minecraft::client::periodic_notification_manager::PeriodicNotificationManager, narrator: &crate::net::minecraft::client::game_narrator::GameNarrator, timer_query: &mut crate::com::mojang::blaze3d::systems::timer_query::TimerQuery, audio: Option<( crate::l
/// note: @Inject at INVOKE: a call-site injector (`hook::call_site`) - `ci` is the CALL's (its arguments; cancel() skips that call, not the rest of the method)
/// note: `within` net.minecraft.client.Minecraft#exitWorldAndClose: no Rust counterpart - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
fn close_images(ci: &mut CallbackInfo<'_>) {
    crate::dev::isxander::yacl3::gui::image::image_renderer_manager::ImageRendererManager::close_all();
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - `within` net.minecraft.client.Minecraft#exitWorldAndClose: no Rust counterpart - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
    // report(hook::call_site(`net.minecraft.client.Minecraft#exitWorldAndClose`, `net.minecraft.client.Minecraft#close`).priority(1000).inject(Shift::Before, close_images));
}
