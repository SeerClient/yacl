//! Ported from dev.isxander.yacl3.gui.render.BaseRenderState by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::render::gui_render_state_sink;
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup;
use mcrust::net::minecraft::client::renderer::texture::abstract_texture::AbstractTexture;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::org::joml::matrix3x2f::Matrix3x2f;

/// java: public record BaseRenderState( RenderPipeline pipeline, TextureSetup textureSetup, Matrix3x2f pose, @Nullable ScreenRectangle bounds, @Nullable ScreenRectangle scissorArea )
#[derive(Clone, Debug)]
pub struct BaseRenderState {
    /// java: RenderPipeline pipeline
    pub pipeline: RenderPipeline,
    /// java: TextureSetup textureSetup
    pub texture_setup: TextureSetup,
    /// java: Matrix3x2f pose
    pub pose: Matrix3x2f,
    /// java: ScreenRectangle bounds
    pub bounds: Option<ScreenRectangle>,
    /// java: ScreenRectangle scissorArea
    pub scissor_area: Option<ScreenRectangle>,
}

impl BaseRenderState {
    /// java: the canonical record constructor
    pub fn new(mut pipeline: RenderPipeline, mut texture_setup: TextureSetup, mut pose: Matrix3x2f, mut bounds: Option<ScreenRectangle>, mut scissor_area: Option<ScreenRectangle>) -> Self {
        Self { pipeline, texture_setup, pose, bounds, scissor_area }
    }

    /// java: the record accessor `pipeline()`
    pub fn pipeline(&self) -> RenderPipeline {
        self.pipeline.clone()
    }

    /// java: the record accessor `textureSetup()`
    pub fn texture_setup(&self) -> TextureSetup {
        self.texture_setup.clone()
    }

    /// java: the record accessor `pose()`
    pub fn pose(&self) -> Matrix3x2f {
        self.pose.clone()
    }

    /// java: the record accessor `bounds()`
    pub fn bounds(&self) -> Option<ScreenRectangle> {
        self.bounds.clone()
    }

    /// java: the record accessor `scissorArea()`
    pub fn scissor_area(&self) -> Option<ScreenRectangle> {
        self.scissor_area.clone()
    }

    /// java: public static BaseRenderState create(GuiGraphicsExtractor graphics, @Nullable Identifier texture, int x0, int y0, int x1, int y1)
    pub fn create(graphics: &GuiGraphicsExtractor, mut texture: Option<Identifier>, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32) -> BaseRenderState {
        let mut scissor_area: ScreenRectangle = gui_render_state_sink::peek_scissor_stack(graphics);
        // TODO(fabric2seer): `pose` (a posestack) used as a value
        //   java: ScreenRectangle bounds = boundsFromMaxPoints(x0, y0, x1, y1, graphics.pose(), scissorArea);
        let mut bounds: ScreenRectangle = todo!("fabric2seer: the initialiser of `bounds` (TODO above)");
        // TODO(fabric2seer): `pose` (a posestack) used as a value
        //   java: return new BaseRenderState(
        //   java:                 texture != null ? RenderPipelines.GUI_TEXTURED : RenderPipelines.GUI,
        //   java:                 textureSetup(texture),
        //   java:                 new Matrix3x2f(graphics.pose()),
        //   java:                 bounds, scissorArea
        //   java:         );
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static BaseRenderState create(GuiGraphicsExtractor graphics, @Nullable Identifier texture)
    pub fn create_2(graphics: &GuiGraphicsExtractor, mut texture: Option<Identifier>) -> BaseRenderState {
        // TODO(fabric2seer): `pose` (a posestack) used as a value
        //   java: return new BaseRenderState(
        //   java:                 texture != null ? RenderPipelines.GUI_TEXTURED : RenderPipelines.GUI,
        //   java:                 textureSetup(texture),
        //   java:                 new Matrix3x2f(graphics.pose()),
        //   java:                 null, GuiRenderStateSink.peekScissorStack(graphics)
        //   java:         );
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static TextureSetup textureSetup(@Nullable Identifier textureId)
    pub fn texture_setup(mut texture_id: Option<Identifier>) -> TextureSetup {
        if let Some(texture_id) = &texture_id {
            let mut texture: AbstractTexture = compat::minecraft().get_texture_manager().get_texture_or_request(texture_id).unwrap();
            return TextureSetup::single_texture(texture.get_texture_view().clone(), texture.get_sampler().clone());
        }
        return TextureSetup::no_texture();
    }

    /// java: private static ScreenRectangle boundsFromMaxPoints(int x0, int y0, int x1, int y1, Matrix3x2f pose, @Nullable ScreenRectangle scissorArea)
    pub fn bounds_from_max_points(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, pose: &Matrix3x2f, mut scissor_area: Option<ScreenRectangle>) -> ScreenRectangle {
        let mut bounds: ScreenRectangle = (ScreenRectangle::new_at(x0, y0, x1.wrapping_sub(x0), y1.wrapping_sub(y0))).transform_max_bounds(pose);
        return (if scissor_area.is_some() { scissor_area.as_ref().unwrap().intersection(bounds.clone()) } else { Some(bounds.clone()) }).unwrap();
    }

}
