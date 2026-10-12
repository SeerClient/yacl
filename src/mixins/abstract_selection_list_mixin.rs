//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.AbstractSelectionListMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.AbstractSelectionList)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapOperation modifyChildrenCall
///   java: @WrapOperation(
/// note: @At("FIELD") is not expressible: SeerClient hooks fire at a method's HEAD, RETURN and TAIL, and at a hooked CALLEE inside another hooked method (`within`); field access, `new`, constants and jumps are not hook points
fn modify_children_call(ci: &mut CallbackInfo<'_>, original: &mut Original<'_>) {
    // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
    //   java: return children();
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): @WrapOperation modifyChildrenCall: nothing to register - @At("FIELD") is not expressible: SeerClient hooks fire at a method's HEAD, RETURN and TAIL, and at a hooked CALLEE inside another hooked method (`within`); field access, `new`, constants and jumps are not hook points
}
