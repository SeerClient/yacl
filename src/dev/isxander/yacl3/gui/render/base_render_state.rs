//! Ported from dev.isxander.yacl3.gui.render.BaseRenderState.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::render::gui_render_state_sink;
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::{self, GuiGraphicsExtractor};
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup;
use mcrust::net::minecraft::client::renderer::render_pipelines;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::org::joml::matrix3x2f::Matrix3x2f;
use std::sync::Arc;

/// java: public record BaseRenderState(RenderPipeline pipeline, TextureSetup textureSetup, Matrix3x2f pose,
/// @Nullable ScreenRectangle bounds, @Nullable ScreenRectangle scissorArea)
#[derive(Clone)]
pub struct BaseRenderState {
    /// java: RenderPipeline pipeline
    pub pipeline: Arc<RenderPipeline>,
    /// java: TextureSetup textureSetup
    pub texture_setup: TextureSetup,
    /// java: Matrix3x2f pose
    pub pose: Matrix3x2f,
    /// java: @Nullable ScreenRectangle bounds
    pub bounds: Option<ScreenRectangle>,
    /// java: @Nullable ScreenRectangle scissorArea
    pub scissor_area: Option<ScreenRectangle>,
}

impl BaseRenderState {
    /// java: the canonical record constructor
    pub fn new(
        pipeline: Arc<RenderPipeline>,
        texture_setup: TextureSetup,
        pose: Matrix3x2f,
        bounds: Option<ScreenRectangle>,
        scissor_area: Option<ScreenRectangle>,
    ) -> Self {
        Self { pipeline, texture_setup, pose, bounds, scissor_area }
    }

    /// java: the record accessor `pipeline()`
    pub fn pipeline(&self) -> Arc<RenderPipeline> {
        self.pipeline.clone()
    }

    /// java: the record accessor `textureSetup()`
    pub fn texture_setup(&self) -> &TextureSetup {
        &self.texture_setup
    }

    /// java: the record accessor `pose()`
    pub fn pose(&self) -> Matrix3x2f {
        self.pose
    }

    /// java: the record accessor `bounds()`
    pub fn bounds(&self) -> Option<ScreenRectangle> {
        self.bounds
    }

    /// java: the record accessor `scissorArea()`
    pub fn scissor_area(&self) -> Option<ScreenRectangle> {
        self.scissor_area
    }

    /// java: public static BaseRenderState create(GuiGraphicsExtractor graphics, @Nullable Identifier texture,
    /// int x0, int y0, int x1, int y1)
    ///
    /// `None` only when `texture` names a texture the manager has not loaded yet (see `texture_setup_for`).
    pub fn create(
        graphics: &GuiGraphicsExtractor,
        texture: Option<&Identifier>,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
    ) -> Option<BaseRenderState> {
        let scissor_area = gui_render_state_sink::peek_scissor_stack(graphics);
        let pose = gui_graphics_extractor::current_pose();
        let bounds = BaseRenderState::bounds_from_max_points(x0, y0, x1, y1, &pose, scissor_area);

        Some(BaseRenderState::new(
            if texture.is_some() { render_pipelines::gui_textured() } else { render_pipelines::gui() },
            BaseRenderState::texture_setup_for(texture)?,
            pose,
            bounds,
            scissor_area,
        ))
    }

    /// java: public static BaseRenderState create(GuiGraphicsExtractor graphics, @Nullable Identifier texture)
    pub fn create_2(graphics: &GuiGraphicsExtractor, texture: Option<&Identifier>) -> Option<BaseRenderState> {
        Some(BaseRenderState::new(
            if texture.is_some() { render_pipelines::gui_textured() } else { render_pipelines::gui() },
            BaseRenderState::texture_setup_for(texture)?,
            gui_graphics_extractor::current_pose(),
            None,
            gui_render_state_sink::peek_scissor_stack(graphics),
        ))
    }

    /// java: private static TextureSetup textureSetup(@Nullable Identifier textureId)
    ///
    /// `TextureManager.getTexture` loads a missing texture synchronously; the port's manager requests it and answers
    /// `None` this frame (as the port's `GuiGraphicsExtractor.innerBlit` does, which then draws nothing).
    fn texture_setup_for(texture_id: Option<&Identifier>) -> Option<TextureSetup> {
        if let Some(texture_id) = texture_id {
            let mc = compat::minecraft();
            let texture = mc.get_texture_manager().get_texture_or_request(texture_id)?;
            return Some(TextureSetup::single_texture(texture.get_texture_view().clone(), texture.get_sampler().clone()));
        }
        Some(TextureSetup::no_texture())
    }

    /// java: private static ScreenRectangle boundsFromMaxPoints(int x0, int y0, int x1, int y1, Matrix3x2f pose,
    /// @Nullable ScreenRectangle scissorArea)
    fn bounds_from_max_points(
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        pose: &Matrix3x2f,
        scissor_area: Option<ScreenRectangle>,
    ) -> Option<ScreenRectangle> {
        let bounds = ScreenRectangle::new_at(x0, y0, x1 - x0, y1 - y0).transform_max_bounds(pose);
        match scissor_area {
            Some(scissor_area) => scissor_area.intersection(bounds),
            None => Some(bounds),
        }
    }
}
