//! Ported from dev.isxander.yacl3.config.v2.api.autogen.AutoGenField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface AutoGenField
pub trait AutoGenField: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: String category(); (dispatch: the body for this class)
    fn category(&self) -> fn(&compat::Shared<dyn AutoGenField>) -> String;

    /// java: Optional<String> group(); (dispatch: the body for this class)
    fn group(&self) -> fn(&compat::Shared<dyn AutoGenField>) -> Option<String>;

}
