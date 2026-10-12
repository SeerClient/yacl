//! Ported from dev.isxander.yacl3.gui.tab.ScrollableNavigationBar by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::render::color_gradient_render_state::ColorGradientRenderState;
use crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::tabs::menu_tab_bar::MenuTabBar;
use mcrust::net::minecraft::client::gui::components::tabs::tab::Tab;
use mcrust::net::minecraft::client::gui::components::tabs::tab_manager::TabManager;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::layout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::util::mth;

/// java: private static final int NAVBAR_MARGIN = 28;
pub const NAVBAR_MARGIN: i32 = 28;
/// java: private static final Font font = Minecraft.getInstance().font;
pub static FONT: compat::MainThread<Font> = compat::MainThread::new(|| compat::font());

/// java: public class ScrollableNavigationBar extends MenuTabBar
pub struct ScrollableNavigationBar {
    /// java: extends net.minecraft.client.gui.components.tabs.MenuTabBar
    pub base: MenuTabBar,
    /// java: private int scrollOffset;
    pub scroll_offset: i32,
    /// java: private int maxScrollOffset;
    pub max_scroll_offset: i32,
    /// java: private final TabNavigationBarAccessor accessor;
    pub accessor: Option<crate::unresolved::TabNavigationBarAccessor>,
}

impl std::ops::Deref for ScrollableNavigationBar {
    type Target = MenuTabBar;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ScrollableNavigationBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ScrollableNavigationBar {
    /// java: public ScrollableNavigationBar(int width, TabManager tabManager, Iterable<? extends Tab> tabs)
    pub fn new(mut width: i32, mut tab_manager: TabManager, mut tabs: Vec<Box<dyn Tab>>) -> Self {
        let mut tabs_list = tabs.iter().cloned().collect::<Vec<_>>();
        // TODO(fabric2seer): `new MenuTabButton(..)`: the port has no `MenuTabButton::new`
        // rust: let mut tab_buttons_list = (tabs_list.iter().cloned().map(|tab| AbstractWidget::new(tab_manager.clone(), tab.clone(), 0, 24)).collect::<Vec<_>>()).iter().cloned().collect::<Vec<_>>();
        // TODO(fabric2seer): `new MenuTabBar(..)`: the port has no `MenuTabBar::new`
        //   java:
        let mut this: Self = todo!("constructor");
        // TODO(fabric2seer): super statement
        //   java: super(0, 0, width, 24, tabManager, tabButtonsList, tabsList);
        this.accessor = Some(this);
        // TODO(fabric2seer): `.yacl$getTabButtons(..)` on a TabNavigationBarAccessor
        //   java: for (TabButton tabButton : accessor.yacl$getTabButtons()) {
        //   java:             if (tabButton.tab() instanceof TabExt tab) {
        //   java:                 tabButton.setTooltip(tab.getTooltip());
        //   java:             }
        //   java:         }
        this
    }

    /// java: @Override public void arrangeElements(final int pWidth)
    pub fn arrange_elements(&mut self, mut p_width: i32) {
        let mut width: i32 = p_width;
        // TODO(fabric2seer): `MenuTabBar.layout` has no Rust field in the port (searched net.minecraft.client.gui.components.tabs.MenuTabBar and its supertypes)
        // rust: let layout = &mut self.layout;
        let mut layout: LayoutElement = todo!("fabric2seer: the initialiser of `layout` (TODO above)");
        // TODO(fabric2seer): `.yacl$getTabButtons(..)` on a TabNavigationBarAccessor
        //   java: ImmutableList<TabButton> tabButtons = accessor.yacl$getTabButtons();
        let mut tab_buttons: Vec<AbstractWidget> = todo!("fabric2seer: the initialiser of `tabButtons` (TODO above)");
        let mut no_scroll_width: i32 = width.wrapping_sub(NAVBAR_MARGIN.wrapping_mul(2));
        let mut all_tabs_width: i32 = 0;
        for tab_button in tab_buttons.iter_mut() {
            let mut button_width: i32 = (FONT.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::component_of(tab_button.get_message_component())))).wrapping_add(20);
            all_tabs_width = all_tabs_width.wrapping_add(button_width);
            tab_button.set_width(button_width);
        }
        if all_tabs_width < no_scroll_width {
            let mut equal_width: i32 = no_scroll_width.wrapping_div(tab_buttons.len() as i32);
            let mut small_tabs = tab_buttons.iter().cloned().filter(|btn| (btn.get_width() < equal_width)).collect::<Vec<_>>();
            let mut big_tabs = tab_buttons.iter().cloned().filter(|btn| (btn.get_width() >= equal_width)).collect::<Vec<_>>();
            let mut leftover_width: i32 = no_scroll_width.wrapping_sub(big_tabs.iter().cloned().map(|__a0| __a0.get_width()).sum::<i32>());
            let mut equal_width_for_small_tabs: i32 = leftover_width.wrapping_div(small_tabs.len() as i32);
            for tab_button in small_tabs.iter_mut() {
                tab_button.set_width(equal_width_for_small_tabs);
            }
            all_tabs_width = no_scroll_width;
        }
        // TODO(fabric2seer): `arrange_elements` takes 1 argument(s) in the port (`root: &mut LayoutElement`), Java passes 0
        // rust: layout::arrange_elements();
        layout.set_y(0);
        self.scroll_offset = 0;
        layout.set_x(((width.wrapping_sub(all_tabs_width)).wrapping_div(2)).max(NAVBAR_MARGIN));
        self.max_scroll_offset = 0i32.max(all_tabs_width.wrapping_sub(no_scroll_width));
    }

    /// java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let __pose_1 = gui_graphics_extractor::push_pose();
        // TODO(fabric2seer): `extract_header_separators` takes 2 argument(s) in the port (`gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft`), Java passes 4
        // rust: self.base.extract_header_separators(graphics, &mouse_x, mouse_y, a);
        // TODO(fabric2seer): `MenuTabBar.layout` has no Rust field in the port (searched net.minecraft.client.gui.components.tabs.MenuTabBar and its supertypes)
        // rust: let layout = &self.layout;
        let mut layout: LayoutElement = todo!("fabric2seer: the initialiser of `layout` (TODO above)");
        // TODO(fabric2seer): `AbstractContainerWidget::get_width` ports `getWidth`, but the port's `MenuTabBar` holds no `AbstractContainerWidget` (no Deref, no `base` field) to call it on
        // rust: let mut width: i32 = self.get_width();
        let mut width: i32 = todo!("fabric2seer: the initialiser of `width` (TODO above)");
        if self.scroll_offset < self.max_scroll_offset.wrapping_sub(NAVBAR_MARGIN) {
            let mut right: i32 = width;
            ({ let __r = &ColorGradientRenderState::create_horizontal(graphics, right.wrapping_sub(40), layout.get_y(), right, layout.get_y().wrapping_add(layout.get_height()), 0x00000000, 0xFF000000u32 as i32); let __f = compat::dispatch(__r, |__t| __t.submit()); __f(&compat::view(__r, |a| a as compat::Shared<dyn YACLGuiElementRenderState>), graphics) });
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("→"), right.wrapping_sub(10), layout.get_y().wrapping_add((layout.get_height().wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_div(2)), 0xFFFFFFFFu32 as i32, false);
        }
        if self.scroll_offset > NAVBAR_MARGIN {
            ({ let __r = &ColorGradientRenderState::create_horizontal(graphics, 0, layout.get_y(), 40, layout.get_y().wrapping_add(layout.get_height()), 0xFF000000u32 as i32, 0x00000000); let __f = compat::dispatch(__r, |__t| __t.submit()); __f(&compat::view(__r, |a| a as compat::Shared<dyn YACLGuiElementRenderState>), graphics) });
            gui_graphics_extractor::text_string_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), Some("←"), 5, layout.get_y().wrapping_add((layout.get_height().wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_div(2)), 0xFFFFFFFFu32 as i32, false);
        }
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    pub fn mouse_scrolled(&mut self, mut mouse_x: f64, mut mouse_y: f64, mut horizontal: f64, mut vertical: f64) -> bool {
        { let __s0 = (self.scroll_offset.wrapping_sub(((vertical * (15 as f64))) as i32)).wrapping_sub(((horizontal * (15 as f64))) as i32); self.set_scroll_offset(__s0) };
        return true;
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    pub fn is_mouse_over(&self, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        return (mouse_y <= (24 as f64));
    }

    /// java: public void setScrollOffset(int scrollOffset)
    pub fn set_scroll_offset(&mut self, mut scroll_offset: i32) {
        // TODO(fabric2seer): `MenuTabBar.layout` has no Rust field in the port (searched net.minecraft.client.gui.components.tabs.MenuTabBar and its supertypes)
        // rust: let layout = &mut self.layout;
        let mut layout: LayoutElement = todo!("fabric2seer: the initialiser of `layout` (TODO above)");
        layout.set_x(layout.get_x().wrapping_add(self.scroll_offset));
        self.scroll_offset = mth::clamp_i32(scroll_offset, 0, self.max_scroll_offset);
        layout.set_x(layout.get_x().wrapping_sub(self.scroll_offset));
    }

    /// java: public int getScrollOffset()
    pub fn get_scroll_offset(&self) -> i32 {
        return self.scroll_offset;
    }

    /// java: @Override public void setFocused(@Nullable GuiEventListener child)
    pub fn set_focused(&mut self, mut child: Option<Box<dyn GuiEventListener>>) {
        // TODO(fabric2seer): `MenuTabBar.setFocused` has no Rust method in the port (looked for `set_focused` in MenuTabBar and its supertypes)
        // rust: self.base.set_focused(child.as_ref().unwrap());
        if let Some(tab_button) = child.as_ref().unwrap().as_any().downcast_ref::<AbstractWidget>() {
            self.ensure_visible(tab_button);
        }
    }

    /// java: protected void ensureVisible(TabButton tabButton)
    pub fn ensure_visible(&mut self, tab_button: &AbstractWidget) {
        // TODO(fabric2seer): `AbstractContainerWidget::get_width` ports `getWidth`, but the port's `MenuTabBar` holds no `AbstractContainerWidget` (no Deref, no `base` field) to call it on
        // rust: let mut width: i32 = self.get_width();
        let mut width: i32 = todo!("fabric2seer: the initialiser of `width` (TODO above)");
        if tab_button.get_x() < NAVBAR_MARGIN {
            { let __s0 = self.scroll_offset.wrapping_sub(NAVBAR_MARGIN.wrapping_sub(tab_button.get_x())); self.set_scroll_offset(__s0) };
        } else if tab_button.get_x().wrapping_add(tab_button.get_width()) > width.wrapping_sub(NAVBAR_MARGIN) {
            { let __s0 = self.scroll_offset.wrapping_add((tab_button.get_x().wrapping_add(tab_button.get_width())).wrapping_sub(width.wrapping_sub(NAVBAR_MARGIN))); self.set_scroll_offset(__s0) };
        }
    }

    /// java: public void updateTabNames()
    pub fn update_tab_names(&self) {
        // TODO(fabric2seer): `.yacl$getTabButtons(..)` on a TabNavigationBarAccessor
        //   java: for (TabButton tabButton : accessor.yacl$getTabButtons()) {
        //   java:             tabButton.setMessage(tabButton.tab().getTabTitle());
        //   java:         }
    }

    /// java: public ImmutableList<Tab> getTabs()
    pub fn get_tabs(&self) -> Vec<Box<dyn Tab>> {
        // TODO(fabric2seer): `.yacl$getTabs(..)` on a TabNavigationBarAccessor
        //   java: return accessor.yacl$getTabs();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public TabManager getTabManager()
    pub fn get_tab_manager(&self) -> TabManager {
        // TODO(fabric2seer): `.yacl$getTabManager(..)` on a TabNavigationBarAccessor
        //   java: return accessor.yacl$getTabManager();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

// NOTE(fabric2seer): arrangeElements, extractWidgetRenderState, isMouseOver, mouseScrolled, setFocused override(s) of MenuTabBar - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
