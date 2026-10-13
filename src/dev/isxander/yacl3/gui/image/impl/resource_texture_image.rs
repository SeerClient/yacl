//! Ported from dev.isxander.yacl3.gui.image.impl.ResourceTextureImage.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::{ImageRendererFactory, ImageSupplier, OnThread};
use compat::Graphics;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class ResourceTextureImage implements ImageRenderer
pub struct ResourceTextureImage {
    /// java: private final Identifier location;
    location: Identifier,
    /// java: private final int width, height;
    width: i32,
    height: i32,
    /// java: private final int textureWidth, textureHeight;
    texture_width: i32,
    texture_height: i32,
    /// java: private final float u, v;
    u: f32,
    v: f32,
}

impl ResourceTextureImage {
    /// java: public ResourceTextureImage(Identifier location, float u, float v, int width, int height, int textureWidth,
    /// int textureHeight)
    pub fn new(location: Identifier, u: f32, v: f32, width: i32, height: i32, texture_width: i32, texture_height: i32) -> Self {
        Self { location, width, height, texture_width, texture_height, u, v }
    }

    /// java: public static ImageRendererFactory createFactory(Identifier location, float u, float v, int width,
    /// int height, int textureWidth, int textureHeight)
    pub fn create_factory(
        location: Identifier,
        u: f32,
        v: f32,
        width: i32,
        height: i32,
        texture_width: i32,
        texture_height: i32,
    ) -> Box<dyn ImageRendererFactory> {
        Box::new(OnThread(move || -> Result<Box<dyn ImageSupplier>, String> {
            let location = location.clone();
            Ok(Box::new(move || -> Result<Arc<dyn ImageRenderer>, String> {
                Ok(Arc::new(ResourceTextureImage::new(location, u, v, width, height, texture_width, texture_height)))
            }))
        }))
    }
}

impl ImageRenderer for ResourceTextureImage {
    /// java: @Override public int render(GuiGraphicsExtractor graphics, int x, int y, int renderWidth, float tickDelta)
    fn render(&self, graphics: &Graphics<'_>, x: i32, y: i32, render_width: i32, _tick_delta: f32) -> i32 {
        let ratio = render_width as f32 / self.width as f32;
        let target_height = (self.height as f32 * ratio) as i32;

        graphics.push_pose();
        graphics.translate(x as f32, y as f32);
        graphics.scale(ratio, ratio);
        graphics.blit(&self.location, 0, 0, self.u, self.v, self.width, self.height, self.texture_width, self.texture_height);
        graphics.pop_pose();

        target_height
    }

    /// java: @Override public void close()
    fn close(&self) {}
}
