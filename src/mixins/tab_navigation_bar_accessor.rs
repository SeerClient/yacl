//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.TabNavigationBarAccessor (the 26.2 branch).
//! @Mixin(net.minecraft.client.gui.components.tabs.TabNavigationBar)
//!
//! `yacl$getTabManager()`, `yacl$getTabs()`, `yacl$getTabButtons()` - the bar's private fields, read by
//! `ScrollableNavigationBar`. The YACL bar's base is `compat::TabNavigationBar` (PATTERNS_Y2.md P3), whose fields are
//! readable; these are the accessors over it.

#[allow(unused_imports)]
use crate::prelude::*;
use compat::{MenuTabButton, Tab, TabManager, TabNavigationBar};

/// @Accessor("tabManager") TabManager yacl$getTabManager();
pub fn yacl_get_tab_manager(obj: &TabNavigationBar) -> Rc<TabManager> {
    obj.tab_manager()
}

/// @Accessor("tabs") ImmutableList<Tab> yacl$getTabs();
pub fn yacl_get_tabs(obj: &TabNavigationBar) -> &[Arc<dyn Tab>] {
    obj.get_tabs()
}

/// @Accessor("tabButtons") ImmutableList<TabButton> yacl$getTabButtons();
pub fn yacl_get_tab_buttons(obj: &TabNavigationBar) -> &[Arc<MenuTabButton>] {
    &obj.tab_buttons
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this. (An accessor registers nothing.)
pub fn register() {}
