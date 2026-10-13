//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.ContainerEventHandlerMixin.
//! @Mixin(net.minecraft.client.gui.components.events.ContainerEventHandler)
//!
//! `@Redirect` of `children()` inside `nextFocusPathVaguelyInDirection` / `nextFocusPathInDirection`: on a
//! HORIZONTAL move the tab bar is not a focus candidate, so arrowing right from the option list reaches the action
//! buttons / description and not the always-at-the-top tab bar.
//!
//! rust: port gap (GAPS_Y2.md G7) - the port's arrow navigation (`screen.rs`, `container_event_handler.rs`) walks
//! the screen's widget tree in functions with no hook point for the `children()` call, and the YACL screen draws and
//! dispatches its own widgets (PATTERNS_Y2.md P3) with no arrow navigation of its own yet. The redirect's body is
//! below as the filter a focus walk applies to its candidates; nothing registers it.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::navigation::screen_axis::ScreenAxis;
use mcrust::net::minecraft::client::gui::navigation::screen_direction::ScreenDirection;

/// @Redirect modifyFocusCandidates - `children` is `instance.children()`; `is_tab_navigation_bar` is
/// `child instanceof TabNavigationBar`.
pub fn modify_focus_candidates<C: Clone>(
    children: &[C],
    direction: ScreenDirection,
    is_tab_navigation_bar: impl Fn(&C) -> bool,
) -> Vec<C> {
    if direction.get_axis() == ScreenAxis::Horizontal {
        return children.iter().filter(|child| !is_tab_navigation_bar(child)).cloned().collect();
    }
    children.to_vec()
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this. (Nothing to register: see the header.)
pub fn register() {}
