//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.AbstractDropdownController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;

/// java: public abstract class AbstractDropdownController<T> implements IStringController<T>
#[derive(Clone)]
pub struct AbstractDropdownControllerBase<T> {
    /// java: protected final Option<T> option;
    pub option: Option<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>>,
    /// java: private final List<String> allowedValues;
    pub allowed_values: Vec<String>,
    /// java: public final boolean allowEmptyValue;
    pub allow_empty_value: bool,
    /// java: public final boolean allowAnyValue;
    pub allow_any_value: bool,
}

impl<T: Clone + 'static> AbstractDropdownControllerBase<T> {
    /// java: protected AbstractDropdownController(Option<T> option, List<String> allowedValues, boolean allowEmptyValue, boolean allowAnyValue)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, mut allowed_values: Vec<String>, mut allow_empty_value: bool, mut allow_any_value: bool) -> Self {
        let __init_option = Some(option.clone());
        let __init_allowed_values = allowed_values.clone();
        let __init_allow_empty_value = allow_empty_value;
        let __init_allow_any_value = allow_any_value;
        let mut this = Self {
            option: __init_option,
            allowed_values: __init_allowed_values,
            allow_empty_value: __init_allow_empty_value,
            allow_any_value: __init_allow_any_value,
        };
        this
    }

    /// java: protected AbstractDropdownController(Option<T> option, List<String> allowedValues)
    pub fn new_2(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, mut allowed_values: Vec<String>) -> Self {
        let mut this = Self::new(option.clone(), allowed_values.clone(), false, false);
        this
    }

    /// java: protected AbstractDropdownController(Option<T> option)
    pub fn new_1(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> Self {
        let mut this = Self::new_2(option.clone(), Default::default());
        this
    }

}

/// java: the virtual methods of AbstractDropdownController (implemented by AbstractDropdownControllerBase and every subclass)
pub trait AbstractDropdownController<T: Clone + 'static>: IStringController {
    fn abstract_dropdown_controller_base(&self) -> &AbstractDropdownControllerBase<T>;
    fn abstract_dropdown_controller_base_mut(&mut self) -> &mut AbstractDropdownControllerBase<T>;

    /// java: public List<String> getAllowedValues()
    fn get_allowed_values(&self) -> Vec<String> {
        return self.get_allowed_values_1("".to_string());
    }

    /// java: public List<String> getAllowedValues(String inputField)
    fn get_allowed_values_1(&self, mut input_field: String) -> Vec<String> {
        let mut values: Vec<String> = self.abstract_dropdown_controller_base().allowed_values.iter().cloned().collect::<Vec<_>>();
        if self.abstract_dropdown_controller_base().allow_empty_value && !(values.iter().any(|e| *e == "")) {
            values.push("".to_string());
        }
        if self.abstract_dropdown_controller_base().allow_any_value && !(compat::is_blank(&input_field)) && !(self.abstract_dropdown_controller_base().allowed_values.iter().any(|e| *e == input_field.clone())) {
            values.push(input_field.clone());
        }
        let mut current_value: String = self.get_string();
        if self.abstract_dropdown_controller_base().allow_any_value && !(self.abstract_dropdown_controller_base().allowed_values.iter().any(|e| *e == current_value.clone())) {
            values.push(current_value.clone());
        }
        return values;
    }

    /// java: public boolean isValueValid(String value)
    fn is_value_valid(&self, mut value: String) -> bool {
        if compat::is_blank(&value) {
            return self.abstract_dropdown_controller_base().allow_empty_value;
        }
        return (self.abstract_dropdown_controller_base().allow_any_value || self.get_allowed_values().iter().any(|e| *e == value.clone()));
    }

    /// java: protected String getValidValue(String value)
    fn get_valid_value(&self, value: &str) -> String {
        return self.get_valid_value_2(value, 0);
    }

    /// java: protected String getValidValue(String value, int offset)
    fn get_valid_value_2(&self, value: &str, mut offset: i32) -> String {
        if offset == -1 {
            return self.get_string();
        }
        let mut value_lower_case: String = value.to_lowercase();
        return compat::sorted_by(self.get_allowed_values_1(value.to_string()).into_iter().filter(|val| val.to_lowercase().contains(&value_lower_case)), |s1, s2| {
            let mut s1_lower_case: String = s1.to_lowercase();
            let mut s2_lower_case: String = s2.to_lowercase();
            if s1_lower_case.starts_with(&value_lower_case) && !(s2_lower_case.starts_with(&value_lower_case)) {
                return -1;
            }
            if !(s1_lower_case.starts_with(&value_lower_case)) && s2_lower_case.starts_with(&value_lower_case) {
                return 1;
            }
            return compat::compare_to(&s1, s2);
        }).skip((offset) as usize).next().unwrap_or_else(|| self.get_string());
    }

}

// AbstractDropdownController.option implements Controller.option: it is emitted in each concrete subclass's `impl Controller` (Rust has no inherited impls)
