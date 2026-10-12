//! Ported from dev.isxander.yacl3.gui.YACLScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::api::custom_tab_provider::CustomTabProvider;
use crate::dev::isxander::yacl3::api::option_event_listener::Event;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::placeholder_category::PlaceholderCategory;
use crate::dev::isxander::yacl3::api::utils::dimension;
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
use crate::dev::isxander::yacl3::gui::widget_and_type::WidgetAndType;
use crate::dev::isxander::yacl3::gui::yacl_selection_list::YACLSelectionList;
use crate::dev::isxander::yacl3::gui::yacl_selection_list::YACLSelectionListBase;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::multi_line_label::MultiLineLabel;
use mcrust::net::minecraft::client::gui::components::tabs::tab::Tab;
use mcrust::net::minecraft::client::gui::components::tabs::tab_manager::TabManager;
use mcrust::net::minecraft::client::gui::components::tabs::tab_navigation_bar;
use mcrust::net::minecraft::client::gui::components::tooltip::Tooltip;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::screens::inventory::tooltip::tooltip_render_util;
use mcrust::net::minecraft::client::gui::screens::worldselection::create_world_screen::CreateWorldScreen;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use crate::dev::isxander::yacl3::api::option::Option as _;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final Identifier DARKER_BG = YACLPlatform.mcRl("textures/gui/menu_list_background.png");
pub static CATEGORY_TAB_DARKER_BG: compat::MainThread<Identifier> = compat::MainThread::new(|| YACLPlatform::mc_rl("textures/gui/menu_list_background.png"));

/// java: public class YACLScreen extends Screen
pub struct YACLScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: crate::unresolved::Screen,
    /// java: public final YetAnotherConfigLib config;
    pub config: compat::Shared<dyn YetAnotherConfigLib>,
    /// java: private final Screen parent;
    pub parent: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: public final TabManager tabManager = new TabManager(this::addRenderableWidget, this::removeWidget);
    pub tab_manager: Option<TabManager>,
    /// java: public ScrollableNavigationBar tabNavigationBar;
    pub tab_navigation_bar: Option<ScrollableNavigationBar>,
    /// java: public ScreenRectangle tabArea;
    pub tab_area: Option<ScreenRectangle>,
    /// java: public Component saveButtonMessage;
    pub save_button_message: Option<MutableComponent>,
    /// java: public Tooltip saveButtonTooltipMessage;
    pub save_button_tooltip_message: Option<Tooltip>,
    /// java: private int saveButtonMessageTime;
    pub save_button_message_time: i32,
    /// java: private boolean pendingChanges;
    pub pending_changes: bool,
    /// java: public ControllerPopupWidget<?> currentPopupController = null;
    pub current_popup_controller: Option<compat::Shared<dyn ControllerPopupWidget<compat::Object>>>,
    /// java: public boolean popupControllerVisible = false;
    pub popup_controller_visible: bool,
    /// java: private @Nullable CategoryTab preferredTab = null;
    pub preferred_tab: Option<compat::Shared<CategoryTab>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for YACLScreen {
    type Target = crate::unresolved::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for YACLScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl YACLScreen {
    /// java: public YACLScreen(YetAnotherConfigLib config, Screen parent)
    pub fn new(mut config: compat::Shared<dyn YetAnotherConfigLib>, mut parent: mcrust::net::minecraft::client::gui::screens::screen::Screen) -> compat::Shared<Self> {
        let __init_config = config.clone();
        // TODO(fabric2seer): the mod keeps the game's `Screen` (Java holds a reference to the live object); the port's `Screen` is not a value to copy and the port lends the mod no handle to it
        //   java: this.parent = parent;
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(({ let __f = compat::dispatch(&config, |__t| __t.title()); __f(&config) }).get_string()),
            self_ref: std::sync::Weak::new(),
            config: __init_config,
            parent: todo!("fabric2seer: the field value (TODO above)"),
            tab_manager: None,
            tab_navigation_bar: None,
            tab_area: None,
            save_button_message: None,
            save_button_tooltip_message: None,
            save_button_message_time: 0,
            pending_changes: false,
            current_popup_controller: None,
            popup_controller_visible: false,
            preferred_tab: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // TODO(fabric2seer): the port's `new` ports this Java method with other parameters ( for Java's add_widget, remove_widget)
        // TODO(fabric2seer): `new` takes 0 argument(s) in the port (``), Java passes 2
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Consumer<net.minecraft.client.gui.components.AbstractWidget>`)
        // TODO(fabric2seer): `Screen.removeWidget` has no Rust method in the port (looked for `remove_widget` in Screen and its supertypes)
        // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        // rust: let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        // rust: { let __v = Some(TabManager::new(|__a0| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let (__q0,) = (__a0.clone(),); __this_1.lock().borrow_mut().add_renderable_widget(__q0) }; } }, |__a0| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; { __this_2.lock().borrow().remove_widget(&__a0); } })); compat::write(&__this, |__t| __t.tab_manager = __v); }
        let __this_4 = compat::RenderThread::new(__this.clone());
        OptionUtils::for_each_options(&config, Arc::new(move |option| {
            let __this_3 = compat::RenderThread::new((*__this_4.get()).clone());
            { let (__a0,) = (Arc::new(move |opt, event| {
                if event != Event::Initial {
                    YACLScreen::on_option_changed(&(*__this_3.get()), &opt);
                }
            }),); option.lock().borrow_mut().add_event_listener(__a0) };
        }));
        __this
    }

    /// java: public static void renderMultilineTooltip(GuiGraphicsExtractor graphics, Font font, MultiLineLabel text, int centerX, int yAbove, int yBelow, int screenWidth, int screenHeight)
    pub fn render_multiline_tooltip(graphics: &GuiGraphicsExtractor, font: &Font, text: &MultiLineLabel, mut center_x: i32, mut y_above: i32, mut y_below: i32, mut screen_width: i32, mut screen_height: i32) {
        if text.get_line_count() > 0 {
            let mut max_width: i32 = text.get_width();
            let mut line_height: i32 = (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(1);
            let mut height: i32 = (text.get_line_count().wrapping_mul(line_height)).wrapping_sub(1);
            let mut below_y: i32 = y_below.wrapping_add(12);
            let mut above_y: i32 = (y_above.wrapping_sub(height)).wrapping_add(12);
            let mut max_below: i32 = screen_height.wrapping_sub(below_y.wrapping_add(height));
            let mut min_above: i32 = above_y.wrapping_sub(height);
            let mut y: i32 = above_y;
            if min_above < 8 {
                y = (if max_below > min_above { below_y } else { above_y });
            }
            let mut x: i32 = ((center_x.wrapping_sub(text.get_width().wrapping_div(2))).wrapping_sub(12)).max(-6);
            let mut draw_x: i32 = x.wrapping_add(12);
            let mut draw_y: i32 = y.wrapping_sub(12);
            let __pose_1 = gui_graphics_extractor::push_pose();
            tooltip_render_util::extract_tooltip_background(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), draw_x, draw_y, max_width, height, None);
            // TODO(fabric2seer): the port's `text_renderer` ports this Java method with other parameters (mouse_x, mouse_y for Java's )
            // TODO(fabric2seer): `text_renderer` takes 2 argument(s) in the port (`mouse_x: f64, mouse_y: f64`), Java passes 0
            // rust: text.visit_lines(LEFT, draw_x, draw_y, line_height, &mut gui_graphics_extractor::text_renderer());
            gui_graphics_extractor::pop_pose(__pose_1);
        }
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `Screen.width` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // TODO(fabric2seer): `Screen.height` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // rust: { let __v = Some(ScreenRectangle::new_at(0, 24i32.wrapping_sub(1), compat::read(__this, |__t| __t.width.clone()), ((compat::read(__this, |__t| __t.height.clone())).wrapping_sub(24)).wrapping_add(1))); compat::write(__this, |__t| __t.tab_area = __v); }
        let mut current_tab: i32 = (if compat::read(__this, |__t| __t.tab_navigation_bar.clone()).is_some() { __this.lock().borrow().tab_navigation_bar.as_ref().unwrap().get_tabs().iter().position(|e| *e == __this.lock().borrow().tab_manager.as_ref().unwrap().get_current_tab().unwrap()).map_or(-1, |i| i as i32) } else { 0 });
        if current_tab == -1 {
            current_tab = 0;
        }
        // TODO(fabric2seer): `Screen.width` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        // rust: { let __v = Some(ScrollableNavigationBar::new(compat::read(__this, |__t| __t.width.clone()), compat::read(__this, |__t| __t.tab_manager.clone()).unwrap(), ({ let __r = &compat::read(__this, |__t| __t.config.clone()); let __f = compat::dispatch(__r, |__t| __t.categories()); __f(__r) }).into_iter().map(|category| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
        // rust: if let Some(tab_provider) = category.lock().borrow().as_any().downcast_ref::<CustomTabProvider>() {
        // rust: return (tab_provider)((&__this_1).clone(), compat::read((&__this_1), |__t| __t.tab_area.clone()).unwrap());
        // rust: }
        // rust: if let Some(placeholder) = category.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn PlaceholderCategory>>() {
        // rust: return PlaceholderTab::new(placeholder.clone(), (&__this_1).clone());
        // rust: }
        // rust: return CategoryTab::new((&__this_1).clone(), category.clone(), compat::read((&__this_1), |__t| __t.tab_area.clone()).unwrap());
        // rust: } }).collect::<Vec<_>>())); compat::write(__this, |__t| __t.tab_navigation_bar = __v); }
        // TODO(fabric2seer): `select_tab` takes 3 argument(s) in the port (`index: usize, is_focused: bool, active: bool`), Java passes 2
        // rust: tab_navigation_bar::select_tab((current_tab) as usize, false);
        let __h0 = __this.lock().borrow().tab_area.as_ref().unwrap().width();
        __this.lock().borrow_mut().tab_navigation_bar.as_mut().unwrap().arrange_elements(__h0);
        let __r0 = compat::read(__this, |__t| __t.tab_area.clone());
        { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().tab_manager.as_mut().unwrap().set_tab_area(__q0) };
        let __r0 = compat::read(__this, |__t| __t.tab_navigation_bar.clone());
        { let (__q0,) = (__r0.unwrap(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        (({ let __r = &compat::read(__this, |__t| __t.config.clone()); let __f = compat::dispatch(__r, |__t| __t.init_consumer()); __f(__r) }))(__this.clone());
    }

    /// java: public void addPopupControllerWidget(ControllerPopupWidget<?> controllerPopupWidget)
    pub fn add_popup_controller_widget(__this: &compat::Shared<Self>, mut controller_popup_widget: compat::Shared<dyn ControllerPopupWidget<compat::Object>>) {
        if compat::read(__this, |__t| __t.current_popup_controller.clone()).is_some() {
            YACLScreen::clear_popup_controller_widget(__this);
        }
        { let __v = Some(controller_popup_widget.clone()); compat::write(__this, |__t| __t.current_popup_controller = __v); }
        { let __v = true; compat::write(__this, |__t| __t.popup_controller_visible = __v); }
        let mut option_list_widget: Option<compat::Shared<OptionListWidget>> = None;
        if let Some(category_tab) = __this.lock().borrow().tab_navigation_bar.as_ref().unwrap().get_tab_manager().get_current_tab().unwrap().as_any().downcast_ref::<compat::Shared<CategoryTab>>() {
            option_list_widget = Some(category_tab.lock().borrow().option_list.as_ref().unwrap().get_type());
        }
        if let Some(option_list_widget) = &option_list_widget {
            GuiUtils::set_screen_1(&PopupControllerScreen::new(__this.clone(), controller_popup_widget.clone()));
        }
    }

    /// java: public void clearPopupControllerWidget()
    pub fn clear_popup_controller_widget(__this: &compat::Shared<Self>) {
        if let Some(popup_controller_screen) = mcrust::mcrust::modding::game::current_mod_screen::<compat::ScreenAdapter<PopupControllerScreen>, _>(|__s| __s.object().clone()) {
            PopupControllerScreen::on_close(&popup_controller_screen);
        }
        { let __v = false; compat::write(__this, |__t| __t.popup_controller_visible = __v); }
        { let __v = None; compat::write(__this, |__t| __t.current_popup_controller = __v); }
    }

    /// java: @Override public void extractBackground(GuiGraphicsExtractor guiGraphics, int mouseX, int mouseY, float partialTick)
    pub fn extract_background(__this: &compat::Shared<Self>, gui_graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut partial_tick: f32) {
        // TODO(fabric2seer): `Screen.extractBackground` has no Rust method in the port (looked for `extract_background` in Screen and its supertypes)
        // rust: __this.lock().borrow_mut().base.extract_background(gui_graphics, mouse_x, mouse_y, partial_tick);
        if let Some(tab) = __this.lock().borrow().tab_manager.as_ref().unwrap().get_current_tab().unwrap().as_any().downcast_ref::<compat::Shared<dyn TabExt>>() {
            ({ let __f = compat::dispatch(tab, |__t| __t.render_background()); __f(tab, gui_graphics) });
        }
    }

    /// java: public void finishOrSave()
    pub fn finish_or_save(__this: &compat::Shared<Self>) {
        { let __v = None; compat::write(__this, |__t| __t.save_button_message = __v); }
        if YACLScreen::pending_changes(__this) {
            let mut flags: std::collections::HashSet<compat::Shared<dyn OptionFlag>> = std::collections::HashSet::new();
            OptionUtils::for_each_options(&compat::read(__this, |__t| __t.config.clone()), Arc::new(move |option| {
                if option.lock().borrow().apply_value() {
                    flags.extend(option.lock().borrow().flags().into_iter());
                }
            }));
            OptionUtils::for_each_options(&compat::read(__this, |__t| __t.config.clone()), Arc::new(move |option| {
                if option.lock().borrow().changed() {
                    option.lock().borrow_mut().forget_pending_value();
                    logger().error(&format!("Option '{}' value mismatch after applying! Reset to binding's getter.", option.lock().borrow().name().get_string()));
                }
            }));
            (({ let __r = &compat::read(__this, |__t| __t.config.clone()); let __f = compat::dispatch(__r, |__t| __t.save_function()); __f(__r) }))();
            for flag in flags.iter() {
                // TODO(fabric2seer): `accept` is not a method of OptionFlag
                //   java: flag.accept(..);
            }
            { let __v = false; compat::write(__this, |__t| __t.pending_changes = __v); }
            if let Some(category_tab) = __this.lock().borrow().tab_manager.as_ref().unwrap().get_current_tab().unwrap().as_any().downcast_ref::<compat::Shared<CategoryTab>>() {
                CategoryTab::update_buttons(category_tab);
            }
        } else {
            YACLScreen::on_close(__this);
        }
    }

    /// java: public void cancelOrReset()
    pub fn cancel_or_reset(__this: &compat::Shared<Self>) {
        if YACLScreen::pending_changes(__this) {
            OptionUtils::for_each_options(&compat::read(__this, |__t| __t.config.clone()), Arc::new(move |__a0| { __a0.lock().borrow_mut().forget_pending_value(); }));
            YACLScreen::on_close(__this);
        } else {
            OptionUtils::for_each_options(&compat::read(__this, |__t| __t.config.clone()), Arc::new(move |__a0| { __a0.lock().borrow_mut().request_set_default(); }));
        }
    }

    /// java: public void undo()
    pub fn undo(__this: &compat::Shared<Self>) {
        OptionUtils::for_each_options(&compat::read(__this, |__t| __t.config.clone()), Arc::new(move |__a0| { __a0.lock().borrow_mut().forget_pending_value(); }));
    }

    /// java: @Override public void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        if let Some(tab_ext) = __this.lock().borrow().tab_manager.as_ref().unwrap().get_current_tab().unwrap().as_any().downcast_ref::<compat::Shared<dyn TabExt>>() {
            ({ let __f = compat::dispatch(tab_ext, |__t| __t.tick()); __f(tab_ext) });
        }
        if let Some(category_tab) = __this.lock().borrow().tab_manager.as_ref().unwrap().get_current_tab().unwrap().as_any().downcast_ref::<compat::Shared<CategoryTab>>() {
            if compat::read(__this, |__t| __t.save_button_message.clone()).is_some() {
                if compat::read(__this, |__t| __t.save_button_message_time) > 140 {
                    { let __v = None; compat::write(__this, |__t| __t.save_button_message = __v); }
                    { let __v = None; compat::write(__this, |__t| __t.save_button_tooltip_message = __v); }
                    { let __v = 0; compat::write(__this, |__t| __t.save_button_message_time = __v); }
                } else {
                    { let __v = compat::read(__this, |__t| __t.save_button_message_time) + 1; compat::write(__this, |__t| __t.save_button_message_time = __v); }
                    let __r0 = compat::read(__this, |__t| __t.save_button_message.clone());
                    { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__r0.unwrap()),); category_tab.lock().borrow_mut().save_finished_button.as_mut().unwrap().set_message_text(__q0) };
                    if compat::read(__this, |__t| __t.save_button_tooltip_message.clone()).is_some() {
                        let __r0 = compat::read(__this, |__t| __t.save_button_tooltip_message.clone());
                        { let (__q0,) = (__r0.unwrap(),); category_tab.lock().borrow_mut().save_finished_button.as_mut().unwrap().set_tooltip(__q0) };
                    }
                }
            }
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent mouseButtonEvent, boolean bl)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, mouse_button_event: &MouseButtonEvent, mut bl: bool) -> bool {
        // TODO(fabric2seer): `mouse_clicked` takes 3 argument(s) in the port (`button: i32, x: f64, y: f64`), Java passes 2
        if { let (__q0, __q1,) = (mouse_button_event.clone(), bl,); __this.lock().borrow_mut().base.mouse_clicked(__q0, __q1) }.is_some() {
            // TODO(fabric2seer): `AbstractContainerEventHandler::set_dragging` ports `setDragging`, but the port's `Screen` holds no `AbstractContainerEventHandler` (no Deref, no `base` field) to call it on
            // rust: __this.lock().borrow_mut().set_dragging(true);
            return true;
        }
        return false;
    }

    /// java: @Override public boolean mouseDragged(MouseButtonEvent mouseButtonEvent, double d, double e)
    pub fn mouse_dragged(__this: &compat::Shared<Self>, mouse_button_event: &MouseButtonEvent, mut d: f64, mut e: f64) -> bool {
        // TODO(fabric2seer): `mouse_dragged` takes 5 argument(s) in the port (`_button: i32, _x: f64, _y: f64, _dx: f64, _dy: f64`), Java passes 3
        // rust: return (__this.lock().borrow().get_focused().is_some() && __this.lock().borrow().is_dragging() && (mouse_button_event.button() == 0 || mouse_button_event.button() == 1) && { let (__q0, __q1, __q2,) = (mouse_button_event.clone(), d, e,); __this.lock().borrow().get_focused().unwrap().mouse_dragged(__q0, __q1, __q2) });
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: public void setSaveButtonMessage(Component message, Component tooltip)
    pub fn set_save_button_message(__this: &compat::Shared<Self>, mut message: MutableComponent, tooltip: &MutableComponent) {
        { let __v = Some(message.clone()); compat::write(__this, |__t| __t.save_button_message = __v); }
        { let __v = Some(Tooltip::create(tooltip.get_string())); compat::write(__this, |__t| __t.save_button_tooltip_message = __v); }
        { let __v = 0; compat::write(__this, |__t| __t.save_button_message_time = __v); }
    }

    /// java: public boolean pendingChanges()
    pub fn pending_changes(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.pending_changes);
    }

    /// java: private void onOptionChanged(Option<?> option)
    pub fn on_option_changed(__this: &compat::Shared<Self>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) {
        { let __v = false; compat::write(__this, |__t| __t.pending_changes = __v); }
        let __this_1 = compat::RenderThread::new(__this.clone());
        OptionUtils::consume_options(&compat::read(__this, |__t| __t.config.clone()), Arc::new(move |opt| {
            { let __v = (compat::read(__this_1.get(), |__t| __t.pending_changes) | opt.lock().borrow().changed()); compat::write(__this_1.get(), |__t| __t.pending_changes = __v); }
            return compat::read(__this_1.get(), |__t| __t.pending_changes);
        }));
        if let Some(category_tab) = __this.lock().borrow().tab_manager.as_ref().unwrap().get_current_tab().unwrap().as_any().downcast_ref::<compat::Shared<CategoryTab>>() {
            CategoryTab::update_buttons(category_tab);
        }
    }

    /// java: @Override public boolean shouldCloseOnEsc()
    pub fn should_close_on_esc(__this: &compat::Shared<Self>) -> bool {
        if YACLScreen::pending_changes(__this) {
            YACLScreen::set_save_button_message(__this, component::translatable("yacl.gui.save_before_exit").with_style_format(ChatFormatting::Red), &component::translatable("yacl.gui.save_before_exit.tooltip"));
            return false;
        }
        return true;
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        GuiUtils::set_screen_1(&compat::read(__this, |__t| __t.parent.clone()));
    }

    /// java: public void updateGlobalSearch(String search)
    pub fn update_global_search(__this: &compat::Shared<Self>, search: &str) {
        let mut next_tab_with_search: Option<Box<dyn Tab>> = None;
        if compat::read(__this, |__t| __t.preferred_tab.clone()).is_some() {
            // TODO(fabric2seer): `.updateSearchQuery(..)` on a T
            //   java: preferredTab.optionList.getType().updateSearchQuery(search);
            if CategoryTab::has_search(&compat::read(__this, |__t| __t.preferred_tab.clone()).unwrap()) {
                next_tab_with_search = compat::read(__this, |__t| __t.preferred_tab.clone());
            }
        }
        let mut current_tab: Box<dyn Tab> = __this.lock().borrow().tab_navigation_bar.as_ref().unwrap().get_tab_manager().get_current_tab().unwrap();
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: int cursorPos = currentTab instanceof CategoryTab categoryTab ? categoryTab.searchField.getCursorPosition() : -1;
        let mut cursor_pos: i32 = todo!("fabric2seer: the initialiser of `cursorPos` (TODO above)");
        for i in 0..(__this.lock().borrow().tab_navigation_bar.as_ref().unwrap().get_tabs().len() as i32) {
            let tab = &__this.lock().borrow().tab_navigation_bar.as_ref().unwrap().get_tabs()[(i) as usize];
            if std::ptr::eq(tab, &compat::read(__this, |__t| __t.preferred_tab.clone()).unwrap()) {
                continue;
            }
            if let Some(category_tab) = tab.as_any().downcast_ref::<compat::Shared<CategoryTab>>() {
                // TODO(fabric2seer): `.updateSearchQuery(..)` on a T
                //   java: categoryTab.optionList.getType().updateSearchQuery(search);
                category_tab.lock().borrow_mut().search_field.as_mut().unwrap().set_value_do_not_update(search);
                if cursor_pos != -1 {
                    // TODO(fabric2seer): `set_cursor_position` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, pos: i32`), Java passes 1
                    // rust: category_tab.lock().borrow_mut().search_field.as_mut().unwrap().set_cursor_position(&cursor_pos);
                }
                if next_tab_with_search.is_none() && CategoryTab::has_search(category_tab) {
                    next_tab_with_search = Some(category_tab.clone());
                }
            }
        }
        // TODO(fabric2seer): an `instanceof` pattern under `||` / `!`
        //   java: if (nextTabWithSearch != null && nextTabWithSearch != currentTab && (nextTabWithSearch == preferredTab || !(currentTab instanceof CategoryTab categoryTab && categoryTab.hasSearch()))) {
        //   java:             tabManager.setCurrentTab(nextTabWithSearch, false);
        //   java:             if (nextTabWithSearch instanceof CategoryTab newTab) {
        //   java:                 setFocused(newTab.searchField);
        //   java:             }
        //   java:         }
        __this.lock().borrow().tab_navigation_bar.as_ref().unwrap().update_tab_names();
    }

    /// java: @Override public void setFocused(@Nullable GuiEventListener focused)
    pub fn set_focused(__this: &compat::Shared<Self>, mut focused: Option<Box<dyn GuiEventListener>>) {
        // TODO(fabric2seer): `Screen.setFocused` has no Rust method in the port (looked for `set_focused` in Screen and its supertypes)
        // rust: let __h0 = focused.as_ref().unwrap();
        // rust: __this.lock().borrow_mut().base.set_focused(__h0);
        if focused.is_some() && !(focused.as_ref().unwrap().as_any().is::<SearchFieldWidget>()) {
            { let __v = None; compat::write(__this, |__t| __t.preferred_tab = __v); }
        }
    }

}

// NOTE(fabric2seer): extractBackground, init, mouseClicked, mouseDragged, onClose, setFocused, shouldCloseOnEsc, tick override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: public static class CategoryTab implements TabExt
pub struct CategoryTab {
    /// java: private final YACLScreen screen;
    pub screen: compat::Shared<YACLScreen>,
    /// java: private final ConfigCategory category;
    pub category: compat::Shared<dyn ConfigCategory>,
    /// java: private final Tooltip tooltip;
    pub tooltip: Tooltip,
    /// java: private WidgetAndType<OptionListWidget> optionList;
    pub option_list: Option<compat::Shared<dyn WidgetAndType<compat::Shared<OptionListWidget>>>>,
    /// java: public final Button saveFinishedButton;
    pub save_finished_button: Option<AbstractWidget>,
    /// java: public final Button cancelResetButton;
    pub cancel_reset_button: Option<AbstractWidget>,
    /// java: public final Button undoButton;
    pub undo_button: Option<AbstractWidget>,
    /// java: private final SearchFieldWidget searchField;
    pub search_field: Option<SearchFieldWidget>,
    /// java: private OptionDescriptionWidget descriptionWidget;
    pub description_widget: Option<compat::Shared<OptionDescriptionWidget>>,
    /// java: private final ScreenRectangle rightPaneDim;
    pub right_pane_dim: Option<ScreenRectangle>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl CategoryTab {
    /// java: public CategoryTab(YACLScreen screen, ConfigCategory category, ScreenRectangle tabArea)
    pub fn new(mut screen: compat::Shared<YACLScreen>, mut category: compat::Shared<dyn ConfigCategory>, mut tab_area: ScreenRectangle) -> compat::Shared<Self> {
        let __init_screen = screen.clone();
        let __init_category = category.clone();
        let __init_tooltip = Tooltip::create(({ let __f = compat::dispatch(&category, |__t| __t.tooltip()); __f(&category) }).get_string());
        // TODO(fabric2seer): `Screen.width` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // rust: let mut column_width: i32 = (compat::read(&screen, |__t| __t.width.clone())).wrapping_div(3);
        let mut column_width: i32 = todo!("fabric2seer: the initialiser of `columnWidth` (TODO above)");
        let mut padding: i32 = column_width.wrapping_div(20);
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            screen: __init_screen,
            category: __init_category,
            tooltip: __init_tooltip,
            option_list: None,
            save_finished_button: None,
            cancel_reset_button: None,
            undo_button: None,
            search_field: None,
            description_widget: None,
            right_pane_dim: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        column_width = column_width.min(400);
        let mut padded_width: i32 = column_width.wrapping_sub(padding.wrapping_mul(2));
        // TODO(fabric2seer): `Screen.width` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // rust: { let __v = Some(ScreenRectangle::new_at(((compat::read(&screen, |__t| __t.width.clone())).wrapping_div(3)).wrapping_mul(2), tab_area.top().wrapping_add(1), (compat::read(&screen, |__t| __t.width.clone())).wrapping_div(3), tab_area.height())); compat::write(&__this, |__t| __t.right_pane_dim = __v); }
        // TODO(fabric2seer): `Screen.width` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // TODO(fabric2seer): `Screen.height` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // rust: let mut action_dim: compat::Shared<dyn MutableDimension<i32>> = dimension::of_int((((compat::read(&screen, |__t| __t.width.clone())).wrapping_div(3)).wrapping_mul(2)).wrapping_add((compat::read(&screen, |__t| __t.width.clone())).wrapping_div(6)), ((compat::read(&screen, |__t| __t.height.clone())).wrapping_sub(padding)).wrapping_sub(20), padded_width, 20);
        let mut action_dim: compat::Shared<dyn MutableDimension<i32>> = todo!("fabric2seer: the initialiser of `actionDim` (TODO above)");
        // TODO(fabric2seer): `Builder.pos` has no Rust method in the port (looked for `pos` in Button and its supertypes)
        // TODO(fabric2seer): `Builder.size` has no Rust method in the port (looked for `size` in Button and its supertypes)
        // rust: { let __v = Some((button::builder(component::literal("Done").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { YACLScreen::finish_or_save(&screen); })))).pos((action_dim.lock().borrow().x()).wrapping_sub((action_dim.lock().borrow().width()).wrapping_div(2)), &action_dim.lock().borrow().y()).size(&action_dim.lock().borrow().width(), &action_dim.lock().borrow().height()).build()); compat::write(&__this, |__t| __t.save_finished_button = __v); }
        { let (__a0, __a1,) = (&((-action_dim.lock().borrow().width()).wrapping_div(2)).wrapping_sub(2), &-22,); ({ let (__a0, __a1,) = (&((-action_dim.lock().borrow().width()).wrapping_div(2)).wrapping_sub(2), &0,); action_dim.lock().borrow_mut().expand(__a0, __a1) }.lock().borrow_mut()).r#move(__a0, __a1) };
        // TODO(fabric2seer): `Builder.pos` has no Rust method in the port (looked for `pos` in Button and its supertypes)
        // TODO(fabric2seer): `Builder.size` has no Rust method in the port (looked for `size` in Button and its supertypes)
        // rust: { let __v = Some((button::builder(component::literal("Cancel").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { YACLScreen::cancel_or_reset(&screen); })))).pos((action_dim.lock().borrow().x()).wrapping_sub((action_dim.lock().borrow().width()).wrapping_div(2)), &action_dim.lock().borrow().y()).size(&action_dim.lock().borrow().width(), &action_dim.lock().borrow().height()).build()); compat::write(&__this, |__t| __t.cancel_reset_button = __v); }
        { let (__a0, __a1,) = (&(action_dim.lock().borrow().width()).wrapping_add(4), &0,); action_dim.lock().borrow_mut().r#move(__a0, __a1) };
        // TODO(fabric2seer): `Builder.pos` has no Rust method in the port (looked for `pos` in Button and its supertypes)
        // TODO(fabric2seer): `Builder.size` has no Rust method in the port (looked for `size` in Button and its supertypes)
        // rust: { let __v = Some(({ let (__q0,) = (Some(Tooltip::create(component::translatable("yacl.gui.undo.tooltip").get_string()).message.clone()),); (button::builder(component::translatable("yacl.gui.undo").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { YACLScreen::undo(&screen); })))).pos((action_dim.lock().borrow().x()).wrapping_sub((action_dim.lock().borrow().width()).wrapping_div(2)), &action_dim.lock().borrow().y()).size(&action_dim.lock().borrow().width(), &action_dim.lock().borrow().height()).tooltip_text(__q0) }).build()); compat::write(&__this, |__t| __t.undo_button = __v); }
        // TODO(fabric2seer): `Screen.font` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // TODO(fabric2seer): `Screen.width` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        // rust: { let __v = Some(SearchFieldWidget::new(screen.clone(), compat::read(&screen, |__t| __t.font.clone()), (((((compat::read(&screen, |__t| __t.width.clone())).wrapping_div(3)).wrapping_mul(2)).wrapping_add((compat::read(&screen, |__t| __t.width.clone())).wrapping_div(6))).wrapping_sub(padded_width.wrapping_div(2))).wrapping_add(1), (__this.lock().borrow().undo_button.as_ref().unwrap().get_y()).wrapping_sub(22), padded_width.wrapping_sub(2), 18, component::translatable("gui.recipebook.search_hint"), component::translatable("gui.recipebook.search_hint"), Arc::new(move |s| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
        // rust: if compat::read(&screen, |__t| __t.preferred_tab.clone()).is_none() {
        // rust: { let __v = Some(__this_1.clone()); compat::write(&screen, |__t| __t.preferred_tab = __v); }
        // rust: }
        // rust: YACLScreen::update_global_search(&screen, &s);
        // rust: } }))); compat::write(&__this, |__t| __t.search_field = __v); }
        // TODO(fabric2seer): `Screen.minecraft` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // TODO(fabric2seer): `Screen.width` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // TODO(fabric2seer): `Screen.height` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // rust: let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        // rust: { let __v = Some(YACLSelectionListBase::as_widget(&(OptionListWidget::new(screen.clone(), category.clone(), &mut compat::read(&screen, |__t| __t.minecraft.clone()), 0, 0, (((compat::read(&screen, |__t| __t.width.clone())).wrapping_div(3)).wrapping_mul(2)).wrapping_add(1), compat::read(&screen, |__t| __t.height.clone()), Arc::new(move |desc| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; {
        // rust: OptionDescriptionWidget::set_option_description(&compat::read(&__this_2, |__t| __t.description_widget.clone()).unwrap(), desc.clone());
        // rust: } })) as compat::Shared<dyn YACLSelectionList<compat::Object>>))); compat::write(&__this, |__t| __t.option_list = __v); }
        // TODO(fabric2seer): `Screen.width` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // rust: let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        // rust: { let __v = Some(OptionDescriptionWidget::new(Arc::new(move || { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; ScreenRectangle::new_at((((compat::read(&screen, |__t| __t.width.clone())).wrapping_div(3)).wrapping_mul(2)).wrapping_add(padding), tab_area.top().wrapping_add(padding), padded_width, (((__this_3.lock().borrow().search_field.as_ref().unwrap().get_y()).wrapping_sub(1)).wrapping_sub(tab_area.top())).wrapping_sub(padding.wrapping_mul(2))) }), None)); compat::write(&__this, |__t| __t.description_widget = __v); }
        CategoryTab::update_buttons(&__this);
        __this
    }

    /// java: public boolean hasSearch()
    pub fn has_search(__this: &compat::Shared<Self>) -> bool {
        // TODO(fabric2seer): `.children(..)` on a T
        //   java: return optionList.getType().children().stream().anyMatch(o -> o.searchQueryMatches);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Component getTabTitle()
    pub fn get_tab_title(__this: &compat::Shared<Self>) -> MutableComponent {
        let mut copy: MutableComponent = ({ let __r = &compat::read(__this, |__t| __t.category.clone()); let __f = compat::dispatch(__r, |__t| __t.name()); __f(__r) }).copy();
        if !(CategoryTab::has_search(__this)) {
            copy = copy.with_style_formats(&[ChatFormatting::DarkGray, ChatFormatting::Strikethrough]);
        }
        return copy;
    }

    /// java: @Override public void visitChildren(Consumer<AbstractWidget> consumer)
    pub fn visit_children(__this: &compat::Shared<Self>, consumer: Arc<dyn Fn(AbstractWidget)>) {
        (consumer)(__this.lock().borrow().option_list.as_ref().unwrap().get_widget());
        (consumer)(compat::read(__this, |__t| __t.save_finished_button.clone()));
        (consumer)(compat::read(__this, |__t| __t.cancel_reset_button.clone()));
        (consumer)(compat::read(__this, |__t| __t.undo_button.clone()));
        (consumer)(compat::read(__this, |__t| __t.search_field.clone()));
        (consumer)(compat::read(__this, |__t| __t.description_widget.clone()));
    }

    /// java: @Override public void doLayout(ScreenRectangle tabArea)
    pub fn do_layout(__this: &compat::Shared<Self>, tab_area: &ScreenRectangle) {
        let mut rect = ScreenRectangle::new(tab_area.position(), (tab_area.width().wrapping_div(3)).wrapping_mul(2), tab_area.height());
        // TODO(fabric2seer): `.setX(..)` on a T
        //   java: optionList.getType().setX(rect.left());
        // TODO(fabric2seer): `.setY(..)` on a T
        //   java: optionList.getType().setY(rect.top());
        // TODO(fabric2seer): `.setWidth(..)` on a T
        //   java: optionList.getType().setWidth(rect.width());
        // TODO(fabric2seer): `.setHeight(..)` on a T
        //   java: optionList.getType().setHeight(rect.height());
    }

    /// java: public void updateButtons()
    pub fn update_buttons(__this: &compat::Shared<Self>) {
        let mut pending_changes: bool = YACLScreen::pending_changes(&compat::read(__this, |__t| __t.screen.clone()));
        __this.lock().borrow_mut().undo_button.as_mut().unwrap().active = pending_changes;
        { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((if pending_changes { component::translatable("yacl.gui.save") } else { GuiUtils::translatable_fallback("yacl.gui.done", &mcrust::net::minecraft::network::chat::common_components::gui_done()) })),); __this.lock().borrow_mut().save_finished_button.as_mut().unwrap().set_message_text(__q0) };
        { let (__q0,) = (Tooltip::create((if pending_changes { component::translatable("yacl.gui.save.tooltip") } else { component::translatable("yacl.gui.finished.tooltip") }).get_string()),); __this.lock().borrow_mut().save_finished_button.as_mut().unwrap().set_tooltip(__q0) };
        { let (__q0,) = (mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from((if pending_changes { GuiUtils::translatable_fallback("yacl.gui.cancel", &mcrust::net::minecraft::network::chat::common_components::gui_cancel()) } else { component::translatable("controls.reset") })),); __this.lock().borrow_mut().cancel_reset_button.as_mut().unwrap().set_message_text(__q0) };
        { let (__q0,) = (Tooltip::create((if pending_changes { component::translatable("yacl.gui.cancel.tooltip") } else { component::translatable("yacl.gui.reset.tooltip") }).get_string()),); __this.lock().borrow_mut().cancel_reset_button.as_mut().unwrap().set_tooltip(__q0) };
    }

}

impl TabExt for CategoryTab {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void renderBackground(GuiGraphicsExtractor graphics) (dispatch: the body for this class)
    fn render_background(&self) -> fn(&compat::Shared<dyn TabExt>, &GuiGraphicsExtractor) {
        CategoryTab::render_background_dyn
    }

    /// java: @Override public void tick() (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn TabExt>) {
        CategoryTab::tick_dyn
    }

    /// java: @Nullable @Override public Tooltip getTooltip() (dispatch: the body for this class)
    fn get_tooltip(&self) -> fn(&compat::Shared<dyn TabExt>) -> Option<Tooltip> {
        CategoryTab::get_tooltip_dyn
    }

}

impl CategoryTab {
    /// java: @Override public void renderBackground(GuiGraphicsExtractor graphics)
    pub fn render_background_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor) {
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), CATEGORY_TAB_DARKER_BG.get_mut(), __this.lock().borrow().right_pane_dim.as_ref().unwrap().left(), __this.lock().borrow().right_pane_dim.as_ref().unwrap().top(), (((__this.lock().borrow().right_pane_dim.as_ref().unwrap().right()).wrapping_add(2)) as f32), (((__this.lock().borrow().right_pane_dim.as_ref().unwrap().bottom()).wrapping_add(2)) as f32), (__this.lock().borrow().right_pane_dim.as_ref().unwrap().width()).wrapping_add(2), (__this.lock().borrow().right_pane_dim.as_ref().unwrap().height()).wrapping_add(2), 32, 32);
        let __pose_1 = gui_graphics_extractor::push_pose();
        // TODO(fabric2seer): static `CreateWorldScreen.HEADER_SEPARATOR` has no Rust counterpart in the port's CreateWorldScreen file
        // rust: gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &CreateWorldScreen::HEADER_SEPARATOR, (__this.lock().borrow().right_pane_dim.as_ref().unwrap().left()).wrapping_sub(1), (__this.lock().borrow().right_pane_dim.as_ref().unwrap().top()).wrapping_sub(2), 0.0f32, 0.0f32, (__this.lock().borrow().right_pane_dim.as_ref().unwrap().width()).wrapping_add(1), 2, 32, 2);
        gui_graphics_extractor::pop_pose(__pose_1);
        let __pose_2 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((__this.lock().borrow().right_pane_dim.as_ref().unwrap().left()) as f32, ((__this.lock().borrow().right_pane_dim.as_ref().unwrap().top()).wrapping_sub(1)) as f32);
        gui_graphics_extractor::pose_rotate(((((90 as f64).to_radians()) as f32)) as f32);
        // TODO(fabric2seer): static `CreateWorldScreen.FOOTER_SEPARATOR` has no Rust counterpart in the port's CreateWorldScreen file
        // rust: gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &CreateWorldScreen::FOOTER_SEPARATOR, 0, 0, 0.0f32, 0.0f32, (__this.lock().borrow().right_pane_dim.as_ref().unwrap().height()).wrapping_add(1), 2, 32, 2);
        gui_graphics_extractor::pop_pose(__pose_2);
    }

    /// CategoryTab.renderBackground as the dispatch of TabExt.renderBackground
    pub fn render_background_dyn(__this: &compat::Shared<dyn TabExt>, __p0: &GuiGraphicsExtractor) {
        Self::render_background_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void tick()
    pub fn tick_impl(__this: &compat::Shared<Self>) {
        OptionDescriptionWidget::tick(&compat::read(__this, |__t| __t.description_widget.clone()).unwrap());
    }

    /// CategoryTab.tick as the dispatch of TabExt.tick
    pub fn tick_dyn(__this: &compat::Shared<dyn TabExt>) {
        Self::tick_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Nullable @Override public Tooltip getTooltip()
    pub fn get_tooltip_impl(__this: &compat::Shared<Self>) -> Option<Tooltip> {
        return Some(compat::read(__this, |__t| __t.tooltip.clone()));
    }

    /// CategoryTab.getTooltip as the dispatch of TabExt.getTooltip
    pub fn get_tooltip_dyn(__this: &compat::Shared<dyn TabExt>) -> Option<Tooltip> {
        Self::get_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public static class PlaceholderTab implements TabExt
#[derive(Clone)]
pub struct PlaceholderTab {
    /// java: private final YACLScreen screen;
    pub screen: compat::Shared<YACLScreen>,
    /// java: private final PlaceholderCategory category;
    pub category: compat::Shared<dyn PlaceholderCategory>,
    /// java: private final Tooltip tooltip;
    pub tooltip: Tooltip,
}

impl PlaceholderTab {
    /// java: public PlaceholderTab(PlaceholderCategory category, YACLScreen screen)
    pub fn new(mut category: compat::Shared<dyn PlaceholderCategory>, mut screen: compat::Shared<YACLScreen>) -> compat::Shared<Self> {
        let __init_screen = screen.clone();
        let __init_category = category.clone();
        let __init_tooltip = Tooltip::create((({ let __f = compat::dispatch(&category, |__t| __t.tooltip()); __f(&compat::view(&category, |a| a as compat::Shared<dyn ConfigCategory>)) })).get_string());
        let __this = compat::shared(Self {
            screen: __init_screen,
            category: __init_category,
            tooltip: __init_tooltip,
        });
        __this
    }

    /// java: @Override public Component getTabTitle()
    pub fn get_tab_title(__this: &compat::Shared<Self>) -> MutableComponent {
        return ({ let __r = &compat::read(__this, |__t| __t.category.clone()); let __f = compat::dispatch(__r, |__t| __t.name()); __f(&compat::view(__r, |a| a as compat::Shared<dyn ConfigCategory>)) });
    }

    /// java: @Override public void visitChildren(Consumer<AbstractWidget> consumer)
    pub fn visit_children(__this: &compat::Shared<Self>, consumer: Arc<dyn Fn(AbstractWidget)>) {
    }

    /// java: @Override public void doLayout(ScreenRectangle screenRectangle)
    pub fn do_layout(__this: &compat::Shared<Self>, screen_rectangle: &ScreenRectangle) {
        // TODO(fabric2seer): `Screen.minecraft` has no Rust field in the port (searched dev.isxander.yacl3.api.net.minecraft.client.gui.screens.Screen and its supertypes)
        // rust: GuiUtils::set_screen_1(&(({ let __r = &compat::read(__this, |__t| __t.category.clone()); let __f = compat::dispatch(__r, |__t| __t.screen()); __f(__r) }))(compat::read(&compat::read(__this, |__t| __t.screen.clone()), |__t| __t.minecraft.clone()), compat::read(__this, |__t| __t.screen.clone())));
    }

}

impl TabExt for PlaceholderTab {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @Nullable Tooltip getTooltip() (dispatch: the body for this class)
    fn get_tooltip(&self) -> fn(&compat::Shared<dyn TabExt>) -> Option<Tooltip> {
        PlaceholderTab::get_tooltip_dyn
    }

}

impl PlaceholderTab {
    /// java: @Override public @Nullable Tooltip getTooltip()
    pub fn get_tooltip_impl(__this: &compat::Shared<Self>) -> Option<Tooltip> {
        return Some(compat::read(__this, |__t| __t.tooltip.clone()));
    }

    /// PlaceholderTab.getTooltip as the dispatch of TabExt.getTooltip
    pub fn get_tooltip_dyn(__this: &compat::Shared<dyn TabExt>) -> Option<Tooltip> {
        Self::get_tooltip_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
