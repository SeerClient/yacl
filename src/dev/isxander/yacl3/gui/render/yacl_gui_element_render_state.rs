//! Ported from dev.isxander.yacl3.gui.render.YACLGuiElementRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::render::base_render_state::BaseRenderState;
use crate::dev::isxander::yacl3::gui::render::gui_render_state_sink;
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup;

/// java: public interface YACLGuiElementRenderState extends GuiElementRenderState
pub trait YACLGuiElementRenderState: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: BaseRenderState baseState(); (dispatch: the body for this class)
    fn base_state(&self) -> fn(&compat::Shared<dyn YACLGuiElementRenderState>) -> BaseRenderState;

    /// java: @Override default void buildVertices(VertexConsumer vertexConsumer) (dispatch: the body for this class)
    fn build_vertices(&self) -> fn(&compat::Shared<dyn YACLGuiElementRenderState>, &dyn VertexConsumer) {
        <dyn YACLGuiElementRenderState>::build_vertices_body
    }

    /// java: void buildVertices(VertexConsumer vertexConsumer, float z); (dispatch: the body for this class)
    fn build_vertices_2(&self) -> fn(&compat::Shared<dyn YACLGuiElementRenderState>, &dyn VertexConsumer, f32);

    /// java: @Override default @NotNull RenderPipeline pipeline() (dispatch: the body for this class)
    fn pipeline(&self) -> fn(&compat::Shared<dyn YACLGuiElementRenderState>) -> RenderPipeline {
        <dyn YACLGuiElementRenderState>::pipeline_body
    }

    /// java: @Override default @NotNull TextureSetup textureSetup() (dispatch: the body for this class)
    fn texture_setup(&self) -> fn(&compat::Shared<dyn YACLGuiElementRenderState>) -> TextureSetup {
        <dyn YACLGuiElementRenderState>::texture_setup_body
    }

    /// java: @Override default @Nullable ScreenRectangle scissorArea() (dispatch: the body for this class)
    fn scissor_area(&self) -> fn(&compat::Shared<dyn YACLGuiElementRenderState>) -> Option<ScreenRectangle> {
        <dyn YACLGuiElementRenderState>::scissor_area_body
    }

    /// java: @Override default @Nullable ScreenRectangle bounds() (dispatch: the body for this class)
    fn bounds(&self) -> fn(&compat::Shared<dyn YACLGuiElementRenderState>) -> Option<ScreenRectangle> {
        <dyn YACLGuiElementRenderState>::bounds_body
    }

    /// java: default VertexConsumer add2DVertex( VertexConsumer vertexConsumer, float x, float y, float z ) (dispatch: the body for this class)
    fn add2_d_vertex(&self) -> fn(&compat::Shared<dyn YACLGuiElementRenderState>, &dyn VertexConsumer, f32, f32, f32) -> Box<dyn VertexConsumer> {
        <dyn YACLGuiElementRenderState>::add2_d_vertex_body
    }

    /// java: default void submit(GuiGraphicsExtractor graphics) (dispatch: the body for this class)
    fn submit(&self) -> fn(&compat::Shared<dyn YACLGuiElementRenderState>, &GuiGraphicsExtractor) {
        <dyn YACLGuiElementRenderState>::submit_body
    }

}

impl dyn YACLGuiElementRenderState {
    /// java: @Override default void buildVertices(VertexConsumer vertexConsumer)
    pub fn build_vertices_body(__this: &compat::Shared<dyn YACLGuiElementRenderState>, vertex_consumer: &dyn VertexConsumer) {
        ({ let __f = compat::dispatch(__this, |__t| __t.build_vertices_2()); __f(__this, vertex_consumer, ((0) as f32) as f32) });
    }

    /// java: @Override default @NotNull RenderPipeline pipeline()
    pub fn pipeline_body(__this: &compat::Shared<dyn YACLGuiElementRenderState>) -> RenderPipeline {
        return ({ let __f = compat::dispatch(__this, |__t| __t.base_state()); __f(__this) }).pipeline();
    }

    /// java: @Override default @NotNull TextureSetup textureSetup()
    pub fn texture_setup_body(__this: &compat::Shared<dyn YACLGuiElementRenderState>) -> TextureSetup {
        return ({ let __f = compat::dispatch(__this, |__t| __t.base_state()); __f(__this) }).texture_setup();
    }

    /// java: @Override default @Nullable ScreenRectangle scissorArea()
    pub fn scissor_area_body(__this: &compat::Shared<dyn YACLGuiElementRenderState>) -> Option<ScreenRectangle> {
        return ({ let __f = compat::dispatch(__this, |__t| __t.base_state()); __f(__this) }).scissor_area();
    }

    /// java: @Override default @Nullable ScreenRectangle bounds()
    pub fn bounds_body(__this: &compat::Shared<dyn YACLGuiElementRenderState>) -> Option<ScreenRectangle> {
        return ({ let __f = compat::dispatch(__this, |__t| __t.base_state()); __f(__this) }).bounds();
    }

    /// java: default VertexConsumer add2DVertex( VertexConsumer vertexConsumer, float x, float y, float z )
    pub fn add2_d_vertex_body(__this: &compat::Shared<dyn YACLGuiElementRenderState>, vertex_consumer: &dyn VertexConsumer, mut x: f32, mut y: f32, mut z: f32) -> Box<dyn VertexConsumer> {
        return { vertex_consumer.add_vertex_with_2d_pose(&({ let __f = compat::dispatch(__this, |__t| __t.base_state()); __f(__this) }).pose(), x, y); &mut *vertex_consumer }.clone();
    }

    /// java: default void submit(GuiGraphicsExtractor graphics)
    pub fn submit_body(__this: &compat::Shared<dyn YACLGuiElementRenderState>, graphics: &GuiGraphicsExtractor) {
        gui_render_state_sink::submit(graphics, __this);
    }

}
