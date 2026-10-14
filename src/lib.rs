//! YetAnotherConfigLib - ported from Fabric by fabric2seer (SeerClient native mod).
//! See PORTING_REPORT.md for what was translated and what is left as TODO.
#![allow(unused_imports, unused_mut, unused_variables, unused_parens, dead_code, non_snake_case,
         clippy::all)]

// The game crate. Linked as the prebuilt SDK `mcrust.dll` (`-L crate=<sdk>/bin`);
// an explicit `extern crate` because a command-line `--extern mcrust` breaks
// resolving it through another mod (dandelion -> yacl).
extern crate mcrust;

pub mod compat;
pub mod prelude;
pub mod mixins;
pub mod entry;
pub mod dev;
pub mod unresolved;

use mcrust::mcrust::modding::entrypoint::{ClientModInitializer, ModInitializer, PreLaunchEntrypoint};

/// Registers every ported Mixin (preLaunch: before the game bootstraps, as Mixin applies).
#[derive(Default)]
pub struct Mixins;

impl PreLaunchEntrypoint for Mixins {
    fn on_pre_launch(&self) {
        mixins::abstract_selection_list_accessor::register();
        mixins::abstract_selection_list_mixin::register();
        mixins::gui_accessor::register();
        mixins::minecraft_mixin::register();
        mixins::option_instance_accessor::register();
        mixins::tab_navigation_bar_accessor::register();
        mixins::gui_graphics_extractor_mixin::register();
        mixins::container_event_handler_mixin::register();
    }
}

/// the "client" entrypoint
#[derive(Default)]
pub struct Client;

impl ClientModInitializer for Client {
    fn on_initialize_client(&self) {
        compat::install(); // Minecraft.getInstance() for the translated classes
        entry::platform_entrypoint::on_initialize_client();
    }
}

mcrust::declare_mod! {
    client: [Client],
    pre_launch: [Mixins],
}
