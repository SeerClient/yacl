//! Ported from dev.isxander.yacl3.impl.controller.LongSliderControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::long_slider_controller_builder::LongSliderControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::gui::controllers::slider::long_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::slider::long_slider_controller::LongSliderController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;

/// java: public class LongSliderControllerBuilderImpl extends AbstractControllerBuilderImpl<Long> implements LongSliderControllerBuilder
#[derive(Clone)]
pub struct LongSliderControllerBuilderImpl {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<i64>,
    /// java: private long min, max;
    pub min: i64,
    /// java: private long min, max;
    pub max: i64,
    /// java: private long step;
    pub step: i64,
    /// java: private ValueFormatter<Long> formatter = LongSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: compat::Shared<dyn ValueFormatter<i64>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for LongSliderControllerBuilderImpl {
    type Target = AbstractControllerBuilderImplBase<i64>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LongSliderControllerBuilderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LongSliderControllerBuilderImpl {
    /// java: public LongSliderControllerBuilderImpl(Option<Long> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i64>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractControllerBuilderImplBase::<i64>::new(option.clone()),
            self_ref: std::sync::Weak::new(),
            min: 0,
            max: 0,
            step: 0,
            formatter: Arc::new(move |__a0| (long_slider_controller::DEFAULT_FORMATTER.get_mut())(__a0.clone())),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl AbstractControllerBuilderImpl<i64> for LongSliderControllerBuilderImpl {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<i64> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<i64> {
        &mut self.base
    }

}

impl ControllerBuilder<compat::Object> for LongSliderControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<Long> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<i64>> {
        return (compat::shared(LongSliderController::create_internal(&self.option, self.min, self.max, self.step, &self.formatter)) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl LongSliderControllerBuilder for LongSliderControllerBuilderImpl {
}

impl SliderControllerBuilder<compat::Object, compat::Object> for LongSliderControllerBuilderImpl {
    /// java: @Override public LongSliderControllerBuilder range(Long min, Long max)
    fn range(&mut self, min: &i64, max: &i64) -> compat::Shared<dyn LongSliderControllerBuilder> {
        self.min = *min;
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn LongSliderControllerBuilder>);
    }

    /// java: @Override public LongSliderControllerBuilder step(Long step)
    fn step(&mut self, step: &i64) -> compat::Shared<dyn LongSliderControllerBuilder> {
        self.step = *step;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn LongSliderControllerBuilder>);
    }

}

impl ValueFormattableController<compat::Object, compat::Object> for LongSliderControllerBuilderImpl {
    /// java: @Override public LongSliderControllerBuilder formatValue(ValueFormatter<Long> formatter)
    fn format_value(&mut self, mut formatter: compat::Shared<dyn ValueFormatter<i64>>) -> compat::Shared<dyn LongSliderControllerBuilder> {
        self.formatter = formatter.clone();
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn LongSliderControllerBuilder>);
    }

}
