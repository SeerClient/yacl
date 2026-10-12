//! Ported from dev.isxander.yacl3.api.NameableEnum by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface NameableEnum
/// a functional interface: `getDisplayName()` - a lambda / method reference is this closure type
pub type NameableEnum = Arc<dyn Fn() -> MutableComponent>;
