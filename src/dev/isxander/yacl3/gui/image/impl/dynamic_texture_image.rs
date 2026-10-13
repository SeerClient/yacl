//! Ported from dev.isxander.yacl3.gui.image.impl.DynamicTextureImage.
//!
//! Java builds the `DynamicTexture` in the constructor (on the render thread) and registers it with the
//! `TextureManager` under `uniqueLocation`, then blits by that id. The port's `DynamicTexture` constructor takes the
//! frame's GL handle, which a mod only holds while it draws, so the texture is built on the first `render` and drawn
//! through its own view and sampler (the same texture `TextureManager.getTexture(uniqueLocation)` would answer);
//! `close` closes it where Java's `textureManager.release(uniqueLocation)` would.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::{ImageRendererFactory, ImageSupplier, OnThread};
use compat::Graphics;
use mcrust::com::mojang::blaze3d::platform::native_image::NativeImage;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::renderer::texture::dynamic_texture::DynamicTexture;
use mcrust::net::minecraft::resources::identifier::Identifier;
use std::cell::RefCell;

/// java: public class DynamicTextureImage implements ImageRenderer
pub struct DynamicTextureImage {
    /// java: protected NativeImage image; (until the texture is built from it on the first render)
    pub image: RefCell<Option<NativeImage>>,
    /// java: protected DynamicTexture texture;
    pub texture: RefCell<Option<DynamicTexture>>,
    /// `image != null` - Java nulls it in `close()`
    open: std::cell::Cell<bool>,
    /// java: protected final Identifier uniqueLocation;
    pub unique_location: Identifier,
    /// java: protected final int width, height;
    pub width: i32,
    pub height: i32,
    /// java: protected final boolean textureFiltering;
    pub texture_filtering: bool,
}

impl DynamicTextureImage {
    /// java: public DynamicTextureImage(NativeImage image, Identifier location, boolean textureFiltering)
    pub fn new(image: NativeImage, location: Identifier, texture_filtering: bool) -> Self {
        let width = image.get_width();
        let height = image.get_height();
        Self {
            image: RefCell::new(Some(image)),
            texture: RefCell::new(None),
            open: std::cell::Cell::new(true),
            unique_location: location,
            width,
            height,
            texture_filtering,
        }
    }

    /// `image == null`
    pub fn is_closed(&self) -> bool {
        !self.open.get()
    }

    /// `new DynamicTexture(location::toString, image)` on the first draw (see the header), then
    /// `graphics.blit(GUI_TEXTURED, uniqueLocation, 0, 0, u, v, width, height, textureWidth, textureHeight)`.
    #[allow(clippy::too_many_arguments)]
    pub fn blit(&self, graphics: &Graphics<'_>, u: f32, v: f32, width: i32, height: i32, texture_width: i32, texture_height: i32) {
        if self.texture.borrow().is_none() {
            let Some(image) = self.image.borrow_mut().take() else {
                return;
            };
            *self.texture.borrow_mut() = Some(DynamicTexture::new(graphics.gl, self.unique_location.to_string(), image));
        }
        let texture = self.texture.borrow();
        let Some(texture) = texture.as_ref() else {
            return;
        };
        if !texture.texture.is_loaded() {
            return;
        }
        gui_graphics_extractor::blit_atlas_uv(
            texture.get_texture_view().clone(),
            texture.texture.get_sampler().clone(),
            0,
            0,
            width,
            height,
            u / texture_width as f32,
            (u + width as f32) / texture_width as f32,
            v / texture_height as f32,
            (v + height as f32) / texture_height as f32,
            -1,
        );
    }

    /// java: public static ImageRendererFactory fromPath(Path imagePath, Identifier location, boolean textureFiltering)
    pub fn from_path(image_path: std::path::PathBuf, location: Identifier, texture_filtering: bool) -> Box<dyn ImageRendererFactory> {
        Box::new(OnThread(move || -> Result<Box<dyn ImageSupplier>, String> {
            let (path, location) = (image_path.clone(), location.clone());
            Ok(Box::new(move || -> Result<Arc<dyn ImageRenderer>, String> {
                // NativeImage.read(new FileInputStream(imagePath.toFile()))
                let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
                let image = NativeImage::read(&bytes)?;
                Ok(Arc::new(DynamicTextureImage::new(image, location, texture_filtering)))
            }))
        }))
    }
}

impl ImageRenderer for DynamicTextureImage {
    /// java: @Override public int render(GuiGraphicsExtractor graphics, int x, int y, int renderWidth, float tickDelta)
    fn render(&self, graphics: &Graphics<'_>, x: i32, y: i32, render_width: i32, _tick_delta: f32) -> i32 {
        if self.is_closed() {
            return 0;
        }

        let ratio = render_width as f32 / self.width as f32;
        let target_height = (self.height as f32 * ratio) as i32;

        graphics.push_pose();
        graphics.translate(x as f32, y as f32);
        graphics.scale(ratio, ratio);
        self.blit(graphics, 0.0, 0.0, self.width, self.height, self.width, self.height);
        graphics.pop_pose();

        target_height
    }

    /// java: @Override public void close()
    fn close(&self) {
        if let Some(mut image) = self.image.borrow_mut().take() {
            image.close();
        }
        self.open.set(false);
        if let Some(mut texture) = self.texture.borrow_mut().take() {
            texture.close();
        }
    }
}
