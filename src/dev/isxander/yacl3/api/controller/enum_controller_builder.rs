//! Ported from dev.isxander.yacl3.api.controller.EnumControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::r#impl::controller::enum_controller_builder_impl::EnumControllerBuilderImpl;

/// java: public interface EnumControllerBuilder<T extends Enum<T>> extends ValueFormattableController<T, EnumControllerBuilder<T>>
pub trait EnumControllerBuilder<T: compat::JavaEnum>: ValueFormattableController<T> {
    /// java: EnumControllerBuilder<T> enumClass(Class<T> enumClass);
    fn enum_class(self, enum_class: compat::ClassOf<T>) -> Self;
}

/// java: static <T extends Enum<T>> EnumControllerBuilder<T> create(Option<T> option)
pub fn create<T: compat::JavaEnum>(option: Arc<dyn YOption<T>>) -> EnumControllerBuilderImpl<T> {
    EnumControllerBuilderImpl::new(option)
}
