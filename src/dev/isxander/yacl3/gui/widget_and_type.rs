//! Ported from dev.isxander.yacl3.gui.WidgetAndType by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;

/// java: public interface WidgetAndType<T>
pub trait WidgetAndType<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: T getType();
    fn get_type(&self) -> T;

    /// java: AbstractWidget getWidget();
    fn get_widget(&self) -> AbstractWidget;

}

/// java: static <T extends AbstractWidget> WidgetAndType<T> ofWidget(T widget)
pub fn of_widget<T: 'static>(widget: &T) -> compat::Shared<dyn WidgetAndType<T>> {
    // TODO(fabric2seer): `new WidgetAndType(..) {..}` (an anonymous class with state or several methods)
    //   java: return new WidgetAndType<>() {
    //   java:             @Override
    //   java:             public T getType() {
    //   java:                 return widget;
    //   java:             }
    //   java:
    //   java:             @Override
    //   java:             public AbstractWidget getWidget() {
    //   java:                 return widget;
    //   java:             }
    //   java:         };
    todo!("fabric2seer: the value of this function is in a TODO above")
}
