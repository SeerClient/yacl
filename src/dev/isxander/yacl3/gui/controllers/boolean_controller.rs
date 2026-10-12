//! Ported from dev.isxander.yacl3.gui.controllers.BooleanController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
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
use crate::dev::isxander::yacl3::api::option::Option as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final Function<Boolean, Component> ON_OFF_FORMATTER = (state) -> state ? CommonComponents.OPTION_ON : CommonComponents.OPTION_OFF;
pub static ON_OFF_FORMATTER: compat::MainThread<Arc<dyn Fn(bool) -> MutableComponent>> = compat::MainThread::new(|| Arc::new(move |state| (if state { mcrust::net::minecraft::network::chat::common_components::option_on() } else { mcrust::net::minecraft::network::chat::common_components::option_off() })));
/// java: public static final Function<Boolean, Component> TRUE_FALSE_FORMATTER = (state) -> state ? Component.translatable("yacl.control.boolean.true") : Component.translatable("yacl.control.boolean.false");
pub static TRUE_FALSE_FORMATTER: compat::MainThread<Arc<dyn Fn(bool) -> MutableComponent>> = compat::MainThread::new(|| Arc::new(move |state| (if state { component::translatable("yacl.control.boolean.true") } else { component::translatable("yacl.control.boolean.false") })));
/// java: public static final Function<Boolean, Component> YES_NO_FORMATTER = (state) -> state ? CommonComponents.GUI_YES : CommonComponents.GUI_NO;
pub static YES_NO_FORMATTER: compat::MainThread<Arc<dyn Fn(bool) -> MutableComponent>> = compat::MainThread::new(|| Arc::new(move |state| (if state { mcrust::net::minecraft::network::chat::common_components::gui_yes() } else { mcrust::net::minecraft::network::chat::common_components::gui_no() })));

/// java: public class BooleanController implements Controller<Boolean>
#[derive(Clone)]
pub struct BooleanController {
    /// java: private final Option<Boolean> option;
    pub option: Option<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>>>,
    /// java: private final ValueFormatter<Boolean> valueFormatter;
    pub value_formatter: Option<compat::Shared<dyn ValueFormatter<bool>>>,
    /// java: private final boolean coloured;
    pub coloured: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl BooleanController {
    /// java: public BooleanController(Option<Boolean> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>>) -> compat::Shared<Self> {
        let __this = Self::new_3(option.clone(), ON_OFF_FORMATTER.get_mut().clone(), false);
        __this
    }

    /// java: public BooleanController(Option<Boolean> option, boolean coloured)
    pub fn new_2(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>>, mut coloured: bool) -> compat::Shared<Self> {
        let __this = Self::new_3(option.clone(), ON_OFF_FORMATTER.get_mut().clone(), coloured);
        __this
    }

    /// java: public BooleanController(Option<Boolean> option, Function<Boolean, Component> valueFormatter, boolean coloured)
    pub fn new_3(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>>, value_formatter: Arc<dyn Fn(bool) -> MutableComponent>, mut coloured: bool) -> compat::Shared<Self> {
        let __init_option = Some(option.clone());
        let __init_value_formatter = Some(Arc::new(move |__a0| (value_formatter)(__a0.clone())));
        let __init_coloured = coloured;
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            option: __init_option,
            value_formatter: __init_value_formatter,
            coloured: __init_coloured,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @ApiStatus.Internal public static BooleanController createInternal(Option<Boolean> option, ValueFormatter<Boolean> formatter, boolean coloured)
    pub fn create_internal(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>>, formatter: &compat::Shared<dyn ValueFormatter<bool>>, mut coloured: bool) -> compat::Shared<BooleanController> {
        return BooleanController::new_3(option.clone(), Arc::new(move |__a0| formatter.lock().borrow().format(&__a0)), coloured);
    }

    /// java: public boolean coloured()
    pub fn coloured(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.coloured);
    }

}

impl Controller<bool> for BooleanController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<Boolean> option()
    fn option(&self) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<bool>> {
        return self.option.clone().unwrap();
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        return { let (__a0,) = (&self.option().lock().borrow().pending_value(),); self.value_formatter.as_ref().unwrap().lock().borrow().format(__a0) };
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        return (BooleanControllerElement::new(self.self_ref.upgrade().unwrap(), screen.clone(), widget_dimension.clone()) as compat::Shared<dyn AbstractWidget>);
    }

}

/// java: public static class BooleanControllerElement extends ControllerWidget<BooleanController>
pub struct BooleanControllerElement {
    /// java: extends ControllerWidget
    pub base: ControllerWidgetBase<compat::Shared<BooleanController>>,
}

impl std::ops::Deref for BooleanControllerElement {
    type Target = ControllerWidgetBase<compat::Shared<BooleanController>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BooleanControllerElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BooleanControllerElement {
    /// java: public BooleanControllerElement(BooleanController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(mut control: compat::Shared<BooleanController>, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: ControllerWidgetBase::<compat::Shared<BooleanController>>::new(control.clone(), screen.clone(), dim.clone()),
        });
        __this
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), event.x(), event.y()) }) || !(__this.lock().borrow().is_available()) {
            return false;
        }
        BooleanControllerElement::toggle_setting(__this);
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
        if !(__this.lock().borrow().is_focused()) {
            return false;
        }
        if event.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_RETURN || event.key() == 32 || event.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_NUMPADENTER {
            BooleanControllerElement::toggle_setting(__this);
            return true;
        }
        return false;
    }

}

impl ControllerWidget<compat::Shared<crate::dev::isxander::yacl3::gui::controllers::boolean_controller::BooleanController>> for BooleanControllerElement {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<compat::Shared<crate::dev::isxander::yacl3::gui::controllers::boolean_controller::BooleanController>> {
        &self.base
    }
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<compat::Shared<crate::dev::isxander::yacl3::gui::controllers::boolean_controller::BooleanController>> {
        &mut self.base
    }

    /// java: @Override protected void extractHoveredControl(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_hovered_control(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
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

    /// java: @Override protected Component getValueText()
    fn get_value_text(&self) -> MutableComponent {
        // TODO(fabric2seer): the condition: `.coloured(..)` on a T
        //   java: control.coloured(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.option(..)` on a T
            //   java: return super.getValueText().copy().withStyle(control.option().pendingValue() ? ChatFormatting.GREEN : ChatFormatting.RED);
        }
        return self.base.get_value_text();
    }

}

impl AbstractWidget for BooleanControllerElement {
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
