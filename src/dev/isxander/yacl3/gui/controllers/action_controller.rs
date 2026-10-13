//! Ported from dev.isxander.yacl3.gui.controllers.ActionController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::button_option::{ButtonAction, ButtonOption};
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::{self, ControllerWidget, ControllerWidgetBase};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::{CursorKind, Graphics};
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public static final Component DEFAULT_TEXT = Component.translatable("yacl.control.action.execute");
pub fn default_text() -> MutableComponent {
    component::translatable("yacl.control.action.execute")
}

/// java: public class ActionController implements Controller<BiConsumer<YACLScreen, ButtonOption>>
///
/// Simple controller that simply runs the button action on press and renders a text on the right.
pub struct ActionController {
    /// java: private final ButtonOption option;
    option: Arc<dyn ButtonOption>,
    /// java: private final Component text;
    text: MutableComponent,
}

impl ActionController {
    /// java: public ActionController(ButtonOption option) - with the default text `DEFAULT_TEXT`
    pub fn new(option: Arc<dyn ButtonOption>) -> Self {
        Self::new_1(option, default_text())
    }

    /// java: public ActionController(ButtonOption option, Component text)
    pub fn new_1(option: Arc<dyn ButtonOption>, text: MutableComponent) -> Self {
        Self { option, text }
    }

    /// `option()` with its declared type `ButtonOption` (Java's covariant override).
    pub fn button_option(&self) -> Arc<dyn ButtonOption> {
        self.option.clone()
    }
}

impl Controller<ButtonAction> for ActionController {
    /// java: @Override public ButtonOption option()
    fn option(&self) -> Arc<dyn YOption<ButtonAction>> {
        self.option.clone()
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        self.text.clone()
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(
        self: Arc<Self>,
        screen: &Arc<YACLScreen>,
        widget_dimension: Arc<dyn Dimension<i32>>,
    ) -> Arc<dyn abstract_widget::AbstractWidget> {
        Arc::new(ActionControllerElement::new(self, screen, widget_dimension))
    }
}

/// java: public static class ActionControllerElement extends ControllerWidget<ActionController>
pub struct ActionControllerElement {
    /// java: extends ControllerWidget<ActionController>
    pub cw: ControllerWidgetBase,
    /// java: protected final T control;
    pub control: Arc<ActionController>,
    /// java: private final String buttonString;
    button_string: String,
}

impl ActionControllerElement {
    /// java: public ActionControllerElement(ActionController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(control: Arc<ActionController>, screen: &Arc<YACLScreen>, dim: Arc<dyn Dimension<i32>>) -> Self {
        let cw = ControllerWidgetBase::new::<ButtonAction, ActionController>(&control, screen, dim);
        let button_string = control.format_value().get_string().to_lowercase();
        Self { cw, control, button_string }
    }

    /// java: public void executeAction()
    pub fn execute_action(&self) {
        ControllerWidget::play_down_sound(self);
        let option = self.control.button_option();
        option.action().accept(&self.cw.screen(), &*option);
    }
}

impl ControllerWidget for ActionControllerElement {
    fn controller_widget(&self) -> &ControllerWidgetBase {
        &self.cw
    }

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        controller_widget::extract_value_text(self, graphics, mouse_x, mouse_y, a);

        if self.cw.hovered.get() {
            graphics.request_cursor(if self.is_available() { CursorKind::PointingHand } else { CursorKind::NotAllowed });
        }
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked(&self, event: &MouseButtonEvent, _double_click: bool) -> bool {
        if ControllerWidget::is_mouse_over(self, event.x(), event.y()) && self.is_available() {
            self.execute_action();
            return true;
        }
        false
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        if !self.cw.focused.get() {
            return false;
        }

        if event.key() == input_constants::KEY_RETURN
            || event.key() == input_constants::KEY_SPACE
            || event.key() == input_constants::KEY_NUMPADENTER
        {
            self.execute_action();
            return true;
        }

        false
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        self.get_unhovered_control_width()
    }

    /// java: @Override public boolean canReset()
    fn can_reset(&self) -> bool {
        false
    }

    /// java: @Override public boolean matchesSearch(String query)
    fn matches_search(&self, query: &str) -> bool {
        controller_widget::matches_search(self, query) || self.button_string.contains(query)
    }
}
