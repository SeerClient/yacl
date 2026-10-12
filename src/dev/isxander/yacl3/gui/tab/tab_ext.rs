//! Ported from dev.isxander.yacl3.gui.tab.TabExt by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface TabExt extends Tab
pub trait TabExt: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: @Nullable Tooltip getTooltip(); (dispatch: the body for this class)
    fn get_tooltip(&self) -> fn(&compat::Shared<dyn TabExt>) -> Option<Tooltip>;

    /// java: default void tick() (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn TabExt>) {
        <dyn TabExt>::tick_body
    }

    /// java: default void renderBackground(GuiGraphicsExtractor graphics) (dispatch: the body for this class)
    fn render_background(&self) -> fn(&compat::Shared<dyn TabExt>, &GuiGraphicsExtractor) {
        <dyn TabExt>::render_background_body
    }

    /// java: @Override default @NotNull Component getTabExtraNarration() (dispatch: the body for this class)
    fn get_tab_extra_narration(&self) -> fn(&compat::Shared<dyn TabExt>) -> MutableComponent {
        <dyn TabExt>::get_tab_extra_narration_body
    }

    /// java: @Override default @NonNull Layout getLayout() (dispatch: the body for this class)
    fn get_layout(&self) -> fn(&compat::Shared<dyn TabExt>) -> LayoutElement {
        <dyn TabExt>::get_layout_body
    }

}

impl dyn TabExt {
    /// java: default void tick()
    pub fn tick_body(__this: &compat::Shared<dyn TabExt>) {
    }

    /// java: default void renderBackground(GuiGraphicsExtractor graphics)
    pub fn render_background_body(__this: &compat::Shared<dyn TabExt>, graphics: &GuiGraphicsExtractor) {
    }

    /// java: @Override default @NotNull Component getTabExtraNarration()
    pub fn get_tab_extra_narration_body(__this: &compat::Shared<dyn TabExt>) -> MutableComponent {
        return component::empty();
    }

    /// java: @Override default @NonNull Layout getLayout()
    pub fn get_layout_body(__this: &compat::Shared<dyn TabExt>) -> LayoutElement {
        return LayoutElement::Frame(FrameLayout::new());
    }

}
