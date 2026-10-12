//! Ported from dev.isxander.yacl3.gui.render.ColorGradientRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::render::base_render_state::BaseRenderState;
use crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;

/// java: public record ColorGradientRenderState( BaseRenderState baseState, int x0, int y0, int x1, int y1, int x0y0Color, int x1y0Color, int x0y1Color, int x1y1Color ) implements YACLGuiElementRenderState
#[derive(Clone, Debug)]
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
    pub fn new(mut base_state: BaseRenderState, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut x0y0_color: i32, mut x1y0_color: i32, mut x0y1_color: i32, mut x1y1_color: i32) -> Self {
        Self { base_state, x0, y0, x1, y1, x0y0_color, x1y0_color, x0y1_color, x1y1_color }
    }

    /// java: the record accessor `baseState()`
    pub fn base_state(&self) -> BaseRenderState {
        self.base_state.clone()
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

    /// java: public static ColorGradientRenderState create( GuiGraphicsExtractor graphics, int x0, int y0, int x1, int y1, int x0y0Color, int x1y0Color, int x0y1Color, int x1y1Color )
    pub fn create(graphics: &GuiGraphicsExtractor, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut x0y0_color: i32, mut x1y0_color: i32, mut x0y1_color: i32, mut x1y1_color: i32) -> ColorGradientRenderState {
        return ColorGradientRenderState::new(BaseRenderState::create(graphics, None, x0, y0, x1, y1), x0, y0, x1, y1, x0y0_color, x1y0_color, x0y1_color, x1y1_color);
    }

    /// java: public static ColorGradientRenderState createHorizontal( GuiGraphicsExtractor graphics, int x0, int y0, int x1, int y1, int leftColor, int rightColor )
    pub fn create_horizontal(graphics: &GuiGraphicsExtractor, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut left_color: i32, mut right_color: i32) -> ColorGradientRenderState {
        return ColorGradientRenderState::create(graphics, x0, y0, x1, y1, left_color, right_color, left_color, right_color);
    }

    /// java: public static ColorGradientRenderState createVertical( GuiGraphicsExtractor graphics, int x0, int y0, int x1, int y1, int topColor, int bottomColor )
    pub fn create_vertical(graphics: &GuiGraphicsExtractor, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut top_color: i32, mut bottom_color: i32) -> ColorGradientRenderState {
        return ColorGradientRenderState::create(graphics, x0, y0, x1, y1, top_color, top_color, bottom_color, bottom_color);
    }

}

impl YACLGuiElementRenderState for ColorGradientRenderState {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void buildVertices(VertexConsumer vertexConsumer, float z) (dispatch: the body for this class)
    fn build_vertices_2(&self) -> fn(&compat::Shared<dyn YACLGuiElementRenderState>, &dyn VertexConsumer, f32) {
        ColorGradientRenderState::build_vertices_2_dyn
    }

}

impl ColorGradientRenderState {
    /// java: @Override public void buildVertices(VertexConsumer vertexConsumer, float z)
    pub fn build_vertices_2_impl(&self, vertex_consumer: &dyn VertexConsumer, mut z: f32) {
        // TODO(fabric2seer): `add2DVertex(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: add2DVertex(vertexConsumer, this.x0(), this.y0(), z).setColor(this.x0y0Color());
        // TODO(fabric2seer): `add2DVertex(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: add2DVertex(vertexConsumer, this.x0(), this.y1(), z).setColor(this.x0y1Color());
        // TODO(fabric2seer): `add2DVertex(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: add2DVertex(vertexConsumer, this.x1(), this.y1(), z).setColor(this.x1y1Color());
        // TODO(fabric2seer): `add2DVertex(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: add2DVertex(vertexConsumer, this.x1(), this.y0(), z).setColor(this.x1y0Color());
    }

    /// ColorGradientRenderState.buildVertices as the dispatch of YACLGuiElementRenderState.buildVertices
    pub fn build_vertices_2_dyn(__this: &compat::Shared<dyn YACLGuiElementRenderState>, __p0: &dyn VertexConsumer, __p1: f32) {
        compat::read_as::<Self, _, _>(__this, |__s| __s.build_vertices_2_impl(__p0, __p1))
    }

}
