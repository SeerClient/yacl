//! Ported from dev.isxander.yacl3.gui.controllers.BooleanController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::{self, ControllerWidget, ControllerWidgetBase};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::{CursorKind, Graphics};
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::common_components;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public static final Function<Boolean, Component> ON_OFF_FORMATTER
pub fn on_off_formatter(state: bool) -> MutableComponent {
    compat::component_of(if state { common_components::option_on() } else { common_components::option_off() })
}

/// java: public static final Function<Boolean, Component> TRUE_FALSE_FORMATTER
pub fn true_false_formatter(state: bool) -> MutableComponent {
    if state { component::translatable("yacl.control.boolean.true") } else { component::translatable("yacl.control.boolean.false") }
}

/// java: public static final Function<Boolean, Component> YES_NO_FORMATTER
pub fn yes_no_formatter(state: bool) -> MutableComponent {
    compat::component_of(if state { common_components::gui_yes() } else { common_components::gui_no() })
}

/// java: public class BooleanController implements Controller<Boolean>
///
/// This controller renders a simple formatted `Component`.
pub struct BooleanController {
    /// java: private final Option<Boolean> option;
    option: Arc<dyn YOption<bool>>,
    /// java: private final ValueFormatter<Boolean> valueFormatter;
    value_formatter: Arc<dyn ValueFormatter<bool>>,
    /// java: private final boolean coloured;
    coloured: bool,
}

impl BooleanController {
    /// java: public BooleanController(Option<Boolean> option) - `this(option, ON_OFF_FORMATTER, false)`
    pub fn new(option: Arc<dyn YOption<bool>>) -> Self {
        Self::new_3(option, Arc::new(on_off_formatter), false)
    }

    /// java: public BooleanController(Option<Boolean> option, boolean coloured)
    pub fn new_2(option: Arc<dyn YOption<bool>>, coloured: bool) -> Self {
        Self::new_3(option, Arc::new(on_off_formatter), coloured)
    }

    /// java: public BooleanController(Option<Boolean> option, Function<Boolean, Component> valueFormatter, boolean coloured)
    pub fn new_3(option: Arc<dyn YOption<bool>>, value_formatter: Arc<dyn ValueFormatter<bool>>, coloured: bool) -> Self {
        Self { option, value_formatter, coloured }
    }

    /// java: @ApiStatus.Internal public static BooleanController createInternal(Option<Boolean> option,
    /// ValueFormatter<Boolean> formatter, boolean coloured)
    pub fn create_internal(
        option: Arc<dyn YOption<bool>>,
        formatter: Arc<dyn ValueFormatter<bool>>,
        coloured: bool,
    ) -> Arc<dyn Controller<bool>> {
        Arc::new(BooleanController::new_3(option, formatter, coloured))
    }

    /// java: public boolean coloured() - value format is green or red depending on the state
    pub fn coloured(&self) -> bool {
        self.coloured
    }
}

impl Controller<bool> for BooleanController {
    /// java: @Override public Option<Boolean> option()
    fn option(&self) -> Arc<dyn YOption<bool>> {
        self.option.clone()
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        self.value_formatter.format(self.option.pending_value())
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(
        self: Arc<Self>,
        screen: &Arc<YACLScreen>,
        widget_dimension: Arc<dyn Dimension<i32>>,
    ) -> Arc<dyn abstract_widget::AbstractWidget> {
        Arc::new(BooleanControllerElement::new(self, screen, widget_dimension))
    }
}

/// java: public static class BooleanControllerElement extends ControllerWidget<BooleanController>
pub struct BooleanControllerElement {
    /// java: extends ControllerWidget<BooleanController>
    pub cw: ControllerWidgetBase,
    /// java: protected final T control;
    pub control: Arc<BooleanController>,
}

impl BooleanControllerElement {
    /// java: public BooleanControllerElement(BooleanController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(control: Arc<BooleanController>, screen: &Arc<YACLScreen>, dim: Arc<dyn Dimension<i32>>) -> Self {
        Self { cw: ControllerWidgetBase::new::<bool, BooleanController>(&control, screen, dim), control }
    }

    /// java: public void toggleSetting()
    pub fn toggle_setting(&self) {
        let option = self.control.option();
        option.request_set(!option.pending_value());
        ControllerWidget::play_down_sound(self);
    }
}

impl ControllerWidget for BooleanControllerElement {
    fn controller_widget(&self) -> &ControllerWidgetBase {
        &self.cw
    }

    /// java: @Override protected void extractHoveredControl(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_hovered_control(&self, _graphics: &Graphics<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {}

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        controller_widget::extract_value_text(self, graphics, mouse_x, mouse_y, a);

        if self.cw.hovered.get() {
            graphics.request_cursor(if self.is_available() { CursorKind::PointingHand } else { CursorKind::NotAllowed });
        }
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked(&self, event: &MouseButtonEvent, _double_click: bool) -> bool {
        if !ControllerWidget::is_mouse_over(self, event.x(), event.y()) || !self.is_available() {
            return false;
        }

        self.toggle_setting();
        true
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        self.get_unhovered_control_width()
    }

    /// java: @Override protected Component getValueText()
    fn get_value_text(&self) -> MutableComponent {
        if self.control.coloured() {
            // super.getValueText().copy().withStyle(..)
            return self.cw.control_format_value().copy().with_style_format(if self.control.option().pending_value() {
                ChatFormatting::Green
            } else {
                ChatFormatting::Red
            });
        }

        self.cw.control_format_value()
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event)
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        if !ControllerWidget::is_focused(self) {
            return false;
        }

        if event.key() == input_constants::KEY_RETURN
            || event.key() == input_constants::KEY_SPACE
            || event.key() == input_constants::KEY_NUMPADENTER
        {
            self.toggle_setting();
            return true;
        }

        false
    }
}
