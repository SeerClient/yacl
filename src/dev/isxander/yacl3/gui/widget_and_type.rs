//! Ported from dev.isxander.yacl3.gui.WidgetAndType.
//!
//! `getWidget()` answers the game's `AbstractWidget`; the YACL screen's widgets are its own `GuiEventListener`s
//! (PATTERNS_Y2.md P3), so the widget half is that.

#[allow(unused_imports)]
use crate::prelude::*;
use compat::GuiEventListener;

/// java: public interface WidgetAndType<T>
pub trait WidgetAndType<T: ?Sized> {
    /// java: T getType();
    fn get_type(&self) -> Arc<T>;

    /// java: AbstractWidget getWidget();
    fn get_widget(&self) -> Arc<dyn GuiEventListener>;
}

/// java: static <T extends AbstractWidget> WidgetAndType<T> ofWidget(T widget) - the anonymous class over `widget`
struct OfWidget<T: GuiEventListener + 'static> {
    widget: Arc<T>,
}

impl<T: GuiEventListener + 'static> WidgetAndType<T> for OfWidget<T> {
    fn get_type(&self) -> Arc<T> {
        self.widget.clone()
    }

    fn get_widget(&self) -> Arc<dyn GuiEventListener> {
        self.widget.clone()
    }
}

/// java: static <T extends AbstractWidget> WidgetAndType<T> ofWidget(T widget)
pub fn of_widget<T: GuiEventListener + 'static>(widget: Arc<T>) -> Arc<dyn WidgetAndType<T>> {
    Arc::new(OfWidget { widget })
}
