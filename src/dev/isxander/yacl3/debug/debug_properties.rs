//! Ported from dev.isxander.yacl3.debug.DebugProperties by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;

/// java: public static final boolean IMAGE_FILTERING = boolProp("imageFiltering", false, false);
pub static IMAGE_FILTERING: LazyLock<bool> = LazyLock::new(|| DebugProperties::bool_prop("imageFiltering", false, false));

/// java: public final class DebugProperties
#[derive(Clone)]
pub struct DebugProperties {}

impl DebugProperties {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private static boolean boolProp(String name, boolean defProd, boolean defDebug)
    pub fn bool_prop(name: &str, mut def_prod: bool, mut def_debug: bool) -> bool {
        let mut default_value: bool = (if YACLPlatform::is_development_env() { def_debug } else { def_prod });
        return (std::env::var(&format!("yacl3.debug.{}", name)).ok().unwrap()).eq_ignore_ascii_case("true");
    }

}
