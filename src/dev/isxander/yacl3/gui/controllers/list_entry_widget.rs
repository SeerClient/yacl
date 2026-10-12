//! Ported from dev.isxander.yacl3.gui.controllers.ListEntryWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::list_option::ListOption;
use crate::dev::isxander::yacl3::api::list_option_entry::ListOptionEntry;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::tooltip_button_widget::TooltipButtonWidget;
use crate::dev::isxander::yacl3::gui::tooltip_button_widget::TooltipButtonWidgetBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use crate::dev::isxander::yacl3::api::option::Option as _;
use crate::dev::isxander::yacl3::api::utils::mutable_dimension::MutableDimension as _;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class ListEntryWidget extends AbstractWidget implements ContainerEventHandler
pub struct ListEntryWidget {
    /// java: extends AbstractWidget
    pub base: AbstractWidgetBase,
    /// java: private final TooltipButtonWidget removeButton, moveUpButton, moveDownButton;
    pub remove_button: Option<compat::Shared<dyn TooltipButtonWidget>>,
    /// java: private final TooltipButtonWidget removeButton, moveUpButton, moveDownButton;
    pub move_up_button: Option<compat::Shared<dyn TooltipButtonWidget>>,
    /// java: private final TooltipButtonWidget removeButton, moveUpButton, moveDownButton;
    pub move_down_button: Option<compat::Shared<dyn TooltipButtonWidget>>,
    /// java: private final AbstractWidget entryWidget;
    pub entry_widget: compat::Shared<dyn AbstractWidget>,
    /// java: private final ListOption<?> listOption;
    pub list_option: compat::Shared<dyn ListOption<compat::Object>>,
    /// java: private final ListOptionEntry<?> listOptionEntry;
    pub list_option_entry: compat::Shared<dyn ListOptionEntry<compat::Object>>,
    /// java: private final String optionNameString;
    pub option_name_string: String,
    /// java: private GuiEventListener focused;
    pub focused: Option<Box<dyn GuiEventListener>>,
    /// java: private boolean dragging;
    pub dragging: bool,
}

impl std::ops::Deref for ListEntryWidget {
    type Target = AbstractWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ListEntryWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ListEntryWidget {
    /// java: public ListEntryWidget(YACLScreen screen, ListOptionEntry<?> listOptionEntry, AbstractWidget entryWidget)
    pub fn new(mut screen: compat::Shared<YACLScreen>, mut list_option_entry: compat::Shared<dyn ListOptionEntry<compat::Object>>, mut entry_widget: compat::Shared<dyn AbstractWidget>) -> compat::Shared<Self> {
        let __init_list_option_entry = list_option_entry.clone();
        let __init_list_option = list_option_entry.lock().borrow().parent_group();
        let __init_option_name_string = list_option_entry.lock().borrow().name().get_string().to_lowercase();
        let __init_entry_widget = entry_widget.clone();
        let mut dim: compat::Shared<dyn Dimension<i32>> = ({ let __f = compat::dispatch(&entry_widget, |__t| __t.get_dimension()); __f(&entry_widget) });
        let (_, __this) = AbstractWidgetBase::new_in({ let (__a0,) = (&((({ let __f = compat::dispatch(&entry_widget, |__t| __t.get_dimension()); __f(&entry_widget) }).lock().borrow().height()).min(20)).wrapping_sub(if list_option_entry.lock().borrow().parent_group().lock().borrow().index_of(&list_option_entry) == ((({ let __r = &list_option_entry.lock().borrow().parent_group(); let __f = compat::dispatch(__r, |__t| __t.options()); __f(&compat::view(__r, |a| a as compat::Shared<dyn OptionGroup>)) })).len() as i32).wrapping_sub(1) { 0 } else { 2 }),); ({ let __f = compat::dispatch(&entry_widget, |__t| __t.get_dimension()); __f(&entry_widget) }).lock().borrow().with_height(__a0) }, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                remove_button: None,
                move_up_button: None,
                move_down_button: None,
                entry_widget: __init_entry_widget,
                list_option: __init_list_option,
                list_option_entry: __init_list_option_entry,
                option_name_string: __init_option_name_string,
                focused: None,
                dragging: false,
            });
            ((__h.clone() as compat::Shared<dyn AbstractWidget>), __h)
        });
        ({ let __f = compat::dispatch(&entry_widget, |__t| __t.set_dimension()); __f(&entry_widget, ({ let (__a0, __a1,) = (&-20i32.wrapping_mul(3), &0,); ({ let (__a0, __a1,) = (&20i32.wrapping_mul(2), &0,); dim.lock().borrow().clone().lock().borrow_mut().r#move(__a0, __a1) }.lock().borrow_mut()).expand(__a0, __a1) } as compat::Shared<dyn Dimension<compat::Object>>)) });
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = Some((TooltipButtonWidgetBase::new(screen.clone(), (dim.lock().borrow().x_limit()).wrapping_sub(20), dim.lock().borrow().y(), 20, 20, component::literal("❌"), component::translatable("yacl.list.remove"), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
            __this_1.lock().borrow_mut().list_option.remove_entry(&list_option_entry);
            ListEntryWidget::update_button_states(&__this_1);
        } }))) as compat::Shared<dyn TooltipButtonWidget>)); compat::write(&__this, |__t| __t.remove_button = __v); }
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = Some((TooltipButtonWidgetBase::new(screen.clone(), dim.lock().borrow().x(), dim.lock().borrow().y(), 20, 20, component::literal("↑"), component::translatable("yacl.list.move_up"), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; {
            let mut index: i32 = (__this_2.lock().borrow().list_option.index_of(&list_option_entry)).wrapping_sub(1);
            if index >= 0 {
                __this_2.lock().borrow_mut().list_option.remove_entry(&list_option_entry);
                __this_2.lock().borrow_mut().list_option.insert_entry(index, &list_option_entry);
                ListEntryWidget::update_button_states(&__this_2);
            }
        } }))) as compat::Shared<dyn TooltipButtonWidget>)); compat::write(&__this, |__t| __t.move_up_button = __v); }
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let __v = Some((TooltipButtonWidgetBase::new(screen.clone(), (dim.lock().borrow().x()).wrapping_add(20), dim.lock().borrow().y(), 20, 20, component::literal("↓"), component::translatable("yacl.list.move_down"), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; {
            let mut index: i32 = (__this_3.lock().borrow().list_option.index_of(&list_option_entry)).wrapping_add(1);
            if index < ((({ let __r = &compat::read(&__this_3, |__t| __t.list_option.clone()); let __f = compat::dispatch(__r, |__t| __t.options()); __f(&compat::view(__r, |a| a as compat::Shared<dyn OptionGroup>)) })).len() as i32) {
                __this_3.lock().borrow_mut().list_option.remove_entry(&list_option_entry);
                __this_3.lock().borrow_mut().list_option.insert_entry(index, &list_option_entry);
                ListEntryWidget::update_button_states(&__this_3);
            }
        } }))) as compat::Shared<dyn TooltipButtonWidget>)); compat::write(&__this, |__t| __t.move_down_button = __v); }
        ListEntryWidget::update_button_states(&__this);
        __this
    }

    /// java: @Override public void extractRenderState(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ListEntryWidget::update_button_states(__this);
        // TODO(fabric2seer): `Plain.setY` has no Rust method in the port (looked for `set_y` in Button and its supertypes)
        // rust: let __wa1 = (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y();
        // rust: let __o1 = compat::read(__this, |__t| __t.remove_button.clone()).unwrap();
        // rust: compat::write(&__o1, |__t| __t.tooltip_button_widget_base_mut().set_y(&__wa1));
        // TODO(fabric2seer): `Plain.setY` has no Rust method in the port (looked for `set_y` in Button and its supertypes)
        // rust: let __wa2 = (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y();
        // rust: let __o2 = compat::read(__this, |__t| __t.move_up_button.clone()).unwrap();
        // rust: compat::write(&__o2, |__t| __t.tooltip_button_widget_base_mut().set_y(&__wa2));
        // TODO(fabric2seer): `Plain.setY` has no Rust method in the port (looked for `set_y` in Button and its supertypes)
        // rust: let __wa3 = (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y();
        // rust: let __o3 = compat::read(__this, |__t| __t.move_down_button.clone()).unwrap();
        // rust: compat::write(&__o3, |__t| __t.tooltip_button_widget_base_mut().set_y(&__wa3));
        ({ let __r = &compat::read(__this, |__t| __t.entry_widget.clone()); let __f = compat::dispatch(__r, |__t| __t.set_dimension()); __f(__r, { let (__a0,) = (&(({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y(),); ({ let __r = &compat::read(__this, |__t| __t.entry_widget.clone()); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(__r) }).lock().borrow().with_y(__a0) }) });
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
        // rust: let __wa4 = graphics.clone();
        // rust: let __wa5 = mouse_x;
        // rust: let __wa6 = mouse_y;
        // rust: let __wa7 = a;
        // rust: let __o4 = compat::read(__this, |__t| __t.remove_button.clone()).unwrap();
        // rust: compat::write(&__o4, |__t| __t.tooltip_button_widget_base_mut().extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa5, __wa6, __wa7));
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
        // rust: let __wa8 = graphics.clone();
        // rust: let __wa9 = mouse_x;
        // rust: let __wa10 = mouse_y;
        // rust: let __wa11 = a;
        // rust: let __o5 = compat::read(__this, |__t| __t.move_up_button.clone()).unwrap();
        // rust: compat::write(&__o5, |__t| __t.tooltip_button_widget_base_mut().extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa9, __wa10, __wa11));
        // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
        // rust: let __wa12 = graphics.clone();
        // rust: let __wa13 = mouse_x;
        // rust: let __wa14 = mouse_y;
        // rust: let __wa15 = a;
        // rust: let __o6 = compat::read(__this, |__t| __t.move_down_button.clone()).unwrap();
        // rust: compat::write(&__o6, |__t| __t.tooltip_button_widget_base_mut().extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa13, __wa14, __wa15));
        // TODO(fabric2seer): `extractRenderState` is not a method of AbstractWidget
        //   java: entryWidget.extractRenderState(graphics, mouseX, mouseY, a);
    }

    /// java: protected void updateButtonStates()
    pub fn update_button_states(__this: &compat::Shared<Self>) {
        { let __v = (__this.lock().borrow().list_option.available() && __this.lock().borrow().list_option.number_of_entries() > __this.lock().borrow().list_option.minimum_number_of_entries()); compat::write(&compat::read(__this, |__t| __t.remove_button.clone()).unwrap(), |__t| __t.tooltip_button_widget_base_mut().active = __v); }
        { let __v = ({ let (__a0,) = (&compat::read(__this, |__t| __t.list_option_entry.clone()),); __this.lock().borrow().list_option.index_of(__a0) } > 0 && __this.lock().borrow().list_option.available()); compat::write(&compat::read(__this, |__t| __t.move_up_button.clone()).unwrap(), |__t| __t.tooltip_button_widget_base_mut().active = __v); }
        { let __v = ({ let (__a0,) = (&compat::read(__this, |__t| __t.list_option_entry.clone()),); __this.lock().borrow().list_option.index_of(__a0) } < ((({ let __r = &compat::read(__this, |__t| __t.list_option.clone()); let __f = compat::dispatch(__r, |__t| __t.options()); __f(&compat::view(__r, |a| a as compat::Shared<dyn OptionGroup>)) })).len() as i32).wrapping_sub(1) && __this.lock().borrow().list_option.available()); compat::write(&compat::read(__this, |__t| __t.move_down_button.clone()).unwrap(), |__t| __t.tooltip_button_widget_base_mut().active = __v); }
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return vec![compat::read(__this, |__t| __t.move_up_button.clone()).unwrap(), compat::read(__this, |__t| __t.move_down_button.clone()).unwrap(), compat::read(__this, |__t| __t.entry_widget.clone()), compat::read(__this, |__t| __t.remove_button.clone()).unwrap()];
    }

    /// java: @Override public boolean isDragging()
    pub fn is_dragging(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.dragging);
    }

    /// java: @Override public void setDragging(boolean dragging)
    pub fn set_dragging(__this: &compat::Shared<Self>, mut dragging: bool) {
        { let __v = dragging; compat::write(__this, |__t| __t.dragging = __v); }
    }

    /// java: @Nullable @Override public GuiEventListener getFocused()
    pub fn get_focused(__this: &compat::Shared<Self>) -> Option<Box<dyn GuiEventListener>> {
        return compat::read(__this, |__t| __t.focused.clone());
    }

    /// java: @Override public void setFocused(@Nullable GuiEventListener focused)
    pub fn set_focused(__this: &compat::Shared<Self>, mut focused: Option<Box<dyn GuiEventListener>>) {
        { let __v = focused.clone(); compat::write(__this, |__t| __t.focused = __v); }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent mouseButtonEvent, boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, mouse_button_event: &MouseButtonEvent, mut double_click: bool) -> bool {
        // TODO(fabric2seer): `.mouseClicked(..)` on a value of unknown type
        //   java: return ContainerEventHandler.super.mouseClicked(mouseButtonEvent, doubleClick);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean mouseReleased(MouseButtonEvent mouseButtonEvent)
    pub fn mouse_released(__this: &compat::Shared<Self>, mouse_button_event: &MouseButtonEvent) -> bool {
        // TODO(fabric2seer): `.mouseReleased(..)` on a value of unknown type
        //   java: return ContainerEventHandler.super.mouseReleased(mouseButtonEvent);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent mouseButtonEvent, double dx, double dy)
    pub fn mouse_dragged(__this: &compat::Shared<Self>, mouse_button_event: &MouseButtonEvent, mut dx: f64, mut dy: f64) -> bool {
        // TODO(fabric2seer): `.mouseDragged(..)` on a value of unknown type
        //   java: return ContainerEventHandler.super.mouseDragged(mouseButtonEvent, dx, dy);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyEvent)
    pub fn key_pressed(__this: &compat::Shared<Self>, key_event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // TODO(fabric2seer): `.keyPressed(..)` on a value of unknown type
        //   java: return ContainerEventHandler.super.keyPressed(keyEvent);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean keyReleased(KeyEvent keyEvent)
    pub fn key_released(__this: &compat::Shared<Self>, key_event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        // TODO(fabric2seer): `.keyReleased(..)` on a value of unknown type
        //   java: return ContainerEventHandler.super.keyReleased(keyEvent);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean charTyped(CharacterEvent characterEvent)
    pub fn char_typed(__this: &compat::Shared<Self>, character_event: &CharacterEvent) -> bool {
        // TODO(fabric2seer): `.charTyped(..)` on a value of unknown type
        //   java: return ContainerEventHandler.super.charTyped(characterEvent);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl AbstractWidget for ListEntryWidget {
    fn abstract_widget_base(&self) -> &AbstractWidgetBase {
        &self.base
    }
    fn abstract_widget_base_mut(&mut self) -> &mut AbstractWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void unfocus() (dispatch: the body for this class)
    fn unfocus(&self) -> fn(&compat::Shared<dyn AbstractWidget>) {
        ListEntryWidget::unfocus_dyn
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_narration(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &mut dyn NarrationElementOutput) {
        ListEntryWidget::update_narration_dyn
    }

    /// java: @Override public boolean matchesSearch(String query) (dispatch: the body for this class)
    fn matches_search(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &str) -> bool {
        ListEntryWidget::matches_search_dyn
    }

}

impl ListEntryWidget {
    /// java: @Override public void unfocus()
    pub fn unfocus_impl(__this: &compat::Shared<Self>) {
        ({ let __r = &compat::read(__this, |__t| __t.entry_widget.clone()); let __f = compat::dispatch(__r, |__t| __t.unfocus()); __f(__r) });
    }

    /// ListEntryWidget.unfocus as the dispatch of AbstractWidget.unfocus
    pub fn unfocus_dyn(__this: &compat::Shared<dyn AbstractWidget>) {
        Self::unfocus_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder)
    pub fn update_narration_impl(__this: &compat::Shared<Self>, builder: &mut dyn NarrationElementOutput) {
        ({ let __r = &compat::read(__this, |__t| __t.entry_widget.clone()); let __f = compat::dispatch(__r, |__t| __t.update_narration()); __f(__r, builder) });
    }

    /// ListEntryWidget.updateNarration as the dispatch of AbstractWidget.updateNarration
    pub fn update_narration_dyn(__this: &compat::Shared<dyn AbstractWidget>, __p0: &mut dyn NarrationElementOutput) {
        Self::update_narration_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean matchesSearch(String query)
    pub fn matches_search_impl(__this: &compat::Shared<Self>, query: &str) -> bool {
        let __h0 = &query.to_lowercase();
        return __this.lock().borrow().option_name_string.contains(__h0);
    }

    /// ListEntryWidget.matchesSearch as the dispatch of AbstractWidget.matchesSearch
    pub fn matches_search_dyn(__this: &compat::Shared<dyn AbstractWidget>, __p0: &str) -> bool {
        Self::matches_search_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
