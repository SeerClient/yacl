//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.DropdownStringControllerElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::AbstractDropdownControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::AbstractDropdownControllerElementBase;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::dropdown_string_controller::DropdownStringController;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElementBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;

/// java: public class DropdownStringControllerElement extends AbstractDropdownControllerElement<String, String>
pub struct DropdownStringControllerElement {
    /// java: extends AbstractDropdownControllerElement
    pub base: AbstractDropdownControllerElementBase<String, String>,
    /// java: private final DropdownStringController controller;
    pub controller: Option<compat::Shared<DropdownStringController>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for DropdownStringControllerElement {
    type Target = AbstractDropdownControllerElementBase<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DropdownStringControllerElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DropdownStringControllerElement {
    /// java: public DropdownStringControllerElement(DropdownStringController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(mut control: compat::Shared<DropdownStringController>, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractDropdownControllerElementBase::<String, String>::new((control.clone() as compat::Shared<dyn AbstractDropdownController<compat::Object>>), screen.clone(), dim.clone()),
            self_ref: std::sync::Weak::new(),
            controller: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(control.clone()); compat::write(&__this, |__t| __t.controller = __v); }
        __this
    }

}

impl AbstractDropdownControllerElement<String, String> for DropdownStringControllerElement {
    fn abstract_dropdown_controller_element_base(&self) -> &AbstractDropdownControllerElementBase<String, String> {
        &self.base
    }
    fn abstract_dropdown_controller_element_base_mut(&mut self) -> &mut AbstractDropdownControllerElementBase<String, String> {
        &mut self.base
    }

    /// java: @Override public List<String> computeMatchingValues()
    fn compute_matching_values(&mut self) -> Vec<String> {
        return compat::sorted_by({ let (__a0,) = (self.input_field.clone(),); self.controller.as_ref().unwrap().lock().borrow().get_allowed_values_1(__a0) }.into_iter().filter(|__a0| self.matching_value(__a0)), |s1, s2| {
            if s1.starts_with(&self.input_field) && !(s2.starts_with(&self.input_field)) {
                return -1;
            }
            if !(s1.starts_with(&self.input_field)) && s2.starts_with(&self.input_field) {
                return 1;
            }
            return compat::compare_to(&s1, s2);
        }).collect::<Vec<_>>();
    }

    /// java: public String getString(String object)
    fn get_string(&self, object: &String) -> String {
        return object.to_string();
    }

}

impl StringControllerElement for DropdownStringControllerElement {
    fn string_controller_element_base(&self) -> &StringControllerElementBase {
        &self.base.base
    }
    fn string_controller_element_base_mut(&mut self) -> &mut StringControllerElementBase {
        &mut self.base.base
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn StringControllerElement>, &MouseButtonEvent, bool) -> bool {
        <dyn AbstractDropdownControllerElement>::mouse_clicked_from::<Self>
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event) (dispatch: the body for this class)
    fn key_pressed(&self) -> fn(&compat::Shared<dyn StringControllerElement>, &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        <dyn AbstractDropdownControllerElement>::key_pressed_from::<Self>
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent event) (dispatch: the body for this class)
    fn char_typed(&self) -> fn(&compat::Shared<dyn StringControllerElement>, &CharacterEvent) -> bool {
        <dyn AbstractDropdownControllerElement>::char_typed_from::<Self>
    }

    /// java: @Override public boolean modifyInput(Consumer<StringBuilder> builder) (dispatch: the body for this class)
    fn modify_input(&self) -> fn(&compat::Shared<dyn StringControllerElement>, Arc<dyn Fn(String)>) -> bool {
        <dyn AbstractDropdownControllerElement>::modify_input_from::<Self>
    }

}

impl ControllerWidget<compat::Object> for DropdownStringControllerElement {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<compat::Object> {
        &self.base.base.base
    }
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<compat::Object> {
        &mut self.base.base.base
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        // TODO(fabric2seer): `getUnshiftedLength(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: return Math.min(textRenderer.width(getValueText()), getUnshiftedLength());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl AbstractWidget for DropdownStringControllerElement {
    fn abstract_widget_base(&self) -> &AbstractWidgetBase {
        &self.base.base.base.base
    }
    fn abstract_widget_base_mut(&mut self) -> &mut AbstractWidgetBase {
        &mut self.base.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean canReset() (dispatch: the body for this class)
    fn can_reset(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> bool {
        <dyn ControllerWidget>::can_reset_from::<Self>
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim) (dispatch: the body for this class)
    fn set_dimension(&self) -> fn(&compat::Shared<dyn AbstractWidget>, compat::Shared<dyn Dimension<i32>>) {
        <dyn AbstractDropdownControllerElement>::set_dimension_from::<Self>
    }

    /// java: @Override public NarrationPriority narrationPriority() (dispatch: the body for this class)
    fn narration_priority(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> crate::unresolved::NarrationPriority {
        <dyn ControllerWidget>::narration_priority_from::<Self>
    }

    /// java: @Override public void unfocus() (dispatch: the body for this class)
    fn unfocus(&self) -> fn(&compat::Shared<dyn AbstractWidget>) {
        <dyn AbstractDropdownControllerElement>::unfocus_from::<Self>
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
