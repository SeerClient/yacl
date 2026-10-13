//! Ported from dev.isxander.yacl3.debug.DebugProperties.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;

/// java: public static final boolean IMAGE_FILTERING = boolProp("imageFiltering", false, false);
/// (Applies GL filtering to rendering images.)
pub static IMAGE_FILTERING: LazyLock<bool> = LazyLock::new(|| DebugProperties::bool_prop("imageFiltering", false, false));

/// java: public final class DebugProperties
pub struct DebugProperties;

impl DebugProperties {
    /// java: private static boolean boolProp(String name, boolean defProd, boolean defDebug)
    ///
    /// `System.getProperty(key, default)`: a JVM system property is a process environment variable here.
    pub fn bool_prop(name: &str, def_prod: bool, def_debug: bool) -> bool {
        let default_value = if YACLPlatform::is_development_env() { def_debug } else { def_prod };
        let value = std::env::var(format!("yacl3.debug.{name}")).unwrap_or_else(|_| default_value.to_string());
        // java: Boolean.parseBoolean
        value.eq_ignore_ascii_case("true")
    }
}
