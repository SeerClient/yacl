//! Ported from dev.isxander.yacl3.impl.controller.CyclingListControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::cycling_list_controller_builder::CyclingListControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::gui::controllers::cycling::cycling_list_controller::CyclingListControllerBase;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;

/// java: public final class CyclingListControllerBuilderImpl<T> extends AbstractControllerBuilderImpl<T> implements CyclingListControllerBuilder<T>
#[derive(Clone)]
pub struct CyclingListControllerBuilderImpl<T> {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<T>,
    /// java: private Iterable<? extends T> values;
    pub values: Vec<T>,
    /// java: private ValueFormatter<T> formatter = null;
    pub formatter: Option<compat::Shared<dyn ValueFormatter<T>>>,
}

impl<T: 'static> std::ops::Deref for CyclingListControllerBuilderImpl<T> {
    type Target = AbstractControllerBuilderImplBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for CyclingListControllerBuilderImpl<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> CyclingListControllerBuilderImpl<T> {
    /// java: public CyclingListControllerBuilderImpl(Option<T> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> Self {
        let mut this = Self {
            base: AbstractControllerBuilderImplBase::<T>::new(option.clone()),
            values: Default::default(),
            formatter: None,
        };
        this
    }

}

impl<T: Clone + 'static> AbstractControllerBuilderImpl<T> for CyclingListControllerBuilderImpl<T> {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<T> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<T> {
        &mut self.base
    }

}

impl<T: Clone + 'static> ControllerBuilder<compat::Object> for CyclingListControllerBuilderImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<T> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<T>> {
        return (CyclingListControllerBase::create_internal(&self.option, &self.values, self.formatter.as_ref().unwrap()) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl<T: Clone + 'static> CyclingListControllerBuilder<T> for CyclingListControllerBuilderImpl<T> {
    /// java: @Override public CyclingListControllerBuilder<T> values(Iterable<? extends T> values)
    fn values_iterable(&mut self, mut values: Vec<T>) -> compat::Shared<dyn CyclingListControllerBuilder<T>> {
        self.values = values.clone();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SafeVarargs @Override public final CyclingListControllerBuilder<T> values(T... values)
    fn values(&mut self, values: &Vec<T>) -> compat::Shared<dyn CyclingListControllerBuilder<T>> {
        self.values = values.iter().cloned().collect::<Vec<_>>();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl<T: Clone + 'static> ValueFormattableController<compat::Object, compat::Object> for CyclingListControllerBuilderImpl<T> {
    /// java: @Override public CyclingListControllerBuilder<T> formatValue(ValueFormatter<T> formatter)
    fn format_value(&mut self, mut formatter: compat::Shared<dyn ValueFormatter<T>>) -> compat::Shared<dyn CyclingListControllerBuilder<T>> {
        self.formatter = Some(formatter.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
