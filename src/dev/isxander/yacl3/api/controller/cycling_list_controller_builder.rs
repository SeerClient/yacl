//! Ported from dev.isxander.yacl3.api.controller.CyclingListControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::r#impl::controller::cycling_list_controller_builder_impl::CyclingListControllerBuilderImpl;

/// java: public interface CyclingListControllerBuilder<T> extends ValueFormattableController<T, CyclingListControllerBuilder<T>>
pub trait CyclingListControllerBuilder<T: Clone + 'static>: ValueFormattableController {
    /// java: @SuppressWarnings("unchecked") CyclingListControllerBuilder<T> values(T... values);
    fn values(&mut self, values: &Vec<T>) -> compat::Shared<dyn CyclingListControllerBuilder<T>>;

    /// java: CyclingListControllerBuilder<T> values(Iterable<? extends T> values);
    fn values_iterable(&mut self, values: Vec<T>) -> compat::Shared<dyn CyclingListControllerBuilder<T>>;

}

/// java: static <T> CyclingListControllerBuilder<T> create(Option<T> option)
pub fn create<T: 'static>(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> compat::Shared<dyn CyclingListControllerBuilder<T>> {
    return (compat::shared(CyclingListControllerBuilderImpl::new(option.clone())) as compat::Shared<dyn CyclingListControllerBuilder<compat::Object>>);
}
