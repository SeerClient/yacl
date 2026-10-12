//! Ported from dev.isxander.yacl3.impl.controller.StringControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::string_controller_builder::StringControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller::StringController;

/// java: public class StringControllerBuilderImpl extends AbstractControllerBuilderImpl<String> implements StringControllerBuilder
pub struct StringControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<String>>,
}

impl StringControllerBuilderImpl {
    /// java: public StringControllerBuilderImpl(Option<String> option)
    pub fn new(option: Arc<dyn YOption<String>>) -> Self {
        Self { option }
    }
}

impl ControllerBuilder<String> for StringControllerBuilderImpl {
    /// java: @Override public Controller<String> build()
    fn build(&self) -> Arc<dyn Controller<String>> {
        Arc::new(StringController::new(self.option.clone()))
    }
}

impl StringControllerBuilder for StringControllerBuilderImpl {}
