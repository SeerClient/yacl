//! Ported from dev.isxander.yacl3.gui.controllers.TickBoxController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidgetBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public record TickBoxController(Option<Boolean> option) implements Controller<Boolean>
#[derive(Clone, Debug)]
pub struct TickBoxController {
    /// java: Option<Boolean> option
    pub option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>>,
}

impl TickBoxController {
    /// java: the canonical record constructor
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>>) -> Self {
        Self { option }
    }

}

impl Controller<bool> for TickBoxController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<Boolean> option()
    fn option(&self) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>> {
        return self.option.clone();
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        return component::empty();
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        return (TickBoxControllerElement::new(self.clone(), screen.clone(), widget_dimension.clone()) as compat::Shared<dyn AbstractWidget>);
    }

}

/// java: public static class TickBoxControllerElement extends ControllerWidget<TickBoxController>
pub struct TickBoxControllerElement {
    /// java: extends ControllerWidget
    pub base: ControllerWidgetBase<TickBoxController>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for TickBoxControllerElement {
    type Target = ControllerWidgetBase<TickBoxController>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TickBoxControllerElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TickBoxControllerElement {
    /// java: public TickBoxControllerElement(TickBoxController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(mut control: TickBoxController, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: ControllerWidgetBase::<TickBoxController>::new(control.clone(), screen.clone(), dim.clone()),
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), event.x(), event.y()) }) || !(__this.lock().borrow().is_available()) {
            return false;
        }
        TickBoxControllerElement::toggle_setting(__this);
        return true;
    }

    /// java: public void toggleSetting()
    pub fn toggle_setting(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `.option(..)` on a T
        //   java: control.option().requestSet(!control.option().pendingValue());
        ({ let __f = compat::dispatch(__this, |__t| __t.play_down_sound()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) });
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    pub fn key_pressed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if !(compat::read(__this, |__t| __t.focused)) {
            return false;
        }
        if event.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_RETURN || event.key() == 32 || event.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_NUMPADENTER {
            TickBoxControllerElement::toggle_setting(__this);
            return true;
        }
        return false;
    }

}

impl ControllerWidget<crate::dev::isxander::yacl3::gui::controllers::tick_box_controller::TickBoxController> for TickBoxControllerElement {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<crate::dev::isxander::yacl3::gui::controllers::tick_box_controller::TickBoxController> {
        &self.base
    }
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<crate::dev::isxander::yacl3::gui::controllers::tick_box_controller::TickBoxController> {
        &mut self.base
    }

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut outline_size: i32 = 10;
        let mut outline_x1: i32 = (((({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x_limit()).wrapping_sub(self.get_x_padding())).wrapping_sub(outline_size);
        let mut outline_y1: i32 = ((({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).center_y()).wrapping_sub(outline_size.wrapping_div(2));
        let mut outline_x2: i32 = ((({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x_limit()).wrapping_sub(self.get_x_padding());
        let mut outline_y2: i32 = ((({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).center_y()).wrapping_add(outline_size.wrapping_div(2));
        let mut color: i32 = self.get_value_color();
        let mut shadow_color: i32 = ({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.multiply_color()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>), color, 0.25f32) });
        gui_graphics_extractor::outline(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), outline_x1.wrapping_add(1), outline_y1.wrapping_add(1), (outline_x2.wrapping_add(1)).wrapping_sub(outline_x1.wrapping_add(1)), (outline_y2.wrapping_add(1)).wrapping_sub(outline_y1.wrapping_add(1)), shadow_color);
        gui_graphics_extractor::outline(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), outline_x1, outline_y1, outline_x2.wrapping_sub(outline_x1), outline_y2.wrapping_sub(outline_y1), color);
        // TODO(fabric2seer): the condition: `.option(..)` on a T
        //   java: control.option(..).pendingValue(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), outline_x1.wrapping_add(3), outline_y1.wrapping_add(3), outline_x2.wrapping_sub(1), outline_y2.wrapping_sub(1), shadow_color);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), outline_x1.wrapping_add(2), outline_y1.wrapping_add(2), outline_x2.wrapping_sub(2), outline_y2.wrapping_sub(2), color);
        }
        if self.hovered {
            // TODO(fabric2seer): static `CursorTypes.POINTING_HAND` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): static `CursorTypes.NOT_ALLOWED` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&(if self.is_available() { CursorTypes::POINTING_HAND.clone() } else { CursorTypes::NOT_ALLOWED.clone() }));
        }
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        return 10;
    }

    /// java: @Override protected int getUnhoveredControlWidth()
    fn get_unhovered_control_width(&self) -> i32 {
        return 10;
    }

}

impl AbstractWidget for TickBoxControllerElement {
    fn abstract_widget_base(&self) -> &AbstractWidgetBase {
        &self.base.base
    }
    fn abstract_widget_base_mut(&mut self) -> &mut AbstractWidgetBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean canReset() (dispatch: the body for this class)
    fn can_reset(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> bool {
        <dyn ControllerWidget>::can_reset_from::<Self>
    }

    /// java: @Override public NarrationPriority narrationPriority() (dispatch: the body for this class)
    fn narration_priority(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> crate::unresolved::NarrationPriority {
        <dyn ControllerWidget>::narration_priority_from::<Self>
    }

    /// java: @Override public void unfocus() (dispatch: the body for this class)
    fn unfocus(&self) -> fn(&compat::Shared<dyn AbstractWidget>) {
        <dyn ControllerWidget>::unfocus_from::<Self>
    }

    /// java: @Override public boolean matchesSearch(String query) (dispatch: the body for this class)
    fn matches_search(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &str) -> bool {
        <dyn ControllerWidget>::matches_search_from::<Self>
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_narration(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &mut dyn NarrationElementOutput) {
        <dyn ControllerWidget>::update_narration_from::<Self>
    }

}
