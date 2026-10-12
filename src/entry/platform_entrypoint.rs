//! Ported from dev.isxander.yacl3.platform.PlatformEntrypoint by fabric2seer. Entrypoint: client.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::dev::isxander::yacl3::platform::yacl_config;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler as _;


pub fn on_initialize_client() {
    yacl_config::HANDLER.get_mut().lock().borrow_mut().load();
    // TODO(fabric2seer): Fabric API `ResourceLoader.get` has no SeerClient equivalent
    //   java: ResourceLoader.get(PackType.CLIENT_RESOURCES).registerReloadListener(YACLImageReloadListener.getId(), new YACLImageReloadListener());
}
