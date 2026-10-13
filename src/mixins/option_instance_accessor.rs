//! Ported from the Fabric mixin dev.isxander.yacl3.mixin.OptionInstanceAccessor.
//! @Mixin(net.minecraft.client.OptionInstance)
//!
//! `@Accessor T getInitialValue()` - the port's `OptionInstance` exposes the field (`initial_value()`), which is what
//! `Binding.minecraft(..)` reads.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::option_instance::OptionInstance;

/// @Accessor getInitialValue -> net.minecraft.client.OptionInstance.initialValue
pub fn get_initial_value<T: Clone + PartialEq + std::fmt::Debug + Send + Sync + 'static>(obj: &OptionInstance<T>) -> &T {
    obj.initial_value()
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this. (An accessor registers nothing.)
pub fn register() {}
