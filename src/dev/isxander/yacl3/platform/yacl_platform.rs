//! Ported from dev.isxander.yacl3.platform.YACLPlatform (the 26.2-fabric branch).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::platform::env::Env;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public final class YACLPlatform
pub struct YACLPlatform;

impl YACLPlatform {
    /// java: public static Identifier parseRl(String rl)
    pub fn parse_rl(rl: &str) -> Identifier {
        Identifier::parse(rl)
    }

    /// java: public static Identifier rl(String path)
    pub fn rl(path: &str) -> Identifier {
        YACLPlatform::rl_2("yet_another_config_lib_v3", path)
    }

    /// java: public static Identifier rl(String namespace, String path)
    pub fn rl_2(namespace: &str, path: &str) -> Identifier {
        Identifier::from_namespace_and_path(namespace, path)
    }

    /// java: public static Identifier mcRl(String path)
    pub fn mc_rl(path: &str) -> Identifier {
        YACLPlatform::rl_2("minecraft", path)
    }

    /// java: public static Env getEnvironment() - `FabricLoader.getEnvironmentType()`: SeerClient runs mods on the
    /// client only.
    pub fn get_environment() -> Env {
        Env::Client
    }

    /// java: public static Path getConfigDir() - `FabricLoader.getConfigDir()`
    pub fn get_config_dir() -> std::path::PathBuf {
        mcrust::mcrust::modding::config::config_dir()
    }

    /// java: public static boolean isDevelopmentEnv() - `FabricLoader.isDevelopmentEnvironment()`: a SeerClient game
    /// is a production install.
    pub fn is_development_env() -> bool {
        false
    }
}
