//! Ported from dev.isxander.yacl3.api.controller.EnumDropdownControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::r#impl::controller::enum_dropdown_controller_builder_impl::EnumDropdownControllerBuilderImpl;

/// java: public interface EnumDropdownControllerBuilder<E extends Enum<E>> extends ValueFormattableController<E, EnumDropdownControllerBuilder<E>>
pub trait EnumDropdownControllerBuilder<E: Clone + 'static>: ValueFormattableController {
}

/// java: static <E extends Enum<E>> EnumDropdownControllerBuilder<E> create(Option<E> option)
pub fn create<E: 'static>(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<E>>) -> compat::Shared<dyn EnumDropdownControllerBuilder<E>> {
    return (compat::shared(EnumDropdownControllerBuilderImpl::new(option.clone())) as compat::Shared<dyn EnumDropdownControllerBuilder<compat::Object>>);
}
