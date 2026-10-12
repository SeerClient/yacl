//! Ported from dev.isxander.yacl3.gui.controllers.string.StringController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElementBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public record StringController(Option<String> option) implements IStringController<String>
#[derive(Clone, Debug)]
pub struct StringController {
    /// java: Option<String> option
    pub option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<String>>,
}

impl StringController {
    /// java: the canonical record constructor
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<String>>) -> Self {
        Self { option }
    }

}

impl IStringController<String> for StringController {
    /// java: @Override public String getString()
    fn get_string(&self) -> String {
        return self.option().lock().borrow().pending_value();
    }

    /// java: @Override public void setFromString(String value)
    fn set_from_string(&self, mut value: String) {
        { let (__a0,) = (value.clone(),); self.option().lock().borrow_mut().request_set(__a0) };
    }

}

impl Controller<compat::Object> for StringController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<String> option()
    fn option(&self) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<String>> {
        return self.option.clone();
    }

    /// java: @Override default Component formatValue()
    fn format_value(&self) -> MutableComponent {
        return component::literal(&self.get_string());
    }

    /// java: @Override default AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return new StringControllerElement(this, screen, widgetDimension, true);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
