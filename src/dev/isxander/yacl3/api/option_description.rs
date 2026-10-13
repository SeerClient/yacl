//! Ported from dev.isxander.yacl3.api.OptionDescription.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::r#impl::option_description_impl::{BuilderImpl, OptionDescriptionImpl};
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// `CompletableFuture<Optional<ImageRenderer>>`: completed on the render thread (the image objects are not `Send`).
pub type ImageFuture = compat::LocalFuture<Option<Arc<dyn ImageRenderer>>>;

thread_local! {
    static EMPTY_CELL: Arc<dyn OptionDescription> = Arc::new(OptionDescriptionImpl::new(
        // CommonComponents.EMPTY = Component.empty()
        mcrust::net::minecraft::network::chat::component::empty(),
        compat::LocalFuture::completed_future(None),
    ));
}

/// java: OptionDescription EMPTY = new OptionDescriptionImpl(CommonComponents.EMPTY, CompletableFuture.completedFuture(Optional.empty()));
/// One object (it is compared by identity where a hovered description is remembered).
#[allow(non_snake_case)]
pub fn EMPTY() -> Arc<dyn OptionDescription> {
    EMPTY_CELL.with(|d| d.clone())
}

/// java: public interface OptionDescription
pub trait OptionDescription: compat::DynAny {
    /// java: Component text();
    fn text(&self) -> MutableComponent;

    /// java: CompletableFuture<Optional<ImageRenderer>> image();
    fn image(&self) -> ImageFuture;
}

/// java: static Builder createBuilder()
pub fn create_builder() -> BuilderImpl {
    BuilderImpl::new()
}

/// java: static OptionDescription of(Component... description)
pub fn of(description: Vec<MutableComponent>) -> Arc<dyn OptionDescription> {
    create_builder().text(description).build()
}

/// java: interface Builder (setters by value, Java's `return this`)
pub trait Builder: Sized {
    /// java: Builder text(Component... description);
    fn text(self, description: Vec<MutableComponent>) -> Self;

    /// java: Builder text(Collection<? extends Component> lines);
    fn text_collection(self, lines: Vec<MutableComponent>) -> Self;

    /// java: Builder image(Identifier image, int width, int height);
    fn image(self, image: Identifier, width: i32, height: i32) -> Self;

    /// java: Builder image(Identifier image, float u, float v, int width, int height, int textureWidth, int textureHeight);
    #[allow(clippy::too_many_arguments)]
    fn image_7(self, image: Identifier, u: f32, v: f32, width: i32, height: i32, texture_width: i32, texture_height: i32) -> Self;

    /// java: Builder image(Path path, Identifier uniqueLocation);
    fn image_2(self, path: std::path::PathBuf, unique_location: Identifier) -> Self;

    /// java: Builder webpImage(Identifier image);
    fn webp_image(self, image: Identifier) -> Self;

    /// java: Builder webpImage(Path path, Identifier uniqueLocation);
    fn webp_image_2(self, path: std::path::PathBuf, unique_location: Identifier) -> Self;

    /// java: Builder customImage(CompletableFuture<Optional<ImageRenderer>> image);
    fn custom_image(self, image: ImageFuture) -> Self;

    /// java: default Builder customImage(ImageRenderer image)
    fn custom_image_image_renderer(self, image: Arc<dyn ImageRenderer>) -> Self {
        self.custom_image(compat::LocalFuture::completed_future(Some(image)))
    }

    /// java: @Deprecated Builder gifImage(Identifier image);
    fn gif_image(self, image: Identifier) -> Self;

    /// java: @Deprecated Builder gifImage(Path path, Identifier uniqueLocation);
    fn gif_image_2(self, path: std::path::PathBuf, unique_location: Identifier) -> Self;

    /// java: OptionDescription build();
    fn build(self) -> Arc<dyn OptionDescription>;
}
