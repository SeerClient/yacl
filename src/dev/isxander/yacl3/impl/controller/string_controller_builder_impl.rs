//! Ported from dev.isxander.yacl3.impl.controller.StringControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::string_controller_builder::StringControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller::StringController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;

/// java: public class StringControllerBuilderImpl extends AbstractControllerBuilderImpl<String> implements StringControllerBuilder
#[derive(Clone)]
pub struct StringControllerBuilderImplBase {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<String>,
}

impl std::ops::Deref for StringControllerBuilderImplBase {
    type Target = AbstractControllerBuilderImplBase<String>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for StringControllerBuilderImplBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl StringControllerBuilderImplBase {
    /// java: public StringControllerBuilderImpl(Option<String> option) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<String>>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn StringControllerBuilderImpl>, __R)) -> (compat::Shared<dyn StringControllerBuilderImpl>, __R) {
        let this = Self {
            base: AbstractControllerBuilderImplBase::<String>::new(option.clone()),
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: public StringControllerBuilderImpl(Option<String> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<String>>) -> compat::Shared<Self> {
        Self::new_in(option, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn StringControllerBuilderImpl>), __h)
        }).1
    }

}

/// java: the virtual methods of StringControllerBuilderImpl (implemented by StringControllerBuilderImplBase and every subclass)
pub trait StringControllerBuilderImpl: AbstractControllerBuilderImpl + StringControllerBuilder {
    fn string_controller_builder_impl_base(&self) -> &StringControllerBuilderImplBase;
    fn string_controller_builder_impl_base_mut(&mut self) -> &mut StringControllerBuilderImplBase;

}

impl dyn StringControllerBuilderImpl {
    /// java: @Override public Controller<String> build()
    pub fn build_body(__this: &compat::Shared<dyn StringControllerBuilderImpl>) -> compat::Shared<dyn Controller<String>> {
        return (compat::shared(StringController::new(compat::read(__this, |__t| __t.string_controller_builder_impl_base().option.clone()))) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl StringControllerBuilderImpl for StringControllerBuilderImplBase {
    fn string_controller_builder_impl_base(&self) -> &StringControllerBuilderImplBase {
        self
    }
    fn string_controller_builder_impl_base_mut(&mut self) -> &mut StringControllerBuilderImplBase {
        self
    }

}

impl AbstractControllerBuilderImpl<String> for StringControllerBuilderImplBase {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<String> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<String> {
        &mut self.base
    }

}

impl ControllerBuilder<compat::Object> for StringControllerBuilderImplBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<String> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<String>> {
        return (compat::shared(StringController::new(self.option.clone())) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl StringControllerBuilder for StringControllerBuilderImplBase {
}
