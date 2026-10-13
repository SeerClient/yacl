//! Ported from dev.isxander.yacl3.gui.RequireRestartScreen.
//!
//! `extends ConfirmScreen`: the game's dialog (`confirm_screen::init_components`) in a mod screen whose two buttons
//! answer the callback YACL hands `super(..)`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::{OnPress, PressLambda};
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use std::cell::RefCell;

/// java: public class RequireRestartScreen extends ConfirmScreen
pub struct RequireRestartScreen {
    /// java: `ConfirmScreen.callback` - `option -> { if (option) Minecraft.getInstance().stop(); else
    /// GuiUtils.setScreen(parent); }`
    callback: compat::RenderThread<Rc<dyn Fn(bool)>>,
    /// java: `ConfirmScreen.title`
    title: FormattedText,
    /// java: `ConfirmScreen.message`
    message: FormattedText,
    /// java: `ConfirmScreen.yesButton`
    yes_button: FormattedText,
    /// java: `ConfirmScreen.noButton`
    no_button: FormattedText,
}

// SAFETY: the callback is reached only through `RenderThread::get`, which checks the thread; the texts are plain data.
unsafe impl Send for RequireRestartScreen {}

impl RequireRestartScreen {
    /// java: public RequireRestartScreen(Screen parent) - `parent` is the screen it was opened over
    /// (`GuiUtils.getCurrentScreen()`), which it returns to.
    pub fn new(parent: compat::ScreenRef) -> Self {
        let parent = RefCell::new(Some(parent));
        let callback: Rc<dyn Fn(bool)> = Rc::new(move |option: bool| {
            if option {
                // Minecraft.getInstance().stop();
                mcrust::net::minecraft::client::minecraft::stop();
            } else {
                // GuiUtils.setScreen(parent); - the screen below this one, or the reference it was handed
                let parent = match parent.borrow_mut().take() {
                    Some(compat::ScreenRef::Current) | None => compat::ScreenRef::Parent,
                    Some(other) => other,
                };
                GuiUtils::set_screen_1(parent);
            }
        });
        Self {
            callback: compat::RenderThread::new(callback),
            title: FormattedText::from(
                component::translatable("yacl.restart.title").with_style_formats(&[ChatFormatting::Red, ChatFormatting::Bold]),
            ),
            message: FormattedText::from(component::translatable("yacl.restart.message")),
            yes_button: FormattedText::from(component::translatable("yacl.restart.yes")),
            no_button: FormattedText::from(component::translatable("yacl.restart.no")),
        }
    }
}

impl ModScreen for RequireRestartScreen {
    fn get_title(&self) -> String {
        self.title.get_string()
    }

    /// java: ConfirmScreen.init - the dialog's widgets, its two buttons answering the callback
    fn init(&mut self, screen: &mut Screen) {
        let font = compat::font();
        let built = mcrust::net::minecraft::client::gui::screens::confirm_screen::init_components(
            screen.width,
            screen.height,
            &self.title,
            &self.message,
            &self.yes_button,
            &self.no_button,
            &font.splitter(),
        );
        let renderables: Vec<usize> = built.screen.renderables().to_vec();
        for (i, widget) in built.screen.children().iter().enumerate() {
            let mut widget = widget.clone();
            if i == built.yes_index || i == built.no_index {
                let answer = i == built.yes_index;
                let callback = compat::RenderThread::new(self.callback.get().clone());
                widget.on_press = OnPress::Lambda(PressLambda::new(move |_screen, _index| (callback.get())(answer)));
            }
            let index = screen.add_widget(widget);
            if renderables.contains(&i) {
                screen.mark_renderable(index);
            }
        }
    }

    /// java: ConfirmScreen.keyPressed - Escape answers `callback.accept(false)`
    fn key_pressed(&mut self, _screen: &mut Screen, event: &KeyEvent) -> bool {
        if event.is_escape() {
            (self.callback.get())(false);
            return true;
        }
        false
    }

    /// java: ConfirmScreen.shouldCloseOnEsc - `false`
    fn should_close_on_esc(&self) -> bool {
        false
    }

    fn class_name(&self) -> &'static str {
        "dev.isxander.yacl3.gui.RequireRestartScreen"
    }
}
