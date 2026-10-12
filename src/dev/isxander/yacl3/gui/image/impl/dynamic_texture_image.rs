//! Ported from dev.isxander.yacl3.gui.image.impl.DynamicTextureImage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::ImageRendererFactory;
use mcrust::com::mojang::blaze3d::platform::native_image::NativeImage;
use mcrust::com::mojang::blaze3d::systems::render_system;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::renderer::texture::dynamic_texture::DynamicTexture;
use mcrust::net::minecraft::client::renderer::texture::texture_manager::TextureManager;
use mcrust::net::minecraft::resources::identifier::Identifier;

// TODO(fabric2seer): the mod keeps the game's `TextureManager` (Java holds a reference to the live object); the port's `TextureManager` is not a value to copy and the port lends the mod no handle to it
//   java: protected static final TextureManager textureManager = Minecraft.getInstance().getTextureManager();
/// java: protected static final TextureManager textureManager = Minecraft.getInstance().getTextureManager();
pub static TEXTURE_MANAGER: compat::MainThread<TextureManager> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `textureManager` - see the TODO above"));

/// java: public class DynamicTextureImage implements ImageRenderer
pub struct DynamicTextureImageBase {
    /// java: protected NativeImage image;
    pub image: Option<NativeImage>,
    /// java: protected DynamicTexture texture;
    pub texture: Option<DynamicTexture>,
    /// java: protected final Identifier uniqueLocation;
    pub unique_location: Option<Identifier>,
    /// java: protected final int width, height;
    pub width: i32,
    /// java: protected final int width, height;
    pub height: i32,
    /// java: protected final boolean textureFiltering;
    pub texture_filtering: bool,
}

impl DynamicTextureImageBase {
    /// java: public DynamicTextureImage(NativeImage image, Identifier location, boolean textureFiltering) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut image: NativeImage, mut location: Identifier, mut texture_filtering: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn DynamicTextureImage>, __R)) -> (compat::Shared<dyn DynamicTextureImage>, __R) {
        let this = Self {
            image: None,
            texture: None,
            unique_location: None,
            width: 0,
            height: 0,
            texture_filtering: false,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        render_system::assert_on_render_thread();
        { let __v = Some(image.clone()); compat::write(&__this, |__t| __t.dynamic_texture_image_base_mut().image = __v); }
        // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // TODO(fabric2seer): a lambda for a parameter the port does not type as a closure (`Supplier<String>`)
        // rust: { let __v = Some(DynamicTexture::new(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), || location.fmt(), image.clone())); compat::write(&__this, |__t| __t.dynamic_texture_image_base_mut().texture = __v); }
        { let __v = texture_filtering; compat::write(&__this, |__t| __t.dynamic_texture_image_base_mut().texture_filtering = __v); }
        { let __v = Some(location.clone()); compat::write(&__this, |__t| __t.dynamic_texture_image_base_mut().unique_location = __v); }
        TEXTURE_MANAGER.get_mut().register(compat::read(&__this, |__t| __t.dynamic_texture_image_base().unique_location.clone()).unwrap(), compat::read(&__this, |__t| __t.dynamic_texture_image_base().texture.clone()).unwrap());
        { let __v = image.get_width(); compat::write(&__this, |__t| __t.dynamic_texture_image_base_mut().width = __v); }
        { let __v = image.get_height(); compat::write(&__this, |__t| __t.dynamic_texture_image_base_mut().height = __v); }
        (__this, __r)
    }

    /// java: public DynamicTextureImage(NativeImage image, Identifier location, boolean textureFiltering)
    pub fn new(mut image: NativeImage, mut location: Identifier, mut texture_filtering: bool) -> compat::Shared<Self> {
        Self::new_in(image, location, texture_filtering, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn DynamicTextureImage>), __h)
        }).1
    }

    /// java: public static ImageRendererFactory fromPath(Path imagePath, Identifier location, boolean textureFiltering)
    pub fn from_path(image_path: &std::path::PathBuf, location: &Identifier, mut texture_filtering: bool) -> compat::Shared<dyn ImageRendererFactory> {
        // TODO(fabric2seer): `new FileInputStream(..)`
        //   java: return (ImageRendererFactory.OnThread) () -> () -> new DynamicTextureImage(NativeImage.read(new FileInputStream(imagePath.toFile())), location, textureFiltering);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: the virtual methods of DynamicTextureImage (implemented by DynamicTextureImageBase and every subclass)
pub trait DynamicTextureImage: ImageRenderer {
    fn dynamic_texture_image_base(&self) -> &DynamicTextureImageBase;
    fn dynamic_texture_image_base_mut(&mut self) -> &mut DynamicTextureImageBase;

}

impl dyn DynamicTextureImage {
    /// java: @Override public int render(GuiGraphicsExtractor graphics, int x, int y, int renderWidth, float tickDelta)
    pub fn render_body(__this: &compat::Shared<dyn DynamicTextureImage>, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut render_width: i32, mut tick_delta: f32) -> i32 {
        if compat::read(__this, |__t| __t.dynamic_texture_image_base().image.clone()).is_none() {
            return 0;
        }
        let mut ratio: f32 = ((render_width as f32) / ((compat::read(__this, |__t| __t.dynamic_texture_image_base().width)) as f32));
        let mut target_height: i32 = ((((compat::read(__this, |__t| __t.dynamic_texture_image_base().height) as f32) * ratio)) as i32);
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((x) as f32, (y) as f32);
        gui_graphics_extractor::pose_scale((ratio) as f32, (ratio) as f32);
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), &compat::read(__this, |__t| __t.dynamic_texture_image_base().unique_location.clone()).unwrap(), 0, 0, ((0) as f32), ((0) as f32), compat::read(__this, |__t| __t.dynamic_texture_image_base().width), compat::read(__this, |__t| __t.dynamic_texture_image_base().height), compat::read(__this, |__t| __t.dynamic_texture_image_base().width), compat::read(__this, |__t| __t.dynamic_texture_image_base().height));
        gui_graphics_extractor::pop_pose(__pose_1);
        return target_height;
    }

    /// DynamicTextureImage.render as the dispatch of ImageRenderer.render for the concrete class `__C`
    pub fn render_from<__C: DynamicTextureImage + 'static>(__this: &compat::Shared<dyn ImageRenderer>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: i32, __p4: f32) -> i32 {
        <dyn DynamicTextureImage>::render_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn DynamicTextureImage>), __p0, __p1, __p2, __p3, __p4)
    }

    /// java: @Override public void close()
    pub fn close_body(__this: &compat::Shared<dyn DynamicTextureImage>) {
        __this.lock().borrow_mut().dynamic_texture_image_base_mut().image.as_mut().unwrap().close();
        { let __v = None; compat::write(__this, |__t| __t.dynamic_texture_image_base_mut().image = __v); }
        { let __v = None; compat::write(__this, |__t| __t.dynamic_texture_image_base_mut().texture = __v); }
        TEXTURE_MANAGER.get_mut().release(&compat::read(__this, |__t| __t.dynamic_texture_image_base().unique_location.clone()).unwrap());
    }

    /// DynamicTextureImage.close as the dispatch of ImageRenderer.close for the concrete class `__C`
    pub fn close_from<__C: DynamicTextureImage + 'static>(__this: &compat::Shared<dyn ImageRenderer>) {
        <dyn DynamicTextureImage>::close_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn DynamicTextureImage>))
    }

}

impl DynamicTextureImage for DynamicTextureImageBase {
    fn dynamic_texture_image_base(&self) -> &DynamicTextureImageBase {
        self
    }
    fn dynamic_texture_image_base_mut(&mut self) -> &mut DynamicTextureImageBase {
        self
    }

}

impl ImageRenderer for DynamicTextureImageBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public int render(GuiGraphicsExtractor graphics, int x, int y, int renderWidth, float tickDelta) (dispatch: the body for this class)
    fn render(&self) -> fn(&compat::Shared<dyn ImageRenderer>, &GuiGraphicsExtractor, i32, i32, i32, f32) -> i32 {
        <dyn DynamicTextureImage>::render_from::<Self>
    }

    /// java: @Override public void close() (dispatch: the body for this class)
    fn close(&self) -> fn(&compat::Shared<dyn ImageRenderer>) {
        <dyn DynamicTextureImage>::close_from::<Self>
    }

}
