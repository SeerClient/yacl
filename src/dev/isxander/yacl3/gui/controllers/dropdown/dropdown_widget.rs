//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.DropdownWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::api::utils::mutable_dimension::MutableDimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::ControllerPopupWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::ControllerPopupWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::AbstractDropdownControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElement;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public static final int MAX_SHOWN_NUMBER_OF_ITEMS = 7;
pub const MAX_SHOWN_NUMBER_OF_ITEMS: i32 = 7;
/// java: public static final int DROPDOWN_PADDING = 2;
pub const DROPDOWN_PADDING: i32 = 2;

/// java: public class DropdownWidget<T> extends ControllerPopupWidget<AbstractDropdownController<T>>
pub struct DropdownWidget<T> {
    /// java: extends ControllerPopupWidget
    pub base: ControllerPopupWidgetBase<compat::Shared<dyn AbstractDropdownController<T>>>,
    /// java: private final AbstractDropdownControllerElement<T, ?> dropdownElement;
    pub dropdown_element: compat::Shared<dyn AbstractDropdownControllerElement<T, compat::Object>>,
    /// java: protected Dimension<Integer> dropdownDim;
    pub dropdown_dim: Option<compat::Shared<dyn Dimension<i32>>>,
    /// java: protected int firstVisibleIndex = 0;
    pub first_visible_index: i32,
    /// java: protected int selectedIndex = 0;
    pub selected_index: i32,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl<T: 'static> std::ops::Deref for DropdownWidget<T> {
    type Target = ControllerPopupWidgetBase<compat::Shared<dyn AbstractDropdownController<T>>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for DropdownWidget<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> DropdownWidget<T> {
    /// java: public DropdownWidget(AbstractDropdownController<T> control, YACLScreen screen, Dimension<Integer> dim, AbstractDropdownControllerElement<T, ?> dropdownElement)
    pub fn new(mut control: compat::Shared<dyn AbstractDropdownController<T>>, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>, mut dropdown_element: compat::Shared<dyn AbstractDropdownControllerElement<T, compat::Object>>) -> compat::Shared<Self> {
        let __init_dropdown_element = dropdown_element.clone();
        let __this = compat::shared(Self {
            base: ControllerPopupWidgetBase::<compat::Shared<dyn AbstractDropdownController<T>>>::new(control.clone(), screen.clone(), dim.clone(), (dropdown_element.clone() as compat::Shared<dyn ControllerWidget<compat::Object>>)),
            self_ref: std::sync::Weak::new(),
            dropdown_element: __init_dropdown_element,
            dropdown_dim: None,
            first_visible_index: 0,
            selected_index: 0,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        ({ let __f = compat::dispatch(&__this, |__t| __t.set_dimension()); __f(&compat::view(&__this, |a| a as compat::Shared<dyn AbstractWidget>), dim.clone()) });
        __this
    }

    /// java: public int entryHeight()
    pub fn entry_height(__this: &compat::Shared<Self>) -> i32 {
        return (({ let __r = &compat::read(__this, |__t| __t.dropdown_element.clone()); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).height();
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        if { let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), event.x(), event.y()) } {
            ({ let __r = &compat::read(__this, |__t| __t.dropdown_element.clone()); let __f = compat::dispatch(__r, |__t| __t.unfocus()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>)) });
            return true;
        } else if { let __r = &compat::read(__this, |__t| __t.dropdown_element.clone()); let __f = compat::dispatch(__r, |__t| __t.is_mouse_over()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>), event.x(), event.y()) } {
            return ({ let __r = &compat::read(__this, |__t| __t.dropdown_element.clone()); let __f = compat::dispatch(__r, |__t| __t.mouse_clicked()); __f(&compat::view(__r, |a| a as compat::Shared<dyn StringControllerElement>), event, double_click) });
        } else {
            __this.lock().borrow_mut().close();
            return false;
        }
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal: f64, mut vertical: f64) -> bool {
        if { let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), mouse_x, mouse_y) } {
            if vertical < (0 as f64) {
                DropdownWidget::scroll_down(__this);
            } else {
                DropdownWidget::scroll_up(__this);
            }
            return true;
        }
        // TODO(fabric2seer): super.mouseScrolled(..)
        //   java: return super.mouseScrolled(mouseX, mouseY, horizontal, vertical);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public void mouseMoved(double mouseX, double mouseY)
    pub fn mouse_moved(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) {
        if { let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), mouse_x, mouse_y) } {
            let mut index: i32 = ((((mouse_y - __this.lock().borrow().dropdown_dim.as_ref().unwrap().y()) / (DropdownWidget::entry_height(__this) as f64))) as i32);
            DropdownWidget::select_visible_item(__this, index);
        }
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent event)
    pub fn char_typed(__this: &compat::Shared<Self>, event: &CharacterEvent) -> bool {
        return ({ let __r = &compat::read(__this, |__t| __t.dropdown_element.clone()); let __f = compat::dispatch(__r, |__t| __t.char_typed()); __f(&compat::view(__r, |a| a as compat::Shared<dyn StringControllerElement>), event) });
    }

    /// java: public int dropdownLength()
    pub fn dropdown_length(__this: &compat::Shared<Self>) -> i32 {
        return (compat::read(__this, |__t| __t.dropdown_element.clone()).lock().borrow().abstract_dropdown_controller_element_base().matching_values.as_ref().unwrap().len() as i32);
    }

    /// java: public int numberOfVisibleItems()
    pub fn number_of_visible_items(__this: &compat::Shared<Self>) -> i32 {
        return MAX_SHOWN_NUMBER_OF_ITEMS.min(DropdownWidget::dropdown_length(__this));
    }

    /// java: public int lastVisibleIndex()
    pub fn last_visible_index(__this: &compat::Shared<Self>) -> i32 {
        return ((compat::read(__this, |__t| __t.first_visible_index)).wrapping_add(MAX_SHOWN_NUMBER_OF_ITEMS)).min(DropdownWidget::dropdown_length(__this));
    }

    /// java: public int selectedIndex()
    pub fn selected_index(__this: &compat::Shared<Self>) -> i32 {
        return compat::read(__this, |__t| __t.selected_index);
    }

    /// java: public void resetSelectedIndex()
    pub fn reset_selected_index(__this: &compat::Shared<Self>) {
        { let __v = 0; compat::write(__this, |__t| __t.selected_index = __v); }
    }

    /// java: public int selectedVisibleIndex()
    pub fn selected_visible_index(__this: &compat::Shared<Self>) -> i32 {
        return (compat::read(__this, |__t| __t.selected_index)).wrapping_sub(compat::read(__this, |__t| __t.first_visible_index));
    }

    /// java: public void selectVisibleItem(int visibleIndex)
    pub fn select_visible_item(__this: &compat::Shared<Self>, mut visible_index: i32) {
        { let __v = ((compat::read(__this, |__t| __t.first_visible_index)).wrapping_add(visible_index)).min((DropdownWidget::dropdown_length(__this)).wrapping_sub(1)); compat::write(__this, |__t| __t.selected_index = __v); }
    }

    /// java: public void selectNextEntry()
    pub fn select_next_entry(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.selected_index) == (DropdownWidget::dropdown_length(__this)).wrapping_sub(1) {
            { let __v = 0; compat::write(__this, |__t| __t.selected_index = __v); }
        } else {
            { let __v = compat::read(__this, |__t| __t.selected_index) + 1; compat::write(__this, |__t| __t.selected_index = __v); }
        }
        if (compat::read(__this, |__t| __t.selected_index)).wrapping_sub(compat::read(__this, |__t| __t.first_visible_index)) >= MAX_SHOWN_NUMBER_OF_ITEMS.wrapping_div(2) {
            DropdownWidget::center_on_selected_item(__this);
        }
    }

    /// java: public void selectPreviousEntry()
    pub fn select_previous_entry(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.selected_index) == 0 {
            { let __v = (DropdownWidget::dropdown_length(__this)).wrapping_sub(1); compat::write(__this, |__t| __t.selected_index = __v); }
        } else {
            { let __v = compat::read(__this, |__t| __t.selected_index) - 1; compat::write(__this, |__t| __t.selected_index = __v); }
        }
        if (compat::read(__this, |__t| __t.selected_index)).wrapping_sub(compat::read(__this, |__t| __t.first_visible_index)) <= MAX_SHOWN_NUMBER_OF_ITEMS.wrapping_div(2) {
            DropdownWidget::center_on_selected_item(__this);
        }
    }

    /// java: private void centerOnSelectedItem()
    pub fn center_on_selected_item(__this: &compat::Shared<Self>) {
        let mut begin: i32 = 0i32.max((compat::read(__this, |__t| __t.selected_index)).wrapping_sub(MAX_SHOWN_NUMBER_OF_ITEMS.wrapping_div(2)));
        let mut end: i32 = begin.wrapping_add(MAX_SHOWN_NUMBER_OF_ITEMS);
        if end >= DropdownWidget::dropdown_length(__this) {
            end = DropdownWidget::dropdown_length(__this);
            begin = 0i32.max(end.wrapping_sub(MAX_SHOWN_NUMBER_OF_ITEMS));
        }
        { let __v = begin; compat::write(__this, |__t| __t.first_visible_index = __v); }
    }

    /// java: public void scrollDown()
    pub fn scroll_down(__this: &compat::Shared<Self>) {
        if ((compat::read(__this, |__t| __t.first_visible_index)).wrapping_add(1)).wrapping_add(MAX_SHOWN_NUMBER_OF_ITEMS) <= DropdownWidget::dropdown_length(__this) {
            { let __v = compat::read(__this, |__t| __t.first_visible_index) + 1; compat::write(__this, |__t| __t.first_visible_index = __v); }
        }
        if compat::read(__this, |__t| __t.selected_index) < compat::read(__this, |__t| __t.first_visible_index) {
            { let __v = compat::read(__this, |__t| __t.first_visible_index); compat::write(__this, |__t| __t.selected_index = __v); }
        }
    }

    /// java: public void scrollUp()
    pub fn scroll_up(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.first_visible_index) > 0 {
            { let __v = compat::read(__this, |__t| __t.first_visible_index) - 1; compat::write(__this, |__t| __t.first_visible_index = __v); }
        }
        if compat::read(__this, |__t| __t.selected_index) > ((compat::read(__this, |__t| __t.first_visible_index)).wrapping_add(MAX_SHOWN_NUMBER_OF_ITEMS)).wrapping_sub(1) {
            { let __v = ((compat::read(__this, |__t| __t.first_visible_index)).wrapping_add(MAX_SHOWN_NUMBER_OF_ITEMS)).wrapping_sub(1); compat::write(__this, |__t| __t.selected_index = __v); }
        }
    }

}

impl<T: Clone + 'static> ControllerPopupWidget<compat::Shared<dyn crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController<T>>> for DropdownWidget<T> {
    fn controller_popup_widget_base(&self) -> &ControllerPopupWidgetBase<compat::Shared<dyn crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController<T>>> {
        &self.base
    }
    fn controller_popup_widget_base_mut(&mut self) -> &mut ControllerPopupWidgetBase<compat::Shared<dyn crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController<T>>> {
        &mut self.base
    }

    /// java: @Override public void close()
    fn close(&mut self) {
        self.dropdown_element.lock().borrow_mut().remove_dropdown_widget();
    }

    /// java: @Override public Component popupTitle()
    fn popup_title(&self) -> MutableComponent {
        return component::translatable("yacl.control.dropdown.dropdown_widget_title");
    }

}

impl<T: Clone + 'static> ControllerWidget<compat::Object> for DropdownWidget<T> {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<compat::Object> {
        &self.base.base
    }
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<compat::Object> {
        &mut self.base.base
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if DropdownWidget::dropdown_length(&self.self_ref.upgrade().unwrap()) == 0 {
            return;
        }
        let mut x: i32 = self.dropdown_dim.as_ref().unwrap().lock().borrow().x();
        let mut y1: i32 = self.dropdown_dim.as_ref().unwrap().lock().borrow().y();
        let mut texture_width: i32 = self.dropdown_dim.as_ref().unwrap().lock().borrow().width();
        let mut texture_height: i32 = self.dropdown_dim.as_ref().unwrap().lock().borrow().height();
        // TODO(fabric2seer): static `Screen.MENU_BACKGROUND` has no Rust counterpart in the port's Screen file
        // rust: gui_graphics_extractor::blit_identifier_with_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &mcrust::net::minecraft::client::gui::screens::screen::Screen::MENU_BACKGROUND, x, y1, 0.0f32, 0.0f32, texture_width, texture_height, 32, 32, 0xFF3F3F3Fu32 as i32);
        // TODO(fabric2seer): `GuiGraphicsExtractor.outline` has no Rust method in the port (looked for `outline` in GuiGraphicsExtractor and its supertypes)
        // rust: graphics.outline(&self.dropdown_dim.as_ref().unwrap().lock().borrow().x(), &self.dropdown_dim.as_ref().unwrap().lock().borrow().y(), &self.dropdown_dim.as_ref().unwrap().lock().borrow().width(), &self.dropdown_dim.as_ref().unwrap().lock().borrow().height(), -1);
        let mut y: i32 = ((self.dropdown_dim.as_ref().unwrap().lock().borrow().y()).wrapping_add(2)).wrapping_add((DropdownWidget::entry_height(&self.self_ref.upgrade().unwrap())).wrapping_mul(DropdownWidget::selected_visible_index(&self.self_ref.upgrade().unwrap())));
        graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), self.dropdown_dim.as_ref().unwrap().lock().borrow().x(), y, self.dropdown_dim.as_ref().unwrap().lock().borrow().x_limit(), y.wrapping_add(DropdownWidget::entry_height(&self.self_ref.upgrade().unwrap())), 0x7F000000);
        // TODO(fabric2seer): `GuiGraphicsExtractor.outline` has no Rust method in the port (looked for `outline` in GuiGraphicsExtractor and its supertypes)
        // rust: graphics.outline(&self.dropdown_dim.as_ref().unwrap().lock().borrow().x(), y, &self.dropdown_dim.as_ref().unwrap().lock().borrow().width(), DropdownWidget::entry_height(&self.self_ref.upgrade().unwrap()), -1);
        let mut entry_dimension: compat::Shared<dyn MutableDimension<i32>> = dimension::of_int((self.dropdown_dim.as_ref().unwrap().lock().borrow().x()).wrapping_sub(self.dropdown_element.lock().borrow().get_decoration_padding()), (self.dropdown_dim.as_ref().unwrap().lock().borrow().y()).wrapping_add(2), self.dropdown_dim.as_ref().unwrap().lock().borrow().width(), DropdownWidget::entry_height(&self.self_ref.upgrade().unwrap()));
        for i in self.first_visible_index..DropdownWidget::last_visible_index(&self.self_ref.upgrade().unwrap()) {
            { let (__a0, __a1, __a2,) = (graphics, &(entry_dimension.clone() as compat::Shared<dyn Dimension<compat::Object>>), i,); self.dropdown_element.lock().borrow().extract_dropdown_entry(__a0, __a1, __a2) };
            { let (__a0, __a1,) = (&0, &DropdownWidget::entry_height(&self.self_ref.upgrade().unwrap()),); entry_dimension.lock().borrow_mut().r#move(__a0, __a1) };
        }
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        return 0;
    }

}

impl<T: Clone + 'static> AbstractWidget for DropdownWidget<T> {
    fn abstract_widget_base(&self) -> &AbstractWidgetBase {
        &self.base.base.base
    }
    fn abstract_widget_base_mut(&mut self) -> &mut AbstractWidgetBase {
        &mut self.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim) (dispatch: the body for this class)
    fn set_dimension(&self) -> fn(&compat::Shared<dyn AbstractWidget>, compat::Shared<dyn Dimension<i32>>) {
        DropdownWidget::set_dimension_dyn
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY) (dispatch: the body for this class)
    fn is_mouse_over(&self) -> fn(&compat::Shared<dyn AbstractWidget>, f64, f64) -> bool {
        DropdownWidget::is_mouse_over_dyn
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

impl<T: Clone + 'static> DropdownWidget<T> {
    /// java: @Override public void setDimension(Dimension<Integer> dim)
    pub fn set_dimension_impl(__this: &compat::Shared<Self>, mut dim: compat::Shared<dyn Dimension<i32>>) {
        <dyn AbstractWidget>::set_dimension_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), dim.clone());
        let mut dropdown_height: i32 = (dim.lock().borrow().height()).wrapping_mul(DropdownWidget::number_of_visible_items(__this));
        let mut dropdown_y: i32 = ((dim.lock().borrow().y()).wrapping_sub(dropdown_height)).wrapping_sub(DROPDOWN_PADDING);
        if dropdown_y < compat::read(__this, |__t| __t.screen.clone()).lock().borrow().tab_area.as_ref().unwrap().top() {
            dropdown_y = (dim.lock().borrow().y_limit()).wrapping_add(DROPDOWN_PADDING);
        }
        { let __v = Some((dimension::of_int(dim.lock().borrow().x(), dropdown_y, dim.lock().borrow().width(), dropdown_height) as compat::Shared<dyn Dimension<compat::Object>>)); compat::write(__this, |__t| __t.dropdown_dim = __v); }
    }

    /// DropdownWidget.setDimension as the dispatch of AbstractWidget.setDimension
    pub fn set_dimension_dyn(__this: &compat::Shared<dyn AbstractWidget>, __p0: compat::Shared<dyn Dimension<i32>>) {
        Self::set_dimension_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    pub fn is_mouse_over_impl(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return { let (__a0, __a1,) = (&((mouse_x) as i32), &((mouse_y) as i32),); __this.lock().borrow().dropdown_dim.as_ref().unwrap().is_point_inside(__a0, __a1) };
    }

    /// DropdownWidget.isMouseOver as the dispatch of AbstractWidget.isMouseOver
    pub fn is_mouse_over_dyn(__this: &compat::Shared<dyn AbstractWidget>, __p0: f64, __p1: f64) -> bool {
        Self::is_mouse_over_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
