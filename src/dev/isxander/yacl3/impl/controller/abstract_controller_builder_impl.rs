//! Ported from dev.isxander.yacl3.impl.controller.AbstractControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;

/// java: public abstract class AbstractControllerBuilderImpl<T> implements ControllerBuilder<T>
///
/// Its one field is the option; each builder keeps it itself (`option`), so this struct is that field for a builder
/// that wants it by name.
pub struct AbstractControllerBuilderImpl<T: Clone + 'static> {
    /// java: protected final Option<T> option;
    pub option: Arc<dyn YOption<T>>,
}

impl<T: Clone + 'static> AbstractControllerBuilderImpl<T> {
    /// java: protected AbstractControllerBuilderImpl(Option<T> option)
    pub fn new(option: Arc<dyn YOption<T>>) -> Self {
        Self { option }
    }
}
