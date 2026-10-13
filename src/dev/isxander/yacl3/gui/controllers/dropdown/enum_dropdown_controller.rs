//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.EnumDropdownController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::{
    self as dropdown_controller, AbstractDropdownController, AbstractDropdownControllerBase,
};
use crate::dev::isxander::yacl3::gui::controllers::dropdown::enum_dropdown_controller_element::EnumDropdownControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::{self, IStringController};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class EnumDropdownController<E extends Enum<E>> extends AbstractDropdownController<E>
pub struct EnumDropdownController<E: compat::JavaEnum> {
    /// java: extends AbstractDropdownController<E>
    pub base: AbstractDropdownControllerBase<E>,
    /// java: protected final ValueFormatter<E> formatter;
    pub formatter: Arc<dyn ValueFormatter<E>>,
}

impl<E: compat::JavaEnum> EnumDropdownController<E> {
    /// java: public EnumDropdownController(Option<E> option, ValueFormatter<E> formatter)
    pub fn new(option: Arc<dyn YOption<E>>, formatter: Arc<dyn ValueFormatter<E>>) -> Arc<Self> {
        // option.pendingValue().getDeclaringClass().getEnumConstants(), each `formatter.format(..).getString()`
        let allowed_values = E::enum_constants().into_iter().map(|constant| formatter.format(constant).get_string()).collect();
        Arc::new(Self { base: AbstractDropdownControllerBase::new_2(option, allowed_values), formatter })
    }

    /// java: private E getEnumFromString(String value)
    fn get_enum_from_string(&self, value: &str) -> E {
        let value = value.to_lowercase();
        for constant in E::enum_constants() {
            if self.formatter.format(constant.clone()).get_string().to_lowercase() == value {
                return constant;
            }
        }
        self.base.option.pending_value()
    }

    /// java: @NotNull protected Stream<String> getValidEnumConstants(String value)
    pub fn get_valid_enum_constants(&self, value: &str) -> Vec<String> {
        let value_lower_case = value.to_lowercase();
        let mut values: Vec<String> = self
            .get_allowed_values()
            .into_iter()
            .filter(|constant| constant.to_lowercase().contains(&value_lower_case))
            .collect();
        values.sort_by(|s1, s2| dropdown_controller::compare_by_prefix(s1, s2, &value_lower_case));
        values
    }
}

impl<E: compat::JavaEnum> Controller<E> for EnumDropdownController<E> {
    /// java: AbstractDropdownController.option()
    fn option(&self) -> Arc<dyn YOption<E>> {
        self.base.option.clone()
    }

    /// java: IStringController.formatValue() (the interface default)
    fn format_value(&self) -> MutableComponent {
        i_string_controller::format_value::<E, Self>(self)
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        EnumDropdownControllerElement::new(self, screen, widget_dimension)
    }
}

impl<E: compat::JavaEnum> IStringController<E> for EnumDropdownController<E> {
    /// java: @Override public String getString()
    fn get_string(&self) -> String {
        self.formatter.format(self.base.option.pending_value()).get_string()
    }

    /// java: @Override public void setFromString(String value)
    fn set_from_string(&self, value: &str) {
        self.base.option.request_set(self.get_enum_from_string(value));
    }
}

impl<E: compat::JavaEnum> AbstractDropdownController<E> for EnumDropdownController<E> {
    fn dropdown_controller(&self) -> &AbstractDropdownControllerBase<E> {
        &self.base
    }

    /// java: @Override public boolean isValueValid(String value)
    fn is_value_valid(&self, value: &str) -> bool {
        let value = value.to_lowercase();
        for constant in self.get_allowed_values() {
            if constant == value {
                return true;
            }
        }
        false
    }

    /// java: @Override protected String getValidValue(String value, int offset)
    fn get_valid_value_2(&self, value: &str, offset: i32) -> String {
        self.get_valid_enum_constants(value).into_iter().nth(offset.max(0) as usize).unwrap_or_else(|| self.get_string())
    }
}
