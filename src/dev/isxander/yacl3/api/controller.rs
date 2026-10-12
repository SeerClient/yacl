//! Ported from dev.isxander.yacl3.api.Controller.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface Controller<T>
///
/// `provideWidget` takes the controller's own reference (`self: Arc<Self>`): the widget it builds keeps the controller
/// (Java's `new XControllerElement(this, ..)`).
pub trait Controller<T: Clone + 'static>: compat::DynAny {
    /// java: Option<T> option();
    fn option(&self) -> Arc<dyn YOption<T>>;

    /// java: Component formatValue();
    fn format_value(&self) -> MutableComponent;

    /// java: AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension);
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget>;
}

// the package's other modules
pub mod boolean_controller_builder;
pub mod color_controller_builder;
pub mod controller_builder;
pub mod cycling_list_controller_builder;
pub mod double_field_controller_builder;
pub mod double_slider_controller_builder;
pub mod dropdown_string_controller_builder;
pub mod enum_controller_builder;
pub mod enum_dropdown_controller_builder;
pub mod float_field_controller_builder;
pub mod float_slider_controller_builder;
pub mod integer_field_controller_builder;
pub mod integer_slider_controller_builder;
pub mod item_controller_builder;
pub mod long_field_controller_builder;
pub mod long_slider_controller_builder;
pub mod number_field_controller_builder;
pub mod slider_controller_builder;
pub mod string_controller_builder;
pub mod tick_box_controller_builder;
pub mod value_formattable_controller;
pub mod value_formatter;
