//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.EnumDropdownControllerElement.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::{
    self as dropdown_element, AbstractDropdownControllerElement, AbstractDropdownControllerElementClass, DropdownElement,
};
use crate::dev::isxander::yacl3::gui::controllers::dropdown::enum_dropdown_controller::EnumDropdownController;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::{StringControllerElement, StringControllerElementClass};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use std::sync::Weak;

/// java: public class EnumDropdownControllerElement<E extends Enum<E>> extends AbstractDropdownControllerElement<E, String>
pub struct EnumDropdownControllerElement<E: compat::JavaEnum> {
    /// java: extends AbstractDropdownControllerElement<E, String>
    pub dropdown: AbstractDropdownControllerElement<String>,
    /// java: private final EnumDropdownController<E> controller;
    controller: Arc<EnumDropdownController<E>>,
    /// `this`
    this: Weak<EnumDropdownControllerElement<E>>,
}

impl<E: compat::JavaEnum> EnumDropdownControllerElement<E> {
    /// java: public EnumDropdownControllerElement(EnumDropdownController<E> control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(control: Arc<EnumDropdownController<E>>, screen: &Arc<YACLScreen>, dim: Arc<dyn Dimension<i32>>) -> Arc<Self> {
        let element = Arc::new_cyclic(|this| Self {
            dropdown: AbstractDropdownControllerElement::new::<E, EnumDropdownController<E>>(control.clone(), screen, dim),
            controller: control,
            this: this.clone(),
        });
        dropdown_element::register_matching_values_listener(&element);
        element
    }
}

impl<E: compat::JavaEnum> StringControllerElementClass for EnumDropdownControllerElement<E> {
    fn string_element(&self) -> &StringControllerElement {
        &self.dropdown.string
    }

    crate::abstract_dropdown_controller_element_overrides!();
}

impl<E: compat::JavaEnum> AbstractDropdownControllerElementClass for EnumDropdownControllerElement<E> {
    type Value = String;

    fn dropdown_element(&self) -> &AbstractDropdownControllerElement<String> {
        &self.dropdown
    }

    fn this_dropdown(&self) -> (Weak<dyn DropdownElement>, Weak<dyn ControllerWidget>) {
        (self.this.clone(), self.this.clone())
    }

    /// java: @Override public List<String> computeMatchingValues()
    fn compute_matching_values(&self) -> Vec<String> {
        self.controller.get_valid_enum_constants(&self.dropdown.string.input_field())
    }

    /// java: @Override public String getString(String object)
    fn get_string_of(&self, object: &String) -> String {
        object.clone()
    }
}
