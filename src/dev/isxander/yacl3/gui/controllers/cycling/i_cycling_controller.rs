//! Ported from dev.isxander.yacl3.gui.controllers.cycling.ICyclingController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::cycling::cycling_controller_element::CyclingControllerElement;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;

/// java: public interface ICyclingController<T> extends Controller<T>
pub trait ICyclingController<T: Clone + 'static>: Controller {
    /// java: void setPendingValue(int ordinal);
    fn set_pending_value(&self, ordinal: i32);

    /// java: int getPendingValue();
    fn get_pending_value(&self) -> i32;

    /// java: int getCycleLength();
    fn get_cycle_length(&self) -> i32;

    /// java: @Override default AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return new CyclingControllerElement(this, screen, widgetDimension);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
