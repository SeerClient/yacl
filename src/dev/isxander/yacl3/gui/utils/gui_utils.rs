//! Ported from dev.isxander.yacl3.gui.utils.GuiUtils (the 26.2-fabric branch).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::net::minecraft::client::gui::components::widget_sprites::WidgetSprites;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::util::formatted_char_sequence::{FormattedCharSequence, Seq};
use mcrust::net::minecraft::util::formatted_char_sink::FormattedCharSink;
use std::sync::LazyLock;

/// java: public static final WidgetSprites BUTTON_SPRITES = new WidgetSprites(mcRl("widget/button") /* normal */,
/// mcRl("widget/button_disabled") /* disabled & !focused */, mcRl("widget/button_highlighted") /* !disabled & focused */,
/// mcRl("widget/slider_highlighted") /* disabled & focused */);
pub static BUTTON_SPRITES: LazyLock<WidgetSprites> = LazyLock::new(|| {
    WidgetSprites::new(
        YACLPlatform::mc_rl("widget/button"),
        YACLPlatform::mc_rl("widget/button_disabled"),
        YACLPlatform::mc_rl("widget/button_highlighted"),
        YACLPlatform::mc_rl("widget/slider_highlighted"),
    )
});

/// java: public class GuiUtils
pub struct GuiUtils;

/// `output -> seq.accept((c, s, t) -> output.accept(c, style, t))`
struct OverrideStyle {
    seq: Seq,
    style: Style,
}

impl FormattedCharSequence for OverrideStyle {
    fn accept(&self, output: &mut dyn FormattedCharSink) -> bool {
        let style = &self.style;
        self.seq.accept(&mut |c: i32, _s: &Style, t: u32| output.accept(c, style, t))
    }
}

impl GuiUtils {
    /// `GuiUtils.BUTTON_SPRITES`
    pub fn button_sprites() -> &'static WidgetSprites {
        &BUTTON_SPRITES
    }

    /// java: public static MutableComponent translatableFallback(String key, Component fallback)
    pub fn translatable_fallback(key: &str, fallback: &MutableComponent) -> MutableComponent {
        if language::has(key) {
            return component::translatable(key);
        }
        fallback.copy()
    }

    /// java: public static String shortenString(String string, Font font, int maxWidth, String suffix)
    pub fn shorten_string(string: String, font: &Font<'_>, max_width: i32, suffix: &str) -> String {
        let mut string = string;
        if string.is_empty() {
            return string;
        }

        let mut first_iter = true;
        while font.width(&string) > max_width {
            let end = (compat::str_len(&string) - 1 - (if first_iter { 1 } else { compat::str_len(suffix) + 1 })).max(0);
            string = compat::trim(&compat::substring2(&string, 0, end));
            string.push_str(suffix);

            if string == suffix {
                break;
            }

            first_iter = false;
        }

        string
    }

    /// java: public static FormattedCharSequence overrideStyle(FormattedCharSequence seq, Style style)
    pub fn override_style(seq: &Seq, style: &Style) -> Seq {
        Arc::new(OverrideStyle { seq: seq.clone(), style: style.clone() })
    }

    /// java: public static void setScreen(Screen screen, boolean ignoreVanilla)
    ///
    /// `ignoreVanilla` is `((GuiAccessor) minecraft.gui).yacl$setScreen(screen)`: the field write without
    /// `Gui.setScreen`'s `removed()` / `init` / `added()`. The port's screen stack has no such write
    /// (`mixins::gui_accessor`); the request goes through `Gui.setScreen` (GAPS_Y2.md G6).
    pub fn set_screen(screen: compat::ScreenRef, ignore_vanilla: bool) {
        if ignore_vanilla {
            crate::mixins::gui_accessor::yacl_set_screen(screen);
        } else {
            screen.set();
        }
    }

    /// java: public static void setScreen(Screen screen)
    pub fn set_screen_1(screen: compat::ScreenRef) {
        GuiUtils::set_screen(screen, false);
    }

    /// java: public static Screen getCurrentScreen() - `Minecraft.getInstance().gui.screen()`, as the reference a new
    /// screen keeps for its parent.
    pub fn get_current_screen() -> compat::ScreenRef {
        compat::ScreenRef::Current
    }
}
