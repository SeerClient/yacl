//! Ported from dev.isxander.yacl3.gui.AbstractWidget.
//!
//! YACL's own widget class (not the game's `net.minecraft...AbstractWidget`): it implements `GuiEventListener`,
//! `Renderable` and `NarratableEntry`, which is `compat::GuiEventListener` plus this trait. A widget is held as
//! `Arc<dyn AbstractWidget>`; its methods take `&self` and its fields are cells (PATTERNS_Y2.md P1).
//!
//! A class that `extends AbstractWidget` keeps an `AbstractWidgetBase` (the `dim` field) and implements both traits;
//! the base bodies are the free functions below, which an override calls for `super.x(..)`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::render::color_gradient_render_state::ColorGradientRenderState;
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;
use compat::{Graphics, GuiEventListener};
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarrationPriority;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use std::cell::RefCell;

/// java: public abstract class AbstractWidget implements GuiEventListener, Renderable, NarratableEntry - its fields
pub struct AbstractWidgetBase {
    /// java: private Dimension<Integer> dim;
    pub dim: RefCell<Option<Arc<dyn Dimension<i32>>>>,
}

impl AbstractWidgetBase {
    /// java: protected final int inactiveColor = 0xFFA0A0A0;
    pub const INACTIVE_COLOR: i32 = 0xFFA0A0A0u32 as i32;

    /// java: public AbstractWidget(Dimension<Integer> dim)
    pub fn new(dim: Arc<dyn Dimension<i32>>) -> AbstractWidgetBase {
        AbstractWidgetBase { dim: RefCell::new(Some(dim)) }
    }

    /// java: public void setDimension(Dimension<Integer> dim)
    pub fn set_dimension(&self, dim: Arc<dyn Dimension<i32>>) {
        *self.dim.borrow_mut() = Some(dim);
    }

    /// java: public Dimension<Integer> getDimension()
    pub fn get_dimension(&self) -> Arc<dyn Dimension<i32>> {
        self.dim.borrow().clone().expect("AbstractWidget.getDimension() before a dimension was set")
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    pub fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        match &*self.dim.borrow() {
            None => false,
            Some(dim) => dim.is_point_inside(mouse_x as i32, mouse_y as i32),
        }
    }
}

/// java: the methods `AbstractWidget` declares (its `GuiEventListener` / `Renderable` half is the supertrait)
pub trait AbstractWidget: GuiEventListener + compat::DynAny {
    /// java: public boolean canReset()
    fn can_reset(&self) -> bool {
        false
    }

    /// java: public void setDimension(Dimension<Integer> dim)
    fn set_dimension(&self, dim: Arc<dyn Dimension<i32>>);

    /// java: public Dimension<Integer> getDimension()
    fn get_dimension(&self) -> Arc<dyn Dimension<i32>>;

    /// java: @Override public NarrationPriority narrationPriority()
    fn narration_priority(&self) -> NarrationPriority {
        NarrationPriority::None
    }

    /// java: public void unfocus()
    fn unfocus(&self) {}

    /// java: public boolean matchesSearch(String query)
    fn matches_search(&self, query: &str) -> bool {
        let _ = query;
        true
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder)
    fn update_narration(&self, builder: &mut dyn NarrationElementOutput) {
        let _ = builder;
    }

    /// java: public void playDownSound()
    fn play_down_sound(&self) {
        compat::play_down_sound();
    }
}

/// java: protected final Font textRenderer = client.font;
pub fn text_renderer() -> mcrust::net::minecraft::client::gui::font::Font<'static> {
    compat::font()
}

/// java: protected void drawButtonRect(GuiGraphicsExtractor graphics, int x1, int y1, int x2, int y2, boolean hovered, boolean enabled)
#[allow(clippy::too_many_arguments)]
pub fn draw_button_rect(graphics: &Graphics<'_>, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, hovered: bool, enabled: bool) {
    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
    }
    if y1 > y2 {
        std::mem::swap(&mut y1, &mut y2);
    }
    let width = x2 - x1;
    let height = y2 - y1;
    let sprite = GuiUtils::button_sprites().get(enabled, hovered).clone();
    graphics.blit_sprite(&sprite, x1, y1, width, height);
}

/// java: protected void drawRainbowGradient(GuiGraphicsExtractor graphics, int x1, int y1, int x2, int y2)
pub fn draw_rainbow_gradient(graphics: &Graphics<'_>, x1: i32, y1: i32, x2: i32, y2: i32) {
    // Draws a rainbow gradient, left to right
    let colors: [i32; 7] = [
        0xFFFF0000u32 as i32,
        0xFFFFFF00u32 as i32,
        0xFF00FF00u32 as i32,
        0xFF00FFFFu32 as i32,
        0xFF0000FFu32 as i32,
        0xFFFF00FFu32 as i32,
        0xFFFF0000u32 as i32,
    ];
    let width = x2 - x1;
    let max_colors = colors.len() as i32 - 1;
    for color in 0..max_colors {
        ColorGradientRenderState::create_horizontal(
            graphics,
            x1 + (width / max_colors * color),
            y1,
            if color == max_colors - 1 { x2 } else { x1 + (width / max_colors * (color + 1)) },
            y2,
            colors[color as usize],
            colors[color as usize + 1],
        )
        .submit(graphics);
    }
}

/// java: protected int multiplyColor(int hex, float amount)
pub fn multiply_color(hex: i32, amount: f32) -> i32 {
    let color = Color::from_rgba(hex, true);
    Color::new_with_alpha(
        ((color.get_red() as f32 * amount) as i32).max(0),
        ((color.get_green() as f32 * amount) as i32).max(0),
        ((color.get_blue() as f32 * amount) as i32).max(0),
        color.get_alpha(),
    )
    .get_rgb()
}
