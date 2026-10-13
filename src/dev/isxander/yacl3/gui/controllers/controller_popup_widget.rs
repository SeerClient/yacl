//! Ported from dev.isxander.yacl3.gui.controllers.ControllerPopupWidget.
//!
//! `public abstract class ControllerPopupWidget<T extends Controller<?>> extends ControllerWidget<Controller<?>>
//! implements GuiEventListener`. A subclass keeps a `ControllerPopupWidgetBase` (the class's fields: its
//! `ControllerWidget` half and `entryWidget`) and implements `ControllerWidget` (calling `key_pressed` /
//! `get_hovered_control_width` below for the two methods this class overrides, unless it overrides them again) and
//! this trait (the methods the class adds).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::{ControllerWidget, ControllerWidgetBase};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::Graphics;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::sync::Weak;

/// java: the fields of `ControllerPopupWidget<T extends Controller<?>>`
pub struct ControllerPopupWidgetBase {
    /// java: extends ControllerWidget<Controller<?>>
    pub cw: ControllerWidgetBase,
    /// java: public final ControllerWidget<?> entryWidget; (held weakly: the entry widget holds its popup)
    pub entry_widget: Weak<dyn ControllerWidget>,
}

impl ControllerPopupWidgetBase {
    /// java: public ControllerPopupWidget(T control, YACLScreen screen, Dimension<Integer> dim, ControllerWidget<?> entryWidget)
    pub fn new<T: Clone + 'static, C: Controller<T> + ?Sized + 'static>(
        control: &Arc<C>,
        screen: &Arc<YACLScreen>,
        dim: Arc<dyn Dimension<i32>>,
        entry_widget: Weak<dyn ControllerWidget>,
    ) -> ControllerPopupWidgetBase {
        ControllerPopupWidgetBase { cw: ControllerWidgetBase::new::<T, C>(control, screen, dim), entry_widget }
    }

    /// java: public ControllerWidget<?> entryWidget()
    pub fn entry_widget(&self) -> Arc<dyn ControllerWidget> {
        self.entry_widget.upgrade().expect("a YACL popup outlived its entry widget")
    }
}

/// java: the methods `ControllerPopupWidget` adds to `ControllerWidget`
pub trait ControllerPopupWidget: ControllerWidget {
    /// The class's fields.
    fn controller_popup_widget(&self) -> &ControllerPopupWidgetBase;

    /// java: public ControllerWidget<?> entryWidget()
    fn entry_widget(&self) -> Arc<dyn ControllerWidget> {
        self.controller_popup_widget().entry_widget()
    }

    /// java: public void extractBackground(GuiGraphicsExtractor guiGraphics, int mouseX, int mouseY, float partialTick)
    fn extract_background(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, partial_tick: f32) {
        let _ = (graphics, mouse_x, mouse_y, partial_tick);
    }

    /// java: public void close()
    fn close(&self) {}

    /// java: public Component popupTitle()
    fn popup_title(&self) -> MutableComponent {
        component::translatable("yacl.control.text.blank")
    }
}

/// java: @Override public boolean keyPressed(@NonNull KeyEvent event) - `entryWidget.keyPressed(event)`
pub fn key_pressed<W: ControllerPopupWidget + ?Sized>(this: &W, event: &KeyEvent) -> bool {
    ControllerWidget::key_pressed(&*this.entry_widget(), event)
}

/// java: @Override protected int getHoveredControlWidth()
pub fn get_hovered_control_width<W: ControllerPopupWidget + ?Sized>(_this: &W) -> i32 {
    0
}
