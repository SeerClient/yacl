//! Ported from dev.isxander.yacl3.impl.controller.LongFieldControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::long_field_controller_builder::LongFieldControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::gui::controllers::slider::long_slider_controller;
use crate::dev::isxander::yacl3::gui::controllers::string::number::long_field_controller::LongFieldController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;

/// java: public class LongFieldControllerBuilderImpl extends AbstractControllerBuilderImpl<Long> implements LongFieldControllerBuilder
#[derive(Clone)]
pub struct LongFieldControllerBuilderImpl {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<i64>,
    /// java: private long min = Long.MIN_VALUE;
    pub min: i64,
    /// java: private long max = Long.MAX_VALUE;
    pub max: i64,
    /// java: private ValueFormatter<Long> formatter = LongSliderController.DEFAULT_FORMATTER::apply;
    pub formatter: compat::Shared<dyn ValueFormatter<i64>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for LongFieldControllerBuilderImpl {
    type Target = AbstractControllerBuilderImplBase<i64>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LongFieldControllerBuilderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LongFieldControllerBuilderImpl {
    /// java: public LongFieldControllerBuilderImpl(Option<Long> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i64>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractControllerBuilderImplBase::<i64>::new(option.clone()),
            self_ref: std::sync::Weak::new(),
            min: i64::MIN,
            max: i64::MAX,
            formatter: Arc::new(move |__a0| (long_slider_controller::DEFAULT_FORMATTER.get_mut())(__a0.clone())),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl AbstractControllerBuilderImpl<i64> for LongFieldControllerBuilderImpl {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<i64> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<i64> {
        &mut self.base
    }

}

impl ControllerBuilder<compat::Object> for LongFieldControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<Long> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<i64>> {
        return (compat::shared(LongFieldController::create_internal(&self.option, self.min, self.max, &self.formatter)) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl LongFieldControllerBuilder for LongFieldControllerBuilderImpl {
}

impl NumberFieldControllerBuilder<compat::Object, compat::Object> for LongFieldControllerBuilderImpl {
    /// java: @Override public LongFieldControllerBuilder min(Long min)
    fn min(&mut self, min: &i64) -> compat::Shared<dyn LongFieldControllerBuilder> {
        self.min = *min;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn LongFieldControllerBuilder>);
    }

    /// java: @Override public LongFieldControllerBuilder max(Long max)
    fn max(&mut self, max: &i64) -> compat::Shared<dyn LongFieldControllerBuilder> {
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn LongFieldControllerBuilder>);
    }

    /// java: @Override public LongFieldControllerBuilder range(Long min, Long max)
    fn range(&mut self, min: &i64, max: &i64) -> compat::Shared<dyn LongFieldControllerBuilder> {
        self.min = *min;
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn LongFieldControllerBuilder>);
    }

}

impl ValueFormattableController<compat::Object, compat::Object> for LongFieldControllerBuilderImpl {
    /// java: @Override public LongFieldControllerBuilder formatValue(ValueFormatter<Long> formatter)
    fn format_value(&mut self, mut formatter: compat::Shared<dyn ValueFormatter<i64>>) -> compat::Shared<dyn LongFieldControllerBuilder> {
        self.formatter = formatter.clone();
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn LongFieldControllerBuilder>);
    }

}
