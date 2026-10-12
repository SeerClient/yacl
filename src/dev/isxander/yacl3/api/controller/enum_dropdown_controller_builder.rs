//! Ported from dev.isxander.yacl3.api.controller.EnumDropdownControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::r#impl::controller::enum_dropdown_controller_builder_impl::EnumDropdownControllerBuilderImpl;

/// java: public interface EnumDropdownControllerBuilder<E extends Enum<E>> extends ValueFormattableController<E, EnumDropdownControllerBuilder<E>>
pub trait EnumDropdownControllerBuilder<E: compat::JavaEnum>: ValueFormattableController<E> {}

/// java: static <E extends Enum<E>> EnumDropdownControllerBuilder<E> create(Option<E> option)
pub fn create<E: compat::JavaEnum>(option: Arc<dyn YOption<E>>) -> EnumDropdownControllerBuilderImpl<E> {
    EnumDropdownControllerBuilderImpl::new(option)
}
