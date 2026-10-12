//! Ported from dev.isxander.yacl3.gui.image.impl.ResourceTextureImage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::ImageRendererFactory;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class ResourceTextureImage implements ImageRenderer
#[derive(Clone)]
pub struct ResourceTextureImage {
    /// java: private final Identifier location;
    pub location: Identifier,
    /// java: private final int width, height;
    pub width: i32,
    /// java: private final int width, height;
    pub height: i32,
    /// java: private final int textureWidth, textureHeight;
    pub texture_width: i32,
    /// java: private final int textureWidth, textureHeight;
    pub texture_height: i32,
    /// java: private final float u, v;
    pub u: f32,
    /// java: private final float u, v;
    pub v: f32,
}

impl ResourceTextureImage {
    /// java: public ResourceTextureImage(Identifier location, float u, float v, int width, int height, int textureWidth, int textureHeight)
    pub fn new(mut location: Identifier, mut u: f32, mut v: f32, mut width: i32, mut height: i32, mut texture_width: i32, mut texture_height: i32) -> compat::Shared<Self> {
        let __init_location = location.clone();
        let __init_width = width;
        let __init_height = height;
        let __init_texture_width = texture_width;
        let __init_texture_height = texture_height;
        let __init_u = u;
        let __init_v = v;
        let __this = compat::shared(Self {
            location: __init_location,
            width: __init_width,
            height: __init_height,
            texture_width: __init_texture_width,
            texture_height: __init_texture_height,
            u: __init_u,
            v: __init_v,
        });
        __this
    }

    /// java: public static ImageRendererFactory createFactory(Identifier location, float u, float v, int width, int height, int textureWidth, int textureHeight)
    pub fn create_factory(location: &Identifier, mut u: f32, mut v: f32, mut width: i32, mut height: i32, mut texture_width: i32, mut texture_height: i32) -> compat::Shared<dyn ImageRendererFactory> {
        return Arc::new(move || Arc::new(move || ResourceTextureImage::new(location.clone(), u, v, width, height, texture_width, texture_height)));
    }

}

impl ImageRenderer for ResourceTextureImage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public int render(GuiGraphicsExtractor graphics, int x, int y, int renderWidth, float tickDelta) (dispatch: the body for this class)
    fn render(&self) -> fn(&compat::Shared<dyn ImageRenderer>, &GuiGraphicsExtractor, i32, i32, i32, f32) -> i32 {
        ResourceTextureImage::render_dyn
    }

    /// java: @Override public void close() (dispatch: the body for this class)
    fn close(&self) -> fn(&compat::Shared<dyn ImageRenderer>) {
        ResourceTextureImage::close_dyn
    }

}

impl ResourceTextureImage {
    /// java: @Override public int render(GuiGraphicsExtractor graphics, int x, int y, int renderWidth, float tickDelta)
    pub fn render_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut render_width: i32, mut tick_delta: f32) -> i32 {
        let mut ratio: f32 = ((render_width as f32) / ((compat::read(__this, |__t| __t.width)) as f32));
        let mut target_height: i32 = ((((compat::read(__this, |__t| __t.height) as f32) * ratio)) as i32);
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((x) as f32, (y) as f32);
        gui_graphics_extractor::pose_scale((ratio) as f32, (ratio) as f32);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &compat::read(__this, |__t| __t.location.clone()), 0, 0, compat::read(__this, |__t| __t.u), compat::read(__this, |__t| __t.v), compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height), compat::read(__this, |__t| __t.texture_width), compat::read(__this, |__t| __t.texture_height));
        gui_graphics_extractor::pop_pose(__pose_1);
        return target_height;
    }

    /// ResourceTextureImage.render as the dispatch of ImageRenderer.render
    pub fn render_dyn(__this: &compat::Shared<dyn ImageRenderer>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: i32, __p4: f32) -> i32 {
        Self::render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void close()
    pub fn close_impl(__this: &compat::Shared<Self>) {
    }

    /// ResourceTextureImage.close as the dispatch of ImageRenderer.close
    pub fn close_dyn(__this: &compat::Shared<dyn ImageRenderer>) {
        Self::close_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
