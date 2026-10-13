//! Ported from dev.isxander.yacl3.gui.render.ColorGradientRenderState.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::render::base_render_state::BaseRenderState;
use crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;

/// java: public record ColorGradientRenderState(BaseRenderState baseState, int x0, int y0, int x1, int y1,
/// int x0y0Color, int x1y0Color, int x0y1Color, int x1y1Color) implements YACLGuiElementRenderState
#[derive(Clone)]
pub struct ColorGradientRenderState {
    /// java: BaseRenderState baseState
    pub base_state: BaseRenderState,
    /// java: int x0
    pub x0: i32,
    /// java: int y0
    pub y0: i32,
    /// java: int x1
    pub x1: i32,
    /// java: int y1
    pub y1: i32,
    /// java: int x0y0Color
    pub x0y0_color: i32,
    /// java: int x1y0Color
    pub x1y0_color: i32,
    /// java: int x0y1Color
    pub x0y1_color: i32,
    /// java: int x1y1Color
    pub x1y1_color: i32,
}

impl ColorGradientRenderState {
    /// java: the canonical record constructor
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        base_state: BaseRenderState,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        x0y0_color: i32,
        x1y0_color: i32,
        x0y1_color: i32,
        x1y1_color: i32,
    ) -> Self {
        Self { base_state, x0, y0, x1, y1, x0y0_color, x1y0_color, x0y1_color, x1y1_color }
    }

    /// java: the record accessor `x0()`
    pub fn x0(&self) -> i32 {
        self.x0
    }

    /// java: the record accessor `y0()`
    pub fn y0(&self) -> i32 {
        self.y0
    }

    /// java: the record accessor `x1()`
    pub fn x1(&self) -> i32 {
        self.x1
    }

    /// java: the record accessor `y1()`
    pub fn y1(&self) -> i32 {
        self.y1
    }

    /// java: the record accessor `x0y0Color()`
    pub fn x0y0_color(&self) -> i32 {
        self.x0y0_color
    }

    /// java: the record accessor `x1y0Color()`
    pub fn x1y0_color(&self) -> i32 {
        self.x1y0_color
    }

    /// java: the record accessor `x0y1Color()`
    pub fn x0y1_color(&self) -> i32 {
        self.x0y1_color
    }

    /// java: the record accessor `x1y1Color()`
    pub fn x1y1_color(&self) -> i32 {
        self.x1y1_color
    }

    /// java: public static ColorGradientRenderState create(GuiGraphicsExtractor graphics, int x0, int y0, int x1,
    /// int y1, int x0y0Color, int x1y0Color, int x0y1Color, int x1y1Color)
    #[allow(clippy::too_many_arguments)]
    pub fn create(
        graphics: &GuiGraphicsExtractor,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        x0y0_color: i32,
        x1y0_color: i32,
        x0y1_color: i32,
        x1y1_color: i32,
    ) -> ColorGradientRenderState {
        // `BaseRenderState.create(graphics, null, ..)`: with no texture it always answers.
        let base_state = BaseRenderState::create(graphics, None, x0, y0, x1, y1)
            .expect("BaseRenderState.create with a null texture always answers");
        ColorGradientRenderState::new(base_state, x0, y0, x1, y1, x0y0_color, x1y0_color, x0y1_color, x1y1_color)
    }

    /// java: public static ColorGradientRenderState createHorizontal(GuiGraphicsExtractor graphics, int x0, int y0,
    /// int x1, int y1, int leftColor, int rightColor)
    pub fn create_horizontal(
        graphics: &GuiGraphicsExtractor,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        left_color: i32,
        right_color: i32,
    ) -> ColorGradientRenderState {
        ColorGradientRenderState::create(graphics, x0, y0, x1, y1, left_color, right_color, left_color, right_color)
    }

    /// java: public static ColorGradientRenderState createVertical(GuiGraphicsExtractor graphics, int x0, int y0,
    /// int x1, int y1, int topColor, int bottomColor)
    pub fn create_vertical(
        graphics: &GuiGraphicsExtractor,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        top_color: i32,
        bottom_color: i32,
    ) -> ColorGradientRenderState {
        ColorGradientRenderState::create(graphics, x0, y0, x1, y1, top_color, top_color, bottom_color, bottom_color)
    }
}

impl YACLGuiElementRenderState for ColorGradientRenderState {
    fn base_state(&self) -> &BaseRenderState {
        &self.base_state
    }

    /// java: @Override public void buildVertices(VertexConsumer vertexConsumer, float z)
    fn build_vertices_2(&self, vertex_consumer: &mut dyn VertexConsumer, z: f32) {
        self.add_2d_vertex(vertex_consumer, self.x0() as f32, self.y0() as f32, z).set_color(self.x0y0_color());
        self.add_2d_vertex(vertex_consumer, self.x0() as f32, self.y1() as f32, z).set_color(self.x0y1_color());
        self.add_2d_vertex(vertex_consumer, self.x1() as f32, self.y1() as f32, z).set_color(self.x1y1_color());
        self.add_2d_vertex(vertex_consumer, self.x1() as f32, self.y0() as f32, z).set_color(self.x1y0_color());
    }
}

crate::impl_gui_element_render_state!(ColorGradientRenderState);
