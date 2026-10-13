//! Ported from dev.isxander.yacl3.gui.controllers.cycling.EnumController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::cycling::cycling_list_controller::CyclingListController;
use crate::dev::isxander::yacl3::gui::controllers::cycling::i_cycling_controller::{self, ICyclingController};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public static <T extends Enum<T>> Function<T, Component> getDefaultFormatter()
///
/// `value instanceof NameableEnum ? getDisplayName() : Component.literal(value.toString())` - an enum's `toString()`
/// is its `name()` (`JavaEnum::name`).
pub fn get_default_formatter<T: compat::JavaEnum>() -> impl Fn(T) -> MutableComponent + 'static {
    |value: T| {
        if let Some(display_name) = value.nameable_display_name() {
            return display_name;
        }
        component::literal(&value.name())
    }
}

/// java: public class EnumController<T extends Enum<T>> extends CyclingListController<T>
///
/// Simple controller type that displays the enum on the right. Cycles forward with left click, cycles backward with
/// right click or when shift is held.
pub struct EnumController<T: compat::JavaEnum> {
    /// java: extends CyclingListController<T>
    pub base: CyclingListController<T>,
}

impl<T: compat::JavaEnum> EnumController<T> {
    /// java: public EnumController(Option<T> option, Class<T> enumClass) -
    /// `this(option, getDefaultFormatter(), enumClass.getEnumConstants())`
    pub fn new(option: Arc<dyn YOption<T>>) -> Self {
        Self::new_3(option, Arc::new(get_default_formatter::<T>()), T::enum_constants())
    }

    /// java: public EnumController(Option<T> option, Function<T, Component> valueFormatter, T[] availableValues)
    pub fn new_3(option: Arc<dyn YOption<T>>, value_formatter: Arc<dyn ValueFormatter<T>>, available_values: Vec<T>) -> Self {
        Self { base: CyclingListController::new_3(option, available_values, value_formatter) }
    }

    /// java: public static <T extends Enum<T>> EnumController<T> createInternal(Option<T> option,
    /// ValueFormatter<T> formatter, T[] values)
    pub fn create_internal(option: Arc<dyn YOption<T>>, formatter: Arc<dyn ValueFormatter<T>>, values: Vec<T>) -> Arc<dyn Controller<T>> {
        Arc::new(EnumController::new_3(option, formatter, values))
    }
}

impl<T: compat::JavaEnum> Controller<T> for EnumController<T> {
    fn option(&self) -> Arc<dyn YOption<T>> {
        self.base.option()
    }

    fn format_value(&self) -> MutableComponent {
        self.base.format_value()
    }

    /// java: ICyclingController.provideWidget (the interface default, inherited)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        i_cycling_controller::provide_widget::<T, Self>(self, screen, widget_dimension)
    }
}

impl<T: compat::JavaEnum> ICyclingController<T> for EnumController<T> {
    fn set_pending_value(&self, ordinal: i32) {
        self.base.set_pending_value(ordinal)
    }

    fn get_pending_value(&self) -> i32 {
        self.base.get_pending_value()
    }

    fn get_cycle_length(&self) -> i32 {
        self.base.get_cycle_length()
    }
}
