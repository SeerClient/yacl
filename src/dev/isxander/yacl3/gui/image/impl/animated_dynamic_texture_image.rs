//! Ported from dev.isxander.yacl3.gui.image.impl.AnimatedDynamicTextureImage.
//!
//! The frame packing, the per-frame delays and the playback are ported. Decoding the frames is not:
//! rust: port gap (GAPS_Y2.md G10) - Java reads GIF through `javax.imageio` and WebP through TwelveMonkeys'
//! `WebPImageReader`; the port's image decoder (`lwjgl::stb_image`) reads PNG only, so `createGIFSupplier` /
//! `createWEBPSupplier` fail and the image is logged and left out (as Java's `safelyPrepareFactory` does for an image
//! that does not decode).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::{FnFactory, ImageRendererFactory, ImageSupplier};
use crate::dev::isxander::yacl3::gui::image::r#impl::dynamic_texture_image::DynamicTextureImage;
use compat::Graphics;
use mcrust::com::mojang::blaze3d::platform::native_image::NativeImage;
use mcrust::net::minecraft::resources::identifier::Identifier;
use std::cell::Cell;

/// java: public class AnimatedDynamicTextureImage extends DynamicTextureImage
pub struct AnimatedDynamicTextureImage {
    /// java: extends DynamicTextureImage
    pub base: DynamicTextureImage,
    /// java: private int currentFrame;
    current_frame: Cell<i32>,
    /// java: private double lastFrameTime;
    last_frame_time: Cell<f64>,
    /// java: private final double[] frameDelays;
    frame_delays: Vec<f64>,
    /// java: private final int frameCount;
    frame_count: i32,
    /// java: private final int packCols, packRows;
    pack_cols: i32,
    #[allow(dead_code)]
    pack_rows: i32,
    /// java: private final int frameWidth, frameHeight;
    frame_width: i32,
    frame_height: i32,
}

impl AnimatedDynamicTextureImage {
    /// java: public AnimatedDynamicTextureImage(NativeImage image, int frameWidth, int frameHeight, int frameCount,
    /// double[] frameDelayMS, int packCols, int packRows, Identifier uniqueLocation)
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        image: NativeImage,
        frame_width: i32,
        frame_height: i32,
        frame_count: i32,
        frame_delay_ms: Vec<f64>,
        pack_cols: i32,
        pack_rows: i32,
        unique_location: Identifier,
    ) -> Self {
        Self {
            // super(image, uniqueLocation, false); // TODO
            base: DynamicTextureImage::new(image, unique_location, false),
            current_frame: Cell::new(0),
            last_frame_time: Cell::new(0.0),
            frame_delays: frame_delay_ms,
            frame_count,
            pack_cols,
            pack_rows,
            frame_width,
            frame_height,
        }
    }

    /// java: public static ImageRendererFactory createGIFFromTexture(Identifier textureLocation)
    pub fn create_gif_from_texture(texture_location: Identifier) -> Box<dyn ImageRendererFactory> {
        Box::new(FnFactory(move || {
            let bytes = read_resource(&texture_location)?;
            create_gif_supplier(&bytes, &texture_location)
        }))
    }

    /// java: public static ImageRendererFactory createGIFFromPath(Path path, Identifier uniqueLocation)
    pub fn create_gif_from_path(path: std::path::PathBuf, unique_location: Identifier) -> Box<dyn ImageRendererFactory> {
        Box::new(FnFactory(move || {
            let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            create_gif_supplier(&bytes, &unique_location)
        }))
    }

    /// java: public static ImageRendererFactory createWEBPFromTexture(Identifier textureLocation)
    pub fn create_webp_from_texture(texture_location: Identifier) -> Box<dyn ImageRendererFactory> {
        Box::new(FnFactory(move || {
            let bytes = read_resource(&texture_location)?;
            create_webp_supplier(&bytes, &texture_location)
        }))
    }

    /// java: public static ImageRendererFactory createWEBPFromPath(Path path, Identifier uniqueLocation)
    pub fn create_webp_from_path(path: std::path::PathBuf, unique_location: Identifier) -> Box<dyn ImageRendererFactory> {
        Box::new(FnFactory(move || {
            let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            create_webp_supplier(&bytes, &unique_location)
        }))
    }
}

impl ImageRenderer for AnimatedDynamicTextureImage {
    /// java: @Override public int render(GuiGraphicsExtractor graphics, int x, int y, int renderWidth, float tickDelta)
    fn render(&self, graphics: &Graphics<'_>, x: i32, y: i32, render_width: i32, _tick_delta: f32) -> i32 {
        if self.base.is_closed() {
            return 0;
        }

        let ratio = render_width as f32 / self.frame_width as f32;
        let target_height = (self.frame_height as f32 * ratio) as i32;

        let current_frame = self.current_frame.get();
        let current_col = current_frame % self.pack_cols;
        let current_row = (current_frame as f64 / self.pack_cols as f64).floor() as i32;

        graphics.push_pose();
        graphics.translate(x as f32, y as f32);
        graphics.scale(ratio, ratio);
        self.base.blit(
            graphics,
            (self.frame_width * current_col) as f32,
            (self.frame_height * current_row) as f32,
            self.frame_width,
            self.frame_height,
            self.base.width,
            self.base.height,
        );
        graphics.pop_pose();

        if self.frame_count > 1 {
            let time_ms = mcrust::com::mojang::blaze3d::blaze3d::get_time() * 1000.0;
            if self.last_frame_time.get() == 0.0 {
                self.last_frame_time.set(time_ms);
            }
            if time_ms - self.last_frame_time.get() >= self.frame_delays[self.current_frame.get() as usize] {
                self.current_frame.set(self.current_frame.get() + 1);
                self.last_frame_time.set(time_ms);
            }
            if self.current_frame.get() >= self.frame_count - 1 {
                self.current_frame.set(0);
            }
        }

        target_height
    }

    /// java: DynamicTextureImage.close()
    fn close(&self) {
        self.base.close();
    }
}

/// `Minecraft.getInstance().getResourceManager().getResource(textureLocation).orElseThrow().open()`
fn read_resource(location: &Identifier) -> Result<Vec<u8>, String> {
    let resource = mcrust::net::minecraft::server::packs::resources::reloadable_resource_manager::resource_manager()
        .get_resource(location)
        .ok_or_else(|| format!("NoSuchElementException: no resource {location}"))?;
    resource.open()
}

/// java: private static ImageRendererFactory.ImageSupplier createGIFSupplier(InputStream is, Identifier uniqueLocation)
///
/// rust: port gap (GAPS_Y2.md G10) - no GIF reader (`ImageIO.getImageReadersBySuffix("gif")`).
fn create_gif_supplier(_bytes: &[u8], unique_location: &Identifier) -> Result<Box<dyn ImageSupplier>, String> {
    Err(format!("Failed to load GIF image '{unique_location}': this port has no GIF decoder (GAPS_Y2 G10)"))
}

/// java: private static ImageRendererFactory.ImageSupplier createWEBPSupplier(InputStream is, Identifier uniqueLocation)
///
/// rust: port gap (GAPS_Y2.md G10) - no WebP reader (TwelveMonkeys `WebPImageReaderSpi`).
fn create_webp_supplier(_bytes: &[u8], unique_location: &Identifier) -> Result<Box<dyn ImageSupplier>, String> {
    Err(format!("Failed to load WEBP image '{unique_location}': this port has no WebP decoder (GAPS_Y2 G10)"))
}

/// java: private static ImageRendererFactory.ImageSupplier createFromImageReader(ImageReader reader,
/// AnimFrameProvider animationProvider, Identifier uniqueLocation) - the packing, over frames a reader decoded
/// (`frames[i]` is frame `i`'s ARGB pixels, its width and height, and its `AnimFrame`).
pub fn create_from_frames(
    frames: Vec<(Vec<i32>, i32, i32, AnimFrame)>,
    unique_location: Identifier,
) -> Result<Box<dyn ImageSupplier>, String> {
    let frame_count = frames.len() as i32;
    // Because this is being backed into a texture atlas, we need a maximum dimension so you can get the texture atlas
    // size. Smaller frames are given black borders
    let frame_width = frames.iter().map(|f| f.1).max().ok_or("NoSuchElementException: no frames")?;
    let frame_height = frames.iter().map(|f| f.2).max().ok_or("NoSuchElementException: no frames")?;

    // Packs the frames into an optimal 1:1 texture. OpenGL can only have texture axis with a max of 32768 pixels, and
    // packing them to that length is not efficient, apparently.
    let ratio = frame_width as f64 / frame_height as f64;
    let cols = ((frame_count as f64).sqrt() / ratio.sqrt()).ceil() as i32;
    let rows = (frame_count as f64 / cols as f64).ceil() as i32;
    let mut image = NativeImage::new(frame_width * cols, frame_height * rows, false);

    // each frame may have a different delay
    let mut frame_delays = vec![0.0; frame_count as usize];
    // the composited frame (delta frames are drawn on top of the previous one)
    let mut canvas: Option<(Vec<i32>, i32, i32)> = None;
    for (i, (pixels, width, height, frame)) in frames.into_iter().enumerate() {
        let i = i as i32;
        if frame_count > 1 {
            // frame will be null if not animation
            frame_delays[i as usize] = frame.duration_ms as f64;
        }
        match canvas.as_mut() {
            None => canvas = Some((pixels, width, height)),
            Some((canvas_pixels, canvas_width, canvas_height)) => {
                // graphics.drawImage(deltaFrame, frame.xOffset, frame.yOffset, null) - source-over
                for h in 0..height {
                    for w in 0..width {
                        let (cx, cy) = (w + frame.x_offset, h + frame.y_offset);
                        if cx < 0 || cy < 0 || cx >= *canvas_width || cy >= *canvas_height {
                            continue;
                        }
                        let argb = pixels[(h * width + w) as usize];
                        if ((argb >> 24) & 0xFF) != 0 {
                            canvas_pixels[(cy * *canvas_width + cx) as usize] = argb;
                        }
                    }
                }
            }
        }
        let (canvas_pixels, canvas_width, canvas_height) = canvas.as_ref().expect("set above");

        // Each frame may have different dimensions, so we need to center them.
        let x_offset = (frame_width - canvas_width) / 2;
        let y_offset = (frame_height - canvas_height) / 2;
        let col = i % cols;
        let row = (i as f64 / cols as f64).floor() as i32;
        for w in 0..*canvas_width {
            for h in 0..*canvas_height {
                let argb = canvas_pixels[(h * canvas_width + w) as usize];
                image.set_pixel(frame_width * col + w + x_offset, frame_height * row + h + y_offset, argb);
            }
        }
    }

    Ok(Box::new(move || -> Result<Arc<dyn ImageRenderer>, String> {
        Ok(Arc::new(AnimatedDynamicTextureImage::new(image, frame_width, frame_height, frame_count, frame_delays, cols, rows, unique_location)))
    }))
}

/// java: private record AnimFrame(int durationMS, int xOffset, int yOffset)
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AnimFrame {
    pub duration_ms: i32,
    pub x_offset: i32,
    pub y_offset: i32,
}
