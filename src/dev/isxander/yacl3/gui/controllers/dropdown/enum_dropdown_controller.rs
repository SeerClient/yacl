//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.EnumDropdownController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownControllerBase;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::enum_dropdown_controller_element::EnumDropdownControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::option::Option as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class EnumDropdownController<E extends Enum<E>> extends AbstractDropdownController<E>
#[derive(Clone)]
pub struct EnumDropdownController<E> {
    /// java: extends AbstractDropdownController
    pub base: AbstractDropdownControllerBase<E>,
    /// java: protected final ValueFormatter<E> formatter;
    pub formatter: compat::Shared<dyn ValueFormatter<E>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl<E: 'static> std::ops::Deref for EnumDropdownController<E> {
    type Target = AbstractDropdownControllerBase<E>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<E: 'static> std::ops::DerefMut for EnumDropdownController<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<E: Clone + 'static> EnumDropdownController<E> {
    /// java: public EnumDropdownController(Option<E> option, ValueFormatter<E> formatter)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<E>>, mut formatter: compat::Shared<dyn ValueFormatter<E>>) -> compat::Shared<Self> {
        let __init_formatter = formatter.clone();
        // TODO(fabric2seer): `.getDeclaringClass(..)` on a T
        //   java: super(option, Arrays.stream(option.pendingValue().getDeclaringClass().getEnumConstants()).map(formatter::format).map(Component::getString).toList());
        let __this: compat::Shared<Self> = todo!("constructor");
        __this
    }

    /// java: private E getEnumFromString(String value)
    pub fn get_enum_from_string(__this: &compat::Shared<Self>, mut value: String) -> E {
        value = value.to_lowercase();
        // TODO(fabric2seer): `.getDeclaringClass(..)` on a T
        //   java: for (E constant : option().pendingValue().getDeclaringClass().getEnumConstants()) {
        //   java:             if (formatter.format(constant).getString().toLowerCase().equals(value)) return constant;
        //   java:         }
        return __this.lock().borrow().option().lock().borrow().pending_value();
    }

    /// java: @NotNull protected Stream<String> getValidEnumConstants(String value)
    pub fn get_valid_enum_constants(__this: &compat::Shared<Self>, value: &str) -> Box<dyn Iterator<Item = String> + '_> {
        let mut value_lower_case: String = value.to_lowercase();
        return Box::new(compat::sorted_by(__this.lock().borrow().get_allowed_values().into_iter().filter(|constant| constant.to_lowercase().contains(&value_lower_case)), |s1, s2| {
            let mut s1_lower_case: String = s1.to_lowercase();
            let mut s2_lower_case: String = s2.to_lowercase();
            if s1_lower_case.starts_with(&value_lower_case) && !(s2_lower_case.starts_with(&value_lower_case)) {
                return -1;
            }
            if !(s1_lower_case.starts_with(&value_lower_case)) && s2_lower_case.starts_with(&value_lower_case) {
                return 1;
            }
            return compat::compare_to(&s1, s2);
        }));
    }

}

impl<E: Clone + 'static> AbstractDropdownController<E> for EnumDropdownController<E> {
    fn abstract_dropdown_controller_base(&self) -> &AbstractDropdownControllerBase<E> {
        &self.base
    }
    fn abstract_dropdown_controller_base_mut(&mut self) -> &mut AbstractDropdownControllerBase<E> {
        &mut self.base
    }

    /// java: @Override public boolean isValueValid(String value)
    fn is_value_valid(&self, mut value: String) -> bool {
        value = value.to_lowercase();
        for constant in self.get_allowed_values() {
            if constant == value {
                return true;
            }
        }
        return false;
    }

    /// java: @Override protected String getValidValue(String value, int offset)
    fn get_valid_value_2(&self, value: &str, mut offset: i32) -> String {
        return EnumDropdownController::get_valid_enum_constants(&self.self_ref.upgrade().unwrap(), value).skip((offset) as usize).next().unwrap_or_else(|| self.get_string());
    }

}

impl<E: Clone + 'static> IStringController<compat::Object> for EnumDropdownController<E> {
    /// java: @Override public String getString()
    fn get_string(&self) -> String {
        return ({ let (__a0,) = (&self.option().lock().borrow().pending_value(),); self.formatter.lock().borrow().format(__a0) }).get_string();
    }

    /// java: @Override public void setFromString(String value)
    fn set_from_string(&self, mut value: String) {
        { let __s0 = EnumDropdownController::get_enum_from_string(&self.self_ref.upgrade().unwrap(), value.clone()); self.option().lock().borrow_mut().request_set(__s0) };
    }

}

impl<E: Clone + 'static> Controller<compat::Object> for EnumDropdownController<E> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        return (EnumDropdownControllerElement::new(self.self_ref.upgrade().unwrap(), screen.clone(), widget_dimension.clone()) as compat::Shared<dyn AbstractWidget>);
    }

    /// java: @Override public Option<T> option()
    fn option(&self) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>> {
        return self.option.clone().unwrap();
    }

    /// java: @Override default Component formatValue()
    fn format_value(&self) -> MutableComponent {
        return component::literal(&self.get_string());
    }

}
