//! Ported from dev.isxander.yacl3.config.v2.api.SerialField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface SerialField
pub trait SerialField: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: String serialName(); (dispatch: the body for this class)
    fn serial_name(&self) -> fn(&compat::Shared<dyn SerialField>) -> String;

    /// java: Optional<String> comment(); (dispatch: the body for this class)
    fn comment(&self) -> fn(&compat::Shared<dyn SerialField>) -> Option<String>;

    /// java: boolean required(); (dispatch: the body for this class)
    fn required(&self) -> fn(&compat::Shared<dyn SerialField>) -> bool;

    /// java: boolean nullable(); (dispatch: the body for this class)
    fn nullable(&self) -> fn(&compat::Shared<dyn SerialField>) -> bool;

}
