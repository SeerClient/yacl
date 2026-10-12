//! Ported from dev.isxander.yacl3.impl.OptionDescriptionImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option_description::{Builder, ImageFuture, OptionDescription};
use crate::dev::isxander::yacl3::debug::debug_properties::DebugProperties;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::gui::image::image_renderer_manager::ImageRendererManager;
use crate::dev::isxander::yacl3::gui::image::r#impl::animated_dynamic_texture_image::AnimatedDynamicTextureImage;
use crate::dev::isxander::yacl3::gui::image::r#impl::dynamic_texture_image::DynamicTextureImage;
use crate::dev::isxander::yacl3::gui::image::r#impl::resource_texture_image::ResourceTextureImage;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public record OptionDescriptionImpl(Component text, CompletableFuture<Optional<ImageRenderer>> image) implements OptionDescription
pub struct OptionDescriptionImpl {
    pub text: MutableComponent,
    pub image: ImageFuture,
}

impl OptionDescriptionImpl {
    /// java: the canonical record constructor
    pub fn new(text: MutableComponent, image: ImageFuture) -> Self {
        Self { text, image }
    }
}

impl OptionDescription for OptionDescriptionImpl {
    /// java: the record accessor `text()`
    fn text(&self) -> MutableComponent {
        self.text.clone()
    }

    /// java: the record accessor `image()`
    fn image(&self) -> ImageFuture {
        self.image.clone()
    }
}

/// `registerOrGetImage(..).thenApply(Optional::of)`
fn optional_of(future: compat::LocalFuture<Arc<dyn ImageRenderer>>) -> ImageFuture {
    future.then_apply(Some)
}

/// java: public static class BuilderImpl implements Builder
pub struct BuilderImpl {
    /// java: private final List<Component> descriptionLines = new ArrayList<>();
    pub description_lines: Vec<MutableComponent>,
    /// java: private CompletableFuture<Optional<ImageRenderer>> image = CompletableFuture.completedFuture(Optional.empty());
    pub image: ImageFuture,
    /// java: private boolean imageUnset = true;
    pub image_unset: bool,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {
            description_lines: Vec::new(),
            image: compat::LocalFuture::completed_future(None),
            image_unset: true,
        }
    }

    fn check_unset(&self) {
        assert!(self.image_unset, "Image already set!");
    }
}

impl Default for BuilderImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder for BuilderImpl {
    /// java: @Override public Builder text(Component... description)
    fn text(mut self, description: Vec<MutableComponent>) -> Self {
        self.description_lines.extend(description);
        self
    }

    /// java: @Override public Builder text(Collection<? extends Component> lines)
    fn text_collection(mut self, lines: Vec<MutableComponent>) -> Self {
        self.description_lines.extend(lines);
        self
    }

    /// java: @Override public Builder image(Identifier image, int width, int height)
    fn image(mut self, image: Identifier, width: i32, height: i32) -> Self {
        self.check_unset();
        assert!(width > 0, "Width must be greater than 0!");
        assert!(height > 0, "Height must be greater than 0!");
        let location = image.clone();
        self.image = optional_of(ImageRendererManager::register_or_get_image(&image, move || {
            ResourceTextureImage::create_factory(location, 0.0, 0.0, width, height, width, height)
        }));
        self.image_unset = false;
        self
    }

    /// java: @Override public Builder image(Identifier image, float u, float v, int width, int height, int textureWidth, int textureHeight)
    fn image_7(mut self, image: Identifier, u: f32, v: f32, width: i32, height: i32, texture_width: i32, texture_height: i32) -> Self {
        self.check_unset();
        assert!(width > 0, "Width must be greater than 0!");
        assert!(height > 0, "Height must be greater than 0!");
        let location = image.clone();
        self.image = optional_of(ImageRendererManager::register_or_get_image(&image, move || {
            ResourceTextureImage::create_factory(location, u, v, width, height, texture_width, texture_height)
        }));
        self.image_unset = false;
        self
    }

    /// java: @Override public Builder image(Path path, Identifier uniqueLocation)
    fn image_2(mut self, path: std::path::PathBuf, unique_location: Identifier) -> Self {
        self.check_unset();
        let location = unique_location.clone();
        self.image = optional_of(ImageRendererManager::register_or_get_image(&unique_location, move || {
            DynamicTextureImage::from_path(path, location, DebugProperties::IMAGE_FILTERING)
        }));
        self.image_unset = false;
        self
    }

    /// java: @Override public Builder webpImage(Identifier image)
    fn webp_image(mut self, image: Identifier) -> Self {
        self.check_unset();
        let location = image.clone();
        self.image = optional_of(ImageRendererManager::register_or_get_image(&image, move || {
            AnimatedDynamicTextureImage::create_webp_from_texture(location)
        }));
        self.image_unset = false;
        self
    }

    /// java: @Override public Builder webpImage(Path path, Identifier uniqueLocation)
    fn webp_image_2(mut self, path: std::path::PathBuf, unique_location: Identifier) -> Self {
        self.check_unset();
        let location = unique_location.clone();
        self.image = optional_of(ImageRendererManager::register_or_get_image(&unique_location, move || {
            AnimatedDynamicTextureImage::create_webp_from_path(path, location)
        }));
        self.image_unset = false;
        self
    }

    /// java: @Override public Builder customImage(CompletableFuture<Optional<ImageRenderer>> image)
    fn custom_image(mut self, image: ImageFuture) -> Self {
        self.check_unset();
        self.image = image;
        self.image_unset = false;
        self
    }

    /// java: @Override public Builder gifImage(Identifier image)
    fn gif_image(mut self, image: Identifier) -> Self {
        self.check_unset();
        let location = image.clone();
        self.image = optional_of(ImageRendererManager::register_or_get_image(&image, move || {
            AnimatedDynamicTextureImage::create_gif_from_texture(location)
        }));
        self.image_unset = false;
        self
    }

    /// java: @Override public Builder gifImage(Path path, Identifier uniqueLocation)
    fn gif_image_2(mut self, path: std::path::PathBuf, unique_location: Identifier) -> Self {
        self.check_unset();
        let location = unique_location.clone();
        self.image = optional_of(ImageRendererManager::register_or_get_image(&unique_location, move || {
            AnimatedDynamicTextureImage::create_gif_from_path(path, location)
        }));
        self.image_unset = false;
        self
    }

    /// java: @Override public OptionDescription build()
    fn build(self) -> Arc<dyn OptionDescription> {
        let mut concatenated_description = component::empty();
        let mut iter = self.description_lines.into_iter().peekable();
        while let Some(line) = iter.next() {
            concatenated_description = concatenated_description.append(line);
            if iter.peek().is_some() {
                concatenated_description = concatenated_description.append_str("\n");
            }
        }
        Arc::new(OptionDescriptionImpl::new(concatenated_description, self.image))
    }
}
