//! Ported from dev.isxander.yacl3.impl.controller.FloatFieldControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::float_field_controller_builder::FloatFieldControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::gui::controllers::slider::float_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::number::float_field_controller::FloatFieldController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;

/// java: public class FloatFieldControllerBuilderImpl extends AbstractControllerBuilderImpl<Float> implements FloatFieldControllerBuilder
#[derive(Clone)]
pub struct FloatFieldControllerBuilderImpl {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<f32>,
    /// java: private float min = -Float.MAX_VALUE;
    pub min: f32,
    /// java: private float max = Float.MAX_VALUE;
    pub max: f32,
    /// java: private ValueFormatter<Float> formatter = FloatSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: compat::Shared<dyn ValueFormatter<f32>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for FloatFieldControllerBuilderImpl {
    type Target = AbstractControllerBuilderImplBase<f32>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FloatFieldControllerBuilderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FloatFieldControllerBuilderImpl {
    /// java: public FloatFieldControllerBuilderImpl(Option<Float> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f32>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractControllerBuilderImplBase::<f32>::new(option.clone()),
            self_ref: std::sync::Weak::new(),
            min: (-f32::MAX),
            max: f32::MAX,
            formatter: Arc::new(move |__a0| (float_slider_controller::DEFAULT_FORMATTER.get_mut())(__a0.clone())),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl AbstractControllerBuilderImpl<f32> for FloatFieldControllerBuilderImpl {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<f32> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<f32> {
        &mut self.base
    }

}

impl ControllerBuilder<compat::Object> for FloatFieldControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<Float> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<f32>> {
        return (compat::shared(FloatFieldController::create_internal(&self.option, self.min, self.max, &self.formatter)) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl FloatFieldControllerBuilder for FloatFieldControllerBuilderImpl {
}

impl NumberFieldControllerBuilder<compat::Object, compat::Object> for FloatFieldControllerBuilderImpl {
    /// java: @Override public FloatFieldControllerBuilder min(Float min)
    fn min(&mut self, min: &f32) -> compat::Shared<dyn FloatFieldControllerBuilder> {
        self.min = *min;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn FloatFieldControllerBuilder>);
    }

    /// java: @Override public FloatFieldControllerBuilder max(Float max)
    fn max(&mut self, max: &f32) -> compat::Shared<dyn FloatFieldControllerBuilder> {
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn FloatFieldControllerBuilder>);
    }

    /// java: @Override public FloatFieldControllerBuilder range(Float min, Float max)
    fn range(&mut self, min: &f32, max: &f32) -> compat::Shared<dyn FloatFieldControllerBuilder> {
        self.min = *min;
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn FloatFieldControllerBuilder>);
    }

}

impl ValueFormattableController<compat::Object, compat::Object> for FloatFieldControllerBuilderImpl {
    /// java: @Override public FloatFieldControllerBuilder formatValue(ValueFormatter<Float> formatter)
    fn format_value(&mut self, mut formatter: compat::Shared<dyn ValueFormatter<f32>>) -> compat::Shared<dyn FloatFieldControllerBuilder> {
        self.formatter = formatter.clone();
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn FloatFieldControllerBuilder>);
    }

}
