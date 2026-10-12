//! Ported from dev.isxander.yacl3.platform.Env by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public enum Env
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Env {
    Client,
    Server,
}

impl Env {
    pub const VALUES: [Env; 2] = [Env::Client, Env::Server];

    /// java: values()
    pub fn values() -> Vec<Env> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Env::Client => "CLIENT",
            Env::Server => "SERVER",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Env {
        match name {
            "CLIENT" => Env::Client,
            "SERVER" => Env::Server,
            _ => panic!("IllegalArgumentException: No enum constant Env.{name}"),
        }
    }

    /// java: public boolean isClient()
    pub fn is_client(&self) -> bool {
        return (*self == Env::Client);
    }

}

impl std::fmt::Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
