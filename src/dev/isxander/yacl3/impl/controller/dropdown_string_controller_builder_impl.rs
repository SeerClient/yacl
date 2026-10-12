//! Ported from dev.isxander.yacl3.impl.controller.DropdownStringControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::dropdown_string_controller_builder::DropdownStringControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::string_controller_builder::StringControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::dropdown_string_controller::DropdownStringController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;
use crate::dev::isxander::yacl3::r#impl::controller::string_controller_builder_impl::StringControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::string_controller_builder_impl::StringControllerBuilderImplBase;

/// java: public class DropdownStringControllerBuilderImpl extends StringControllerBuilderImpl implements DropdownStringControllerBuilder
#[derive(Clone)]
pub struct DropdownStringControllerBuilderImpl {
    /// java: extends StringControllerBuilderImpl
    pub base: StringControllerBuilderImplBase,
    /// java: private List<String> values;
    pub values: Vec<String>,
    /// java: private boolean allowEmptyValue = false;
    pub allow_empty_value: bool,
    /// java: private boolean allowAnyValue = false;
    pub allow_any_value: bool,
}

impl std::ops::Deref for DropdownStringControllerBuilderImpl {
    type Target = StringControllerBuilderImplBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DropdownStringControllerBuilderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DropdownStringControllerBuilderImpl {
    /// java: public DropdownStringControllerBuilderImpl(Option<String> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<String>>) -> compat::Shared<Self> {
        let (_, __this) = StringControllerBuilderImplBase::new_in(option.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                values: Default::default(),
                allow_empty_value: false,
                allow_any_value: false,
            });
            ((__h.clone() as compat::Shared<dyn StringControllerBuilderImpl>), __h)
        });
        __this
    }

}

impl StringControllerBuilderImpl for DropdownStringControllerBuilderImpl {
    fn string_controller_builder_impl_base(&self) -> &StringControllerBuilderImplBase {
        &self.base
    }
    fn string_controller_builder_impl_base_mut(&mut self) -> &mut StringControllerBuilderImplBase {
        &mut self.base
    }

}

impl AbstractControllerBuilderImpl<compat::Object> for DropdownStringControllerBuilderImpl {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<compat::Object> {
        &self.base.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<compat::Object> {
        &mut self.base.base
    }

}

impl ControllerBuilder<compat::Object> for DropdownStringControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<String> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<String>> {
        return (DropdownStringController::new(self.option.clone(), self.values.clone(), self.allow_empty_value, self.allow_any_value) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl StringControllerBuilder for DropdownStringControllerBuilderImpl {
}

impl DropdownStringControllerBuilder for DropdownStringControllerBuilderImpl {
    /// java: @Override public DropdownStringControllerBuilder values(List<String> values) (dispatch: the body for this class)
    fn values(&self) -> fn(&compat::Shared<dyn DropdownStringControllerBuilder>, Vec<String>) -> compat::Shared<dyn DropdownStringControllerBuilder> {
        DropdownStringControllerBuilderImpl::values_dyn
    }

    /// java: @Override public DropdownStringControllerBuilderImpl values(String... values) (dispatch: the body for this class)
    fn values_string(&self) -> fn(&compat::Shared<dyn DropdownStringControllerBuilder>, &Vec<String>) -> compat::Shared<dyn DropdownStringControllerBuilder> {
        DropdownStringControllerBuilderImpl::values_string_dyn
    }

    /// java: @Override public DropdownStringControllerBuilderImpl allowEmptyValue(boolean allowEmptyValue) (dispatch: the body for this class)
    fn allow_empty_value(&self) -> fn(&compat::Shared<dyn DropdownStringControllerBuilder>, bool) -> compat::Shared<dyn DropdownStringControllerBuilder> {
        DropdownStringControllerBuilderImpl::allow_empty_value_dyn
    }

    /// java: @Override public DropdownStringControllerBuilderImpl allowAnyValue(boolean allowAnyValue) (dispatch: the body for this class)
    fn allow_any_value(&self) -> fn(&compat::Shared<dyn DropdownStringControllerBuilder>, bool) -> compat::Shared<dyn DropdownStringControllerBuilder> {
        DropdownStringControllerBuilderImpl::allow_any_value_dyn
    }

}

impl DropdownStringControllerBuilderImpl {
    /// java: @Override public DropdownStringControllerBuilder values(List<String> values)
    pub fn values_impl(__this: &compat::Shared<Self>, mut values: Vec<String>) -> compat::Shared<dyn DropdownStringControllerBuilder> {
        { let __v = values.clone(); compat::write(__this, |__t| __t.values = __v); }
        return (__this.clone() as compat::Shared<dyn DropdownStringControllerBuilder>);
    }

    /// DropdownStringControllerBuilderImpl.values as the dispatch of DropdownStringControllerBuilder.values
    pub fn values_dyn(__this: &compat::Shared<dyn DropdownStringControllerBuilder>, __p0: Vec<String>) -> compat::Shared<dyn DropdownStringControllerBuilder> {
        Self::values_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public DropdownStringControllerBuilderImpl values(String... values)
    pub fn values_string_impl(__this: &compat::Shared<Self>, values: &Vec<String>) -> compat::Shared<DropdownStringControllerBuilderImpl> {
        { let __v = values.to_vec(); compat::write(__this, |__t| __t.values = __v); }
        return __this.clone();
    }

    /// DropdownStringControllerBuilderImpl.values as the dispatch of DropdownStringControllerBuilder.values
    pub fn values_string_dyn(__this: &compat::Shared<dyn DropdownStringControllerBuilder>, __p0: &Vec<String>) -> compat::Shared<dyn DropdownStringControllerBuilder> {
        Self::values_string_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public DropdownStringControllerBuilderImpl allowEmptyValue(boolean allowEmptyValue)
    pub fn allow_empty_value_impl(__this: &compat::Shared<Self>, mut allow_empty_value: bool) -> compat::Shared<DropdownStringControllerBuilderImpl> {
        { let __v = allow_empty_value; compat::write(__this, |__t| __t.allow_empty_value = __v); }
        return __this.clone();
    }

    /// DropdownStringControllerBuilderImpl.allowEmptyValue as the dispatch of DropdownStringControllerBuilder.allowEmptyValue
    pub fn allow_empty_value_dyn(__this: &compat::Shared<dyn DropdownStringControllerBuilder>, __p0: bool) -> compat::Shared<dyn DropdownStringControllerBuilder> {
        Self::allow_empty_value_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public DropdownStringControllerBuilderImpl allowAnyValue(boolean allowAnyValue)
    pub fn allow_any_value_impl(__this: &compat::Shared<Self>, mut allow_any_value: bool) -> compat::Shared<DropdownStringControllerBuilderImpl> {
        { let __v = allow_any_value; compat::write(__this, |__t| __t.allow_any_value = __v); }
        return __this.clone();
    }

    /// DropdownStringControllerBuilderImpl.allowAnyValue as the dispatch of DropdownStringControllerBuilder.allowAnyValue
    pub fn allow_any_value_dyn(__this: &compat::Shared<dyn DropdownStringControllerBuilder>, __p0: bool) -> compat::Shared<dyn DropdownStringControllerBuilder> {
        Self::allow_any_value_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
