//! Ported from dev.isxander.yacl3.gui.controllers.string.IStringController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElementBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface IStringController<T> extends Controller<T>
pub trait IStringController<T: Clone + 'static>: Controller {
    /// java: String getString();
    fn get_string(&self) -> String;

    /// java: void setFromString(String value);
    fn set_from_string(&self, value: String);

    /// java: @Override default Component formatValue()
    fn format_value(&self) -> MutableComponent {
        return component::literal(&self.get_string());
    }

    /// java: default boolean isInputValid(String input)
    fn is_input_valid(&self, mut input: String) -> bool {
        return true;
    }

    /// java: @Override default AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return new StringControllerElement(this, screen, widgetDimension, true);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
