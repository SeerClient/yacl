//! Ported from dev.isxander.yacl3.gui.controllers.string.IStringController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElement;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface IStringController<T> extends Controller<T>
///
/// Its two `Controller` defaults (`formatValue`, `provideWidget`) are the functions below; an implementation's
/// `Controller` impl calls them (Rust has no default override of a supertrait's method).
pub trait IStringController<T: Clone + 'static>: Controller<T> {
    /// java: String getString();
    fn get_string(&self) -> String;

    /// java: void setFromString(String value);
    fn set_from_string(&self, value: &str);

    /// java: default boolean isInputValid(String input)
    fn is_input_valid(&self, input: &str) -> bool {
        let _ = input;
        true
    }
}

/// java: @Override default Component formatValue() - `Component.literal(getString())`
pub fn format_value<T: Clone + 'static, C: IStringController<T> + ?Sized>(this: &C) -> MutableComponent {
    component::literal(&this.get_string())
}

/// java: @Override default AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
pub fn provide_widget<T: Clone + 'static, C: IStringController<T> + ?Sized + 'static>(
    this: Arc<C>,
    screen: &Arc<YACLScreen>,
    widget_dimension: Arc<dyn Dimension<i32>>,
) -> Arc<dyn AbstractWidget> {
    Arc::new(StringControllerElement::new(this, screen, widget_dimension, true))
}
