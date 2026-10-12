//! Ported from dev.isxander.yacl3.impl.controller.IntegerFieldControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::integer_field_controller_builder::IntegerFieldControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::gui::controllers::slider::integer_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::number::integer_field_controller::IntegerFieldController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;

/// java: public class IntegerFieldControllerBuilderImpl extends AbstractControllerBuilderImpl<Integer> implements IntegerFieldControllerBuilder
#[derive(Clone)]
pub struct IntegerFieldControllerBuilderImpl {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<i32>,
    /// java: private int min = Integer.MIN_VALUE;
    pub min: i32,
    /// java: private int max = Integer.MAX_VALUE;
    pub max: i32,
    /// java: private ValueFormatter<Integer> formatter = IntegerSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: compat::Shared<dyn ValueFormatter<i32>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for IntegerFieldControllerBuilderImpl {
    type Target = AbstractControllerBuilderImplBase<i32>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for IntegerFieldControllerBuilderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl IntegerFieldControllerBuilderImpl {
    /// java: public IntegerFieldControllerBuilderImpl(Option<Integer> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i32>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractControllerBuilderImplBase::<i32>::new(option.clone()),
            self_ref: std::sync::Weak::new(),
            min: i32::MIN,
            max: i32::MAX,
            formatter: Arc::new(move |__a0| (integer_slider_controller::DEFAULT_FORMATTER.get_mut())(__a0.clone())),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl AbstractControllerBuilderImpl<i32> for IntegerFieldControllerBuilderImpl {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<i32> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<i32> {
        &mut self.base
    }

}

impl ControllerBuilder<compat::Object> for IntegerFieldControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<Integer> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<i32>> {
        return (compat::shared(IntegerFieldController::create_internal(&self.option, self.min, self.max, &self.formatter)) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl IntegerFieldControllerBuilder for IntegerFieldControllerBuilderImpl {
}

impl NumberFieldControllerBuilder<compat::Object, compat::Object> for IntegerFieldControllerBuilderImpl {
    /// java: @Override public IntegerFieldControllerBuilder min(Integer min)
    fn min(&mut self, min: &i32) -> compat::Shared<dyn IntegerFieldControllerBuilder> {
        self.min = *min;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn IntegerFieldControllerBuilder>);
    }

    /// java: @Override public IntegerFieldControllerBuilder max(Integer max)
    fn max(&mut self, max: &i32) -> compat::Shared<dyn IntegerFieldControllerBuilder> {
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn IntegerFieldControllerBuilder>);
    }

    /// java: @Override public IntegerFieldControllerBuilder range(Integer min, Integer max)
    fn range(&mut self, min: &i32, max: &i32) -> compat::Shared<dyn IntegerFieldControllerBuilder> {
        self.min = *min;
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn IntegerFieldControllerBuilder>);
    }

}

impl ValueFormattableController<compat::Object, compat::Object> for IntegerFieldControllerBuilderImpl {
    /// java: @Override public IntegerFieldControllerBuilder formatValue(ValueFormatter<Integer> formatter)
    fn format_value(&mut self, mut formatter: compat::Shared<dyn ValueFormatter<i32>>) -> compat::Shared<dyn IntegerFieldControllerBuilder> {
        self.formatter = formatter.clone();
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn IntegerFieldControllerBuilder>);
    }

}
