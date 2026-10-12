//! Ported from dev.isxander.yacl3.impl.controller.IntegerSliderControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::integer_slider_controller_builder::IntegerSliderControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::gui::controllers::slider::integer_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::slider::integer_slider_controller::IntegerSliderController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;

/// java: public class IntegerSliderControllerBuilderImpl extends AbstractControllerBuilderImpl<Integer> implements IntegerSliderControllerBuilder
#[derive(Clone)]
pub struct IntegerSliderControllerBuilderImpl {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<i32>,
    /// java: private int min, max;
    pub min: i32,
    /// java: private int min, max;
    pub max: i32,
    /// java: private int step;
    pub step: i32,
    /// java: private ValueFormatter<Integer> formatter = IntegerSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: compat::Shared<dyn ValueFormatter<i32>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for IntegerSliderControllerBuilderImpl {
    type Target = AbstractControllerBuilderImplBase<i32>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for IntegerSliderControllerBuilderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl IntegerSliderControllerBuilderImpl {
    /// java: public IntegerSliderControllerBuilderImpl(Option<Integer> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i32>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractControllerBuilderImplBase::<i32>::new(option.clone()),
            self_ref: std::sync::Weak::new(),
            min: 0,
            max: 0,
            step: 0,
            formatter: Arc::new(move |__a0| (integer_slider_controller::DEFAULT_FORMATTER.get_mut())(__a0.clone())),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl AbstractControllerBuilderImpl<i32> for IntegerSliderControllerBuilderImpl {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<i32> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<i32> {
        &mut self.base
    }

}

impl ControllerBuilder<compat::Object> for IntegerSliderControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<Integer> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<i32>> {
        return (compat::shared(IntegerSliderController::create_internal(&self.option, self.min, self.max, self.step, &self.formatter)) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl IntegerSliderControllerBuilder for IntegerSliderControllerBuilderImpl {
}

impl SliderControllerBuilder<compat::Object, compat::Object> for IntegerSliderControllerBuilderImpl {
    /// java: @Override public IntegerSliderControllerBuilder range(Integer min, Integer max)
    fn range(&mut self, min: &i32, max: &i32) -> compat::Shared<dyn IntegerSliderControllerBuilder> {
        self.min = *min;
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn IntegerSliderControllerBuilder>);
    }

    /// java: @Override public IntegerSliderControllerBuilder step(Integer step)
    fn step(&mut self, step: &i32) -> compat::Shared<dyn IntegerSliderControllerBuilder> {
        self.step = *step;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn IntegerSliderControllerBuilder>);
    }

}

impl ValueFormattableController<compat::Object, compat::Object> for IntegerSliderControllerBuilderImpl {
    /// java: @Override public IntegerSliderControllerBuilder formatValue(ValueFormatter<Integer> formatter)
    fn format_value(&mut self, mut formatter: compat::Shared<dyn ValueFormatter<i32>>) -> compat::Shared<dyn IntegerSliderControllerBuilder> {
        self.formatter = formatter.clone();
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn IntegerSliderControllerBuilder>);
    }

}
