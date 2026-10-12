//! Ported from dev.isxander.yacl3.gui.controllers.cycling.CyclingControllerElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::cycling::i_cycling_controller::ICyclingController;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::input_with_modifiers::InputWithModifiers as _;

/// java: public class CyclingControllerElement extends ControllerWidget<ICyclingController<?>>
pub struct CyclingControllerElement {
    /// java: extends ControllerWidget
    pub base: ControllerWidgetBase<compat::Shared<dyn ICyclingController<compat::Object>>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for CyclingControllerElement {
    type Target = ControllerWidgetBase<compat::Shared<dyn ICyclingController<compat::Object>>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CyclingControllerElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CyclingControllerElement {
    /// java: public CyclingControllerElement(ICyclingController<?> control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(mut control: compat::Shared<dyn ICyclingController<compat::Object>>, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: ControllerWidgetBase::<compat::Shared<dyn ICyclingController<compat::Object>>>::new(control.clone(), screen.clone(), dim.clone()),
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: public void cycleValue(int increment)
    pub fn cycle_value(__this: &compat::Shared<Self>, mut increment: i32) {
        // TODO(fabric2seer): `.getPendingValue(..)` on a T
        //   java: int targetIdx = control.getPendingValue() + increment;
        let mut target_idx: i32 = todo!("fabric2seer: the initialiser of `targetIdx` (TODO above)");
        // TODO(fabric2seer): the condition: `.getCycleLength(..)` on a T
        //   java: targetIdx >= control.getCycleLength()
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.getCycleLength(..)` on a T
            //   java: targetIdx -= control.getCycleLength();
        } else if target_idx < 0 {
            // TODO(fabric2seer): `.getCycleLength(..)` on a T
            //   java: targetIdx += control.getCycleLength();
        }
        // TODO(fabric2seer): `.setPendingValue(..)` on a T
        //   java: control.setPendingValue(targetIdx);
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), event.x(), event.y()) }) || event.button() != 0 && event.button() != 1 || !(__this.lock().borrow().is_available()) {
            return false;
        }
        ({ let __f = compat::dispatch(__this, |__t| __t.play_down_sound()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) });
        CyclingControllerElement::cycle_value(__this, (if event.button() == 1 || event.has_shift_down() || event.has_control_down() { -1 } else { 1 }));
        return true;
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    pub fn key_pressed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if !(compat::read(__this, |__t| __t.focused)) {
            return false;
        }
        match event.key() {
            263 => {
                CyclingControllerElement::cycle_value(__this, -1);
            }
            262 => {
                CyclingControllerElement::cycle_value(__this, 1);
            }
            mcrust::com::mojang::blaze3d::platform::input_constants::KEY_RETURN | 32 | mcrust::com::mojang::blaze3d::platform::input_constants::KEY_NUMPADENTER => {
                CyclingControllerElement::cycle_value(__this, (if event.has_control_down() || event.has_shift_down() { -1 } else { 1 }));
            }
            _ => {
                return false;
            }
        }
        return true;
    }

}

impl ControllerWidget<compat::Shared<dyn crate::dev::isxander::yacl3::gui::controllers::cycling::i_cycling_controller::ICyclingController<compat::Object>>> for CyclingControllerElement {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<compat::Shared<dyn crate::dev::isxander::yacl3::gui::controllers::cycling::i_cycling_controller::ICyclingController<compat::Object>>> {
        &self.base
    }
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<compat::Shared<dyn crate::dev::isxander::yacl3::gui::controllers::cycling::i_cycling_controller::ICyclingController<compat::Object>>> {
        &mut self.base
    }

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        self.base.extract_value_text(graphics, mouse_x, mouse_y, a);
        if self.hovered {
            // TODO(fabric2seer): static `CursorTypes.POINTING_HAND` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): static `CursorTypes.NOT_ALLOWED` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: graphics.request_cursor(&(if self.is_available() { CursorTypes::POINTING_HAND.clone() } else { CursorTypes::NOT_ALLOWED.clone() }));
        }
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        return self.get_unhovered_control_width();
    }

}

impl AbstractWidget for CyclingControllerElement {
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
