//! Ported from dev.isxander.yacl3.gui.utils.GuiUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public static final WidgetSprites BUTTON_SPRITES = new WidgetSprites( YACLPlatform.mcRl("widget/button"), // normal YACLPlatform.mcRl("widget/button_disabled"), // disabled & !focused YACLPlatform.mcR
pub static BUTTON_SPRITES: compat::MainThread<WidgetSprites> = compat::MainThread::new(|| WidgetSprites::new(YACLPlatform::mc_rl("widget/button"), YACLPlatform::mc_rl("widget/button_disabled"), YACLPlatform::mc_rl("widget/button_highlighted"), YACLPlatform::mc_rl("widget/slider_highlighted")));

/// java: public class GuiUtils
#[derive(Clone)]
pub struct GuiUtils {}

impl GuiUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static MutableComponent translatableFallback(String key, Component fallback)
    pub fn translatable_fallback(key: &str, fallback: &MutableComponent) -> MutableComponent {
        if language::get_instance().has(key) {
            return component::translatable(key);
        }
        return fallback.copy();
    }

    /// java: public static String shortenString(String string, Font font, int maxWidth, String suffix)
    pub fn shorten_string(mut string: String, font: &Font, mut max_width: i32, mut suffix: String) -> String {
        if string.is_empty() {
            return string;
        }
        let mut first_iter: bool = true;
        while font.width(&string) > max_width {
            string = compat::trim(&(compat::substring2(&string, 0, (((compat::str_len(&string)).wrapping_sub(1)).wrapping_sub(if first_iter { 1 } else { (compat::str_len(&suffix)).wrapping_add(1) })).max(0))));
            string.push_str(&suffix.to_string());
            if string == suffix {
                break;
            }
            first_iter = false;
        }
        return string;
    }

    /// java: public static FormattedCharSequence overrideStyle(FormattedCharSequence seq, Style style)
    pub fn override_style(seq: &Seq, style: &Style) -> Seq {
        // TODO(fabric2seer): `.accept(..)` on a value of unknown type
        //   java: return output -> seq.accept((c, s, t) -> output.accept(c, style, t));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static void setScreen(Screen screen, boolean ignoreVanilla)
    pub fn set_screen(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, mut ignore_vanilla: bool) {
        if ignore_vanilla {
            compat::minecraft().gui.screen;
        } else {
            // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
            // rust: gui::set_screen(&mut screen);
        }
    }

    /// java: public static void setScreen(Screen screen)
    pub fn set_screen_1(screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        GuiUtils::set_screen(screen, false);
    }

    /// java: public static Screen getCurrentScreen()
    pub fn get_current_screen() -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        return compat::minecraft().gui.screen().unwrap();
    }

}
