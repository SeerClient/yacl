//! Ported from dev.isxander.yacl3.impl.controller.EnumControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::enum_controller_builder::EnumControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::cycling::enum_controller::{self, EnumController};

/// java: public class EnumControllerBuilderImpl<T extends Enum<T>> extends AbstractControllerBuilderImpl<T> implements EnumControllerBuilder<T>
pub struct EnumControllerBuilderImpl<T: compat::JavaEnum> {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<T>>,
    /// java: private Class<T> enumClass;
    pub enum_class: Option<compat::ClassOf<T>>,
    /// java: private ValueFormatter<T> formatter = null;
    pub formatter: Option<Arc<dyn ValueFormatter<T>>>,
}

impl<T: compat::JavaEnum> EnumControllerBuilderImpl<T> {
    /// java: public EnumControllerBuilderImpl(Option<T> option)
    pub fn new(option: Arc<dyn YOption<T>>) -> Self {
        Self { option, enum_class: None, formatter: None }
    }
}

impl<T: compat::JavaEnum> EnumControllerBuilder<T> for EnumControllerBuilderImpl<T> {
    /// java: @Override public EnumControllerBuilder<T> enumClass(Class<T> enumClass)
    fn enum_class(mut self, enum_class: compat::ClassOf<T>) -> Self {
        self.enum_class = Some(enum_class);
        self
    }
}

impl<T: compat::JavaEnum> ValueFormattableController<T> for EnumControllerBuilderImpl<T> {
    /// java: @Override public EnumControllerBuilder<T> formatValue(ValueFormatter<T> formatter)
    fn format_value(mut self, formatter: Arc<dyn ValueFormatter<T>>) -> Self {
        self.formatter = Some(formatter);
        self
    }
}

impl<T: compat::JavaEnum> ControllerBuilder<T> for EnumControllerBuilderImpl<T> {
    /// java: @Override public Controller<T> build()
    fn build(&self) -> Arc<dyn Controller<T>> {
        let formatter: Arc<dyn ValueFormatter<T>> = match &self.formatter {
            Some(formatter) => formatter.clone(),
            None => Arc::new(enum_controller::get_default_formatter::<T>()),
        };
        // java: enumClass.getEnumConstants() - the enum's constants (`JavaEnum`); a builder without `.enumClass(..)`
        // throws NullPointerException there
        let _enum_class = self.enum_class.as_ref().expect("NullPointerException: EnumControllerBuilder has no enumClass");
        EnumController::create_internal(self.option.clone(), formatter, T::enum_constants())
    }
}
