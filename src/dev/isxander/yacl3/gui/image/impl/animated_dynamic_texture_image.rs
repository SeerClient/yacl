//! Ported from dev.isxander.yacl3.gui.image.impl.AnimatedDynamicTextureImage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::ImageRendererFactory;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::ImageSupplier;
use crate::dev::isxander::yacl3::gui::image::r#impl::dynamic_texture_image::DynamicTextureImage;
use crate::dev::isxander::yacl3::gui::image::r#impl::dynamic_texture_image::DynamicTextureImageBase;
use mcrust::com::mojang::blaze3d::blaze3d;
use mcrust::com::mojang::blaze3d::platform::native_image::NativeImage;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::crash_report::CrashReport;
use mcrust::net::minecraft::crash_report_category::CrashReportCategory;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::server::packs::resources::resource::Resource;
use mcrust::net::minecraft::server::packs::resources::resource_manager::ResourceManager;

/// java: public class AnimatedDynamicTextureImage extends DynamicTextureImage
pub struct AnimatedDynamicTextureImage {
    /// java: extends DynamicTextureImage
    pub base: DynamicTextureImageBase,
    /// java: private int currentFrame;
    pub current_frame: i32,
    /// java: private double lastFrameTime;
    pub last_frame_time: f64,
    /// java: private final double[] frameDelays;
    pub frame_delays: Vec<f64>,
    /// java: private final int frameCount;
    pub frame_count: i32,
    /// java: private final int packCols, packRows;
    pub pack_cols: i32,
    /// java: private final int packCols, packRows;
    pub pack_rows: i32,
    /// java: private final int frameWidth, frameHeight;
    pub frame_width: i32,
    /// java: private final int frameWidth, frameHeight;
    pub frame_height: i32,
}

impl std::ops::Deref for AnimatedDynamicTextureImage {
    type Target = DynamicTextureImageBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AnimatedDynamicTextureImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AnimatedDynamicTextureImage {
    /// java: public AnimatedDynamicTextureImage(NativeImage image, int frameWidth, int frameHeight, int frameCount, double[] frameDelayMS, int packCols, int packRows, Identifier uniqueLocation)
    pub fn new(mut image: NativeImage, mut frame_width: i32, mut frame_height: i32, mut frame_count: i32, mut frame_delay_ms: Vec<f64>, mut pack_cols: i32, mut pack_rows: i32, mut unique_location: Identifier) -> compat::Shared<Self> {
        let __init_frame_width = frame_width;
        let __init_frame_height = frame_height;
        let __init_frame_count = frame_count;
        let __init_frame_delays = frame_delay_ms.clone();
        let __init_pack_cols = pack_cols;
        let __init_pack_rows = pack_rows;
        let (_, __this) = DynamicTextureImageBase::new_in(image.clone(), unique_location.clone(), false, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                current_frame: 0,
                last_frame_time: 0.0,
                frame_delays: __init_frame_delays,
                frame_count: __init_frame_count,
                pack_cols: __init_pack_cols,
                pack_rows: __init_pack_rows,
                frame_width: __init_frame_width,
                frame_height: __init_frame_height,
            });
            ((__h.clone() as compat::Shared<dyn DynamicTextureImage>), __h)
        });
        __this
    }

    /// java: public static ImageRendererFactory createGIFFromTexture(Identifier textureLocation)
    pub fn create_gif_from_texture(texture_location: &Identifier) -> compat::Shared<dyn ImageRendererFactory> {
        return Arc::new(move || {
            let resource_manager = compat::minecraft().get_resource_manager();
            let mut resource: Resource = resource_manager.get_resource(texture_location).unwrap();
            return AnimatedDynamicTextureImage::create_gif_supplier(&resource.open().unwrap(), texture_location);
        });
    }

    /// java: public static ImageRendererFactory createGIFFromPath(Path path, Identifier uniqueLocation)
    pub fn create_gif_from_path(path: &std::path::PathBuf, unique_location: &Identifier) -> compat::Shared<dyn ImageRendererFactory> {
        // TODO(fabric2seer): `new FileInputStream(..)`
        //   java: return () -> createGIFSupplier(new FileInputStream(path.toFile()), uniqueLocation);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static ImageRendererFactory createWEBPFromTexture(Identifier textureLocation)
    pub fn create_webp_from_texture(texture_location: &Identifier) -> compat::Shared<dyn ImageRendererFactory> {
        return Arc::new(move || {
            let resource_manager = compat::minecraft().get_resource_manager();
            let mut resource: Resource = resource_manager.get_resource(texture_location).unwrap();
            return AnimatedDynamicTextureImage::create_webp_supplier(&resource.open().unwrap(), texture_location);
        });
    }

    /// java: public static ImageRendererFactory createWEBPFromPath(Path path, Identifier uniqueLocation)
    pub fn create_webp_from_path(path: &std::path::PathBuf, unique_location: &Identifier) -> compat::Shared<dyn ImageRendererFactory> {
        // TODO(fabric2seer): `new FileInputStream(..)`
        //   java: return () -> createWEBPSupplier(new FileInputStream(path.toFile()), uniqueLocation);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static ImageRendererFactory.ImageSupplier createGIFSupplier(InputStream is, Identifier uniqueLocation)
    pub fn create_gif_supplier(is: &crate::unresolved::InputStream, unique_location: &Identifier) -> ImageSupplier {
        {
            let _resource = is; // java: try (is) - dropped (closed) at the end
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): static call ImageIO.getImageReadersBySuffix(..)
                //   java: ImageReader reader = ImageIO.getImageReadersBySuffix("gif").next();
                let mut reader: crate::unresolved::ImageReader = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
                // TODO(fabric2seer): `.setInput(..)` on a ImageReader
                //   java: reader.setInput(ImageIO.createImageInputStream(is));
                let mut anim_frame_function: AnimFrameProvider = Arc::new(move |i| {
                    // TODO(fabric2seer): `.getImageMetadata(..)` on a ImageReader
                    //   java: IIOMetadata metadata = reader.getImageMetadata(i);
                    let mut metadata: crate::unresolved::IIOMetadata = todo!("fabric2seer: the initialiser of `metadata` (TODO above)");
                    // TODO(fabric2seer): `.getNativeMetadataFormatName(..)` on a IIOMetadata
                    //   java: String metaFormatName = metadata.getNativeMetadataFormatName();
                    let mut meta_format_name: String = todo!("fabric2seer: the initialiser of `metaFormatName` (TODO above)");
                    // TODO(fabric2seer): `.getAsTree(..)` on a IIOMetadata
                    //   java: IIOMetadataNode root = (IIOMetadataNode) metadata.getAsTree(metaFormatName);
                    let mut root: crate::unresolved::IIOMetadataNode = todo!("fabric2seer: the initialiser of `root` (TODO above)");
                    // TODO(fabric2seer): `.getElementsByTagName(..)` on a IIOMetadataNode
                    //   java: IIOMetadataNode graphicsControlExtensionNode = (IIOMetadataNode) root.getElementsByTagName("GraphicControlExtension").item(0);
                    let mut graphics_control_extension_node: crate::unresolved::IIOMetadataNode = todo!("fabric2seer: the initialiser of `graphicsControlExtensionNode` (TODO above)");
                    // TODO(fabric2seer): `.getAttribute(..)` on a IIOMetadataNode
                    //   java: int delay = Integer.parseInt(graphicsControlExtensionNode.getAttribute("delayTime")) * 10;
                    let mut delay: i32 = todo!("fabric2seer: the initialiser of `delay` (TODO above)");
                    return AnimFrame::new(delay, 0, 0);
                });
                return AnimatedDynamicTextureImage::create_from_image_reader(&reader, anim_frame_function.clone(), unique_location);
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    let mut crash_report: CrashReport = CrashReport::for_throwable(e.clone(), "Failed to load GIF image");
                    let category = crash_report.add_category_with_offset("YACL Gui", 1);
                    // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
                    // TODO(fabric2seer): `CrashReportCategory.setDetail` has no Rust method in the port (looked for `set_detail` in CrashReportCategory and its supertypes)
                    // rust: category.set_detail("Image identifier", &unique_location.fmt());
                    panic!("ReportedException: {}", compat::jstring(&crash_report));
                }
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static ImageRendererFactory.ImageSupplier createWEBPSupplier(InputStream is, Identifier uniqueLocation)
    pub fn create_webp_supplier(is: &crate::unresolved::InputStream, unique_location: &Identifier) -> ImageSupplier {
        {
            let _resource = is; // java: try (is) - dropped (closed) at the end
            // java: try {..} catch (Throwable e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `new WebPImageReaderSpi(..)`
                //   java: ImageReader reader = new WebPImageReaderSpi().createReaderInstance();
                let mut reader: crate::unresolved::ImageReader = todo!("fabric2seer: the initialiser of `reader` (TODO above)");
                // TODO(fabric2seer): `.setInput(..)` on a ImageReader
                //   java: reader.setInput(ImageIO.createImageInputStream(is));
                // TODO(fabric2seer): `.getNumImages(..)` on a ImageReader
                //   java: int numImages = reader.getNumImages(true);
                let mut num_images: i32 = todo!("fabric2seer: the initialiser of `numImages` (TODO above)");
                let mut anim_frame_function: AnimFrameProvider = Arc::new(move |i| None);
                if num_images > 1 {
                    // TODO(fabric2seer): static call Class.forName(..)
                    //   java: Class<?> webpReaderClass = Class.forName("com.twelvemonkeys.imageio.plugins.webp.WebPImageReader");
                    let mut webp_reader_class: compat::ClassOf<compat::Object> = todo!("fabric2seer: the initialiser of `webpReaderClass` (TODO above)");
                    // TODO(fabric2seer): `.getDeclaredField(..)` on a Class
                    //   java: Field framesField = webpReaderClass.getDeclaredField("frames");
                    let mut frames_field: crate::unresolved::Field = todo!("fabric2seer: the initialiser of `framesField` (TODO above)");
                    // TODO(fabric2seer): `.setAccessible(..)` on a Field
                    //   java: framesField.setAccessible(true);
                    // TODO(fabric2seer): `.get(..)` on a Field
                    //   java: java.util.List<?> frames = (List<?>) framesField.get(reader);
                    let mut frames: Vec<compat::Object> = todo!("fabric2seer: the initialiser of `frames` (TODO above)");
                    // TODO(fabric2seer): static call Class.forName(..)
                    //   java: Class<?> animationFrameClass = Class.forName("com.twelvemonkeys.imageio.plugins.webp.AnimationFrame");
                    let mut animation_frame_class: compat::ClassOf<compat::Object> = todo!("fabric2seer: the initialiser of `animationFrameClass` (TODO above)");
                    // TODO(fabric2seer): `.getDeclaredField(..)` on a Class
                    //   java: Field durationField = animationFrameClass.getDeclaredField("duration");
                    let mut duration_field: crate::unresolved::Field = todo!("fabric2seer: the initialiser of `durationField` (TODO above)");
                    // TODO(fabric2seer): `.setAccessible(..)` on a Field
                    //   java: durationField.setAccessible(true);
                    // TODO(fabric2seer): `.getDeclaredField(..)` on a Class
                    //   java: Field boundsField = animationFrameClass.getDeclaredField("bounds");
                    let mut bounds_field: crate::unresolved::Field = todo!("fabric2seer: the initialiser of `boundsField` (TODO above)");
                    // TODO(fabric2seer): `.setAccessible(..)` on a Field
                    //   java: boundsField.setAccessible(true);
                    anim_frame_function = Arc::new(move |i| {
                        // TODO(fabric2seer): `.get(..)` on a Field
                        //   java: Rectangle bounds = (Rectangle) boundsField.get(frames.get(i));
                        let mut bounds: crate::unresolved::Rectangle = todo!("fabric2seer: the initialiser of `bounds` (TODO above)");
                        // TODO(fabric2seer): `.get(..)` on a Field
                        //   java: return new AnimFrame((int) durationField.get(frames.get(i)), bounds.x, bounds.y);
                    });
                }
                return AnimatedDynamicTextureImage::create_from_image_reader(&reader, anim_frame_function.clone(), unique_location);
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    let mut crash_report: CrashReport = CrashReport::for_throwable(e.clone(), "Failed to load WEBP image");
                    let category = crash_report.add_category_with_offset("YACL Gui", 1);
                    // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
                    // TODO(fabric2seer): `CrashReportCategory.setDetail` has no Rust method in the port (looked for `set_detail` in CrashReportCategory and its supertypes)
                    // rust: category.set_detail("Image identifier", &unique_location.fmt());
                    panic!("ReportedException: {}", compat::jstring(&crash_report));
                }
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static ImageRendererFactory.ImageSupplier createFromImageReader(ImageReader reader, AnimFrameProvider animationProvider, Identifier uniqueLocation) throws Exception
    pub fn create_from_image_reader(reader: &crate::unresolved::ImageReader, animation_provider: AnimFrameProvider, unique_location: &Identifier) -> ImageSupplier {
        // TODO(fabric2seer): the condition: `.isSeekForwardOnly(..)` on a ImageReader
        //   java: reader.isSeekForwardOnly(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            panic!("RuntimeException: {}", "Image reader is not seekable");
        }
        // TODO(fabric2seer): `.getNumImages(..)` on a ImageReader
        //   java: int frameCount = reader.getNumImages(true);
        let mut frame_count: i32 = todo!("fabric2seer: the initialiser of `frameCount` (TODO above)");
        let mut frame_width: i32 = (0..frame_count).map(|i| {
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.getWidth(..)` on a ImageReader
                //   java: return reader.getWidth(i);
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    panic!("RuntimeException: {}", compat::panic_message(&e));
                }
            }
        }).max().unwrap();
        let mut frame_height: i32 = (0..frame_count).map(|i| {
            // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.getHeight(..)` on a ImageReader
                //   java: return reader.getHeight(i);
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    panic!("RuntimeException: {}", compat::panic_message(&e));
                }
            }
        }).max().unwrap();
        let mut ratio: f64 = ((frame_width as f64) / ((frame_height) as f64));
        let mut cols: i32 = ((((frame_count as f64).sqrt() / ratio.sqrt()).ceil()) as i32);
        let mut rows: i32 = ((((frame_count as f64) / ((cols) as f64)).ceil()) as i32);
        // TODO(fabric2seer): field `RGBA` of a value of unknown type
        //   java: NativeImage image = new NativeImage(NativeImage.Format.RGBA, frameWidth * cols, frameHeight * rows, false);
        let mut image: NativeImage = todo!("fabric2seer: the initialiser of `image` (TODO above)");
        let mut bi: Option<crate::unresolved::BufferedImage> = None;
        let mut graphics: Option<crate::unresolved::Graphics2D> = None;
        let mut frame_delays: Vec<f64> = vec![0.0; (frame_count) as usize];
        for i in 0..frame_count {
            let mut frame: AnimFrame = (animation_provider)(i);
            if frame_count > 1 {
                frame_delays[(i) as usize] = (frame.duration_ms) as f64;
            }
            if bi.is_none() {
                // TODO(fabric2seer): `.read(..)` on a ImageReader
                //   java: bi = reader.read(i);
                // TODO(fabric2seer): `.createGraphics(..)` on a BufferedImage
                //   java: graphics = bi.createGraphics();
            } else {
                // TODO(fabric2seer): `.read(..)` on a ImageReader
                //   java: BufferedImage deltaFrame = reader.read(i);
                let mut delta_frame: crate::unresolved::BufferedImage = todo!("fabric2seer: the initialiser of `deltaFrame` (TODO above)");
                // TODO(fabric2seer): `.drawImage(..)` on a Graphics2D
                //   java: graphics.drawImage(deltaFrame, frame.xOffset, frame.yOffset, null);
            }
            // TODO(fabric2seer): `.getWidth(..)` on a BufferedImage
            //   java: int xOffset = (frameWidth - bi.getWidth()) / 2;
            let mut x_offset: i32 = todo!("fabric2seer: the initialiser of `xOffset` (TODO above)");
            // TODO(fabric2seer): `.getHeight(..)` on a BufferedImage
            //   java: int yOffset = (frameHeight - bi.getHeight()) / 2;
            let mut y_offset: i32 = todo!("fabric2seer: the initialiser of `yOffset` (TODO above)");
            // TODO(fabric2seer): `.getWidth(..)` on a BufferedImage
            //   java: for (int w = 0; w < bi.getWidth(); w++) {
            //   java:                 for (int h = 0; h < bi.getHeight(); h++) {
            //   java:                     int argb = bi.getRGB(w, h);
            //   java:
            //   java:                     int col = i % cols;
            //   java:                     int row = (int) Math.floor(i / (double)cols);
            //   java:
            //   java:                     image.setPixel(
            //   java:                             frameWidth * col + w + xOffset,
            //   java:                             frameHeight * row + h + yOffset,
            //   java:                             argb
            //   java:                     );
            //   java:                 }
            //   java:             }
        }
        if let Some(graphics) = &graphics {
            // TODO(fabric2seer): `.dispose(..)` on a Graphics2D
            //   java: graphics.dispose();
        }
        // TODO(fabric2seer): `.dispose(..)` on a ImageReader
        //   java: reader.dispose();
        return Arc::new(move || AnimatedDynamicTextureImage::new(image.clone(), frame_width, frame_height, frame_count, frame_delays.clone(), cols, rows, unique_location.clone()));
    }

}

impl DynamicTextureImage for AnimatedDynamicTextureImage {
    fn dynamic_texture_image_base(&self) -> &DynamicTextureImageBase {
        &self.base
    }
    fn dynamic_texture_image_base_mut(&mut self) -> &mut DynamicTextureImageBase {
        &mut self.base
    }

}

impl ImageRenderer for AnimatedDynamicTextureImage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public int render(GuiGraphicsExtractor graphics, int x, int y, int renderWidth, float tickDelta) (dispatch: the body for this class)
    fn render(&self) -> fn(&compat::Shared<dyn ImageRenderer>, &GuiGraphicsExtractor, i32, i32, i32, f32) -> i32 {
        AnimatedDynamicTextureImage::render_dyn
    }

    /// java: @Override public void close() (dispatch: the body for this class)
    fn close(&self) -> fn(&compat::Shared<dyn ImageRenderer>) {
        <dyn DynamicTextureImage>::close_from::<Self>
    }

}

impl AnimatedDynamicTextureImage {
    /// java: @Override public int render(GuiGraphicsExtractor graphics, int x, int y, int renderWidth, float tickDelta)
    pub fn render_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut render_width: i32, mut tick_delta: f32) -> i32 {
        if compat::read(__this, |__t| __t.image.clone()).is_none() {
            return 0;
        }
        let mut ratio: f32 = ((render_width as f32) / ((compat::read(__this, |__t| __t.frame_width)) as f32));
        let mut target_height: i32 = ((((compat::read(__this, |__t| __t.frame_height) as f32) * ratio)) as i32);
        let mut current_col: i32 = (compat::read(__this, |__t| __t.current_frame)).wrapping_rem(compat::read(__this, |__t| __t.pack_cols));
        let mut current_row: i32 = ((((compat::read(__this, |__t| __t.current_frame) as f64) / ((compat::read(__this, |__t| __t.pack_cols)) as f64)).floor()) as i32);
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((x) as f32, (y) as f32);
        gui_graphics_extractor::pose_scale((ratio) as f32, (ratio) as f32);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &compat::read(__this, |__t| __t.unique_location.clone()).unwrap(), 0, 0, (((compat::read(__this, |__t| __t.frame_width)).wrapping_mul(current_col)) as f32), (((compat::read(__this, |__t| __t.frame_height)).wrapping_mul(current_row)) as f32), compat::read(__this, |__t| __t.frame_width), compat::read(__this, |__t| __t.frame_height), compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height));
        gui_graphics_extractor::pop_pose(__pose_1);
        if compat::read(__this, |__t| __t.frame_count) > 1 {
            let mut time_ms: f64 = (blaze3d::get_time() * (1000 as f64));
            if compat::read(__this, |__t| __t.last_frame_time) == (0 as f64) {
                { let __v = time_ms; compat::write(__this, |__t| __t.last_frame_time = __v); }
            }
            if (time_ms - compat::read(__this, |__t| __t.last_frame_time)) >= __this.lock().borrow().frame_delays[(compat::read(__this, |__t| __t.current_frame)) as usize] {
                { let __v = compat::read(__this, |__t| __t.current_frame) + 1; compat::write(__this, |__t| __t.current_frame = __v); }
                { let __v = time_ms; compat::write(__this, |__t| __t.last_frame_time = __v); }
            }
            if compat::read(__this, |__t| __t.current_frame) >= (compat::read(__this, |__t| __t.frame_count)).wrapping_sub(1) {
                { let __v = 0; compat::write(__this, |__t| __t.current_frame = __v); }
            }
        }
        return target_height;
    }

    /// AnimatedDynamicTextureImage.render as the dispatch of ImageRenderer.render
    pub fn render_dyn(__this: &compat::Shared<dyn ImageRenderer>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: i32, __p4: f32) -> i32 {
        Self::render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4)
    }

}

/// java: @FunctionalInterface private interface AnimFrameProvider
/// a functional interface: `get(int frame)` - a lambda / method reference is this closure type
pub type AnimFrameProvider = Arc<dyn Fn(i32) -> AnimFrame>;

/// java: private record AnimFrame(int durationMS, int xOffset, int yOffset)
#[derive(Clone, Debug, PartialEq)]
pub struct AnimFrame {
    /// java: int durationMS
    pub duration_ms: i32,
    /// java: int xOffset
    pub x_offset: i32,
    /// java: int yOffset
    pub y_offset: i32,
}

impl AnimFrame {
    /// java: the canonical record constructor
    pub fn new(mut duration_ms: i32, mut x_offset: i32, mut y_offset: i32) -> Self {
        Self { duration_ms, x_offset, y_offset }
    }

    /// java: the record accessor `durationMS()`
    pub fn duration_ms(&self) -> i32 {
        self.duration_ms
    }

    /// java: the record accessor `xOffset()`
    pub fn x_offset(&self) -> i32 {
        self.x_offset
    }

    /// java: the record accessor `yOffset()`
    pub fn y_offset(&self) -> i32 {
        self.y_offset
    }

}
