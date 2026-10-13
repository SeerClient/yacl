//! Ported from dev.isxander.yacl3.gui.controllers.slider.ISliderController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::slider::slider_controller_element::SliderControllerElement;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;

/// java: public interface ISliderController<T extends Number> extends Controller<T>
///
/// Simple custom slider implementation that shifts the current value across when shown. For simplicity,
/// `SliderControllerElement` works in doubles so each `ISliderController` must cast to double. Its `Controller`
/// default (`provideWidget`) is the function below; an implementation's `Controller` impl calls it.
pub trait ISliderController<T: Clone + 'static>: Controller<T> {
    /// java: double min(); - gets the minimum value for the slider
    fn min(&self) -> f64;

    /// java: double max(); - gets the maximum value for the slider
    fn max(&self) -> f64;

    /// java: double interval(); - gets the interval (or step size) for the slider
    fn interval(&self) -> f64;

    /// java: default double range() - gets the range of the slider
    fn range(&self) -> f64 {
        <Self as ISliderController<T>>::max(self) - <Self as ISliderController<T>>::min(self)
    }

    /// java: void setPendingValue(double value); - sets the option's pending value
    fn set_pending_value(&self, value: f64);

    /// java: double pendingValue(); - gets the option's pending value
    fn pending_value(&self) -> f64;
}

/// java: @Override default AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
pub fn provide_widget<T: Clone + 'static, C: ISliderController<T> + ?Sized + 'static>(
    this: Arc<C>,
    screen: &Arc<YACLScreen>,
    widget_dimension: Arc<dyn Dimension<i32>>,
) -> Arc<dyn AbstractWidget> {
    let (min, max, interval) = (<C as ISliderController<T>>::min(&this), <C as ISliderController<T>>::max(&this), this.interval());
    Arc::new(SliderControllerElement::new(this, screen, widget_dimension, min, max, interval))
}
