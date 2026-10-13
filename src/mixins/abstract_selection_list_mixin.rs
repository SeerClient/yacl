//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.AbstractSelectionListMixin.
//! @Mixin(net.minecraft.client.gui.components.AbstractSelectionList)
//!
//! `@WrapOperation` of the `children` FIELD read inside `nextEntry(ScreenDirection, Predicate, Entry)`: "Mojang use the
//! field access of children to get max index to loop through keyboard navigation to find the next entry. YACL modifies
//! these children() method to filter out hidden entries, so we need to redirect the field access to the method, so we
//! don't get ArrayIndexOutOfBoundsException."
//!
//! The YACL list is the mod's own (`YACLSelectionList`), whose `next_entry` reads the list through its one accessor
//! (`child` / `child_count`) - the method and the field are the same list there, so the wrap is the identity: the
//! function below is the redirect's body, applied by construction.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::yacl_selection_list::{Entry, YACLSelectionList};

/// @WrapOperation modifyChildrenCall - `return children();`
pub fn modify_children_call<E: ?Sized + Entry>(instance: &YACLSelectionList<E>) -> Vec<Arc<E>> {
    instance.children()
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this. (Nothing to register: see the header.)
pub fn register() {}
