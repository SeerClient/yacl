//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.DropdownStringController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownControllerBase;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::dropdown_string_controller_element::DropdownStringControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public class DropdownStringController extends AbstractDropdownController<String>
#[derive(Clone)]
pub struct DropdownStringController {
    /// java: extends AbstractDropdownController
    pub base: AbstractDropdownControllerBase<String>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for DropdownStringController {
    type Target = AbstractDropdownControllerBase<String>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DropdownStringController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DropdownStringController {
    /// java: public DropdownStringController(Option<String> option, List<String> allowedValues, boolean allowEmptyValue, boolean allowAnyValue)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<String>>, mut allowed_values: Vec<String>, mut allow_empty_value: bool, mut allow_any_value: bool) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractDropdownControllerBase::<String>::new(option.clone(), allowed_values.clone(), allow_empty_value, allow_any_value),
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl AbstractDropdownController<String> for DropdownStringController {
    fn abstract_dropdown_controller_base(&self) -> &AbstractDropdownControllerBase<String> {
        &self.base
    }
    fn abstract_dropdown_controller_base_mut(&mut self) -> &mut AbstractDropdownControllerBase<String> {
        &mut self.base
    }

}

impl IStringController<compat::Object> for DropdownStringController {
    /// java: @Override public String getString()
    fn get_string(&self) -> String {
        return self.option().lock().borrow().pending_value();
    }

    /// java: @Override public void setFromString(String value)
    fn set_from_string(&self, mut value: String) {
        { let __s0 = self.get_valid_value(&value); self.option().lock().borrow_mut().request_set(__s0) };
    }

}

impl Controller<compat::Object> for DropdownStringController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        return (DropdownStringControllerElement::new(self.self_ref.upgrade().unwrap(), screen.clone(), widget_dimension.clone()) as compat::Shared<dyn AbstractWidget>);
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
