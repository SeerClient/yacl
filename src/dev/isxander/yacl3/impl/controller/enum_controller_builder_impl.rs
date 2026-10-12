//! Ported from dev.isxander.yacl3.impl.controller.EnumControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::enum_controller_builder::EnumControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::gui::controllers::cycling::enum_controller::EnumController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class EnumControllerBuilderImpl<T extends Enum<T>> extends AbstractControllerBuilderImpl<T> implements EnumControllerBuilder<T>
pub struct EnumControllerBuilderImpl<T> {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<T>,
    /// java: private Class<T> enumClass;
    pub enum_class: Option<compat::ClassOf<T>>,
    /// java: private ValueFormatter<T> formatter = null;
    pub formatter: Option<compat::Shared<dyn ValueFormatter<T>>>,
}

impl<T: 'static> std::ops::Deref for EnumControllerBuilderImpl<T> {
    type Target = AbstractControllerBuilderImplBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for EnumControllerBuilderImpl<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> EnumControllerBuilderImpl<T> {
    /// java: public EnumControllerBuilderImpl(Option<T> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> Self {
        let mut this = Self {
            base: AbstractControllerBuilderImplBase::<T>::new(option.clone()),
            enum_class: None,
            formatter: None,
        };
        this
    }

}

impl<T: Clone + 'static> AbstractControllerBuilderImpl<T> for EnumControllerBuilderImpl<T> {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<T> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<T> {
        &mut self.base
    }

}

impl<T: Clone + 'static> ControllerBuilder<compat::Object> for EnumControllerBuilderImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<T> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<T>> {
        let mut formatter: Option<compat::Shared<dyn ValueFormatter<T>>> = self.formatter.clone();
        if formatter.is_none() {
            let mut format_function: Arc<dyn Fn(T) -> MutableComponent> = EnumController::get_default_formatter();
            formatter = Some(Arc::new(move |__a0| (format_function)(__a0.clone())));
        }
        // TODO(fabric2seer): `.getEnumConstants(..)` on a Class
        //   java: return EnumController.createInternal(option, formatter, enumClass.getEnumConstants());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl<T: Clone + 'static> EnumControllerBuilder<T> for EnumControllerBuilderImpl<T> {
    /// java: @Override public EnumControllerBuilder<T> enumClass(Class<T> enumClass)
    fn enum_class(&mut self, mut enum_class: compat::ClassOf<T>) -> compat::Shared<dyn EnumControllerBuilder<T>> {
        self.enum_class = Some(enum_class.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl<T: Clone + 'static> ValueFormattableController<compat::Object, compat::Object> for EnumControllerBuilderImpl<T> {
    /// java: @Override public EnumControllerBuilder<T> formatValue(ValueFormatter<T> formatter)
    fn format_value(&mut self, mut formatter: compat::Shared<dyn ValueFormatter<T>>) -> compat::Shared<dyn EnumControllerBuilder<T>> {
        self.formatter = Some(formatter.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
