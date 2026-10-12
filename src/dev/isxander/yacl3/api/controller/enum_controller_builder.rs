//! Ported from dev.isxander.yacl3.api.controller.EnumControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::r#impl::controller::enum_controller_builder_impl::EnumControllerBuilderImpl;

/// java: public interface EnumControllerBuilder<T extends Enum<T>> extends ValueFormattableController<T, EnumControllerBuilder<T>>
pub trait EnumControllerBuilder<T: Clone + 'static>: ValueFormattableController {
    /// java: EnumControllerBuilder<T> enumClass(Class<T> enumClass);
    fn enum_class(&mut self, enum_class: compat::ClassOf<T>) -> compat::Shared<dyn EnumControllerBuilder<T>>;

}

/// java: static <T extends Enum<T>> EnumControllerBuilder<T> create(Option<T> option)
pub fn create<T: 'static>(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> compat::Shared<dyn EnumControllerBuilder<T>> {
    return (compat::shared(EnumControllerBuilderImpl::new(option.clone())) as compat::Shared<dyn EnumControllerBuilder<compat::Object>>);
}
