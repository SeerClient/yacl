//! Ported from dev.isxander.yacl3.api.controller.DropdownStringControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::string_controller_builder::StringControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::dropdown_string_controller_builder_impl::DropdownStringControllerBuilderImpl;

/// java: public interface DropdownStringControllerBuilder extends StringControllerBuilder
pub trait DropdownStringControllerBuilder: StringControllerBuilder {
    /// java: DropdownStringControllerBuilder values(List<String> values); (dispatch: the body for this class)
    fn values(&self) -> fn(&compat::Shared<dyn DropdownStringControllerBuilder>, Vec<String>) -> compat::Shared<dyn DropdownStringControllerBuilder>;

    /// java: DropdownStringControllerBuilder values(String... values); (dispatch: the body for this class)
    fn values_string(&self) -> fn(&compat::Shared<dyn DropdownStringControllerBuilder>, &Vec<String>) -> compat::Shared<dyn DropdownStringControllerBuilder>;

    /// java: DropdownStringControllerBuilder allowEmptyValue(boolean allowEmptyValue); (dispatch: the body for this class)
    fn allow_empty_value(&self) -> fn(&compat::Shared<dyn DropdownStringControllerBuilder>, bool) -> compat::Shared<dyn DropdownStringControllerBuilder>;

    /// java: DropdownStringControllerBuilder allowAnyValue(boolean allowAnyValue); (dispatch: the body for this class)
    fn allow_any_value(&self) -> fn(&compat::Shared<dyn DropdownStringControllerBuilder>, bool) -> compat::Shared<dyn DropdownStringControllerBuilder>;

}

/// java: static DropdownStringControllerBuilder create(Option<String> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<String>>) -> compat::Shared<dyn DropdownStringControllerBuilder> {
    return (DropdownStringControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn DropdownStringControllerBuilder>);
}
