//! Ported from dev.isxander.yacl3.config.v2.api.autogen.AutoGenField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface AutoGenField
///
/// Backing interface for the `@AutoGen` annotation.
pub trait AutoGenField: compat::DynAny {
    /// java: String category();
    fn category(&self) -> String;

    /// java: Optional<String> group();
    fn group(&self) -> Option<String>;
}
