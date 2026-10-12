//! Ported from dev.isxander.yacl3.impl.controller.EnumDropdownControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::enum_dropdown_controller_builder::EnumDropdownControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::cycling::enum_controller;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::enum_dropdown_controller::EnumDropdownController;

/// java: public class EnumDropdownControllerBuilderImpl<E extends Enum<E>> extends AbstractControllerBuilderImpl<E> implements EnumDropdownControllerBuilder<E>
pub struct EnumDropdownControllerBuilderImpl<E: compat::JavaEnum> {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<E>>,
    /// java: private ValueFormatter<E> formatter = EnumController.<E>getDefaultFormatter()::apply;
    pub formatter: Arc<dyn ValueFormatter<E>>,
}

impl<E: compat::JavaEnum> EnumDropdownControllerBuilderImpl<E> {
    /// java: public EnumDropdownControllerBuilderImpl(Option<E> option)
    pub fn new(option: Arc<dyn YOption<E>>) -> Self {
        Self { option, formatter: Arc::new(enum_controller::get_default_formatter::<E>()) }
    }
}

impl<E: compat::JavaEnum> ValueFormattableController<E> for EnumDropdownControllerBuilderImpl<E> {
    /// java: @Override public EnumDropdownControllerBuilder<E> formatValue(ValueFormatter<E> formatter)
    fn format_value(mut self, formatter: Arc<dyn ValueFormatter<E>>) -> Self {
        self.formatter = formatter;
        self
    }
}

impl<E: compat::JavaEnum> ControllerBuilder<E> for EnumDropdownControllerBuilderImpl<E> {
    /// java: @Override public Controller<E> build()
    fn build(&self) -> Arc<dyn Controller<E>> {
        EnumDropdownController::new(self.option.clone(), self.formatter.clone())
    }
}

impl<E: compat::JavaEnum> EnumDropdownControllerBuilder<E> for EnumDropdownControllerBuilderImpl<E> {}
