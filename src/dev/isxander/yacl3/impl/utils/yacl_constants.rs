//! Ported from dev.isxander.yacl3.impl.utils.YACLConstants.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::mcrust::modding::log::Logger;

/// java: public static final Logger LOGGER = LoggerFactory.getLogger("YetAnotherConfigLib");
pub static LOGGER: std::sync::LazyLock<Logger> = std::sync::LazyLock::new(|| Logger::new("YetAnotherConfigLib"));

/// java: public class YACLConstants
pub struct YACLConstants;

impl YACLConstants {
    /// java: public static final Logger LOGGER = LoggerFactory.getLogger("YetAnotherConfigLib");
    pub fn logger() -> Logger {
        Logger::new("YetAnotherConfigLib")
    }
}
