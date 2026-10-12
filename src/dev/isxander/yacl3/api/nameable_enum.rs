//! Ported from dev.isxander.yacl3.api.NameableEnum.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface NameableEnum
///
/// `value instanceof NameableEnum nameableEnum` (EnumController's default formatter) is
/// `compat::JavaEnum::nameable_display_name`, which an enum that implements this answers with
/// `Some(self.get_display_name())`.
pub trait NameableEnum {
    /// java: Component getDisplayName();
    fn get_display_name(&self) -> MutableComponent;
}
