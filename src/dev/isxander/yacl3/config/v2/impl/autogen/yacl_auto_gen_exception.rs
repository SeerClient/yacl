//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.YACLAutoGenException by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::lang::throwable::Throwable;

/// java: public class YACLAutoGenException extends RuntimeException
///
/// A Java exception is a Rust panic in this crate (`panic!("YACLAutoGenException: ..")`, caught by `catch_unwind`
/// where Java catches); [`YACLAutoGenException::throw`] is `throw new YACLAutoGenException(..)`.
#[derive(Clone, Debug)]
pub struct YACLAutoGenException {
    /// java: extends RuntimeException - the exception's own `Throwable` state (message, cause)
    pub base: Throwable,
}

impl std::ops::Deref for YACLAutoGenException {
    type Target = Throwable;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for YACLAutoGenException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl YACLAutoGenException {
    /// java: public YACLAutoGenException(String message)
    pub fn new(message: String) -> Self {
        Self { base: Throwable::of_class("dev.isxander.yacl3.config.v2.impl.autogen.YACLAutoGenException", Some(message)) }
    }

    /// java: public YACLAutoGenException(String message, Throwable e)
    pub fn new_2(message: String, e: Throwable) -> Self {
        let mut this = Self::new(message);
        this.base.init_cause(e);
        this
    }

    /// java: throw this
    pub fn throw(self) -> ! {
        match self.base.get_cause().and_then(|c| c.get_message().map(|m| m.to_string())) {
            Some(cause) => panic!("YACLAutoGenException: {} (caused by: {})", self.base.get_message().unwrap_or(""), cause),
            None => panic!("YACLAutoGenException: {}", self.base.get_message().unwrap_or("")),
        }
    }
}

impl std::fmt::Display for YACLAutoGenException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.base.get_class_name(), self.base.get_message().unwrap_or(""))
    }
}
