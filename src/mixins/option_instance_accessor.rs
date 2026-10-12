//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.OptionInstanceAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.OptionInstance)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getInitialValue -> net.minecraft.client.OptionInstance.initialValue
// TODO(fabric2seer): `T` - return `&obj.initial_value` with its Rust type
// pub fn get_initial_value(obj: &mcrust::net::minecraft::client::option_instance::OptionInstance) -> &T { &obj.initial_value }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
