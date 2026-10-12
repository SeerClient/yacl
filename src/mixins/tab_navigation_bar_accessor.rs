//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.TabNavigationBarAccessor by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.tabs.TabNavigationBar)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor yacl$getTabManager -> net.minecraft.client.gui.components.tabs.TabNavigationBar.tabManager
// TODO(fabric2seer): `TabManager` - return `&obj.tab_manager` with its Rust type
// pub fn yacl_get_tab_manager(obj: &mcrust::net::minecraft::client::gui::components::tabs::tab_navigation_bar::TabNavigationBar) -> &T { &obj.tab_manager }

/// @Accessor yacl$getTabs -> net.minecraft.client.gui.components.tabs.TabNavigationBar.tabs
// TODO(fabric2seer): `ImmutableList<Tab>` - return `&obj.tabs` with its Rust type
// pub fn yacl_get_tabs(obj: &mcrust::net::minecraft::client::gui::components::tabs::tab_navigation_bar::TabNavigationBar) -> &T { &obj.tabs }

/// @Accessor yacl$getTabButtons -> net.minecraft.client.gui.components.tabs.TabNavigationBar.tabButtons
// TODO(fabric2seer): `ImmutableList<TabButton>` - return `&obj.tab_buttons` with its Rust type
// pub fn yacl_get_tab_buttons(obj: &mcrust::net::minecraft::client::gui::components::tabs::tab_navigation_bar::TabNavigationBar) -> &T { &obj.tab_buttons }

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
