//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.YACLAutoGenException by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::lang::throwable::Throwable;

/// java: public class YACLAutoGenException extends RuntimeException
pub struct YACLAutoGenException {
    /// java: extends java.lang.RuntimeException
    pub base: crate::unresolved::RuntimeException,
}

impl std::ops::Deref for YACLAutoGenException {
    type Target = crate::unresolved::RuntimeException;
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
    pub fn new(mut message: String) -> Self {
        // TODO(fabric2seer): `new RuntimeException(..)`: the port has no `RuntimeException::new`
        //   java: super(message);
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: public YACLAutoGenException(String message, Throwable e)
    pub fn new_2(mut message: String, mut e: Throwable) -> Self {
        // TODO(fabric2seer): `new RuntimeException(..)`: the port has no `RuntimeException::new`
        //   java: super(message, e);
        let mut this: Self = todo!("constructor");
        this
    }

}
