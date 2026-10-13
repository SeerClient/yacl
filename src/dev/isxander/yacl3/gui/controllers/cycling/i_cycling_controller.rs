//! Ported from dev.isxander.yacl3.gui.controllers.cycling.ICyclingController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::cycling::cycling_controller_element::CyclingControllerElement;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;

/// java: public interface ICyclingController<T> extends Controller<T>
///
/// This interface simply generifies setting and getting of the pending value, using an ordinal so elements can cycle
/// through without knowing the content. Its `Controller` default (`provideWidget`) is the function below; an
/// implementation's `Controller` impl calls it.
pub trait ICyclingController<T: Clone + 'static>: Controller<T> {
    /// java: void setPendingValue(int ordinal); - sets the pending value to whatever corresponds to the ordinal
    fn set_pending_value(&self, ordinal: i32);

    /// java: int getPendingValue(); - gets the pending ordinal that corresponds to the actual value
    fn get_pending_value(&self) -> i32;

    /// java: int getCycleLength(); - allows the element when it should wrap-around back to zeroth ordinal
    fn get_cycle_length(&self) -> i32;
}

/// java: @Override default AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
pub fn provide_widget<T: Clone + 'static, C: ICyclingController<T> + ?Sized + 'static>(
    this: Arc<C>,
    screen: &Arc<YACLScreen>,
    widget_dimension: Arc<dyn Dimension<i32>>,
) -> Arc<dyn AbstractWidget> {
    Arc::new(CyclingControllerElement::new(this, screen, widget_dimension))
}
