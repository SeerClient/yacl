//! Ported from dev.isxander.yacl3.api.controller.DropdownStringControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::string_controller_builder::StringControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::dropdown_string_controller_builder_impl::DropdownStringControllerBuilderImpl;

/// java: public interface DropdownStringControllerBuilder extends StringControllerBuilder
pub trait DropdownStringControllerBuilder: StringControllerBuilder + Sized {
    /// java: DropdownStringControllerBuilder values(List<String> values);
    fn values(self, values: Vec<String>) -> Self;

    /// java: DropdownStringControllerBuilder values(String... values);
    fn values_string(self, values: Vec<String>) -> Self;

    /// java: DropdownStringControllerBuilder allowEmptyValue(boolean allowEmptyValue);
    fn allow_empty_value(self, allow_empty_value: bool) -> Self;

    /// java: DropdownStringControllerBuilder allowAnyValue(boolean allowAnyValue);
    fn allow_any_value(self, allow_any_value: bool) -> Self;
}

/// java: static DropdownStringControllerBuilder create(Option<String> option)
pub fn create(option: Arc<dyn YOption<String>>) -> DropdownStringControllerBuilderImpl {
    DropdownStringControllerBuilderImpl::new(option)
}
