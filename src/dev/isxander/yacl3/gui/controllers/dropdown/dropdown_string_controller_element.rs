//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.DropdownStringControllerElement.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::{
    self as dropdown_element, AbstractDropdownControllerElement, AbstractDropdownControllerElementClass, DropdownElement,
};
use crate::dev::isxander::yacl3::gui::controllers::dropdown::dropdown_string_controller::DropdownStringController;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::{StringControllerElement, StringControllerElementClass};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use std::sync::Weak;

/// java: public class DropdownStringControllerElement extends AbstractDropdownControllerElement<String, String>
pub struct DropdownStringControllerElement {
    /// java: extends AbstractDropdownControllerElement<String, String>
    pub dropdown: AbstractDropdownControllerElement<String>,
    /// java: private final DropdownStringController controller;
    controller: Arc<DropdownStringController>,
    /// `this`
    this: Weak<DropdownStringControllerElement>,
}

impl DropdownStringControllerElement {
    /// java: public DropdownStringControllerElement(DropdownStringController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(control: Arc<DropdownStringController>, screen: &Arc<YACLScreen>, dim: Arc<dyn Dimension<i32>>) -> Arc<Self> {
        let element = Arc::new_cyclic(|this| Self {
            dropdown: AbstractDropdownControllerElement::new::<String, DropdownStringController>(control.clone(), screen, dim),
            controller: control,
            this: this.clone(),
        });
        dropdown_element::register_matching_values_listener(&element);
        element
    }
}

impl StringControllerElementClass for DropdownStringControllerElement {
    fn string_element(&self) -> &StringControllerElement {
        &self.dropdown.string
    }

    crate::abstract_dropdown_controller_element_overrides!();
}

impl AbstractDropdownControllerElementClass for DropdownStringControllerElement {
    type Value = String;

    fn dropdown_element(&self) -> &AbstractDropdownControllerElement<String> {
        &self.dropdown
    }

    fn this_dropdown(&self) -> (Weak<dyn DropdownElement>, Weak<dyn ControllerWidget>) {
        (self.this.clone(), self.this.clone())
    }

    /// java: @Override public List<String> computeMatchingValues()
    fn compute_matching_values(&self) -> Vec<String> {
        let input_field = self.dropdown.string.input_field();
        let mut values: Vec<String> =
            self.controller.get_allowed_values_1(&input_field).into_iter().filter(|value| self.matching_value(value)).collect();
        values.sort_by(|s1, s2| {
            if s1.starts_with(input_field.as_str()) && !s2.starts_with(input_field.as_str()) {
                return std::cmp::Ordering::Less;
            }
            if !s1.starts_with(input_field.as_str()) && s2.starts_with(input_field.as_str()) {
                return std::cmp::Ordering::Greater;
            }
            compat::compare_to(s1, s2).cmp(&0)
        });
        values
    }

    /// java: public String getString(String object)
    fn get_string_of(&self, object: &String) -> String {
        object.clone()
    }
}
