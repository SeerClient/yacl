//! Ported from dev.isxander.yacl3.gui.controllers.ControllerPopupWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidgetBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public abstract class ControllerPopupWidget<T extends Controller<?>> extends ControllerWidget<Controller<?>> implements GuiEventListener
pub struct ControllerPopupWidgetBase<T> {
    /// java: extends ControllerWidget
    pub base: ControllerWidgetBase<compat::Shared<dyn Controller<compat::Object>>>,
    /// java: public final ControllerWidget<?> entryWidget;
    pub entry_widget: compat::Shared<dyn ControllerWidget<compat::Object>>,
}

impl<T: 'static> std::ops::Deref for ControllerPopupWidgetBase<T> {
    type Target = ControllerWidgetBase<compat::Shared<dyn Controller<compat::Object>>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for ControllerPopupWidgetBase<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> ControllerPopupWidgetBase<T> {
    /// java: public ControllerPopupWidget(T control, YACLScreen screen, Dimension<Integer> dim, ControllerWidget<?> entryWidget)
    pub fn new(mut control: T, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>, mut entry_widget: compat::Shared<dyn ControllerWidget<compat::Object>>) -> Self {
        let __init_entry_widget = entry_widget.clone();
        let mut this = Self {
            base: ControllerWidgetBase::<compat::Shared<dyn Controller<compat::Object>>>::new(control.clone(), screen.clone(), dim.clone()),
            entry_widget: __init_entry_widget,
        };
        this
    }

}

/// java: the virtual methods of ControllerPopupWidget (implemented by ControllerPopupWidgetBase and every subclass)
pub trait ControllerPopupWidget<T: Clone + 'static>: ControllerWidget {
    fn controller_popup_widget_base(&self) -> &ControllerPopupWidgetBase<T>;
    fn controller_popup_widget_base_mut(&mut self) -> &mut ControllerPopupWidgetBase<T>;

    /// java: public ControllerWidget<?> entryWidget()
    fn entry_widget(&self) -> compat::Shared<dyn ControllerWidget<compat::Object>> {
        return self.controller_popup_widget_base().entry_widget.clone();
    }

    /// java: public void extractBackground(GuiGraphicsExtractor guiGraphics, int mouseX, int mouseY, float partialTick)
    fn extract_background(&mut self, gui_graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut partial_tick: f32) {
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    fn key_pressed(&self, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // TODO(fabric2seer): `keyPressed` is not a method of ControllerWidget
        //   java: return entryWidget.keyPressed(event);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public void close()
    fn close(&mut self) {
    }

    /// java: public Component popupTitle()
    fn popup_title(&self) -> MutableComponent {
        return component::translatable("yacl.control.text.blank");
    }

}

// ControllerPopupWidget.getHoveredControlWidth implements ControllerWidget.getHoveredControlWidth: it is emitted in each concrete subclass's `impl ControllerWidget` (Rust has no inherited impls)
