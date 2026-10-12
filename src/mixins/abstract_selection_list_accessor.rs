//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.AbstractSelectionListAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.AbstractSelectionList)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor getChildren -> net.minecraft.client.gui.components.AbstractSelectionList.children
// TODO(fabric2seer): `List<?>` - return `&obj.children` with its Rust type
// pub fn get_children(obj: &mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList) -> &T { &obj.children }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
