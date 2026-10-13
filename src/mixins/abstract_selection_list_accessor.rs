//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.AbstractSelectionListAccessor.
//! @Mixin(net.minecraft.client.gui.components.AbstractSelectionList)
//!
//! `@Accessor List<?> getChildren()` - the list's modifiable `children` field. YACL's one caller is
//! `OptionListWidget.superModifiableChildren()`; the YACL list is the mod's own (`YACLSelectionList`, PATTERNS_Y2.md
//! P3), so the accessor is a function over it: the field is `YACLSelectionList::insert_child` / `children`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::yacl_selection_list::{Entry, YACLSelectionList};

/// @Accessor getChildren -> AbstractSelectionList.children (a copy of the list: an entry's handler may change it)
pub fn get_children<E: ?Sized + Entry>(list: &YACLSelectionList<E>) -> Vec<Arc<E>> {
    list.children()
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this. (An accessor registers nothing.)
pub fn register() {}
