//! Ported from dev.isxander.yacl3.gui.controllers.PopupControllerScreen.
//!
//! `public class PopupControllerScreen extends Screen`: drawn and dispatched by itself like the YACL screen it is
//! opened over (PATTERNS_Y2.md P3) - the game's stack holds its `ModScreen` adapter. Its only child is the popup, so
//! `Screen`'s child list and focus are that one widget and two flags.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::ControllerPopupWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;
use crate::dev::isxander::yacl3::gui::yacl_screen::{self, YACLScreen};
use compat::Graphics;
use mcrust::mcrust::modding::screens::{ModScreen, ScreenRenderContext};
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component::Component as _;
use std::cell::Cell;

/// java: public class PopupControllerScreen extends Screen
pub struct PopupControllerScreen {
    /// java: private final YACLScreen backgroundYaclScreen;
    background_yacl_screen: Arc<YACLScreen>,
    /// java: private final ControllerPopupWidget<?> controllerPopup;
    controller_popup: Arc<dyn ControllerPopupWidget>,
    /// `Screen.renderables` holds the popup once `init()` added it
    added: Cell<bool>,
    /// `AbstractContainerEventHandler.focused == controllerPopup`
    focused: Cell<bool>,
    /// `AbstractContainerEventHandler.isDragging`
    dragging: Cell<bool>,
    /// the cursor position the last frame saw (`mouseMoved` is driven from it: see `extract_render_state`)
    last_mouse: Cell<Option<(i32, i32)>>,
}

impl PopupControllerScreen {
    /// java: public PopupControllerScreen(YACLScreen backgroundYaclScreen, ControllerPopupWidget<?> controllerPopup) -
    /// `super(controllerPopup.popupTitle())` (narrated)
    pub fn new(background_yacl_screen: Arc<YACLScreen>, controller_popup: Arc<dyn ControllerPopupWidget>) -> Self {
        Self {
            background_yacl_screen,
            controller_popup,
            added: Cell::new(false),
            focused: Cell::new(false),
            dragging: Cell::new(false),
            last_mouse: Cell::new(None),
        }
    }

    /// `new PopupControllerScreen(this, controllerPopupWidget)` as `GuiUtils.setScreen` takes it: the screen over the
    /// YACL screen, which stays below it (`onClose` returns to it).
    pub fn new_screen(background_yacl_screen: &Arc<YACLScreen>, controller_popup: Arc<dyn ControllerPopupWidget>) -> compat::ScreenRef {
        let screen = PopupControllerScreen::new(background_yacl_screen.clone(), controller_popup);
        compat::ScreenRef::of_box(Box::new(PopupControllerScreenAdapter { screen: compat::RenderThread::new(screen) }), true)
    }

    /// java: @Override protected void init()
    fn init(&self) {
        self.added.set(true);
    }

    /// `Screen.rebuildWidgets()`: `clearWidgets(); clearFocus(); init();`
    fn rebuild_widgets(&self) {
        self.added.set(false);
        self.set_focused_popup(false);
        self.init();
    }

    /// `setFocused(controllerPopup)` / `setFocused(null)` - `AbstractContainerEventHandler`'s
    fn set_focused_popup(&self, focused: bool) {
        if self.focused.get() != focused {
            ControllerWidget::set_focused(&*self.controller_popup, focused);
            self.focused.set(focused);
        }
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&self) {
        self.rebuild_widgets();
        self.on_close();
    }

    /// java: @Override public void extractRenderState(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY,
    /// float a)
    pub fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.controller_popup.extract_background(graphics, mouse_x, mouse_y, a);
        // mouseX/Y set to -1 to prevent hovering outlines
        self.background_yacl_screen.extract_render_state(graphics, -1, -1, a);
        // super.extractRenderState - the popup
        if self.added.get() {
            ControllerWidget::extract_render_state(&*self.controller_popup, graphics, mouse_x, mouse_y, a);
        }
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor guiGraphics, int mouseX, int mouseY, float
    /// partialTick) - the YACL screen's (the game has drawn `Screen`'s own background)
    pub fn extract_background(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, partial_tick: f32) {
        self.background_yacl_screen.extract_background(graphics, mouse_x, mouse_y, partial_tick);
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        // super.mouseClicked - ContainerEventHandler's over the one child
        let clicked = self.added.get() && ControllerWidget::is_mouse_over(&*self.controller_popup, event.x(), event.y()) && {
            if ControllerWidget::mouse_clicked(&*self.controller_popup, event, double_click)
                && ControllerWidget::should_take_focus_after_interaction(&*self.controller_popup)
            {
                self.set_focused_popup(true);
                if event.button() == 1 {
                    self.dragging.set(true);
                }
            }
            true
        };
        if !clicked {
            self.on_close();
            return false;
        }
        true
    }

    /// `ContainerEventHandler.mouseReleased`
    pub fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
        if event.button() == 1 && self.dragging.get() {
            self.dragging.set(false);
            if self.focused.get() {
                return ControllerWidget::mouse_released(&*self.controller_popup, event);
            }
        }
        false
    }

    /// `ContainerEventHandler.mouseDragged`
    pub fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        self.focused.get() && self.dragging.get() && event.button() == 1 && ControllerWidget::mouse_dragged(&*self.controller_popup, event, dx, dy)
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    pub fn mouse_scrolled(&self, mouse_x: f64, mouse_y: f64, horizontal: f64, vertical: f64) -> bool {
        if ControllerWidget::mouse_scrolled(&*self.controller_popup, mouse_x, mouse_y, horizontal, vertical) {
            return true;
        }
        // mouseX & mouseY are needed here
        self.background_yacl_screen.mouse_scrolled(mouse_x, mouse_y, horizontal, vertical);
        // super.mouseScrolled - the child under the cursor
        self.added.get()
            && ControllerWidget::is_mouse_over(&*self.controller_popup, mouse_x, mouse_y)
            && ControllerWidget::mouse_scrolled(&*self.controller_popup, mouse_x, mouse_y, horizontal, vertical)
    }

    /// java: @Override public void mouseMoved(double mouseX, double mouseY)
    pub fn mouse_moved(&self, mouse_x: f64, mouse_y: f64) {
        ControllerWidget::mouse_moved(&*self.controller_popup, mouse_x, mouse_y);
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent characterEvent)
    pub fn char_typed(&self, event: &CharacterEvent) -> bool {
        ControllerWidget::char_typed(&*self.controller_popup, event)
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent keyEvent)
    pub fn key_pressed(&self, event: &KeyEvent) -> bool {
        ControllerWidget::key_pressed(&*self.controller_popup, event)
    }

    /// java: @Override public void tick()
    pub fn tick(&self) {
        self.background_yacl_screen.tick();
    }

    /// java: @Override public void onClose()
    pub fn on_close(&self) {
        PopupControllerScreen::on_close_over(&self.background_yacl_screen, &self.controller_popup);
    }

    /// `onClose()`'s body over the two objects it reads: `YACLScreen.clearPopupControllerWidget` runs it for the popup
    /// screen that is up (`popupControllerScreen.onClose()`), which the YACL screen reaches through itself.
    pub fn on_close_over(background_yacl_screen: &Arc<YACLScreen>, controller_popup: &Arc<dyn ControllerPopupWidget>) {
        // GuiUtils.setScreen(backgroundYaclScreen, true); - the popup screen is no longer the current one from here on
        background_yacl_screen.popup_screen_up.set(false);
        GuiUtils::set_screen(compat::ScreenRef::Parent, true);
        controller_popup.close();
    }
}

/// `PopupControllerScreen` in the game's screen stack.
pub struct PopupControllerScreenAdapter {
    screen: compat::RenderThread<PopupControllerScreen>,
}

impl PopupControllerScreenAdapter {
    /// The popup screen (`minecraft.screen instanceof PopupControllerScreen p` binds this).
    pub fn popup_screen(&self) -> &PopupControllerScreen {
        self.screen.get()
    }
}

impl ModScreen for PopupControllerScreenAdapter {
    fn get_title(&self) -> String {
        self.popup_screen().controller_popup.popup_title().get_string()
    }

    fn init(&mut self, screen: &mut Screen) {
        screen.add_renderable_widget(compat::cursor_sink_widget());
        self.popup_screen().init();
    }

    fn reposition_elements(&mut self, screen: &mut Screen) {
        screen.clear_widgets();
        screen.add_renderable_widget(compat::cursor_sink_widget());
        self.popup_screen().reposition_elements();
    }

    fn tick(&mut self, _screen: &mut Screen) {
        self.popup_screen().tick();
    }

    /// The game's `Screen` background is drawn; then the YACL screen's tab background, the YACL screen, the popup, and
    /// the frame's tooltip. `mouseMoved` has no event of its own for a mod screen: a frame whose cursor moved delivers
    /// it first (`MouseHandler.onMove` -> `screen.mouseMoved`).
    fn extract_render_state(&mut self, _screen: &Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let popup_screen = self.popup_screen();
        if popup_screen.last_mouse.get() != Some((mouse_x, mouse_y)) {
            popup_screen.last_mouse.set(Some((mouse_x, mouse_y)));
            popup_screen.mouse_moved(mouse_x as f64, mouse_y as f64);
        }
        let graphics = Graphics::new(context.gl, &*context.minecraft);
        popup_screen.extract_background(&graphics, mouse_x, mouse_y, a);
        popup_screen.extract_render_state(&graphics, mouse_x, mouse_y, a);
        graphics.extract_deferred_tooltip();
    }

    fn key_pressed(&mut self, _screen: &mut Screen, event: &KeyEvent) -> bool {
        self.popup_screen().key_pressed(event)
    }

    fn char_typed(&mut self, _screen: &mut Screen, c: char) -> bool {
        self.popup_screen().char_typed(&CharacterEvent::new(c as i32))
    }

    fn mouse_clicked(&mut self, _screen: &mut Screen, x: f64, y: f64, button: i32) -> bool {
        self.popup_screen().mouse_clicked(&yacl_screen::mouse_event(x, y, button), false)
    }

    fn mouse_released(&mut self, _screen: &mut Screen, x: f64, y: f64, button: i32) -> bool {
        self.popup_screen().mouse_released(&yacl_screen::mouse_event(x, y, button))
    }

    fn mouse_dragged(&mut self, _screen: &mut Screen, x: f64, y: f64, button: i32, dx: f64, dy: f64) -> bool {
        self.popup_screen().mouse_dragged(&yacl_screen::mouse_event(x, y, button), dx, dy)
    }

    fn mouse_scrolled(&mut self, _screen: &mut Screen, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        self.popup_screen().mouse_scrolled(x, y, scroll_x, scroll_y)
    }

    /// `keyPressed` hands every key to the popup (Escape included), so the game's polled Escape must not close it
    fn should_close_on_esc(&self) -> bool {
        false
    }

    fn on_close(&mut self) {
        self.popup_screen().on_close();
    }

    fn class_name(&self) -> &'static str {
        "dev.isxander.yacl3.gui.controllers.PopupControllerScreen"
    }
}
