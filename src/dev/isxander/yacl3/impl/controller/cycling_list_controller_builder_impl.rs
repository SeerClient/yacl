//! Ported from dev.isxander.yacl3.impl.controller.CyclingListControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::cycling_list_controller_builder::CyclingListControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::cycling::cycling_list_controller::CyclingListController;

/// java: public final class CyclingListControllerBuilderImpl<T> extends AbstractControllerBuilderImpl<T> implements CyclingListControllerBuilder<T>
pub struct CyclingListControllerBuilderImpl<T: Clone + 'static> {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<T>>,
    /// java: private Iterable<? extends T> values;
    pub values: Vec<T>,
    /// java: private ValueFormatter<T> formatter = null;
    pub formatter: Option<Arc<dyn ValueFormatter<T>>>,
}

impl<T: Clone + PartialEq + 'static> CyclingListControllerBuilderImpl<T> {
    /// java: public CyclingListControllerBuilderImpl(Option<T> option)
    pub fn new(option: Arc<dyn YOption<T>>) -> Self {
        Self { option, values: Vec::new(), formatter: None }
    }
}

impl<T: Clone + PartialEq + 'static> CyclingListControllerBuilder<T> for CyclingListControllerBuilderImpl<T> {
    /// java: @SafeVarargs @Override public final CyclingListControllerBuilder<T> values(T... values)
    fn values(mut self, values: Vec<T>) -> Self {
        self.values = values;
        self
    }

    /// java: @Override public CyclingListControllerBuilder<T> values(Iterable<? extends T> values)
    fn values_iterable(mut self, values: impl IntoIterator<Item = T>) -> Self {
        self.values = values.into_iter().collect();
        self
    }
}

impl<T: Clone + PartialEq + 'static> ValueFormattableController<T> for CyclingListControllerBuilderImpl<T> {
    /// java: @Override public CyclingListControllerBuilder<T> formatValue(ValueFormatter<T> formatter)
    fn format_value(mut self, formatter: Arc<dyn ValueFormatter<T>>) -> Self {
        self.formatter = Some(formatter);
        self
    }
}

impl<T: Clone + PartialEq + 'static> ControllerBuilder<T> for CyclingListControllerBuilderImpl<T> {
    /// java: @Override public Controller<T> build()
    fn build(&self) -> Arc<dyn Controller<T>> {
        // java: `formatter::format` on a null formatter throws NullPointerException
        let formatter = self.formatter.clone().expect("NullPointerException: CyclingListControllerBuilder has no formatter");
        CyclingListController::create_internal(self.option.clone(), self.values.clone(), formatter)
    }
}
