//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.AbstractDropdownController.
//!
//! `public abstract class AbstractDropdownController<T> implements IStringController<T>`: a subclass holds an
//! `AbstractDropdownControllerBase<T>` (the class's fields) and implements `AbstractDropdownController<T>`, whose
//! defaults are this class's bodies (the overridable ones a subclass replaces: `isValueValid`, `getValidValue`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;

/// java: the fields of `AbstractDropdownController<T>`
pub struct AbstractDropdownControllerBase<T: Clone + 'static> {
    /// java: protected final Option<T> option;
    pub option: Arc<dyn YOption<T>>,
    /// java: private final List<String> allowedValues;
    allowed_values: Vec<String>,
    /// java: public final boolean allowEmptyValue;
    pub allow_empty_value: bool,
    /// java: public final boolean allowAnyValue;
    pub allow_any_value: bool,
}

impl<T: Clone + 'static> AbstractDropdownControllerBase<T> {
    /// java: protected AbstractDropdownController(Option<T> option, List<String> allowedValues, boolean allowEmptyValue,
    /// boolean allowAnyValue)
    pub fn new(option: Arc<dyn YOption<T>>, allowed_values: Vec<String>, allow_empty_value: bool, allow_any_value: bool) -> Self {
        Self { option, allowed_values, allow_empty_value, allow_any_value }
    }

    /// java: protected AbstractDropdownController(Option<T> option, List<String> allowedValues)
    pub fn new_2(option: Arc<dyn YOption<T>>, allowed_values: Vec<String>) -> Self {
        Self::new(option, allowed_values, false, false)
    }

    /// java: protected AbstractDropdownController(Option<T> option)
    pub fn new_1(option: Arc<dyn YOption<T>>) -> Self {
        Self::new_2(option, Vec::new())
    }

    /// `allowedValues`
    pub fn allowed_values(&self) -> &[String] {
        &self.allowed_values
    }
}

/// java: the overridable methods `AbstractDropdownController<T>` adds, each defaulting to its body
pub trait AbstractDropdownController<T: Clone + 'static>: IStringController<T> {
    /// The class's fields.
    fn dropdown_controller(&self) -> &AbstractDropdownControllerBase<T>;

    /// java: public List<String> getAllowedValues()
    fn get_allowed_values(&self) -> Vec<String> {
        self.get_allowed_values_1("")
    }

    /// java: public List<String> getAllowedValues(String inputField)
    fn get_allowed_values_1(&self, input_field: &str) -> Vec<String> {
        get_allowed_values_1(self, input_field)
    }

    /// java: public boolean isValueValid(String value)
    fn is_value_valid(&self, value: &str) -> bool {
        is_value_valid(self, value)
    }

    /// java: protected String getValidValue(String value)
    fn get_valid_value(&self, value: &str) -> String {
        self.get_valid_value_2(value, 0)
    }

    /// java: protected String getValidValue(String value, int offset)
    fn get_valid_value_2(&self, value: &str, offset: i32) -> String {
        get_valid_value_2(self, value, offset)
    }
}

/// java: public List<String> getAllowedValues(String inputField)
pub fn get_allowed_values_1<T: Clone + 'static, C: AbstractDropdownController<T> + ?Sized>(this: &C, input_field: &str) -> Vec<String> {
    let base = this.dropdown_controller();
    let mut values: Vec<String> = base.allowed_values.clone();
    if base.allow_empty_value && !values.iter().any(|v| v.is_empty()) {
        values.push(String::new());
    }
    if base.allow_any_value && !compat::is_blank(input_field) && !base.allowed_values.iter().any(|v| v == input_field) {
        values.push(input_field.to_string());
    }
    let current_value = this.get_string();
    if base.allow_any_value && !base.allowed_values.contains(&current_value) {
        values.push(current_value);
    }
    values
}

/// java: public boolean isValueValid(String value)
pub fn is_value_valid<T: Clone + 'static, C: AbstractDropdownController<T> + ?Sized>(this: &C, value: &str) -> bool {
    let base = this.dropdown_controller();
    if compat::is_blank(value) {
        return base.allow_empty_value;
    }
    base.allow_any_value || this.get_allowed_values().iter().any(|v| v == value)
}

/// The comparator `getValidValue` / `getValidEnumConstants` sort with: values starting with the input first, then
/// `String.compareTo`.
pub fn compare_by_prefix(s1: &str, s2: &str, value_lower_case: &str) -> std::cmp::Ordering {
    let s1_lower_case = s1.to_lowercase();
    let s2_lower_case = s2.to_lowercase();
    if s1_lower_case.starts_with(value_lower_case) && !s2_lower_case.starts_with(value_lower_case) {
        return std::cmp::Ordering::Less;
    }
    if !s1_lower_case.starts_with(value_lower_case) && s2_lower_case.starts_with(value_lower_case) {
        return std::cmp::Ordering::Greater;
    }
    compat::compare_to(s1, s2).cmp(&0)
}

/// java: protected String getValidValue(String value, int offset)
pub fn get_valid_value_2<T: Clone + 'static, C: AbstractDropdownController<T> + ?Sized>(this: &C, value: &str, offset: i32) -> String {
    if offset == -1 {
        return this.get_string();
    }
    let value_lower_case = value.to_lowercase();
    let mut values: Vec<String> =
        this.get_allowed_values_1(value).into_iter().filter(|val| val.to_lowercase().contains(&value_lower_case)).collect();
    values.sort_by(|s1, s2| compare_by_prefix(s1, s2, &value_lower_case));
    values.into_iter().nth(offset.max(0) as usize).unwrap_or_else(|| this.get_string())
}
