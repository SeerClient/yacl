//! Ported from dev.isxander.yacl3.config.v2.api.SerialField by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface SerialField
///
/// The backing interface for the `@SerialEntry` annotation.
pub trait SerialField: compat::DynAny {
    /// java: String serialName();
    fn serial_name(&self) -> String;

    /// java: Optional<String> comment();
    fn comment(&self) -> Option<String>;

    /// java: boolean required();
    fn required(&self) -> bool;

    /// java: boolean nullable();
    fn nullable(&self) -> bool;
}
