//! Ported from dev.isxander.yacl3.gui.controllers.slider.ISliderController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::slider::slider_controller_element::SliderControllerElement;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;

/// java: public interface ISliderController<T extends Number> extends Controller<T>
pub trait ISliderController<T: Clone + 'static>: Controller {
    /// java: double min();
    fn min(&self) -> f64;

    /// java: double max();
    fn max(&self) -> f64;

    /// java: double interval();
    fn interval(&self) -> f64;

    /// java: default double range()
    fn range(&self) -> f64 {
        return (self.max() - self.min());
    }

    /// java: void setPendingValue(double value);
    fn set_pending_value(&self, value: f64);

    /// java: double pendingValue();
    fn pending_value(&self) -> f64;

    /// java: @Override default AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return new SliderControllerElement(this, screen, widgetDimension, min(), max(), interval());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
