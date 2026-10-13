//! Ported from dev.isxander.yacl3.gui.OptionDescriptionWidget.
//!
//! `extends net.minecraft.client.gui.components.AbstractWidget`: drawn and dispatched by the YACL screen itself
//! (PATTERNS_Y2.md P3), with the game widget's fields in a `compat::WidgetFields`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::description_with_name::DescriptionWithName;
use compat::{Graphics, GuiEventListener, WidgetFields};
use mcrust::com::mojang::blaze3d::platform::input_constants;
use mcrust::net::minecraft::client::gui::active_text_collector;
use mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
use mcrust::net::minecraft::client::gui::narration::narrated_element_type::NarratedElementType;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::util::mth;
use std::cell::{Cell, RefCell};

/// java: private static final int AUTO_SCROLL_TIMER = 1500;
const AUTO_SCROLL_TIMER: i32 = 1500;
/// java: private static final float AUTO_SCROLL_SPEED = 1; // lines per second
const AUTO_SCROLL_SPEED: f32 = 1.0;

/// java: public class OptionDescriptionWidget extends AbstractWidget
pub struct OptionDescriptionWidget {
    /// java: extends AbstractWidget (the game's)
    pub fields: WidgetFields,
    /// java: private @Nullable DescriptionWithName description;
    description: RefCell<Option<DescriptionWithName>>,
    /// java: private List<FormattedCharSequence> wrappedText;
    wrapped_text: RefCell<Option<Vec<FormattedText>>>,
    /// java: private final Supplier<ScreenRectangle> dimensions;
    dimensions: Box<dyn Fn() -> ScreenRectangle>,
    /// java: private float targetScrollAmount, currentScrollAmount;
    target_scroll_amount: Cell<f32>,
    current_scroll_amount: Cell<f32>,
    /// java: private int maxScrollAmount;
    max_scroll_amount: Cell<i32>,
    /// java: private int descriptionY;
    description_y: Cell<i32>,
    /// java: private int lastInteractionTime;
    last_interaction_time: Cell<i32>,
    /// java: private boolean scrollingBackward;
    scrolling_backward: Cell<bool>,
}

impl OptionDescriptionWidget {
    /// java: public OptionDescriptionWidget(Supplier<ScreenRectangle> dimensions, @Nullable DescriptionWithName description)
    pub fn new(dimensions: impl Fn() -> ScreenRectangle + 'static, description: Option<DescriptionWithName>) -> Self {
        let message = description.as_ref().map_or_else(component::empty, |d| d.name());
        let widget = Self {
            fields: WidgetFields::new(0, 0, 0, 0, message),
            description: RefCell::new(None),
            wrapped_text: RefCell::new(None),
            dimensions: Box::new(dimensions),
            target_scroll_amount: Cell::new(0.0),
            current_scroll_amount: Cell::new(0.0),
            max_scroll_amount: Cell::new(0),
            description_y: Cell::new(0),
            last_interaction_time: Cell::new(0),
            scrolling_backward: Cell::new(false),
        };
        widget.set_option_description(description);
        widget
    }

    /// java: protected boolean onMouseClicked(double mouseX, double mouseY)
    pub fn on_mouse_clicked(&self, mouse_x: f64, mouse_y: f64) -> bool {
        let clicked_style = self.get_desc_style(mouse_x as i32, mouse_y as i32);
        if clicked_style.is_some_and(|style| style.get_click_event().is_some()) {
            // TODO: reimplement (YACL's own TODO)
            return false;
        }
        false
    }

    /// java: protected boolean onKeyPressed(int keyCode)
    pub fn on_key_pressed(&self, key_code: i32) -> bool {
        if self.is_focused() {
            let max = self.max_scroll_amount.get() as f32;
            match key_code {
                input_constants::KEY_UP => self.target_scroll_amount.set(mth::clamp_f32(self.target_scroll_amount.get() - 10.0, 0.0, max)),
                input_constants::KEY_DOWN => self.target_scroll_amount.set(mth::clamp_f32(self.target_scroll_amount.get() + 10.0, 0.0, max)),
                _ => return false,
            }
            return true;
        }
        false
    }

    /// java: public void tick()
    pub fn tick(&self) {
        let description = self.description.borrow().clone();
        if let Some(description) = description
            && let Some(image) = description.description().image().get_now(None)
        {
            image.tick();
        }

        let mut px_per_tick = AUTO_SCROLL_SPEED / 20.0 * LINE_HEIGHT as f32;
        if self.max_scroll_amount.get() > 0 && self.current_time_ms() - self.last_interaction_time.get() > AUTO_SCROLL_TIMER {
            if self.scrolling_backward.get() {
                px_per_tick *= -1.0;
                if self.target_scroll_amount.get() + px_per_tick < 0.0 {
                    self.scrolling_backward.set(false);
                    self.last_interaction_time.set(self.current_time_ms());
                }
            } else if self.target_scroll_amount.get() + px_per_tick > self.max_scroll_amount.get() as f32 {
                self.scrolling_backward.set(true);
                self.last_interaction_time.set(self.current_time_ms());
            }

            self.target_scroll_amount.set(mth::clamp_f32(
                self.target_scroll_amount.get() + px_per_tick,
                0.0,
                self.max_scroll_amount.get() as f32,
            ));
        }
    }

    /// java: private Style getDescStyle(int mouseX, int mouseY)
    fn get_desc_style(&self, mouse_x: i32, mouse_y: i32) -> Option<Style> {
        let clicked = self.is_mouse_over(mouse_x as f64, mouse_y as f64);
        if !clicked {
            return None;
        }

        let f = &self.fields;
        let x = mouse_x - f.get_x();
        let y = mouse_y - self.description_y.get();

        if x < 0 || x > f.get_x() + f.get_width() {
            return None;
        }
        if y < 0 || y > f.get_y() + f.get_height() {
            return None;
        }

        let line = y / LINE_HEIGHT;
        if line >= self.wrapped_text.borrow().as_ref().map_or(0, |lines| lines.len() as i32) {
            return None;
        }

        // TODO reimplement (YACL's own TODO)
        None
    }

    /// java: protected void updateWidgetNarration(NarrationElementOutput builder)
    pub fn update_widget_narration(&self, builder: &mut dyn NarrationElementOutput) {
        if let Some(description) = self.description.borrow().as_ref() {
            builder.add_component(NarratedElementType::Title, FormattedText::from(description.name()));
            builder.add_component(NarratedElementType::Hint, FormattedText::from(description.description().text()));
        }
    }

    /// java: public void setOptionDescription(DescriptionWithName description)
    pub fn set_option_description(&self, description: Option<DescriptionWithName>) {
        *self.description.borrow_mut() = description;
        *self.wrapped_text.borrow_mut() = None;
        self.target_scroll_amount.set(0.0);
        self.current_scroll_amount.set(0.0);
        self.last_interaction_time.set(self.current_time_ms());
    }

    /// java: private int currentTimeMS()
    fn current_time_ms(&self) -> i32 {
        (mcrust::com::mojang::blaze3d::blaze3d::get_time() * 1000.0) as i32
    }

    /// java: @Nullable @Override public ComponentPath nextFocusPath(FocusNavigationEvent event) - `null`: prevents
    /// focusing on this widget
    pub fn next_focus_path_is_leaf(&self) -> bool {
        false
    }
}

impl GuiEventListener for OptionDescriptionWidget {
    /// `AbstractWidget.extractRenderState` -> java: @Override public void extractWidgetRenderState(@NonNull
    /// GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let f = &self.fields;
        if !f.visible.get() {
            return;
        }
        f.update_hover(mouse_x, mouse_y);

        let Some(description) = self.description.borrow().clone() else {
            return;
        };

        self.current_scroll_amount.set(mth::lerp_f32(a * 0.5, self.current_scroll_amount.get(), self.target_scroll_amount.get()));

        let dimensions = (self.dimensions)();
        f.set_x(dimensions.left());
        f.set_y(dimensions.top());
        f.set_width(dimensions.width());
        f.set_height(dimensions.height());

        let font = compat::font();
        let mut y = f.get_y();
        let name = FormattedText::from(description.name());
        let name_width = font.width_of(&name);
        if name_width > f.get_width() {
            // graphics.textRendererForWidget(this, TOOLTIP_ONLY).acceptScrollingWithDefaultCenter(name, x, x + width, y, y + lineHeight)
            let placement = active_text_collector::accept_scrolling_with_default_center(
                f.get_x(),
                f.get_x() + f.get_width(),
                y,
                y + LINE_HEIGHT,
                name_width,
                mcrust::net::minecraft::util::util::get_millis(),
            );
            if let Some((left, right, top, bottom)) = placement.scissor {
                graphics.enable_scissor(left, top, right, bottom);
            }
            graphics.text(&name, placement.x, placement.y, mcrust::net::minecraft::util::argb::white_f32(f.alpha.get()), true);
            if placement.scissor.is_some() {
                graphics.disable_scissor();
            }
        } else {
            graphics.text(&name, f.get_x(), y, 0xFFFFFFFFu32 as i32, true);
        }

        y += 5 + LINE_HEIGHT;

        graphics.enable_scissor(f.get_x(), y, f.get_x() + f.get_width(), f.get_y() + f.get_height());

        y -= self.current_scroll_amount.get() as i32;

        let image = description.description().image();
        if image.is_done()
            && let Some(image) = image.join()
        {
            y += image.render(graphics, f.get_x(), y, f.get_width(), a) + 5;
        }

        if self.wrapped_text.borrow().is_none() {
            let lines = font.split(&FormattedText::from(description.description().text()), f.get_width());
            *self.wrapped_text.borrow_mut() = Some(lines);
        }
        self.description_y.set(y);
        for line in self.wrapped_text.borrow().as_ref().into_iter().flatten() {
            // graphics.textRenderer(HoveredTextEffects.TOOLTIP_AND_CURSOR).accept(getX(), y, line) - white, shadowed
            graphics.text(line, f.get_x(), y, -1, true);
            y += LINE_HEIGHT;
        }

        graphics.disable_scissor();

        self.max_scroll_amount.set(0.max(y + self.current_scroll_amount.get() as i32 - f.get_y() - f.get_height()));

        if f.is_hovered_or_focused() {
            self.last_interaction_time.set(self.current_time_ms());
        }

        if self.is_focused() {
            graphics.outline(f.get_x(), f.get_y(), f.get_width(), f.get_height(), -1);
        }
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent mouseButtonEvent, boolean bl)
    fn mouse_clicked(&self, event: &MouseButtonEvent, _double_click: bool) -> bool {
        self.on_mouse_clicked(event.x(), event.y())
    }

    /// java: @Override public boolean mouseScrolled(double mouseX, double mouseY, double horizontal, double vertical)
    fn mouse_scrolled(&self, mouse_x: f64, mouse_y: f64, _horizontal: f64, vertical: f64) -> bool {
        if self.is_mouse_over(mouse_x, mouse_y) {
            self.target_scroll_amount.set(mth::clamp_f32(
                self.target_scroll_amount.get() - (vertical as i32 * 10) as f32,
                0.0,
                self.max_scroll_amount.get() as f32,
            ));
            self.last_interaction_time.set(self.current_time_ms());
            return true;
        }
        false
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyEvent)
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        self.on_key_pressed(event.key())
    }

    /// `AbstractWidget.isMouseOver`
    fn is_mouse_over(&self, x: f64, y: f64) -> bool {
        self.fields.is_mouse_over(x, y)
    }

    fn set_focused(&self, focused: bool) {
        self.fields.focused.set(focused);
    }

    fn is_focused(&self) -> bool {
        self.fields.focused.get()
    }
}
