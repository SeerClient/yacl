//! Ported from dev.isxander.yacl3.impl.controller.BooleanControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::boolean_controller_builder::BooleanControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::gui::controllers::boolean_controller;
use crate::dev::isxander::yacl3::gui::controllers::boolean_controller::BooleanController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;

/// java: public class BooleanControllerBuilderImpl extends AbstractControllerBuilderImpl<Boolean> implements BooleanControllerBuilder
#[derive(Clone)]
pub struct BooleanControllerBuilderImpl {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<bool>,
    /// java: private boolean coloured = false;
    pub coloured: bool,
    /// java: private ValueFormatter<Boolean> formatter = BooleanController.ON_OFF_FORMATTER::apply;
    pub formatter: compat::Shared<dyn ValueFormatter<bool>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for BooleanControllerBuilderImpl {
    type Target = AbstractControllerBuilderImplBase<bool>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BooleanControllerBuilderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BooleanControllerBuilderImpl {
    /// java: public BooleanControllerBuilderImpl(Option<Boolean> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractControllerBuilderImplBase::<bool>::new(option.clone()),
            self_ref: std::sync::Weak::new(),
            coloured: false,
            formatter: Arc::new(move |__a0| (boolean_controller::ON_OFF_FORMATTER.get_mut())(__a0.clone())),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl AbstractControllerBuilderImpl<bool> for BooleanControllerBuilderImpl {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<bool> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<bool> {
        &mut self.base
    }

}

impl ControllerBuilder<compat::Object> for BooleanControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<Boolean> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<bool>> {
        return (BooleanController::create_internal(&self.option, &self.formatter, self.coloured) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl BooleanControllerBuilder for BooleanControllerBuilderImpl {
    /// java: @Override public BooleanControllerBuilder coloured(boolean coloured) (dispatch: the body for this class)
    fn coloured(&self) -> fn(&compat::Shared<dyn BooleanControllerBuilder>, bool) -> compat::Shared<dyn BooleanControllerBuilder> {
        BooleanControllerBuilderImpl::coloured_dyn
    }

    /// java: @Override public BooleanControllerBuilder onOffFormatter() (dispatch: the body for this class)
    fn on_off_formatter(&self) -> fn(&compat::Shared<dyn BooleanControllerBuilder>) -> compat::Shared<dyn BooleanControllerBuilder> {
        BooleanControllerBuilderImpl::on_off_formatter_dyn
    }

    /// java: @Override public BooleanControllerBuilder yesNoFormatter() (dispatch: the body for this class)
    fn yes_no_formatter(&self) -> fn(&compat::Shared<dyn BooleanControllerBuilder>) -> compat::Shared<dyn BooleanControllerBuilder> {
        BooleanControllerBuilderImpl::yes_no_formatter_dyn
    }

    /// java: @Override public BooleanControllerBuilder trueFalseFormatter() (dispatch: the body for this class)
    fn true_false_formatter(&self) -> fn(&compat::Shared<dyn BooleanControllerBuilder>) -> compat::Shared<dyn BooleanControllerBuilder> {
        BooleanControllerBuilderImpl::true_false_formatter_dyn
    }

}

impl ValueFormattableController<compat::Object, compat::Object> for BooleanControllerBuilderImpl {
    /// java: @Override public BooleanControllerBuilder formatValue(ValueFormatter<Boolean> formatter)
    fn format_value(&mut self, mut formatter: compat::Shared<dyn ValueFormatter<bool>>) -> compat::Shared<dyn BooleanControllerBuilder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(formatter, "formatter cannot be null");
        self.formatter = formatter.clone();
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn BooleanControllerBuilder>);
    }

}

impl BooleanControllerBuilderImpl {
    /// java: @Override public BooleanControllerBuilder coloured(boolean coloured)
    pub fn coloured_impl(__this: &compat::Shared<Self>, mut coloured: bool) -> compat::Shared<dyn BooleanControllerBuilder> {
        { let __v = coloured; compat::write(__this, |__t| __t.coloured = __v); }
        return (__this.clone() as compat::Shared<dyn BooleanControllerBuilder>);
    }

    /// BooleanControllerBuilderImpl.coloured as the dispatch of BooleanControllerBuilder.coloured
    pub fn coloured_dyn(__this: &compat::Shared<dyn BooleanControllerBuilder>, __p0: bool) -> compat::Shared<dyn BooleanControllerBuilder> {
        Self::coloured_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public BooleanControllerBuilder onOffFormatter()
    pub fn on_off_formatter_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn BooleanControllerBuilder> {
        { let __v = Arc::new(move |__a0| (boolean_controller::ON_OFF_FORMATTER.get_mut())(__a0.clone())); compat::write(__this, |__t| __t.formatter = __v); }
        return (__this.clone() as compat::Shared<dyn BooleanControllerBuilder>);
    }

    /// BooleanControllerBuilderImpl.onOffFormatter as the dispatch of BooleanControllerBuilder.onOffFormatter
    pub fn on_off_formatter_dyn(__this: &compat::Shared<dyn BooleanControllerBuilder>) -> compat::Shared<dyn BooleanControllerBuilder> {
        Self::on_off_formatter_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public BooleanControllerBuilder yesNoFormatter()
    pub fn yes_no_formatter_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn BooleanControllerBuilder> {
        { let __v = Arc::new(move |__a0| (boolean_controller::YES_NO_FORMATTER.get_mut())(__a0.clone())); compat::write(__this, |__t| __t.formatter = __v); }
        return (__this.clone() as compat::Shared<dyn BooleanControllerBuilder>);
    }

    /// BooleanControllerBuilderImpl.yesNoFormatter as the dispatch of BooleanControllerBuilder.yesNoFormatter
    pub fn yes_no_formatter_dyn(__this: &compat::Shared<dyn BooleanControllerBuilder>) -> compat::Shared<dyn BooleanControllerBuilder> {
        Self::yes_no_formatter_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public BooleanControllerBuilder trueFalseFormatter()
    pub fn true_false_formatter_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn BooleanControllerBuilder> {
        { let __v = Arc::new(move |__a0| (boolean_controller::TRUE_FALSE_FORMATTER.get_mut())(__a0.clone())); compat::write(__this, |__t| __t.formatter = __v); }
        return (__this.clone() as compat::Shared<dyn BooleanControllerBuilder>);
    }

    /// BooleanControllerBuilderImpl.trueFalseFormatter as the dispatch of BooleanControllerBuilder.trueFalseFormatter
    pub fn true_false_formatter_dyn(__this: &compat::Shared<dyn BooleanControllerBuilder>) -> compat::Shared<dyn BooleanControllerBuilder> {
        Self::true_false_formatter_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
