//! Ported from dev.isxander.yacl3.gui.controllers.PopupControllerScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::ControllerPopupWidget;
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class PopupControllerScreen extends Screen
pub struct PopupControllerScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final YACLScreen backgroundYaclScreen;
    pub background_yacl_screen: compat::Shared<YACLScreen>,
    /// java: private final ControllerPopupWidget<?> controllerPopup;
    pub controller_popup: compat::Shared<dyn ControllerPopupWidget<compat::Object>>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for PopupControllerScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PopupControllerScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl PopupControllerScreen {
    /// java: public PopupControllerScreen(YACLScreen backgroundYaclScreen, ControllerPopupWidget<?> controllerPopup)
    pub fn new(mut background_yacl_screen: compat::Shared<YACLScreen>, mut controller_popup: compat::Shared<dyn ControllerPopupWidget<compat::Object>>) -> compat::Shared<Self> {
        let __init_background_yacl_screen = background_yacl_screen.clone();
        let __init_controller_popup = controller_popup.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(controller_popup.lock().borrow().popup_title().get_string()),
            __widgets: Default::default(),
            background_yacl_screen: __init_background_yacl_screen,
            controller_popup: __init_controller_popup,
        });
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        let __r0 = compat::read(__this, |__t| __t.controller_popup.clone());
        { let (__q0,) = (__r0,); __this.lock().borrow_mut().add_renderable_widget(__q0) };
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `reposition_elements` takes 1 argument(s) in the port (`build: impl FnOnce(&mut Screen)`), Java passes 0
        // rust: __this.lock().borrow_mut().base.reposition_elements();
        PopupControllerScreen::on_close(__this);
    }

    /// java: @Override public void extractRenderState(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        __this.lock().borrow_mut().controller_popup.extract_background(graphics, mouse_x, mouse_y, a);
        // TODO(fabric2seer): `Screen.extractRenderState` has no Rust method in the port (looked for `extract_render_state` in Screen and JavaSrc)
        // rust: let __wa1 = graphics.clone();
        // rust: let __wa2 = a;
        // rust: let __o1 = compat::read(__this, |__t| __t.background_yacl_screen.clone());
        // rust: compat::write(&__o1, |__t| __t.extract_render_state(&__wa1, -1, -1, __wa2));
        ();
    }

    /// java: @Override public void extractBackground( GuiGraphicsExtractor guiGraphics, int mouseX, int mouseY, float partialTick )
    pub fn extract_background(__this: &compat::Shared<Self>, gui_graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut partial_tick: f32) {
        YACLScreen::extract_background(&compat::read(__this, |__t| __t.background_yacl_screen.clone()), gui_graphics, mouse_x, mouse_y, partial_tick);
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        if !false {
            PopupControllerScreen::on_close(__this);
            return false;
        }
        return true;
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal: f64, mut vertical: f64) -> bool {
        // TODO(fabric2seer): the condition: `mouseScrolled` is not a method of ControllerPopupWidget
        //   java: controllerPopup.mouseScrolled(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            return true;
        }
        // TODO(fabric2seer): `Screen.mouseScrolled` has no Rust method in the port (looked for `mouse_scrolled` in Screen and JavaSrc)
        // rust: let __wa1 = mouse_x;
        // rust: let __wa2 = mouse_y;
        // rust: let __wa3 = horizontal;
        // rust: let __wa4 = vertical;
        // rust: let __o1 = compat::read(__this, |__t| __t.background_yacl_screen.clone());
        // rust: compat::write(&__o1, |__t| __t.mouse_scrolled(__wa1, __wa2, __wa3, __wa4));
        return false;
    }

    /// java: @Override public void mouseMoved(double mouseX, double mouseY)
    pub fn mouse_moved(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) {
        // TODO(fabric2seer): `mouseMoved` is not a method of ControllerPopupWidget
        //   java: controllerPopup.mouseMoved(mouseX, mouseY);
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent characterEvent)
    pub fn char_typed(__this: &compat::Shared<Self>, character_event: &CharacterEvent) -> bool {
        // TODO(fabric2seer): `charTyped` is not a method of ControllerPopupWidget
        //   java: return controllerPopup.charTyped(characterEvent);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent keyEvent)
    pub fn key_pressed(__this: &compat::Shared<Self>, key_event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        return __this.lock().borrow().controller_popup.key_pressed(key_event);
    }

    /// java: @Override public void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        ();
        YACLScreen::tick(&compat::read(__this, |__t| __t.background_yacl_screen.clone()));
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        GuiUtils::set_screen(&compat::read(__this, |__t| __t.background_yacl_screen.clone()), true);
        __this.lock().borrow_mut().controller_popup.close();
    }

}

/// PopupControllerScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for PopupControllerScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// PopupControllerScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<PopupControllerScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| PopupControllerScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| PopupControllerScreen::reposition_elements(__h))
    }

    /// java: @Override public void tick()
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| PopupControllerScreen::tick(__h))
    }

    /// java: @Override public void extractRenderState(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| PopupControllerScreen::extract_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent keyEvent)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| PopupControllerScreen::key_pressed(__h, event))
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent characterEvent)
    fn char_typed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, c: char) -> bool {
        self.lend(screen, |__h| PopupControllerScreen::char_typed(__h, &CharacterEvent::new(c as i32)))
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |__h| PopupControllerScreen::mouse_clicked(__h, &MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0)), false))
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    fn mouse_scrolled(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        self.lend(screen, |__h| PopupControllerScreen::mouse_scrolled(__h, x, y, scroll_x, scroll_y))
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| PopupControllerScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): charTyped, extractBackground, extractRenderState, init, keyPressed, mouseClicked, mouseMoved, mouseScrolled, onClose, repositionElements, tick override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
