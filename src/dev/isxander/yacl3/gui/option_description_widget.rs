//! Ported from dev.isxander.yacl3.gui.OptionDescriptionWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::gui::description_with_name::DescriptionWithName;
use mcrust::com::mojang::blaze3d::blaze3d;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::component_path::ComponentPath;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narrated_element_type::NarratedElementType;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::focus_navigation_event::FocusNavigationEvent;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::util::mth;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static final int AUTO_SCROLL_TIMER = 1500;
pub const AUTO_SCROLL_TIMER: i32 = 1500;
/// java: private static final float AUTO_SCROLL_SPEED = 1;
pub const AUTO_SCROLL_SPEED: f32 = ((1) as f32) as f32;
/// java: private static final Font font = minecraft.font;
pub static FONT: compat::MainThread<Font> = compat::MainThread::new(|| compat::font());

/// java: public class OptionDescriptionWidget extends AbstractWidget
pub struct OptionDescriptionWidget {
    /// java: extends net.minecraft.client.gui.components.AbstractWidget
    pub base: AbstractWidget,
    /// java: private @Nullable DescriptionWithName description;
    pub description: Option<DescriptionWithName>,
    /// java: private List<FormattedCharSequence> wrappedText;
    pub wrapped_text: Option<Vec<Seq>>,
    /// java: private final Supplier<ScreenRectangle> dimensions;
    pub dimensions: Arc<dyn Fn() -> ScreenRectangle>,
    /// java: private float targetScrollAmount, currentScrollAmount;
    pub target_scroll_amount: f32,
    /// java: private float targetScrollAmount, currentScrollAmount;
    pub current_scroll_amount: f32,
    /// java: private int maxScrollAmount;
    pub max_scroll_amount: i32,
    /// java: private int descriptionY;
    pub description_y: i32,
    /// java: private int lastInteractionTime;
    pub last_interaction_time: i32,
    /// java: private boolean scrollingBackward;
    pub scrolling_backward: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for OptionDescriptionWidget {
    type Target = AbstractWidget;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for OptionDescriptionWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl OptionDescriptionWidget {
    /// java: public OptionDescriptionWidget(Supplier<ScreenRectangle> dimensions, @Nullable DescriptionWithName description)
    pub fn new(dimensions: Arc<dyn Fn() -> ScreenRectangle>, mut description: Option<DescriptionWithName>) -> compat::Shared<Self> {
        let __init_dimensions = dimensions.clone();
        let __this = compat::shared(Self {
            base: AbstractWidget::new(0, 0, 0, 0, (&(if description.is_none() { component::empty() } else { description.as_ref().unwrap().name() }).get_string()).to_string(), WidgetKind::PlainText, OnPress::Inert),
            self_ref: std::sync::Weak::new(),
            description: None,
            wrapped_text: None,
            dimensions: __init_dimensions,
            target_scroll_amount: 0.0,
            current_scroll_amount: 0.0,
            max_scroll_amount: 0,
            description_y: 0,
            last_interaction_time: 0,
            scrolling_backward: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        OptionDescriptionWidget::set_option_description(&__this, description.clone().unwrap());
        __this
    }

    /// java: @Override public void extractWidgetRenderState(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if compat::read(__this, |__t| __t.description.clone()).is_none() {
            return;
        }
        { let __v = mth::lerp_f32((a * 0.5f32), compat::read(__this, |__t| __t.current_scroll_amount), compat::read(__this, |__t| __t.target_scroll_amount)); compat::write(__this, |__t| __t.current_scroll_amount = __v); }
        let mut dimensions: ScreenRectangle = ((compat::read(__this, |__t| __t.dimensions.clone())))();
        { let (__q0,) = (dimensions.left(),); __this.lock().borrow_mut().set_x(__q0) };
        { let (__q0,) = (dimensions.top(),); __this.lock().borrow_mut().set_y(__q0) };
        { let __v = dimensions.width(); compat::write(__this, |__t| __t.width = __v); }
        { let __v = dimensions.height(); compat::write(__this, |__t| __t.height = __v); }
        let mut y: i32 = __this.lock().borrow().get_y();
        let mut name_width: i32 = FONT.get_mut().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__this.lock().borrow().description.as_ref().unwrap().name()));
        if name_width > __this.lock().borrow().get_width() {
            // TODO(fabric2seer): field `TOOLTIP_ONLY` of a value of unknown type
            //   java: graphics.textRendererForWidget(this, GuiGraphicsExtractor.HoveredTextEffects.TOOLTIP_ONLY)
            //   java:                     .acceptScrollingWithDefaultCenter(description.name(), getX(), getX() + getWidth(), y, y + font.lineHeight);
        } else {
            gui_graphics_extractor::draw_formatted_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __this.lock().borrow().get_x(), y, &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__this.lock().borrow().description.as_ref().unwrap().name()), 0xFFFFFFFFu32 as i32);
        }
        y = y.wrapping_add(5i32.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT));
        graphics.enable_scissor(__this.lock().borrow().get_x(), y, (__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width()), (__this.lock().borrow().get_y()).wrapping_add(__this.lock().borrow().get_height()));
        y = y.wrapping_sub((compat::read(__this, |__t| __t.current_scroll_amount)) as i32);
        if ({ let __r = &__this.lock().borrow().description.as_ref().unwrap().description(); let __f = compat::dispatch(__r, |__t| __t.image()); __f(__r) }).is_done() {
            let mut image = ({ let __r = &__this.lock().borrow().description.as_ref().unwrap().description(); let __f = compat::dispatch(__r, |__t| __t.image()); __f(__r) }).join();
            // TODO(fabric2seer): the condition: `.isPresent(..)` on a T
            //   java: image.isPresent(..)
            if todo!("fabric2seer: the condition (TODO above)") {
                // TODO(fabric2seer): `.get(..)` on a T
                //   java: y += image.get().render(graphics, getX(), y, getWidth(), a) + 5;
            }
        }
        if compat::read(__this, |__t| __t.wrapped_text.clone()).is_none() {
            { let __v = Some(FONT.get_mut().split(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(({ let __r = &__this.lock().borrow().description.as_ref().unwrap().description(); let __f = compat::dispatch(__r, |__t| __t.text()); __f(__r) })), __this.lock().borrow().get_width())); compat::write(__this, |__t| __t.wrapped_text = __v); }
        }
        { let __v = y; compat::write(__this, |__t| __t.description_y = __v); }
        for line in __this.lock().borrow().wrapped_text.as_ref().unwrap().iter() {
            // TODO(fabric2seer): field `TOOLTIP_AND_CURSOR` of a value of unknown type
            //   java: graphics.textRenderer(GuiGraphicsExtractor.HoveredTextEffects.TOOLTIP_AND_CURSOR)
            //   java:                     .accept(getX(), y, line);
            y = y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT);
        }
        graphics.disable_scissor();
        { let __v = 0i32.max(((y.wrapping_add((compat::read(__this, |__t| __t.current_scroll_amount)) as i32)).wrapping_sub(__this.lock().borrow().get_y())).wrapping_sub(__this.lock().borrow().get_height())); compat::write(__this, |__t| __t.max_scroll_amount = __v); }
        if __this.lock().borrow().is_hovered_or_focused() {
            { let __v = OptionDescriptionWidget::current_time_ms(__this); compat::write(__this, |__t| __t.last_interaction_time = __v); }
        }
        if __this.lock().borrow().is_focused() {
            gui_graphics_extractor::outline(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __this.lock().borrow().get_x(), __this.lock().borrow().get_y(), __this.lock().borrow().get_width(), __this.lock().borrow().get_height(), -1);
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent mouseButtonEvent, boolean bl)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, mouse_button_event: &MouseButtonEvent, mut bl: bool) -> bool {
        return OptionDescriptionWidget::on_mouse_clicked(__this, mouse_button_event.x(), mouse_button_event.y());
    }

    /// java: protected boolean onMouseClicked(double mouseX, double mouseY)
    pub fn on_mouse_clicked(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        let mut clicked_style: Option<Style> = OptionDescriptionWidget::get_desc_style(__this, ((mouse_x) as i32), ((mouse_y) as i32));
        if clicked_style.is_some() && clicked_style.as_ref().unwrap().get_click_event().is_some() {
            return false;
        }
        return false;
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    pub fn mouse_scrolled(__this: &compat::Shared<Self>, mut mouse_x: f64, mut mouse_y: f64, mut horizontal: f64, mut vertical: f64) -> bool {
        if __this.lock().borrow().is_mouse_over(mouse_x, mouse_y) {
            { let __v = mth::clamp_f32((compat::read(__this, |__t| __t.target_scroll_amount) - (((vertical) as i32).wrapping_mul(10) as f32)), (0) as f32, (compat::read(__this, |__t| __t.max_scroll_amount)) as f32); compat::write(__this, |__t| __t.target_scroll_amount = __v); }
            { let __v = OptionDescriptionWidget::current_time_ms(__this); compat::write(__this, |__t| __t.last_interaction_time = __v); }
            return true;
        }
        return false;
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyEvent)
    pub fn key_pressed(__this: &compat::Shared<Self>, key_event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        return OptionDescriptionWidget::on_key_pressed(__this, key_event.key());
    }

    /// java: protected boolean onKeyPressed(int keyCode)
    pub fn on_key_pressed(__this: &compat::Shared<Self>, mut key_code: i32) -> bool {
        if __this.lock().borrow().is_focused() {
            match key_code {
                265 => {
                    { let __v = mth::clamp_f32((compat::read(__this, |__t| __t.target_scroll_amount) - (10 as f32)), (0) as f32, (compat::read(__this, |__t| __t.max_scroll_amount)) as f32); compat::write(__this, |__t| __t.target_scroll_amount = __v); }
                }
                264 => {
                    { let __v = mth::clamp_f32((compat::read(__this, |__t| __t.target_scroll_amount) + (10 as f32)), (0) as f32, (compat::read(__this, |__t| __t.max_scroll_amount)) as f32); compat::write(__this, |__t| __t.target_scroll_amount = __v); }
                }
                _ => {
                    return false;
                }
            }
            return true;
        }
        return false;
    }

    /// java: public void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        if compat::read(__this, |__t| __t.description.clone()).is_some() {
            // TODO(fabric2seer): `.ifPresent(..)` on a Result
            //   java: description.description().image()
            //   java:                     .getNow(Optional.empty())
            //   java:                     .ifPresent(ImageRenderer::tick);
        }
        let mut px_per_tick: f32 = ((AUTO_SCROLL_SPEED / 20.0f32) * (mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32));
        if compat::read(__this, |__t| __t.max_scroll_amount) > 0 && (OptionDescriptionWidget::current_time_ms(__this)).wrapping_sub(compat::read(__this, |__t| __t.last_interaction_time)) > AUTO_SCROLL_TIMER {
            if compat::read(__this, |__t| __t.scrolling_backward) {
                px_per_tick = (px_per_tick * (-1 as f32));
                if (compat::read(__this, |__t| __t.target_scroll_amount) + px_per_tick) < (0 as f32) {
                    { let __v = false; compat::write(__this, |__t| __t.scrolling_backward = __v); }
                    { let __v = OptionDescriptionWidget::current_time_ms(__this); compat::write(__this, |__t| __t.last_interaction_time = __v); }
                }
            } else {
                if (compat::read(__this, |__t| __t.target_scroll_amount) + px_per_tick) > (compat::read(__this, |__t| __t.max_scroll_amount) as f32) {
                    { let __v = true; compat::write(__this, |__t| __t.scrolling_backward = __v); }
                    { let __v = OptionDescriptionWidget::current_time_ms(__this); compat::write(__this, |__t| __t.last_interaction_time = __v); }
                }
            }
            { let __v = mth::clamp_f32((compat::read(__this, |__t| __t.target_scroll_amount) + px_per_tick), (0) as f32, (compat::read(__this, |__t| __t.max_scroll_amount)) as f32); compat::write(__this, |__t| __t.target_scroll_amount = __v); }
        }
    }

    /// java: private Style getDescStyle(int mouseX, int mouseY)
    pub fn get_desc_style(__this: &compat::Shared<Self>, mut mouse_x: i32, mut mouse_y: i32) -> Option<Style> {
        let mut clicked: bool = { let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().is_mouse_over(__q0, __q1) };
        if !clicked {
            return None;
        }
        let mut x: i32 = mouse_x.wrapping_sub(__this.lock().borrow().get_x());
        let mut y: i32 = mouse_y.wrapping_sub(compat::read(__this, |__t| __t.description_y));
        if x < 0 || x > (__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width()) {
            return None;
        }
        if y < 0 || y > (__this.lock().borrow().get_y()).wrapping_add(__this.lock().borrow().get_height()) {
            return None;
        }
        let mut line: i32 = y.wrapping_div(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT);
        if line >= (__this.lock().borrow().wrapped_text.as_ref().unwrap().len() as i32) {
            return None;
        }
        return None;
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(__this: &compat::Shared<Self>, builder: &mut dyn NarrationElementOutput) {
        if compat::read(__this, |__t| __t.description.clone()).is_some() {
            builder.add_component(NarratedElementType::Title, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__this.lock().borrow().description.as_ref().unwrap().name()));
            builder.add_component(NarratedElementType::Hint, mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(({ let __r = &__this.lock().borrow().description.as_ref().unwrap().description(); let __f = compat::dispatch(__r, |__t| __t.text()); __f(__r) })));
        }
    }

    /// java: public void setOptionDescription(DescriptionWithName description)
    pub fn set_option_description(__this: &compat::Shared<Self>, mut description: DescriptionWithName) {
        { let __v = Some(description.clone()); compat::write(__this, |__t| __t.description = __v); }
        { let __v = None; compat::write(__this, |__t| __t.wrapped_text = __v); }
        { let __v = ((0) as f32) as f32; compat::write(__this, |__t| __t.target_scroll_amount = __v); }
        { let __v = ((0) as f32) as f32; compat::write(__this, |__t| __t.current_scroll_amount = __v); }
        { let __v = OptionDescriptionWidget::current_time_ms(__this); compat::write(__this, |__t| __t.last_interaction_time = __v); }
    }

    /// java: private int currentTimeMS()
    pub fn current_time_ms(__this: &compat::Shared<Self>) -> i32 {
        return (((blaze3d::get_time() * (1000 as f64))) as i32);
    }

    /// java: @Nullable @Override public ComponentPath nextFocusPath(FocusNavigationEvent event)
    pub fn next_focus_path(__this: &compat::Shared<Self>, event: &FocusNavigationEvent) -> Option<ComponentPath> {
        return None;
    }

}

/// OptionDescriptionWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for OptionDescriptionWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base
    }
}

/// OptionDescriptionWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<OptionDescriptionWidget> {
    /// java: @Override public void extractWidgetRenderState(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(widget, |__h| OptionDescriptionWidget::extract_widget_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override protected void updateWidgetNarration(NarrationElementOutput builder)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        self.lend_ref(widget, |__h| OptionDescriptionWidget::update_widget_narration(__h, output))
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent mouseButtonEvent, boolean bl)
    fn mouse_clicked(&mut self, widget: &mut AbstractWidget, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.lend(widget, |__h| OptionDescriptionWidget::mouse_clicked(__h, event, double_click))
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    fn mouse_scrolled(&mut self, widget: &mut AbstractWidget, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        self.lend(widget, |__h| OptionDescriptionWidget::mouse_scrolled(__h, x, y, scroll_x, scroll_y))
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyEvent)
    fn key_pressed(&mut self, widget: &mut AbstractWidget, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(widget, |__h| OptionDescriptionWidget::key_pressed(__h, event))
    }

}

// NOTE(fabric2seer): extractWidgetRenderState, keyPressed, mouseClicked, mouseScrolled, nextFocusPath, updateWidgetNarration override(s) of AbstractWidget - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
