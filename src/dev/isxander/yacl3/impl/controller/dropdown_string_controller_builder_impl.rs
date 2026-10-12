//! Ported from dev.isxander.yacl3.impl.controller.DropdownStringControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::dropdown_string_controller_builder::DropdownStringControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::string_controller_builder::StringControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::dropdown_string_controller::DropdownStringController;
use crate::dev::isxander::yacl3::r#impl::controller::string_controller_builder_impl::StringControllerBuilderImpl;

/// java: public class DropdownStringControllerBuilderImpl extends StringControllerBuilderImpl implements DropdownStringControllerBuilder
pub struct DropdownStringControllerBuilderImpl {
    /// java: extends StringControllerBuilderImpl
    pub base: StringControllerBuilderImpl,
    /// java: private List<String> values;
    pub values: Vec<String>,
    /// java: private boolean allowEmptyValue = false;
    pub allow_empty_value: bool,
    /// java: private boolean allowAnyValue = false;
    pub allow_any_value: bool,
}

impl DropdownStringControllerBuilderImpl {
    /// java: public DropdownStringControllerBuilderImpl(Option<String> option)
    pub fn new(option: Arc<dyn YOption<String>>) -> Self {
        Self { base: StringControllerBuilderImpl::new(option), values: Vec::new(), allow_empty_value: false, allow_any_value: false }
    }
}

impl DropdownStringControllerBuilder for DropdownStringControllerBuilderImpl {
    /// java: @Override public DropdownStringControllerBuilder values(List<String> values)
    fn values(mut self, values: Vec<String>) -> Self {
        self.values = values;
        self
    }

    /// java: @Override public DropdownStringControllerBuilderImpl values(String... values)
    fn values_string(mut self, values: Vec<String>) -> Self {
        self.values = values;
        self
    }

    /// java: @Override public DropdownStringControllerBuilderImpl allowEmptyValue(boolean allowEmptyValue)
    fn allow_empty_value(mut self, allow_empty_value: bool) -> Self {
        self.allow_empty_value = allow_empty_value;
        self
    }

    /// java: @Override public DropdownStringControllerBuilderImpl allowAnyValue(boolean allowAnyValue)
    fn allow_any_value(mut self, allow_any_value: bool) -> Self {
        self.allow_any_value = allow_any_value;
        self
    }
}

impl ControllerBuilder<String> for DropdownStringControllerBuilderImpl {
    /// java: @Override public Controller<String> build()
    fn build(&self) -> Arc<dyn Controller<String>> {
        DropdownStringController::new(self.base.option.clone(), self.values.clone(), self.allow_empty_value, self.allow_any_value)
    }
}

impl StringControllerBuilder for DropdownStringControllerBuilderImpl {}
