//! Ported from dev.isxander.yacl3.gui.controllers.ActionController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::button_option::ButtonOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidgetBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final Component DEFAULT_TEXT = Component.translatable("yacl.control.action.execute");
pub static DEFAULT_TEXT: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::translatable("yacl.control.action.execute"));

/// java: public class ActionController implements Controller<BiConsumer<YACLScreen, ButtonOption>>
#[derive(Clone)]
pub struct ActionController {
    /// java: private final ButtonOption option;
    pub option: Option<compat::Shared<dyn ButtonOption>>,
    /// java: private final Component text;
    pub text: Option<MutableComponent>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl ActionController {
    /// java: public ActionController(ButtonOption option)
    pub fn new(mut option: compat::Shared<dyn ButtonOption>) -> compat::Shared<Self> {
        let __this = Self::new_2(option.clone(), DEFAULT_TEXT.get_mut().clone());
        __this
    }

    /// java: public ActionController(ButtonOption option, Component text)
    pub fn new_2(mut option: compat::Shared<dyn ButtonOption>, mut text: MutableComponent) -> compat::Shared<Self> {
        let __init_option = Some(option.clone());
        let __init_text = Some(text.clone());
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            option: __init_option,
            text: __init_text,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl Controller<Arc<dyn Fn(compat::Shared<crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen>, compat::Shared<dyn crate::dev::isxander::yacl3::api::button_option::ButtonOption>)>> for ActionController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public ButtonOption option()
    fn option(&self) -> compat::Shared<dyn ButtonOption> {
        return self.option.clone().unwrap();
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        return self.text.clone().unwrap();
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        return (ActionControllerElement::new(self.self_ref.upgrade().unwrap(), screen.clone(), widget_dimension.clone()) as compat::Shared<dyn AbstractWidget>);
    }

}

/// java: public static class ActionControllerElement extends ControllerWidget<ActionController>
pub struct ActionControllerElement {
    /// java: extends ControllerWidget
    pub base: ControllerWidgetBase<compat::Shared<ActionController>>,
    /// java: private final String buttonString;
    pub button_string: String,
}

impl std::ops::Deref for ActionControllerElement {
    type Target = ControllerWidgetBase<compat::Shared<ActionController>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ActionControllerElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ActionControllerElement {
    /// java: public ActionControllerElement(ActionController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(mut control: compat::Shared<ActionController>, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>) -> compat::Shared<Self> {
        let __init_button_string = control.lock().borrow().format_value().get_string().to_lowercase();
        let __this = compat::shared(Self {
            base: ControllerWidgetBase::<compat::Shared<ActionController>>::new(control.clone(), screen.clone(), dim.clone()),
            button_string: __init_button_string,
        });
        __this
    }

    /// java: public void executeAction()
    pub fn execute_action(__this: &compat::Shared<Self>) {
        ({ let __f = compat::dispatch(__this, |__t| __t.play_down_sound()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) });
        // TODO(fabric2seer): `.option(..)` on a T
        //   java: control.option().action().accept(screen, control.option());
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        if { let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), event.x(), event.y()) } && __this.lock().borrow().is_available() {
            ActionControllerElement::execute_action(__this);
            return true;
        }
        return false;
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    pub fn key_pressed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if !(compat::read(__this, |__t| __t.focused)) {
            return false;
        }
        if event.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_RETURN || event.key() == 32 || event.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_NUMPADENTER {
            ActionControllerElement::execute_action(__this);
            return true;
        }
        return false;
    }

}

impl ControllerWidget<compat::Shared<crate::dev::isxander::yacl3::gui::controllers::action_controller::ActionController>> for ActionControllerElement {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<compat::Shared<crate::dev::isxander::yacl3::gui::controllers::action_controller::ActionController>> {
        &self.base
    }
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<compat::Shared<crate::dev::isxander::yacl3::gui::controllers::action_controller::ActionController>> {
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

impl AbstractWidget for ActionControllerElement {
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
        ActionControllerElement::can_reset_dyn
    }

    /// java: @Override public boolean matchesSearch(String query) (dispatch: the body for this class)
    fn matches_search(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &str) -> bool {
        ActionControllerElement::matches_search_dyn
    }

    /// java: @Override public NarrationPriority narrationPriority() (dispatch: the body for this class)
    fn narration_priority(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> crate::unresolved::NarrationPriority {
        <dyn ControllerWidget>::narration_priority_from::<Self>
    }

    /// java: @Override public void unfocus() (dispatch: the body for this class)
    fn unfocus(&self) -> fn(&compat::Shared<dyn AbstractWidget>) {
        <dyn ControllerWidget>::unfocus_from::<Self>
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_narration(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &mut dyn NarrationElementOutput) {
        <dyn ControllerWidget>::update_narration_from::<Self>
    }

}

impl ActionControllerElement {
    /// java: @Override public boolean canReset()
    pub fn can_reset_impl(__this: &compat::Shared<Self>) -> bool {
        return false;
    }

    /// ActionControllerElement.canReset as the dispatch of AbstractWidget.canReset
    pub fn can_reset_dyn(__this: &compat::Shared<dyn AbstractWidget>) -> bool {
        Self::can_reset_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean matchesSearch(String query)
    pub fn matches_search_impl(__this: &compat::Shared<Self>, query: &str) -> bool {
        return (__this.base.matches_search(query) || __this.lock().borrow().button_string.contains(query));
    }

    /// ActionControllerElement.matchesSearch as the dispatch of AbstractWidget.matchesSearch
    pub fn matches_search_dyn(__this: &compat::Shared<dyn AbstractWidget>, __p0: &str) -> bool {
        Self::matches_search_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
