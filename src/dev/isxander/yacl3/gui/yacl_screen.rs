//! Ported from dev.isxander.yacl3.gui.YACLScreen (the 26.2 branch).
//!
//! `public class YACLScreen extends Screen`. The YACL screen is callback-heavy (an option's listener reaches back into
//! the screen and its widgets while a widget of it is handling an event), so it is an `Arc<YACLScreen>` with its state
//! in cells (PATTERNS_Y2.md P1), and it draws and dispatches its own widgets (P3): the game's screen stack holds a
//! thin `ModScreen` adapter (`YACLScreenAdapter`) that forwards every event and frame to it, and the game's widget tree
//! holds only `compat::CursorSink`. The `Screen` half Java inherits - the renderable list, `children()`, the focused
//! child and the drag - is `renderables` and `container` here, with `Screen` / `AbstractContainerEventHandler`'s bodies.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::option_event_listener::Event;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::placeholder_category::PlaceholderCategory;
use crate::dev::isxander::yacl3::api::utils::dimension::{self, Dimension};
use crate::dev::isxander::yacl3::api::utils::mutable_dimension::MutableDimension;
use crate::dev::isxander::yacl3::api::utils::option_utils::OptionUtils;
use crate::dev::isxander::yacl3::api::yet_another_config_lib::YetAnotherConfigLib;
use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::ControllerPopupWidget;
use crate::dev::isxander::yacl3::gui::controllers::popup_controller_screen::PopupControllerScreen;
use crate::dev::isxander::yacl3::gui::option_description_widget::OptionDescriptionWidget;
use crate::dev::isxander::yacl3::gui::option_list_widget::OptionListWidget;
use crate::dev::isxander::yacl3::gui::search_field_widget::SearchFieldWidget;
use crate::dev::isxander::yacl3::gui::tab::scrollable_navigation_bar::ScrollableNavigationBar;
use crate::dev::isxander::yacl3::gui::tab::tab_ext::TabExt;
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;
use crate::dev::isxander::yacl3::gui::utils::key_utils::KeyUtils;
use crate::dev::isxander::yacl3::gui::widget_and_type::WidgetAndType;
use crate::dev::isxander::yacl3::gui::yacl_selection_list as selection_list;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER;
use compat::{Button, ContainerEventHandler, Graphics, GuiEventListener, Tab, TabManager};
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::mcrust::modding::screens::{ModScreen, ScreenRenderContext};
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::multi_line_label::MultiLineLabel;
use mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::screens::screen::Screen;
use mcrust::net::minecraft::client::gui::text_alignment::TextAlignment;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::common_components;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use std::cell::{Cell, RefCell};
use std::sync::Weak;

/// `a == b` for two listeners (identity)
fn same_listener(a: &Arc<dyn GuiEventListener>, b: &Arc<dyn GuiEventListener>) -> bool {
    std::ptr::eq(Arc::as_ptr(a) as *const (), Arc::as_ptr(b) as *const ())
}

/// `Screen` held by the constructor (`private final Screen parent`) - where `onClose` returns.
enum Parent {
    /// `null` - `setScreen(null)`
    None,
    /// the screen that was up when this one opened (it is kept below this one)
    Below,
    /// a new screen object the caller built, set when this one closes
    Mod(RefCell<Option<Box<dyn ModScreen>>>),
}

/// java: public class YACLScreen extends Screen
pub struct YACLScreen {
    /// java: public final YetAnotherConfigLib config;
    pub config: Arc<dyn YetAnotherConfigLib>,
    /// java: private final Screen parent;
    parent: Parent,
    /// `Screen.title` - `config.title()`
    pub title: MutableComponent,
    /// java: public final TabManager tabManager = new TabManager(this::addRenderableWidget, this::removeWidget);
    pub tab_manager: Rc<TabManager>,
    /// java: public ScrollableNavigationBar tabNavigationBar;
    pub tab_navigation_bar: RefCell<Option<Arc<ScrollableNavigationBar>>>,
    /// java: public ScreenRectangle tabArea;
    pub tab_area: Cell<ScreenRectangle>,
    /// java: public Component saveButtonMessage;
    pub save_button_message: RefCell<Option<MutableComponent>>,
    /// java: public Tooltip saveButtonTooltipMessage;
    pub save_button_tooltip_message: RefCell<Option<MutableComponent>>,
    /// java: private int saveButtonMessageTime;
    save_button_message_time: Cell<i32>,
    /// java: private boolean pendingChanges;
    pending_changes: Cell<bool>,
    /// java: public ControllerPopupWidget<?> currentPopupController = null;
    pub current_popup_controller: RefCell<Option<Arc<dyn ControllerPopupWidget>>>,
    /// java: public boolean popupControllerVisible = false;
    pub popup_controller_visible: Cell<bool>,
    /// java: private @Nullable CategoryTab preferredTab = null; - the tab where the user started searching
    preferred_tab: RefCell<Option<Weak<CategoryTab>>>,
    /// `GuiUtils.getCurrentScreen() instanceof PopupControllerScreen`: the popup screen this one put up and has not
    /// taken down. Java's `setScreen` is immediate; the port parks the request for the client loop, so the screen this
    /// asks about is the one requested last.
    pub(crate) popup_screen_up: Cell<bool>,
    /// `Screen.width` / `Screen.height`
    width: Cell<i32>,
    height: Cell<i32>,
    /// `Screen.renderables` / `Screen.children` (every child here is both)
    renderables: RefCell<Vec<Arc<dyn GuiEventListener>>>,
    /// `AbstractContainerEventHandler.focused` / `isDragging`
    container: ContainerEventHandler,
    /// `this`
    this: Weak<YACLScreen>,
}

impl YACLScreen {
    /// java: public YACLScreen(YetAnotherConfigLib config, Screen parent)
    fn new(config: Arc<dyn YetAnotherConfigLib>, parent: Parent) -> Arc<YACLScreen> {
        let screen = Arc::new_cyclic(|this: &Weak<YACLScreen>| {
            let (add, remove) = (this.clone(), this.clone());
            YACLScreen {
                title: config.title(),
                config: config.clone(),
                parent,
                tab_manager: Rc::new(TabManager::new(
                    move |widget| {
                        if let Some(screen) = add.upgrade() {
                            screen.add_renderable_widget(widget);
                        }
                    },
                    move |widget| {
                        if let Some(screen) = remove.upgrade() {
                            screen.remove_widget(&widget);
                        }
                    },
                )),
                tab_navigation_bar: RefCell::new(None),
                tab_area: Cell::new(ScreenRectangle::new_at(0, 0, 0, 0)),
                save_button_message: RefCell::new(None),
                save_button_tooltip_message: RefCell::new(None),
                save_button_message_time: Cell::new(0),
                pending_changes: Cell::new(false),
                current_popup_controller: RefCell::new(None),
                popup_controller_visible: Cell::new(false),
                preferred_tab: RefCell::new(None),
                popup_screen_up: Cell::new(false),
                width: Cell::new(0),
                height: Cell::new(0),
                renderables: RefCell::new(Vec::new()),
                container: ContainerEventHandler::new(),
                this: this.clone(),
            }
        });

        // OptionUtils.forEachOptions(config, option -> option.addEventListener((opt, event) -> {
        //     if (event != OptionEventListener.Event.INITIAL) onOptionChanged(opt); }));
        OptionUtils::for_each_options(&*config, |option| {
            let weak = Arc::downgrade(&screen);
            option.add_any_event_listener(Arc::new(move |opt: &dyn AnyOption, event: Event| {
                if event != Event::Initial
                    && let Some(screen) = weak.upgrade()
                {
                    screen.on_option_changed(opt);
                }
            }));
        });
        screen
    }

    /// `new YACLScreen(config, parent)` as `YetAnotherConfigLib.generateScreen(parent)` hands it to
    /// `Minecraft.setScreen(..)`: the screen in its `ModScreen` adapter. `parent` is the screen it returns to on close
    /// (`None` for Java's `null`).
    pub fn new_screen(config: Arc<dyn YetAnotherConfigLib>, parent: Option<compat::ScreenRef>) -> compat::ScreenRef {
        let (parent, keep_parent) = match parent {
            None | Some(compat::ScreenRef::Null) => (Parent::None, false),
            // the screen that is up (or the one it returns to) stays below this one
            Some(compat::ScreenRef::Current) | Some(compat::ScreenRef::Parent) => (Parent::Below, true),
            Some(compat::ScreenRef::Mod { screen, .. }) => (Parent::Mod(RefCell::new(Some(screen))), false),
        };
        let screen = YACLScreen::new(config, parent);
        compat::ScreenRef::of_box(Box::new(YACLScreenAdapter::new(screen)), keep_parent)
    }

    /// `this`
    pub fn this(&self) -> Arc<YACLScreen> {
        self.this.upgrade().expect("YACLScreen.this")
    }

    /// `Screen.width`
    pub fn width(&self) -> i32 {
        self.width.get()
    }

    /// `Screen.height`
    pub fn height(&self) -> i32 {
        self.height.get()
    }

    /// `tabArea`
    pub fn tab_area(&self) -> ScreenRectangle {
        self.tab_area.get()
    }

    /// `tabNavigationBar`
    pub fn tab_navigation_bar(&self) -> Arc<ScrollableNavigationBar> {
        self.tab_navigation_bar.borrow().clone().expect("YACLScreen.tabNavigationBar before init")
    }

    // ---- Screen ---------------------------------------------------------------------------------------------------

    /// `Screen.addRenderableWidget(widget)`
    pub fn add_renderable_widget(&self, widget: Arc<dyn GuiEventListener>) {
        self.renderables.borrow_mut().push(widget);
    }

    /// `Screen.removeWidget(listener)`
    pub fn remove_widget(&self, widget: &Arc<dyn GuiEventListener>) {
        self.renderables.borrow_mut().retain(|w| !same_listener(w, widget));
        if self.container.get_focused().is_some_and(|focused| same_listener(&focused, widget)) {
            self.container.set_focused_field(None);
        }
    }

    /// `Screen.clearWidgets()`
    fn clear_widgets(&self) {
        self.renderables.borrow_mut().clear();
    }

    /// `Screen.clearFocus()` - `setFocused(null)`... with no `setFocused(false)` on a child already removed
    fn clear_focus(&self) {
        self.set_focused_child(None);
    }

    /// `children()`
    pub fn children(&self) -> Vec<Arc<dyn GuiEventListener>> {
        self.renderables.borrow().clone()
    }

    /// `getFocused()`
    pub fn get_focused(&self) -> Option<Arc<dyn GuiEventListener>> {
        self.container.get_focused()
    }

    /// `Screen.init(width, height)` / `rebuildWidgets()`: the size, then (on a screen already initialized) the widgets
    /// cleared and the focus dropped, then `init()`. `setInitialFocus()` focuses nothing for a mouse user.
    fn init_size(&self, width: i32, height: i32) {
        self.width.set(width);
        self.height.set(height);
        self.clear_widgets();
        self.clear_focus();
        self.init();
    }

    /// `Screen.extractRenderState` - every renderable, in order
    pub fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let mut index = 0;
        loop {
            let renderable = self.renderables.borrow().get(index).cloned();
            let Some(renderable) = renderable else {
                break;
            };
            renderable.extract_render_state(graphics, mouse_x, mouse_y, a);
            index += 1;
        }
    }

    /// `Screen.keyPressed:120-151` - Escape closes (when `shouldCloseOnEsc`), else the focused child's
    /// (`super.keyPressed`). The Tab / arrow focus walk that follows is not here (GAPS_Y2.md G7).
    pub fn key_pressed(&self, event: &KeyEvent) -> bool {
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        self.container.key_pressed(event)
    }

    /// `ContainerEventHandler.charTyped` - the focused child's
    pub fn char_typed(&self, event: &CharacterEvent) -> bool {
        self.container.char_typed(event)
    }

    /// `ContainerEventHandler.mouseReleased`
    pub fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
        self.container.mouse_released(event)
    }

    /// `ContainerEventHandler.mouseScrolled` - the child under the cursor
    pub fn mouse_scrolled(&self, mouse_x: f64, mouse_y: f64, horizontal: f64, vertical: f64) -> bool {
        ContainerEventHandler::mouse_scrolled(&self.children(), mouse_x, mouse_y, horizontal, vertical)
    }

    // ---- YACLScreen -----------------------------------------------------------------------------------------------

    /// java: @Override protected void init()
    fn init(&self) {
        let this = self.this();
        let tab_area = ScreenRectangle::new_at(0, 24 - 1, self.width(), self.height() - 24 + 1);
        self.tab_area.set(tab_area);

        let current_tab = match (self.tab_navigation_bar.borrow().as_ref(), self.tab_manager.get_current_tab()) {
            (Some(bar), Some(current)) => bar
                .get_tabs()
                .iter()
                .position(|tab| compat::tabs::same_tab(tab, &current))
                .map_or(-1, |index| index as i32),
            _ => 0,
        };
        let current_tab = if current_tab == -1 { 0 } else { current_tab };

        let tabs: Vec<Arc<dyn Tab>> = self
            .config
            .categories()
            .into_iter()
            .map(|category| -> Arc<dyn Tab> {
                if let Some(tab_provider) = category.as_custom_tab_provider() {
                    return tab_provider.create_tab(&this, tab_area);
                }
                if category.as_placeholder_category().is_some() {
                    return PlaceholderTab::new(category.clone(), &this);
                }
                CategoryTab::new(&this, category, tab_area)
            })
            .collect();
        let tab_navigation_bar = ScrollableNavigationBar::new(self.width(), self.tab_manager.clone(), tabs);
        *self.tab_navigation_bar.borrow_mut() = Some(tab_navigation_bar.clone());
        tab_navigation_bar.select_tab(current_tab as usize, false);
        tab_navigation_bar.arrange_elements(tab_area.width());
        self.tab_manager.set_tab_area(tab_area);
        self.add_renderable_widget(tab_navigation_bar);

        (self.config.init_consumer())(&this);
    }

    /// java: public void addPopupControllerWidget(ControllerPopupWidget<?> controllerPopupWidget)
    pub fn add_popup_controller_widget(&self, controller_popup_widget: Arc<dyn ControllerPopupWidget>) {
        // Safety check for the color picker
        if self.current_popup_controller.borrow().is_some() {
            self.clear_popup_controller_widget();
        }

        *self.current_popup_controller.borrow_mut() = Some(controller_popup_widget.clone());
        self.popup_controller_visible.set(true);

        let option_list_widget = self.current_category_tab().map(|category_tab| category_tab.option_list.get_type());
        if option_list_widget.is_some() {
            self.popup_screen_up.set(true);
            GuiUtils::set_screen_1(PopupControllerScreen::new_screen(&self.this(), controller_popup_widget));
        }
    }

    /// java: public void clearPopupControllerWidget()
    pub fn clear_popup_controller_widget(&self) {
        if self.popup_screen_up.get() {
            let popup = self.current_popup_controller.borrow().clone();
            if let Some(popup) = popup {
                PopupControllerScreen::on_close_over(&self.this(), &popup);
            }
        }
        self.popup_controller_visible.set(false);
        *self.current_popup_controller.borrow_mut() = None;
    }

    /// `tabManager.getCurrentTab() instanceof CategoryTab categoryTab`
    fn current_category_tab(&self) -> Option<Arc<CategoryTab>> {
        let current = self.tab_manager.get_current_tab()?;
        current.dyn_any().downcast_ref::<CategoryTab>().and_then(|tab| tab.this.upgrade())
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor guiGraphics, int mouseX, int mouseY, float
    /// partialTick) - the game has drawn `super.extractBackground` (the screen background) already
    pub fn extract_background(&self, graphics: &Graphics<'_>, _mouse_x: i32, _mouse_y: i32, _partial_tick: f32) {
        if let Some(current) = self.tab_manager.get_current_tab()
            && let Some(tab) = current.as_tab_ext()
        {
            tab.render_background(graphics);
        }
    }

    /// java: public void finishOrSave()
    pub fn finish_or_save(&self) {
        *self.save_button_message.borrow_mut() = None;

        if self.pending_changes() {
            let mut flags: Vec<Arc<dyn OptionFlag>> = Vec::new();
            OptionUtils::for_each_options(&*self.config, |option| {
                if option.apply_value() {
                    for flag in option.flags() {
                        if !flags.iter().any(|f| std::ptr::eq(Arc::as_ptr(f) as *const (), Arc::as_ptr(&flag) as *const ())) {
                            flags.push(flag);
                        }
                    }
                }
            });
            OptionUtils::for_each_options(&*self.config, |option| {
                if option.changed() {
                    // if still changed after applying, reset to the current value from binding as something has gone
                    // wrong.
                    option.forget_pending_value();
                    LOGGER.error(&format!("Option '{}' value mismatch after applying! Reset to binding's getter.", option.name().get_string()));
                }
            });
            (self.config.save_function())();

            for flag in flags {
                flag.accept(compat::minecraft());
            }

            self.pending_changes.set(false);
            if let Some(category_tab) = self.current_category_tab() {
                category_tab.update_buttons();
            }
        } else {
            self.on_close();
        }
    }

    /// java: public void cancelOrReset()
    pub fn cancel_or_reset(&self) {
        if self.pending_changes() {
            // if pending changes, button acts as a cancel button
            OptionUtils::for_each_options(&*self.config, |option| option.forget_pending_value());
            self.on_close();
        } else {
            // if not, button acts as a reset button
            OptionUtils::for_each_options(&*self.config, |option| option.request_set_default());
        }
    }

    /// java: public void undo()
    pub fn undo(&self) {
        OptionUtils::for_each_options(&*self.config, |option| option.forget_pending_value());
    }

    /// java: @Override public void tick()
    pub fn tick(&self) {
        if let Some(current) = self.tab_manager.get_current_tab()
            && let Some(tab_ext) = current.as_tab_ext()
        {
            tab_ext.tick();
        }

        if let Some(category_tab) = self.current_category_tab() {
            let message = self.save_button_message.borrow().clone();
            if let Some(message) = message {
                if self.save_button_message_time.get() > 140 {
                    *self.save_button_message.borrow_mut() = None;
                    *self.save_button_tooltip_message.borrow_mut() = None;
                    self.save_button_message_time.set(0);
                } else {
                    self.save_button_message_time.set(self.save_button_message_time.get() + 1);
                    category_tab.save_finished_button.fields.set_message(message);
                    let tooltip = self.save_button_tooltip_message.borrow().clone();
                    if let Some(tooltip) = tooltip {
                        category_tab.save_finished_button.fields.set_tooltip(Some(tooltip));
                    }
                }
            }
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent mouseButtonEvent, boolean bl)
    pub fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        // super.mouseClicked - ContainerEventHandler's, whose `setFocused` is this class's
        let weak = self.this.clone();
        let clicked = self.container.mouse_clicked_with(&self.children(), event, double_click, &move |child| {
            if let Some(screen) = weak.upgrade() {
                screen.set_focused_child(child);
            }
        });
        if clicked {
            self.container.set_dragging(true);
            return true;
        }
        false
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent mouseButtonEvent, double d, double e)
    pub fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        match self.get_focused() {
            Some(focused) => {
                self.container.is_dragging()
                    && (event.button() == input_constants::MOUSE_BUTTON_LEFT || event.button() == input_constants::MOUSE_BUTTON_RIGHT)
                    && focused.mouse_dragged(event, dx, dy)
            }
            None => false,
        }
    }

    /// java: public void setSaveButtonMessage(Component message, Component tooltip)
    pub fn set_save_button_message(&self, message: MutableComponent, tooltip: MutableComponent) {
        *self.save_button_message.borrow_mut() = Some(message);
        *self.save_button_tooltip_message.borrow_mut() = Some(tooltip);
        self.save_button_message_time.set(0);
    }

    /// java: public boolean pendingChanges()
    pub fn pending_changes(&self) -> bool {
        self.pending_changes.get()
    }

    /// java: private void onOptionChanged(Option<?> option)
    fn on_option_changed(&self, _option: &dyn AnyOption) {
        self.pending_changes.set(false);

        OptionUtils::consume_options(&*self.config, |opt| {
            self.pending_changes.set(self.pending_changes.get() | opt.changed());
            self.pending_changes.get()
        });

        if let Some(category_tab) = self.current_category_tab() {
            category_tab.update_buttons();
        }
    }

    /// java: @Override public boolean shouldCloseOnEsc()
    pub fn should_close_on_esc(&self) -> bool {
        if self.pending_changes() {
            self.set_save_button_message(
                component::translatable("yacl.gui.save_before_exit").with_style_format(ChatFormatting::Red),
                component::translatable("yacl.gui.save_before_exit.tooltip"),
            );
            return false;
        }
        true
    }

    /// java: @Override public void onClose() - `GuiUtils.setScreen(parent)`
    pub fn on_close(&self) {
        let parent = match &self.parent {
            Parent::None => compat::ScreenRef::Null,
            Parent::Below => compat::ScreenRef::Parent,
            Parent::Mod(screen) => match screen.borrow_mut().take() {
                Some(screen) => compat::ScreenRef::of_box(screen, false),
                // the parent object was handed back once already
                None => compat::ScreenRef::Null,
            },
        };
        GuiUtils::set_screen_1(parent);
    }

    /// java: public static void renderMultilineTooltip(GuiGraphicsExtractor graphics, Font font, MultiLineLabel text,
    /// int centerX, int yAbove, int yBelow, int screenWidth, int screenHeight)
    #[allow(clippy::too_many_arguments)]
    pub fn render_multiline_tooltip(
        graphics: &Graphics<'_>,
        text: &MultiLineLabel,
        center_x: i32,
        y_above: i32,
        y_below: i32,
        _screen_width: i32,
        screen_height: i32,
    ) {
        if text.get_line_count() > 0 {
            let max_width = text.get_width();
            let line_height = LINE_HEIGHT + 1;
            let height = text.get_line_count() * line_height - 1;

            let below_y = y_below + 12;
            let above_y = y_above - height + 12;
            let max_below = screen_height - (below_y + height);
            let min_above = above_y - height;
            let mut y = above_y;
            if min_above < 8 {
                y = if max_below > min_above { below_y } else { above_y };
            }

            let x = (center_x - text.get_width() / 2 - 12).max(-6);

            let draw_x = x + 12;
            let draw_y = y - 12;

            graphics.push_pose();
            mcrust::net::minecraft::client::gui::screens::inventory::tooltip::tooltip_render_util::extract_tooltip_background(
                graphics.gl,
                graphics.mc,
                draw_x,
                draw_y,
                max_width,
                height,
                None,
            );

            text.visit_lines(TextAlignment::Left, draw_x, draw_y, line_height, &mut |lx, ly, line: &FormattedText| {
                graphics.text(line, lx, ly, -1, true)
            });

            graphics.pop_pose();
        }
    }

    /// java: public void updateGlobalSearch(String search)
    pub fn update_global_search(&self, search: &str) {
        let mut next_tab_with_search: Option<Arc<CategoryTab>> = None;
        let preferred = self.preferred_tab.borrow().as_ref().and_then(Weak::upgrade);
        if let Some(preferred) = &preferred {
            preferred.option_list.get_type().update_search_query(search);
            if preferred.has_search() {
                next_tab_with_search = Some(preferred.clone());
            }
        }

        let bar = self.tab_navigation_bar();
        let current_tab = bar.get_tab_manager().get_current_tab();
        let current_category_tab = current_tab.as_ref().and_then(|tab| tab.dyn_any().downcast_ref::<CategoryTab>().and_then(|t| t.this.upgrade()));
        let cursor_pos = current_category_tab.as_ref().map_or(-1, |tab| tab.search_field.edit_box.get_cursor_position());

        for tab in bar.get_tabs() {
            let Some(category_tab) = tab.dyn_any().downcast_ref::<CategoryTab>().and_then(|t| t.this.upgrade()) else {
                continue;
            };
            if preferred.as_ref().is_some_and(|preferred| Arc::ptr_eq(preferred, &category_tab)) {
                continue;
            }
            category_tab.option_list.get_type().update_search_query(search);
            category_tab.search_field.set_value_do_not_update(search);
            if cursor_pos != -1 {
                category_tab.search_field.edit_box.set_cursor_position(cursor_pos);
            }
            if next_tab_with_search.is_none() && category_tab.has_search() {
                next_tab_with_search = Some(category_tab);
            }
        }

        // switch if the next tab is the preferred one or switch if the current tab does not have the search
        if let Some(next) = next_tab_with_search {
            let next_is_current = current_category_tab.as_ref().is_some_and(|current| Arc::ptr_eq(current, &next));
            let next_is_preferred = preferred.as_ref().is_some_and(|preferred| Arc::ptr_eq(preferred, &next));
            let current_has_search = current_category_tab.as_ref().is_some_and(|current| current.has_search());
            if !next_is_current && (next_is_preferred || !current_has_search) {
                self.tab_manager.set_current_tab(next.clone(), false);
                let search_field: Arc<dyn GuiEventListener> = next.search_field.clone();
                self.set_focused_child(Some(search_field));
            }
        }
        bar.update_tab_names();
    }

    /// java: @Override public void setFocused(@Nullable GuiEventListener focused)
    pub fn set_focused_child(&self, focused: Option<Arc<dyn GuiEventListener>>) {
        self.container.set_focused(focused.clone());
        if let Some(focused) = focused
            && !focused.dyn_any().is::<SearchFieldWidget>()
        {
            *self.preferred_tab.borrow_mut() = None;
        }
    }
}

// ---- the screen as the game's screen stack holds it ---------------------------------------------------------------

/// The modifier bits `MouseButtonInfo` carries: the port's mouse callbacks hand a mod screen the button alone, so the
/// shift / control state is read off the keyboard (`InputConstants.isKeyDown`), as `MouseHandler` reads it at the press.
pub(crate) fn current_modifiers() -> i32 {
    use mcrust::net::minecraft::client::input::input_with_modifiers::{MOD_CONTROL, MOD_SHIFT};
    let mut modifiers = 0;
    if KeyUtils::has_shift_down() {
        modifiers |= MOD_SHIFT;
    }
    if KeyUtils::has_control_down() {
        modifiers |= MOD_CONTROL;
    }
    modifiers
}

/// `new MouseButtonEvent(x, y, new MouseButtonInfo(button, modifiers))`
pub(crate) fn mouse_event(x: f64, y: f64, button: i32) -> MouseButtonEvent {
    MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, current_modifiers()))
}

/// `YACLScreen` in the game's screen stack: every `Screen` method the game calls, forwarded to the YACL screen.
pub struct YACLScreenAdapter {
    screen: compat::RenderThread<Arc<YACLScreen>>,
}

impl YACLScreenAdapter {
    fn new(screen: Arc<YACLScreen>) -> YACLScreenAdapter {
        YACLScreenAdapter { screen: compat::RenderThread::new(screen) }
    }

    /// The YACL screen (`minecraft.screen instanceof YACLScreen yacl` binds this).
    pub fn yacl(&self) -> &Arc<YACLScreen> {
        self.screen.get()
    }
}

impl ModScreen for YACLScreenAdapter {
    fn get_title(&self) -> String {
        self.yacl().title.get_string()
    }

    /// `Screen.init()` (and every rebuild): the cursor sink in the game's tree, then the YACL screen's `init`
    fn init(&mut self, screen: &mut Screen) {
        screen.add_renderable_widget(compat::cursor_sink_widget());
        self.yacl().init_size(screen.width, screen.height);
    }

    /// `repositionElements()` - except when a `GuiAccessor.yacl$setScreen` raw write put this screen back (the
    /// popup closing), which runs no `init` at all
    fn reposition_elements(&mut self, screen: &mut Screen) {
        if crate::mixins::gui_accessor::consume_raw_screen_write() {
            return;
        }
        screen.rebuild_widgets(&mut |screen| self.init(screen));
    }

    fn tick(&mut self, _screen: &mut Screen) {
        self.yacl().tick();
    }

    fn extract_render_state(&mut self, _screen: &Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let yacl = self.yacl().clone();
        let graphics = Graphics::new(context.gl, &*context.minecraft);
        yacl.extract_background(&graphics, mouse_x, mouse_y, a);
        yacl.extract_render_state(&graphics, mouse_x, mouse_y, a);
        graphics.extract_deferred_tooltip();
    }

    fn key_pressed(&mut self, _screen: &mut Screen, event: &KeyEvent) -> bool {
        self.yacl().key_pressed(event)
    }

    fn char_typed(&mut self, _screen: &mut Screen, c: char) -> bool {
        self.yacl().char_typed(&CharacterEvent::new(c as i32))
    }

    fn mouse_clicked(&mut self, _screen: &mut Screen, x: f64, y: f64, button: i32) -> bool {
        // double clicks are not delivered to a mod screen (GAPS_Y2.md G12)
        self.yacl().mouse_clicked(&mouse_event(x, y, button), false)
    }

    fn mouse_released(&mut self, _screen: &mut Screen, x: f64, y: f64, button: i32) -> bool {
        self.yacl().mouse_released(&mouse_event(x, y, button))
    }

    fn mouse_dragged(&mut self, _screen: &mut Screen, x: f64, y: f64, button: i32, dx: f64, dy: f64) -> bool {
        self.yacl().mouse_dragged(&mouse_event(x, y, button), dx, dy)
    }

    fn mouse_scrolled(&mut self, _screen: &mut Screen, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        self.yacl().mouse_scrolled(x, y, scroll_x, scroll_y)
    }

    /// The game's polled Escape never closes this screen: `key_pressed` asks `shouldCloseOnEsc()` itself (which sets
    /// the save-before-exit message when it answers `false`).
    fn should_close_on_esc(&self) -> bool {
        false
    }

    fn on_close(&mut self) {
        self.yacl().on_close();
    }

    fn class_name(&self) -> &'static str {
        "dev.isxander.yacl3.gui.YACLScreen"
    }
}

// ---- java: public static class CategoryTab implements TabExt --------------------------------------------------------

/// java: private static final Identifier DARKER_BG = YACLPlatform.mcRl("textures/gui/menu_list_background.png");
fn darker_bg() -> Identifier {
    YACLPlatform::mc_rl("textures/gui/menu_list_background.png")
}

/// java: public static class CategoryTab implements TabExt
pub struct CategoryTab {
    /// java: private final YACLScreen screen; (held weakly: the screen holds its tabs)
    screen: Weak<YACLScreen>,
    /// java: private final ConfigCategory category;
    category: Arc<dyn ConfigCategory>,
    /// java: private final Tooltip tooltip;
    tooltip: MutableComponent,
    /// java: private WidgetAndType<OptionListWidget> optionList;
    pub option_list: Arc<dyn WidgetAndType<OptionListWidget>>,
    /// java: public final Button saveFinishedButton;
    pub save_finished_button: Arc<Button>,
    /// java: public final Button cancelResetButton;
    pub cancel_reset_button: Arc<Button>,
    /// java: public final Button undoButton;
    pub undo_button: Arc<Button>,
    /// java: private final SearchFieldWidget searchField;
    pub search_field: Arc<SearchFieldWidget>,
    /// java: private OptionDescriptionWidget descriptionWidget;
    description_widget: Arc<OptionDescriptionWidget>,
    /// java: private final ScreenRectangle rightPaneDim;
    right_pane_dim: ScreenRectangle,
    /// `this`
    this: Weak<CategoryTab>,
}

impl CategoryTab {
    /// java: public CategoryTab(YACLScreen screen, ConfigCategory category, ScreenRectangle tabArea)
    pub fn new(screen: &Arc<YACLScreen>, category: Arc<dyn ConfigCategory>, tab_area: ScreenRectangle) -> Arc<CategoryTab> {
        Arc::new_cyclic(|this: &Weak<CategoryTab>| {
            let tooltip = ConfigCategory::tooltip(&*category);

            let width = screen.width();
            let height = screen.height();
            let mut column_width = width / 3;
            let padding = column_width / 20;
            column_width = column_width.min(400);
            let padded_width = column_width - padding * 2;
            let right_pane_dim = ScreenRectangle::new_at(width / 3 * 2, tab_area.top() + 1, width / 3, tab_area.height());
            let action_dim: Arc<dyn MutableDimension<i32>> =
                dimension::of_int(width / 3 * 2 + width / 6, height - padding - 20, padded_width, 20);

            let weak_screen = Arc::downgrade(screen);
            let save_finished_button = Arc::new(Button::new(
                action_dim.x() - action_dim.width() / 2,
                action_dim.y(),
                action_dim.width(),
                action_dim.height(),
                component::literal("Done"),
                move |_btn| {
                    if let Some(screen) = weak_screen.upgrade() {
                        screen.finish_or_save();
                    }
                },
            ));

            action_dim.expand(-action_dim.width() / 2 - 2, 0).r#move(-action_dim.width() / 2 - 2, -22);
            let weak_screen = Arc::downgrade(screen);
            let cancel_reset_button = Arc::new(Button::new(
                action_dim.x() - action_dim.width() / 2,
                action_dim.y(),
                action_dim.width(),
                action_dim.height(),
                component::literal("Cancel"),
                move |_btn| {
                    if let Some(screen) = weak_screen.upgrade() {
                        screen.cancel_or_reset();
                    }
                },
            ));

            action_dim.r#move(action_dim.width() + 4, 0);
            let weak_screen = Arc::downgrade(screen);
            let undo_button = Arc::new(Button::new(
                action_dim.x() - action_dim.width() / 2,
                action_dim.y(),
                action_dim.width(),
                action_dim.height(),
                component::translatable("yacl.gui.undo"),
                move |_btn| {
                    if let Some(screen) = weak_screen.upgrade() {
                        screen.undo();
                    }
                },
            ));
            undo_button.fields.set_tooltip(Some(component::translatable("yacl.gui.undo.tooltip")));

            let (weak_screen, weak_tab) = (Arc::downgrade(screen), this.clone());
            let search_field = Arc::new(SearchFieldWidget::new(
                screen,
                width / 3 * 2 + width / 6 - padded_width / 2 + 1,
                undo_button.fields.get_y() - 22,
                padded_width - 2,
                18,
                component::translatable("gui.recipebook.search_hint"),
                component::translatable("gui.recipebook.search_hint"),
                move |s: &str| {
                    if let Some(screen) = weak_screen.upgrade() {
                        if screen.preferred_tab.borrow().as_ref().and_then(Weak::upgrade).is_none() {
                            *screen.preferred_tab.borrow_mut() = Some(weak_tab.clone());
                        }
                        screen.update_global_search(s);
                    }
                },
            ));

            let weak_tab = this.clone();
            let option_list = selection_list::as_widget(OptionListWidget::new(
                screen,
                category.clone(),
                0,
                0,
                width / 3 * 2 + 1,
                height,
                move |desc| {
                    if let Some(tab) = weak_tab.upgrade() {
                        tab.description_widget.set_option_description(Some(desc));
                    }
                },
            ));

            let search_field_y = search_field.edit_box.get_y();
            let description_widget = Arc::new(OptionDescriptionWidget::new(
                move || ScreenRectangle::new_at(width / 3 * 2 + padding, tab_area.top() + padding, padded_width, search_field_y - 1 - tab_area.top() - padding * 2),
                None,
            ));

            CategoryTab {
                screen: Arc::downgrade(screen),
                category,
                tooltip,
                option_list,
                save_finished_button,
                cancel_reset_button,
                undo_button,
                search_field,
                description_widget,
                right_pane_dim,
                this: this.clone(),
            }
        })
        .tap(|tab| tab.update_buttons())
    }

    /// `screen`
    fn screen(&self) -> Arc<YACLScreen> {
        self.screen.upgrade().expect("a YACL tab outlived its screen")
    }

    /// java: public boolean hasSearch()
    pub fn has_search(&self) -> bool {
        self.option_list.get_type().list.children().iter().any(|o| o.entry_fields().search_query_matches.get())
    }

    /// java: public void updateButtons()
    pub fn update_buttons(&self) {
        let pending_changes = self.screen().pending_changes();

        self.undo_button.fields.active.set(pending_changes);
        self.save_finished_button.fields.set_message(if pending_changes {
            component::translatable("yacl.gui.save")
        } else {
            GuiUtils::translatable_fallback("yacl.gui.done", &compat::component_of(common_components::gui_done()))
        });
        self.save_finished_button.fields.set_tooltip(Some(if pending_changes {
            component::translatable("yacl.gui.save.tooltip")
        } else {
            component::translatable("yacl.gui.finished.tooltip")
        }));
        self.cancel_reset_button.fields.set_message(if pending_changes {
            GuiUtils::translatable_fallback("yacl.gui.cancel", &compat::component_of(common_components::gui_cancel()))
        } else {
            component::translatable("controls.reset")
        });
        self.cancel_reset_button.fields.set_tooltip(Some(if pending_changes {
            component::translatable("yacl.gui.cancel.tooltip")
        } else {
            component::translatable("yacl.gui.reset.tooltip")
        }));
    }
}

/// `x.tap(f)` - run `f` on a value and hand it on (the constructor's trailing `updateButtons()` needs the finished
/// object).
trait Tap: Sized {
    fn tap(self, f: impl FnOnce(&Self)) -> Self {
        f(&self);
        self
    }
}

impl<T> Tap for T {}

impl Tab for CategoryTab {
    /// java: @Override public Component getTabTitle()
    fn get_tab_title(&self) -> MutableComponent {
        let mut copy = ConfigCategory::name(&*self.category).copy();
        if !self.has_search() {
            copy = copy.with_style_formats(&[ChatFormatting::DarkGray, ChatFormatting::Strikethrough]);
        }
        copy
    }

    /// java: @Override public void visitChildren(Consumer<AbstractWidget> consumer)
    fn visit_children(&self, consumer: &mut dyn FnMut(Arc<dyn GuiEventListener>)) {
        consumer(self.option_list.get_widget());
        consumer(self.save_finished_button.clone());
        consumer(self.cancel_reset_button.clone());
        consumer(self.undo_button.clone());
        consumer(self.search_field.clone());
        consumer(self.description_widget.clone());
    }

    /// java: @Override public void doLayout(ScreenRectangle tabArea)
    fn do_layout(&self, tab_area: ScreenRectangle) {
        let rect = ScreenRectangle::new_at(tab_area.left(), tab_area.top(), tab_area.width() / 3 * 2, tab_area.height());
        let option_list = self.option_list.get_type();
        option_list.list.set_x(rect.left());
        option_list.list.set_y(rect.top());
        option_list.list.set_width(rect.width());
        option_list.list.set_height(rect.height());
    }

    fn as_tab_ext(&self) -> Option<&dyn TabExt> {
        Some(self)
    }
}

impl TabExt for CategoryTab {
    /// java: @Nullable @Override public Tooltip getTooltip()
    fn get_tooltip(&self) -> Option<MutableComponent> {
        Some(self.tooltip.clone())
    }

    /// java: @Override public void tick()
    fn tick(&self) {
        self.description_widget.tick();
    }

    /// java: @Override public void renderBackground(GuiGraphicsExtractor graphics)
    fn render_background(&self, graphics: &Graphics<'_>) {
        let right_pane_dim = self.right_pane_dim;
        // right pane darker bg
        graphics.blit(
            &darker_bg(),
            right_pane_dim.left(),
            right_pane_dim.top(),
            (right_pane_dim.right() + 2) as f32,
            (right_pane_dim.bottom() + 2) as f32,
            right_pane_dim.width() + 2,
            right_pane_dim.height() + 2,
            32,
            32,
        );

        // top separator for right pane
        graphics.push_pose();
        graphics.blit(
            &Identifier::with_default_namespace("textures/gui/header_separator.png"),
            right_pane_dim.left() - 1,
            right_pane_dim.top() - 2,
            0.0,
            0.0,
            right_pane_dim.width() + 1,
            2,
            32,
            2,
        );
        graphics.pop_pose();

        // left separator for right pane
        graphics.push_pose();
        graphics.translate(right_pane_dim.left() as f32, (right_pane_dim.top() - 1) as f32);
        graphics.rotate(90.0f64.to_radians() as f32);
        graphics.blit(
            &Identifier::with_default_namespace("textures/gui/footer_separator.png"),
            0,
            0,
            0.0,
            0.0,
            right_pane_dim.height() + 1,
            2,
            32,
            2,
        );
        graphics.pop_pose();
    }
}

// ---- java: public static class PlaceholderTab implements TabExt -----------------------------------------------------

/// java: public static class PlaceholderTab implements TabExt
pub struct PlaceholderTab {
    /// java: private final YACLScreen screen; (held weakly: the screen holds its tabs)
    screen: Weak<YACLScreen>,
    /// java: private final PlaceholderCategory category;
    category: Arc<dyn ConfigCategory>,
    /// java: private final Tooltip tooltip;
    tooltip: MutableComponent,
}

impl PlaceholderTab {
    /// java: public PlaceholderTab(PlaceholderCategory category, YACLScreen screen)
    pub fn new(category: Arc<dyn ConfigCategory>, screen: &Arc<YACLScreen>) -> Arc<PlaceholderTab> {
        let tooltip = ConfigCategory::tooltip(&*category);
        Arc::new(PlaceholderTab { screen: Arc::downgrade(screen), category, tooltip })
    }

    /// `category` as the `PlaceholderCategory` it is
    fn placeholder(&self) -> &dyn PlaceholderCategory {
        self.category.as_placeholder_category().expect("a PlaceholderTab's category is a PlaceholderCategory")
    }
}

impl Tab for PlaceholderTab {
    /// java: @Override public Component getTabTitle()
    fn get_tab_title(&self) -> MutableComponent {
        ConfigCategory::name(&*self.category)
    }

    /// java: @Override public void visitChildren(Consumer<AbstractWidget> consumer)
    fn visit_children(&self, _consumer: &mut dyn FnMut(Arc<dyn GuiEventListener>)) {}

    /// java: @Override public void doLayout(ScreenRectangle screenRectangle)
    fn do_layout(&self, _screen_rectangle: ScreenRectangle) {
        let Some(screen) = self.screen.upgrade() else {
            return;
        };
        let function = self.placeholder().screen();
        GuiUtils::set_screen_1(function(compat::minecraft(), &screen));
    }

    fn as_tab_ext(&self) -> Option<&dyn TabExt> {
        Some(self)
    }
}

impl TabExt for PlaceholderTab {
    /// java: @Override public @Nullable Tooltip getTooltip()
    fn get_tooltip(&self) -> Option<MutableComponent> {
        Some(self.tooltip.clone())
    }
}
