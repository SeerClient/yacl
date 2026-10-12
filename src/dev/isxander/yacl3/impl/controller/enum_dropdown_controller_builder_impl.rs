//! Ported from dev.isxander.yacl3.impl.controller.EnumDropdownControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::enum_dropdown_controller_builder::EnumDropdownControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::gui::controllers::cycling::enum_controller::EnumController;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::enum_dropdown_controller::EnumDropdownController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;

/// java: public class EnumDropdownControllerBuilderImpl<E extends Enum<E>> extends AbstractControllerBuilderImpl<E> implements EnumDropdownControllerBuilder<E>
#[derive(Clone)]
pub struct EnumDropdownControllerBuilderImpl<E> {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<E>,
    /// java: private ValueFormatter<E> formatter = EnumController.<E>getDefaultFormatter()::apply;
    pub formatter: compat::Shared<dyn ValueFormatter<E>>,
}

impl<E: 'static> std::ops::Deref for EnumDropdownControllerBuilderImpl<E> {
    type Target = AbstractControllerBuilderImplBase<E>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<E: 'static> std::ops::DerefMut for EnumDropdownControllerBuilderImpl<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<E: Clone + 'static> EnumDropdownControllerBuilderImpl<E> {
    /// java: public EnumDropdownControllerBuilderImpl(Option<E> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<E>>) -> Self {
        let mut this = Self {
            base: AbstractControllerBuilderImplBase::<E>::new(option.clone()),
            formatter: Arc::new(move |__a0| (EnumController::get_default_formatter())(__a0.clone())),
        };
        this
    }

}

impl<E: Clone + 'static> AbstractControllerBuilderImpl<E> for EnumDropdownControllerBuilderImpl<E> {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<E> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<E> {
        &mut self.base
    }

}

impl<E: Clone + 'static> ControllerBuilder<compat::Object> for EnumDropdownControllerBuilderImpl<E> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<E> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<E>> {
        return (EnumDropdownController::new(self.option.clone(), self.formatter.clone()) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl<E: Clone + 'static> EnumDropdownControllerBuilder<E> for EnumDropdownControllerBuilderImpl<E> {
}

impl<E: Clone + 'static> ValueFormattableController<compat::Object, compat::Object> for EnumDropdownControllerBuilderImpl<E> {
    /// java: @Override public EnumDropdownControllerBuilder<E> formatValue(ValueFormatter<E> formatter)
    fn format_value(&mut self, mut formatter: compat::Shared<dyn ValueFormatter<E>>) -> compat::Shared<dyn EnumDropdownControllerBuilder<E>> {
        self.formatter = formatter.clone();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
