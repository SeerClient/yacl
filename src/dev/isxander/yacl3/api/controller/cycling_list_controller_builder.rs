//! Ported from dev.isxander.yacl3.api.controller.CyclingListControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::r#impl::controller::cycling_list_controller_builder_impl::CyclingListControllerBuilderImpl;

/// java: public interface CyclingListControllerBuilder<T> extends ValueFormattableController<T, CyclingListControllerBuilder<T>>
pub trait CyclingListControllerBuilder<T: Clone + 'static>: ValueFormattableController<T> {
    /// java: @SuppressWarnings("unchecked") CyclingListControllerBuilder<T> values(T... values);
    fn values(self, values: Vec<T>) -> Self;

    /// java: CyclingListControllerBuilder<T> values(Iterable<? extends T> values);
    fn values_iterable(self, values: impl IntoIterator<Item = T>) -> Self;
}

/// java: static <T> CyclingListControllerBuilder<T> create(Option<T> option)
pub fn create<T: Clone + PartialEq + 'static>(option: Arc<dyn YOption<T>>) -> CyclingListControllerBuilderImpl<T> {
    CyclingListControllerBuilderImpl::new(option)
}
