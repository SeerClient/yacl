//! Ported from dev.isxander.yacl3.gui.tab.ScrollableNavigationBar (the 26.2 branch: `extends MenuTabBar`).
//!
//! The `MenuTabBar` / `TabNavigationBar` half is `compat::TabNavigationBar` (the YACL screen draws its own widgets);
//! `TabNavigationBarAccessor`'s three getters are its fields.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::render::color_gradient_render_state::ColorGradientRenderState;
use crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState as _;
use crate::dev::isxander::yacl3::gui::tab::tab_ext::TabExt;
use compat::{Graphics, GuiEventListener, MenuTabButton, Tab, TabManager, TabNavigationBar};
use mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::util::mth;
use std::cell::Cell;
use std::sync::Weak;

/// java: private static final int NAVBAR_MARGIN = 28;
const NAVBAR_MARGIN: i32 = 28;

/// java: public class ScrollableNavigationBar extends MenuTabBar
pub struct ScrollableNavigationBar {
    /// java: extends MenuTabBar
    pub bar: TabNavigationBar,
    /// java: private int scrollOffset;
    scroll_offset: Cell<i32>,
    /// java: private int maxScrollOffset;
    max_scroll_offset: Cell<i32>,
    /// `this`, for the `setFocused(GuiEventListener)` the base bodies call
    this: Weak<ScrollableNavigationBar>,
}

impl ScrollableNavigationBar {
    /// java: public ScrollableNavigationBar(int width, TabManager tabManager, Iterable<? extends Tab> tabs)
    pub fn new(width: i32, tab_manager: Rc<TabManager>, tabs: Vec<Arc<dyn Tab>>) -> Arc<ScrollableNavigationBar> {
        let tab_buttons: Vec<Arc<MenuTabButton>> =
            tabs.iter().map(|tab| Arc::new(MenuTabButton::new(tab_manager.clone(), tab.clone(), 0, 24))).collect();
        let bar = Arc::new_cyclic(|this| ScrollableNavigationBar {
            // super(0, 0, width, 24, tabManager, tabButtonsList, tabsList);
            bar: TabNavigationBar::new(0, 0, width, 24, tab_manager, tab_buttons, tabs),
            scroll_offset: Cell::new(0),
            max_scroll_offset: Cell::new(0),
            this: this.clone(),
        });

        // add tab tooltips to the tab buttons
        for tab_button in &bar.bar.tab_buttons {
            if let Some(tab) = tab_button.tab().as_tab_ext() {
                tab_button.fields.set_tooltip(tab.get_tooltip());
            }
        }
        bar
    }

    /// java: @Override public void arrangeElements(final int pWidth)
    pub fn arrange_elements(&self, width: i32) {
        let tab_buttons = &self.bar.tab_buttons;
        let no_scroll_width = width - NAVBAR_MARGIN * 2;

        let font = compat::font();
        let mut all_tabs_width = 0;
        // first pass: set the width of each tab button
        for tab_button in tab_buttons {
            let button_width = font.width_of(&FormattedText::from(tab_button.fields.get_message())) + 20;
            all_tabs_width += button_width;
            tab_button.fields.set_width(button_width);
        }

        if all_tabs_width < no_scroll_width {
            let equal_width = no_scroll_width / tab_buttons.len() as i32;
            let small_tabs: Vec<_> = tab_buttons.iter().filter(|btn| btn.fields.get_width() < equal_width).cloned().collect();
            let big_tabs: Vec<_> = tab_buttons.iter().filter(|btn| btn.fields.get_width() >= equal_width).cloned().collect();
            let leftover_width = no_scroll_width - big_tabs.iter().map(|btn| btn.fields.get_width()).sum::<i32>();
            // `leftoverWidth / smallTabs.size()` throws ArithmeticException when there are no small tabs
            let equal_width_for_small_tabs = leftover_width / small_tabs.len() as i32;
            for tab_button in &small_tabs {
                tab_button.fields.set_width(equal_width_for_small_tabs);
            }

            all_tabs_width = no_scroll_width;
        }

        self.bar.layout_arrange_elements();
        self.bar.layout_set_y(0);
        self.scroll_offset.set(0);

        self.bar.layout_set_x(((width - all_tabs_width) / 2).max(NAVBAR_MARGIN));
        self.max_scroll_offset.set(0.max(all_tabs_width - no_scroll_width));
    }

    /// java: public void setScrollOffset(int scrollOffset)
    pub fn set_scroll_offset(&self, scroll_offset: i32) {
        self.bar.layout_set_x(self.bar.layout_x() + self.scroll_offset.get());
        self.scroll_offset.set(mth::clamp_i32(scroll_offset, 0, self.max_scroll_offset.get()));
        self.bar.layout_set_x(self.bar.layout_x() - self.scroll_offset.get());
    }

    /// java: public int getScrollOffset()
    pub fn get_scroll_offset(&self) -> i32 {
        self.scroll_offset.get()
    }

    /// java: @Override public void setFocused(@Nullable GuiEventListener child)
    pub fn set_focused_child(&self, child: Option<Arc<dyn GuiEventListener>>) {
        self.bar.set_focused_child(child.clone());
        if let Some(child) = child
            && let Some(tab_button) = self.bar.button_of(&child)
        {
            self.ensure_visible(&tab_button);
        }
    }

    /// java: protected void ensureVisible(TabButton tabButton)
    fn ensure_visible(&self, tab_button: &MenuTabButton) {
        let width = self.bar.fields.get_width();
        let (x, button_width) = (tab_button.fields.get_x(), tab_button.fields.get_width());

        if x < NAVBAR_MARGIN {
            self.set_scroll_offset(self.scroll_offset.get() - (NAVBAR_MARGIN - x));
        } else if x + button_width > width - NAVBAR_MARGIN {
            self.set_scroll_offset(self.scroll_offset.get() + (x + button_width - (width - NAVBAR_MARGIN)));
        }
    }

    /// java: public void updateTabNames()
    pub fn update_tab_names(&self) {
        for tab_button in &self.bar.tab_buttons {
            tab_button.fields.set_message(tab_button.tab().get_tab_title());
        }
    }

    /// java: public ImmutableList<Tab> getTabs()
    pub fn get_tabs(&self) -> Vec<Arc<dyn Tab>> {
        self.bar.get_tabs().to_vec()
    }

    /// java: public TabManager getTabManager()
    pub fn get_tab_manager(&self) -> Rc<TabManager> {
        self.bar.tab_manager()
    }

    /// java: TabNavigationBar.selectTab(int index, boolean playSound)
    pub fn select_tab(&self, index: usize, play_sound: bool) {
        let this = self.this.clone();
        self.bar.select_tab(index, play_sound, &move |child| {
            if let Some(this) = this.upgrade() {
                this.set_focused_child(child);
            }
        });
    }

    /// `this.setFocused(child)` as the base bodies call it
    fn focus_setter(&self) -> impl Fn(Option<Arc<dyn GuiEventListener>>) + 'static {
        let this = self.this.clone();
        move |child| {
            if let Some(this) = this.upgrade() {
                this.set_focused_child(child);
            }
        }
    }
}

impl GuiEventListener for ScrollableNavigationBar {
    /// `AbstractWidget.extractRenderState` -> java: @Override public void extractWidgetRenderState(GuiGraphicsExtractor
    /// graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        if !self.bar.fields.visible.get() {
            return;
        }
        self.bar.fields.update_hover(mouse_x, mouse_y);

        graphics.push_pose();
        // render option list BELOW the navbar without need to scissor

        // super.extractWidgetRenderState(graphics, mouseX, mouseY, a);
        self.bar.extract_menu_tab_bar(graphics, mouse_x, mouse_y, a);

        let layout_y = self.bar.layout_y();
        let layout_height = self.bar.layout_height();
        let width = self.bar.fields.get_width();
        // draw right fade
        if self.scroll_offset.get() < self.max_scroll_offset.get() - NAVBAR_MARGIN {
            let right = width;
            ColorGradientRenderState::create_horizontal(
                &graphics.extractor,
                right - 40,
                layout_y,
                right,
                layout_y + layout_height,
                0x00000000,
                0xFF000000u32 as i32,
            )
            .submit(&graphics.extractor);

            graphics.text_str("\u{2192}", right - 10, layout_y + (layout_height - LINE_HEIGHT) / 2, 0xFFFFFFFFu32 as i32, false);
        }

        // draw left fade
        if self.scroll_offset.get() > NAVBAR_MARGIN {
            ColorGradientRenderState::create_horizontal(
                &graphics.extractor,
                0,
                layout_y,
                40,
                layout_y + layout_height,
                0xFF000000u32 as i32,
                0x00000000,
            )
            .submit(&graphics.extractor);

            graphics.text_str("\u{2190}", 5, layout_y + (layout_height - LINE_HEIGHT) / 2, 0xFFFFFFFFu32 as i32, false);
        }

        graphics.pop_pose();
    }

    /// `AbstractContainerWidget.mouseClicked` - `updateScrolling` (the bar's content is its own height: never
    /// scrollable) `|| ContainerEventHandler.super.mouseClicked`
    fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        let set_focused = self.focus_setter();
        self.bar.container.mouse_clicked_with(&self.bar.children(), event, double_click, &set_focused)
    }

    /// `AbstractContainerWidget.mouseReleased`
    fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
        self.bar.container.mouse_released(event)
    }

    /// `AbstractContainerWidget.mouseDragged`
    fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        self.bar.container.mouse_dragged(event, dx, dy)
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    fn mouse_scrolled(&self, _mouse_x: f64, _mouse_y: f64, horizontal: f64, vertical: f64) -> bool {
        self.set_scroll_offset(self.scroll_offset.get() - (vertical * 15.0) as i32 - (horizontal * 15.0) as i32);
        true
    }

    /// `TabNavigationBar.keyPressed` - Ctrl+digit / Ctrl+Tab
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        let set_focused = self.focus_setter();
        self.bar.key_pressed(event, &set_focused)
    }

    fn char_typed(&self, event: &CharacterEvent) -> bool {
        self.bar.container.char_typed(event)
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    fn is_mouse_over(&self, _mouse_x: f64, mouse_y: f64) -> bool {
        mouse_y <= 24.0
    }

    /// `TabNavigationBar.setFocused(boolean)` - `super.setFocused(focused)`, then any focused button is let go
    fn set_focused(&self, focused: bool) {
        if !focused {
            self.set_focused_child(None);
        }
        if self.bar.container.get_focused().is_some() {
            self.set_focused_child(None);
        }
    }

    /// `ContainerEventHandler.isFocused()` - `getFocused() != null`
    fn is_focused(&self) -> bool {
        self.bar.container.is_focused()
    }
}
