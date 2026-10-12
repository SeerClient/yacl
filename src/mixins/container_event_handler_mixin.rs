//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.ContainerEventHandlerMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.components.events.ContainerEventHandler)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Redirect modifyFocusCandidates
///   java: @Redirect(
/// target: net.minecraft.client.gui.components.events.ContainerEventHandler#children()Ljava/util/List;  [no Rust counterpart]
/// within: net.minecraft.client.gui.components.events.ContainerEventHandler#nextFocusPathVaguelyInDirection
/// note: no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target) - the @At target of a call site must be a hooked function (hook_table.rs)
/// note: `within` net.minecraft.client.gui.components.events.ContainerEventHandler#nextFocusPathVaguelyInDirection: no Rust counterpart - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
fn modify_focus_candidates(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::gui::components::events::container_event_handler::ContainerEventHandler>()) else { return; };
    // TODO(fabric2seer): the condition: `direction`: this argument is not visible to the injector (see the notes above the handler)
    //   java: direction.getAxis() == ScreenAxis.HORIZONTAL
    if todo!("fabric2seer: the condition (TODO above)") {
        // TODO(fabric2seer): `ContainerEventHandler.children` has no Rust method in the port (looked for `children` in ContainerEventHandler and its supertypes)
        // rust: ci.set_return_value(this.children().into_iter().filter(|child| !(child.as_any().is::<mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget>())).collect::<Vec<_>>());
        // rust: return;
        return todo!("fabric2seer: see the TODO above");
    }
    // TODO(fabric2seer): `ContainerEventHandler.children` has no Rust method in the port (looked for `children` in ContainerEventHandler and its supertypes)
    // rust: ci.set_return_value(this.children());
    // rust: return;
    return todo!("fabric2seer: see the TODO above");
}

/// @Redirect modifyFocusCandidates
///   java: @Redirect(
/// target: net.minecraft.client.gui.components.events.ContainerEventHandler#children()Ljava/util/List;  [no Rust counterpart]
/// within: net.minecraft.client.gui.components.events.ContainerEventHandler#nextFocusPathInDirection
/// note: no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target) - the @At target of a call site must be a hooked function (hook_table.rs)
/// note: `within` net.minecraft.client.gui.components.events.ContainerEventHandler#nextFocusPathInDirection: no Rust counterpart - the caller must be hooked for `within` to see it on the stack; the registration is left commented out
fn modify_focus_candidates_1(ci: &mut CallbackInfo<'_>) {
    let Some(this) = compat::detach(ci.this_mut::<mcrust::net::minecraft::client::gui::components::events::container_event_handler::ContainerEventHandler>()) else { return; };
    // TODO(fabric2seer): the condition: `direction`: this argument is not visible to the injector (see the notes above the handler)
    //   java: direction.getAxis() == ScreenAxis.HORIZONTAL
    if todo!("fabric2seer: the condition (TODO above)") {
        // TODO(fabric2seer): `ContainerEventHandler.children` has no Rust method in the port (looked for `children` in ContainerEventHandler and its supertypes)
        // rust: ci.set_return_value(this.children().into_iter().filter(|child| !(child.as_any().is::<mcrust::net::minecraft::client::gui::components::abstract_container_widget::AbstractContainerWidget>())).collect::<Vec<_>>());
        // rust: return;
        return todo!("fabric2seer: see the TODO above");
    }
    // TODO(fabric2seer): `ContainerEventHandler.children` has no Rust method in the port (looked for `children` in ContainerEventHandler and its supertypes)
    // rust: ci.set_return_value(this.children());
    // rust: return;
    return todo!("fabric2seer: see the TODO above");
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target) - the @At target of a call site must be a hooked function (hook_table.rs)
    // report(hook::call_site(`net.minecraft.client.gui.components.events.ContainerEventHandler#nextFocusPathVaguelyInDirection`, `net.minecraft.client.gui.components.events.ContainerEventHandler#children()Ljava/util/List;`).priority(1000).redirect(modify_focus_candidates));
    // TODO(fabric2seer): not registered - no Rust counterpart: hook_coverage.py: NO RUST - the port has no Rust body for this Java method, so a hook there has nothing to land on - the registration is left commented out (it would never fire, and `seer check` refuses an unhooked target) - the @At target of a call site must be a hooked function (hook_table.rs)
    // report(hook::call_site(`net.minecraft.client.gui.components.events.ContainerEventHandler#nextFocusPathInDirection`, `net.minecraft.client.gui.components.events.ContainerEventHandler#children()Ljava/util/List;`).priority(1000).redirect(modify_focus_candidates_1));
}
