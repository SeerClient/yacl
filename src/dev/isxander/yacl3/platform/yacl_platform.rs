//! Ported from dev.isxander.yacl3.platform.YACLPlatform by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::platform::env::Env;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public final class YACLPlatform
#[derive(Clone)]
pub struct YACLPlatform {}

impl YACLPlatform {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static Identifier parseRl(String rl)
    pub fn parse_rl(rl: &str) -> Identifier {
        return Identifier::parse(rl);
    }

    /// java: public static Identifier rl(String path)
    pub fn rl(path: &str) -> Identifier {
        return YACLPlatform::rl_2("yet_another_config_lib_v3", path);
    }

    /// java: public static Identifier rl(String namespace, String path)
    pub fn rl_2(namespace: &str, path: &str) -> Identifier {
        return Identifier::from_namespace_and_path(namespace, path);
    }

    /// java: public static Identifier mcRl(String path)
    pub fn mc_rl(path: &str) -> Identifier {
        return YACLPlatform::rl_2("minecraft", path);
    }

    /// java: public static Env getEnvironment()
    pub fn get_environment() -> Env {
        // TODO(fabric2seer): FabricLoader.getEnvironmentType
        //   java: return switch (FabricLoader.getInstance().getEnvironmentType()) {
        //   java:             case CLIENT -> Env.CLIENT;
        //   java:             case SERVER -> Env.SERVER;
        //   java:         };
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static Path getConfigDir()
    pub fn get_config_dir() -> std::path::PathBuf {
        return mcrust::mcrust::modding::config::config_dir();
    }

    /// java: public static boolean isDevelopmentEnv()
    pub fn is_development_env() -> bool {
        return false;
    }

}
