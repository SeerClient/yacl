//! Ported from dev.isxander.yacl3.gui.tab.TabExt (the 26.2 branch).

#[allow(unused_imports)]
use crate::prelude::*;
use compat::Graphics;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface TabExt extends Tab
///
/// `getTabExtraNarration()` (`Component.empty()`) is `compat::Tab`'s default already. `getLayout()` (a new
/// `FrameLayout`) has no counterpart: the YACL screen lays its tabs out itself (`compat::Tab::do_layout`).
pub trait TabExt: compat::Tab {
    /// java: @Nullable Tooltip getTooltip(); - the tooltip's text
    fn get_tooltip(&self) -> Option<MutableComponent>;

    /// java: default void tick()
    fn tick(&self) {}

    /// java: default void renderBackground(GuiGraphicsExtractor graphics)
    fn render_background(&self, graphics: &Graphics<'_>) {
        let _ = graphics;
    }
}
