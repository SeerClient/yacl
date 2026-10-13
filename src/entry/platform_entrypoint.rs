//! Ported from dev.isxander.yacl3.platform.PlatformEntrypoint (the fabric branch). Entrypoint: client.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::platform::yacl_config;

/// java: @Override public void onInitializeClient()
pub fn on_initialize_client() {
    yacl_config::HANDLER.get().load();
    // java: ResourceLoader.get(PackType.CLIENT_RESOURCES).registerReloadListener(YACLImageReloadListener.getId(), new YACLImageReloadListener());
    // rust: port gap (GAPS_Y2.md G5) - SeerClient has no mod-side resource reload listener registration; the
    // listener only preloads .webp/.gif images, which then load on demand (ImageRendererManager) instead.
}
