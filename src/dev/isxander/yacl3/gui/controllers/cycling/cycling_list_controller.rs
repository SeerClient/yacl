//! Ported from dev.isxander.yacl3.gui.controllers.cycling.CyclingListController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::cycling::i_cycling_controller::{self, ICyclingController};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class CyclingListController<T> implements ICyclingController<T>
///
/// A controller where once clicked, cycles through elements in the provided list.
pub struct CyclingListController<T: Clone + PartialEq + 'static> {
    /// java: private final Option<T> option;
    option: Arc<dyn YOption<T>>,
    /// java: private final ValueFormatter<T> valueFormatter;
    value_formatter: Arc<dyn ValueFormatter<T>>,
    /// java: private final ImmutableList<T> values;
    values: Vec<T>,
}

impl<T: Clone + PartialEq + 'static> CyclingListController<T> {
    /// java: public CyclingListController(Option<T> option, Iterable<? extends T> values) - with a default value
    /// formatter of `Object.toString()`, which is `to_string` here.
    pub fn new(option: Arc<dyn YOption<T>>, values: Vec<T>, to_string: impl Fn(&T) -> String + 'static) -> Self {
        Self::new_3(option, values, Arc::new(move |value: T| component::literal(&to_string(&value))))
    }

    /// java: public CyclingListController(Option<T> option, Iterable<? extends T> values, Function<T, Component> valueFormatter)
    pub fn new_3(option: Arc<dyn YOption<T>>, values: Vec<T>, value_formatter: Arc<dyn ValueFormatter<T>>) -> Self {
        Self { option, value_formatter, values }
    }

    /// java: @ApiStatus.Internal public static <T> CyclingListController<T> createInternal(Option<T> option,
    /// Iterable<? extends T> values, ValueFormatter<T> formatter)
    pub fn create_internal(option: Arc<dyn YOption<T>>, values: Vec<T>, formatter: Arc<dyn ValueFormatter<T>>) -> Arc<dyn Controller<T>> {
        Arc::new(CyclingListController::new_3(option, values, formatter))
    }
}

impl<T: Clone + PartialEq + 'static> Controller<T> for CyclingListController<T> {
    /// java: @Override public Option<T> option()
    fn option(&self) -> Arc<dyn YOption<T>> {
        self.option.clone()
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        self.value_formatter.format(self.option.pending_value())
    }

    /// java: ICyclingController.provideWidget (the interface default)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        i_cycling_controller::provide_widget::<T, Self>(self, screen, widget_dimension)
    }
}

impl<T: Clone + PartialEq + 'static> ICyclingController<T> for CyclingListController<T> {
    /// java: @Override public void setPendingValue(int ordinal)
    fn set_pending_value(&self, ordinal: i32) {
        // `values.get(ordinal)` throws IndexOutOfBoundsException for a bad ordinal
        self.option.request_set(self.values[ordinal as usize].clone());
    }

    /// java: @Override public int getPendingValue() - `values.indexOf(option().pendingValue())`
    fn get_pending_value(&self) -> i32 {
        let pending = self.option.pending_value();
        self.values.iter().position(|value| *value == pending).map_or(-1, |index| index as i32)
    }

    /// java: @Override public int getCycleLength()
    fn get_cycle_length(&self) -> i32 {
        self.values.len() as i32
    }
}
