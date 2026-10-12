//! Ported from dev.isxander.yacl3.impl.controller.FloatSliderControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::float_slider_controller_builder::FloatSliderControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::gui::controllers::slider::float_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::slider::float_slider_controller::FloatSliderController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;

/// java: public class FloatSliderControllerBuilderImpl extends AbstractControllerBuilderImpl<Float> implements FloatSliderControllerBuilder
#[derive(Clone)]
pub struct FloatSliderControllerBuilderImpl {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<f32>,
    /// java: private float min, max;
    pub min: f32,
    /// java: private float min, max;
    pub max: f32,
    /// java: private float step;
    pub step: f32,
    /// java: private ValueFormatter<Float> formatter = FloatSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: compat::Shared<dyn ValueFormatter<f32>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for FloatSliderControllerBuilderImpl {
    type Target = AbstractControllerBuilderImplBase<f32>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FloatSliderControllerBuilderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FloatSliderControllerBuilderImpl {
    /// java: public FloatSliderControllerBuilderImpl(Option<Float> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f32>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractControllerBuilderImplBase::<f32>::new(option.clone()),
            self_ref: std::sync::Weak::new(),
            min: 0.0,
            max: 0.0,
            step: 0.0,
            formatter: Arc::new(move |__a0| (float_slider_controller::DEFAULT_FORMATTER.get_mut())(__a0.clone())),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl AbstractControllerBuilderImpl<f32> for FloatSliderControllerBuilderImpl {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<f32> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<f32> {
        &mut self.base
    }

}

impl ControllerBuilder<compat::Object> for FloatSliderControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<Float> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<f32>> {
        return (compat::shared(FloatSliderController::create_internal(&self.option, self.min, self.max, self.step, &self.formatter)) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl FloatSliderControllerBuilder for FloatSliderControllerBuilderImpl {
}

impl SliderControllerBuilder<compat::Object, compat::Object> for FloatSliderControllerBuilderImpl {
    /// java: @Override public FloatSliderControllerBuilder range(Float min, Float max)
    fn range(&mut self, min: &f32, max: &f32) -> compat::Shared<dyn FloatSliderControllerBuilder> {
        self.min = *min;
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn FloatSliderControllerBuilder>);
    }

    /// java: @Override public FloatSliderControllerBuilder step(Float step)
    fn step(&mut self, step: &f32) -> compat::Shared<dyn FloatSliderControllerBuilder> {
        self.step = *step;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn FloatSliderControllerBuilder>);
    }

}

impl ValueFormattableController<compat::Object, compat::Object> for FloatSliderControllerBuilderImpl {
    /// java: @Override public FloatSliderControllerBuilder formatValue(ValueFormatter<Float> formatter)
    fn format_value(&mut self, mut formatter: compat::Shared<dyn ValueFormatter<f32>>) -> compat::Shared<dyn FloatSliderControllerBuilder> {
        self.formatter = formatter.clone();
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn FloatSliderControllerBuilder>);
    }

}
