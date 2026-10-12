//! Ported from dev.isxander.yacl3.gui.TooltipButtonWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class TooltipButtonWidget extends Button.Plain
pub struct TooltipButtonWidgetBase {
    /// java: extends net.minecraft.client.gui.components.Button$Plain
    pub base: AbstractWidget,
    /// java: protected final Screen screen;
    pub screen: mcrust::net::minecraft::client::gui::screens::screen::Screen,
}

impl std::ops::Deref for TooltipButtonWidgetBase {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TooltipButtonWidgetBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TooltipButtonWidgetBase {
    /// java: public TooltipButtonWidget(Screen screen, int x, int y, int width, int height, Component message, Component tooltip, OnPress onPress) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut tooltip: MutableComponent, mut on_press: OnPress, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn TooltipButtonWidget>, __R)) -> (compat::Shared<dyn TooltipButtonWidget>, __R) {
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this.screen = screen;
        // TODO(fabric2seer): `new Plain(..)`: the port has no `Plain::new`
        //   java: super(x, y, width, height, message, onPress, DEFAULT_NARRATION);
        let (__this, __r): (compat::Shared<dyn TooltipButtonWidget>, __R) = todo!("constructor");
        if true /* java: tooltip != null: never null here */ {
            { let (__q0,) = (Tooltip::create(tooltip.get_string()),); __this.lock().borrow_mut().tooltip_button_widget_base_mut().set_tooltip(__q0) };
        }
        (__this, __r)
    }

    /// java: public TooltipButtonWidget(Screen screen, int x, int y, int width, int height, Component message, Component tooltip, OnPress onPress)
    pub fn new(mut screen: mcrust::net::minecraft::client::gui::screens::screen::Screen, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut message: MutableComponent, mut tooltip: MutableComponent, mut on_press: OnPress) -> compat::Shared<Self> {
        Self::new_in(screen, x, y, width, height, message, tooltip, on_press, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn TooltipButtonWidget>), __h)
        }).1
    }

}

/// java: the virtual methods of TooltipButtonWidget (implemented by TooltipButtonWidgetBase and every subclass)
pub trait TooltipButtonWidget: compat::DynAny {
    fn tooltip_button_widget_base(&self) -> &TooltipButtonWidgetBase;
    fn tooltip_button_widget_base_mut(&mut self) -> &mut TooltipButtonWidgetBase;
    fn as_any(&self) -> &dyn std::any::Any;

}

impl TooltipButtonWidget for TooltipButtonWidgetBase {
    fn tooltip_button_widget_base(&self) -> &TooltipButtonWidgetBase {
        self
    }
    fn tooltip_button_widget_base_mut(&mut self) -> &mut TooltipButtonWidgetBase {
        self
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
