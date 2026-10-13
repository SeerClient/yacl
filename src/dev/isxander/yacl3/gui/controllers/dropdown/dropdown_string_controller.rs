//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.DropdownStringController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::{
    AbstractDropdownController, AbstractDropdownControllerBase,
};
use crate::dev::isxander::yacl3::gui::controllers::dropdown::dropdown_string_controller_element::DropdownStringControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::{self, IStringController};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class DropdownStringController extends AbstractDropdownController<String>
pub struct DropdownStringController {
    /// java: extends AbstractDropdownController<String>
    pub base: AbstractDropdownControllerBase<String>,
}

impl DropdownStringController {
    /// java: public DropdownStringController(Option<String> option, List<String> allowedValues, boolean allowEmptyValue,
    /// boolean allowAnyValue)
    pub fn new(option: Arc<dyn YOption<String>>, allowed_values: Vec<String>, allow_empty_value: bool, allow_any_value: bool) -> Arc<Self> {
        Arc::new(Self { base: AbstractDropdownControllerBase::new(option, allowed_values, allow_empty_value, allow_any_value) })
    }
}

impl Controller<String> for DropdownStringController {
    /// java: AbstractDropdownController.option()
    fn option(&self) -> Arc<dyn YOption<String>> {
        self.base.option.clone()
    }

    /// java: IStringController.formatValue() (the interface default)
    fn format_value(&self) -> MutableComponent {
        i_string_controller::format_value::<String, Self>(self)
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        DropdownStringControllerElement::new(self, screen, widget_dimension)
    }
}

impl IStringController<String> for DropdownStringController {
    /// java: @Override public String getString()
    fn get_string(&self) -> String {
        self.base.option.pending_value()
    }

    /// java: @Override public void setFromString(String value)
    fn set_from_string(&self, value: &str) {
        self.base.option.request_set(self.get_valid_value(value));
    }
}

impl AbstractDropdownController<String> for DropdownStringController {
    fn dropdown_controller(&self) -> &AbstractDropdownControllerBase<String> {
        &self.base
    }
}
