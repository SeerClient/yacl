//! Ported from dev.isxander.yacl3.gui.controllers.string.StringController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::{self, IStringController};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public record StringController(Option<String> option) implements IStringController<String>
///
/// Simple text field controller.
pub struct StringController {
    /// java: Option<String> option
    pub option: Arc<dyn YOption<String>>,
}

impl StringController {
    /// java: public StringController (the canonical record constructor) - constructs a string controller
    pub fn new(option: Arc<dyn YOption<String>>) -> Self {
        Self { option }
    }
}

impl Controller<String> for StringController {
    /// java: @Override public Option<String> option()
    fn option(&self) -> Arc<dyn YOption<String>> {
        self.option.clone()
    }

    /// java: IStringController.formatValue() (the interface default)
    fn format_value(&self) -> MutableComponent {
        i_string_controller::format_value::<String, Self>(self)
    }

    /// java: IStringController.provideWidget(..) (the interface default)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        i_string_controller::provide_widget::<String, Self>(self, screen, widget_dimension)
    }
}

impl IStringController<String> for StringController {
    /// java: @Override public String getString()
    fn get_string(&self) -> String {
        self.option.pending_value()
    }

    /// java: @Override public void setFromString(String value)
    fn set_from_string(&self, value: &str) {
        self.option.request_set(value.to_string());
    }
}
