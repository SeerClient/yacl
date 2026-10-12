//! Ported from dev.isxander.yacl3.impl.controller.DoubleFieldControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::double_field_controller_builder::DoubleFieldControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::gui::controllers::slider::double_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::number::double_field_controller::DoubleFieldController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;

/// java: public class DoubleFieldControllerBuilderImpl extends AbstractControllerBuilderImpl<Double> implements DoubleFieldControllerBuilder
#[derive(Clone)]
pub struct DoubleFieldControllerBuilderImpl {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<f64>,
    /// java: private double min = -Double.MAX_VALUE;
    pub min: f64,
    /// java: private double max = Double.MAX_VALUE;
    pub max: f64,
    /// java: private ValueFormatter<Double> formatter = DoubleSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: compat::Shared<dyn ValueFormatter<f64>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for DoubleFieldControllerBuilderImpl {
    type Target = AbstractControllerBuilderImplBase<f64>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DoubleFieldControllerBuilderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DoubleFieldControllerBuilderImpl {
    /// java: public DoubleFieldControllerBuilderImpl(Option<Double> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f64>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractControllerBuilderImplBase::<f64>::new(option.clone()),
            self_ref: std::sync::Weak::new(),
            min: (-f64::MAX),
            max: f64::MAX,
            formatter: Arc::new(move |__a0| (double_slider_controller::DEFAULT_FORMATTER.get_mut())(__a0.clone())),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl AbstractControllerBuilderImpl<f64> for DoubleFieldControllerBuilderImpl {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<f64> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<f64> {
        &mut self.base
    }

}

impl ControllerBuilder<compat::Object> for DoubleFieldControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<Double> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<f64>> {
        return (compat::shared(DoubleFieldController::create_internal(&self.option, self.min, self.max, &self.formatter)) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl DoubleFieldControllerBuilder for DoubleFieldControllerBuilderImpl {
}

impl NumberFieldControllerBuilder<compat::Object, compat::Object> for DoubleFieldControllerBuilderImpl {
    /// java: @Override public DoubleFieldControllerBuilder min(Double min)
    fn min(&mut self, min: &f64) -> compat::Shared<dyn DoubleFieldControllerBuilder> {
        self.min = *min;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn DoubleFieldControllerBuilder>);
    }

    /// java: @Override public DoubleFieldControllerBuilder max(Double max)
    fn max(&mut self, max: &f64) -> compat::Shared<dyn DoubleFieldControllerBuilder> {
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn DoubleFieldControllerBuilder>);
    }

    /// java: @Override public DoubleFieldControllerBuilder range(Double min, Double max)
    fn range(&mut self, min: &f64, max: &f64) -> compat::Shared<dyn DoubleFieldControllerBuilder> {
        self.min = *min;
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn DoubleFieldControllerBuilder>);
    }

}

impl ValueFormattableController<compat::Object, compat::Object> for DoubleFieldControllerBuilderImpl {
    /// java: @Override public DoubleFieldControllerBuilder formatValue(ValueFormatter<Double> formatter)
    fn format_value(&mut self, mut formatter: compat::Shared<dyn ValueFormatter<f64>>) -> compat::Shared<dyn DoubleFieldControllerBuilder> {
        self.formatter = formatter.clone();
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn DoubleFieldControllerBuilder>);
    }

}
