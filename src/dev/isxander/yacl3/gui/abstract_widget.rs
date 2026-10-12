//! Ported from dev.isxander.yacl3.gui.AbstractWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::render::color_gradient_render_state::ColorGradientRenderState;
use crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState;
use crate::dev::isxander::yacl3::gui::utils::gui_utils;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::resources::sounds::simple_sound_instance::SimpleSoundInstance;

/// java: public abstract class AbstractWidget implements GuiEventListener, Renderable, NarratableEntry
pub struct AbstractWidgetBase {
    /// java: protected final Font textRenderer = client.font;
    pub text_renderer: Option<Font>,
    /// java: protected final int inactiveColor = 0xFFA0A0A0;
    pub inactive_color: i32,
    /// java: private Dimension<Integer> dim;
    pub dim: Option<compat::Shared<dyn Dimension<i32>>>,
}

impl AbstractWidgetBase {
    /// java: public AbstractWidget(Dimension<Integer> dim) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut dim: compat::Shared<dyn Dimension<i32>>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractWidget>, __R)) -> (compat::Shared<dyn AbstractWidget>, __R) {
        let __init_dim = Some(dim.clone());
        let this = Self {
            text_renderer: None,
            inactive_color: 0xFFA0A0A0u32 as i32,
            dim: __init_dim,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        { let __v = Some(compat::font()); compat::write(&__this, |__t| __t.abstract_widget_base_mut().text_renderer = __v); }
        (__this, __r)
    }

}

/// java: the virtual methods of AbstractWidget (implemented by AbstractWidgetBase and every subclass)
pub trait AbstractWidget: compat::DynAny {
    fn abstract_widget_base(&self) -> &AbstractWidgetBase;
    fn abstract_widget_base_mut(&mut self) -> &mut AbstractWidgetBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public boolean canReset() (dispatch: the body for this class)
    fn can_reset(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> bool {
        <dyn AbstractWidget>::can_reset_body
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY) (dispatch: the body for this class)
    fn is_mouse_over(&self) -> fn(&compat::Shared<dyn AbstractWidget>, f64, f64) -> bool {
        <dyn AbstractWidget>::is_mouse_over_body
    }

    /// java: public void setDimension(Dimension<Integer> dim) (dispatch: the body for this class)
    fn set_dimension(&self) -> fn(&compat::Shared<dyn AbstractWidget>, compat::Shared<dyn Dimension<i32>>) {
        <dyn AbstractWidget>::set_dimension_body
    }

    /// java: public Dimension<Integer> getDimension() (dispatch: the body for this class)
    fn get_dimension(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> compat::Shared<dyn Dimension<i32>> {
        <dyn AbstractWidget>::get_dimension_body
    }

    /// java: @Override public NarrationPriority narrationPriority() (dispatch: the body for this class)
    fn narration_priority(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> crate::unresolved::NarrationPriority {
        <dyn AbstractWidget>::narration_priority_body
    }

    /// java: public void unfocus() (dispatch: the body for this class)
    fn unfocus(&self) -> fn(&compat::Shared<dyn AbstractWidget>) {
        <dyn AbstractWidget>::unfocus_body
    }

    /// java: public boolean matchesSearch(String query) (dispatch: the body for this class)
    fn matches_search(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &str) -> bool {
        <dyn AbstractWidget>::matches_search_body
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_narration(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &mut dyn NarrationElementOutput) {
        <dyn AbstractWidget>::update_narration_body
    }

    /// java: protected void drawButtonRect(GuiGraphicsExtractor graphics, int x1, int y1, int x2, int y2, boolean hovered, boolean enabled) (dispatch: the body for this class)
    fn draw_button_rect(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &GuiGraphicsExtractor, i32, i32, i32, i32, bool, bool) {
        <dyn AbstractWidget>::draw_button_rect_body
    }

    /// java: protected void drawRainbowGradient(GuiGraphicsExtractor graphics, int x1, int y1, int x2, int y2) (dispatch: the body for this class)
    fn draw_rainbow_gradient(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &GuiGraphicsExtractor, i32, i32, i32, i32) {
        <dyn AbstractWidget>::draw_rainbow_gradient_body
    }

    /// java: protected int multiplyColor(int hex, float amount) (dispatch: the body for this class)
    fn multiply_color(&self) -> fn(&compat::Shared<dyn AbstractWidget>, i32, f32) -> i32 {
        <dyn AbstractWidget>::multiply_color_body
    }

    /// java: public void playDownSound() (dispatch: the body for this class)
    fn play_down_sound(&self) -> fn(&compat::Shared<dyn AbstractWidget>) {
        <dyn AbstractWidget>::play_down_sound_body
    }

}

impl dyn AbstractWidget {
    /// java: public boolean canReset()
    pub fn can_reset_body(__this: &compat::Shared<dyn AbstractWidget>) -> bool {
        return false;
    }

    /// java: @Override public boolean isMouseOver(double mouseX, double mouseY)
    pub fn is_mouse_over_body(__this: &compat::Shared<dyn AbstractWidget>, mut mouse_x: f64, mut mouse_y: f64) -> bool {
        if compat::read(__this, |__t| __t.abstract_widget_base().dim.clone()).is_none() {
            return false;
        }
        return { let (__a0, __a1,) = (&((mouse_x) as i32), &((mouse_y) as i32),); __this.lock().borrow().abstract_widget_base().dim.as_ref().unwrap().is_point_inside(__a0, __a1) };
    }

    /// java: public void setDimension(Dimension<Integer> dim)
    pub fn set_dimension_body(__this: &compat::Shared<dyn AbstractWidget>, mut dim: compat::Shared<dyn Dimension<i32>>) {
        { let __v = Some(dim.clone()); compat::write(__this, |__t| __t.abstract_widget_base_mut().dim = __v); }
    }

    /// java: public Dimension<Integer> getDimension()
    pub fn get_dimension_body(__this: &compat::Shared<dyn AbstractWidget>) -> compat::Shared<dyn Dimension<i32>> {
        return compat::read(__this, |__t| __t.abstract_widget_base().dim.clone()).unwrap();
    }

    /// java: @Override public NarrationPriority narrationPriority()
    pub fn narration_priority_body(__this: &compat::Shared<dyn AbstractWidget>) -> crate::unresolved::NarrationPriority {
        // TODO(fabric2seer): static field NarrationPriority.NONE
        //   java: return NarrationPriority.NONE;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public void unfocus()
    pub fn unfocus_body(__this: &compat::Shared<dyn AbstractWidget>) {
    }

    /// java: public boolean matchesSearch(String query)
    pub fn matches_search_body(__this: &compat::Shared<dyn AbstractWidget>, query: &str) -> bool {
        return true;
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder)
    pub fn update_narration_body(__this: &compat::Shared<dyn AbstractWidget>, builder: &mut dyn NarrationElementOutput) {
    }

    /// java: protected void drawButtonRect(GuiGraphicsExtractor graphics, int x1, int y1, int x2, int y2, boolean hovered, boolean enabled)
    pub fn draw_button_rect_body(__this: &compat::Shared<dyn AbstractWidget>, graphics: &GuiGraphicsExtractor, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, mut hovered: bool, mut enabled: bool) {
        if x1 > x2 {
            let mut xx1: i32 = x1;
            x1 = x2;
            x2 = xx1;
        }
        if y1 > y2 {
            let mut yy1: i32 = y1;
            y1 = y2;
            y2 = yy1;
        }
        let mut width: i32 = x2.wrapping_sub(x1);
        let mut height: i32 = y2.wrapping_sub(y1);
        gui_graphics_extractor::blit_sprite_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), gui_utils::BUTTON_SPRITES.get_mut().get(enabled, hovered), x1, y1, width, height);
    }

    /// java: protected void drawRainbowGradient(GuiGraphicsExtractor graphics, int x1, int y1, int x2, int y2)
    pub fn draw_rainbow_gradient_body(__this: &compat::Shared<dyn AbstractWidget>, graphics: &GuiGraphicsExtractor, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32) {
        let mut colors: Vec<i32> = vec![0xFFFF0000u32 as i32, 0xFFFFFF00u32 as i32, 0xFF00FF00u32 as i32, 0xFF00FFFFu32 as i32, 0xFF0000FFu32 as i32, 0xFFFF00FFu32 as i32, 0xFFFF0000u32 as i32];
        let mut width: i32 = x2.wrapping_sub(x1);
        let mut max_colors: i32 = (colors.len() as i32).wrapping_sub(1);
        for color in 0..max_colors {
            ({ let __r = &ColorGradientRenderState::create_horizontal(graphics, x1.wrapping_add((width.wrapping_div(max_colors)).wrapping_mul(color)), y1, (if color == max_colors.wrapping_sub(1) { x2 } else { x1.wrapping_add((width.wrapping_div(max_colors)).wrapping_mul(color.wrapping_add(1))) }), y2, colors[(color) as usize], colors[(color.wrapping_add(1)) as usize]); let __f = compat::dispatch(__r, |__t| __t.submit()); __f(&compat::view(__r, |a| a as compat::Shared<dyn YACLGuiElementRenderState>), graphics) });
        }
    }

    /// java: protected int multiplyColor(int hex, float amount)
    pub fn multiply_color_body(__this: &compat::Shared<dyn AbstractWidget>, mut hex: i32, mut amount: f32) -> i32 {
        // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 2
        // rust: let mut color: Color = Color::new(hex, true);
        let mut color: Color = todo!("fabric2seer: the initialiser of `color` (TODO above)");
        // TODO(fabric2seer): `.getRed(..)` on a Color
        //   java: return new Color(Math.max((int)(color.getRed() * amount), 0),
        //   java:                   Math.max((int)(color.getGreen() * amount), 0),
        //   java:                   Math.max((int)(color.getBlue() * amount), 0),
        //   java:                   color.getAlpha()).getRGB();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public void playDownSound()
    pub fn play_down_sound_body(__this: &compat::Shared<dyn AbstractWidget>) {
        // TODO(fabric2seer): `play` takes 4 argument(s) in the port (`_al: &OpenAl, library: &mut Library, options: &OptionsData, instance: Box<dyn SoundInstance>`), Java passes 1
        // rust: compat::minecraft().get_sound_manager().play(&SimpleSoundInstance::for_ui(mcrust::net::minecraft::sounds::sound_events::ui_button_click(), 1.0f32));
    }

}
