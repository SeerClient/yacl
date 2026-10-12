//! Ported from dev.isxander.yacl3.impl.OptionDescriptionImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option_description::Builder;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::debug::debug_properties;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::gui::image::image_renderer_manager::ImageRendererManager;
use crate::dev::isxander::yacl3::gui::image::r#impl::animated_dynamic_texture_image::AnimatedDynamicTextureImage;
use crate::dev::isxander::yacl3::gui::image::r#impl::dynamic_texture_image::DynamicTextureImageBase;
use crate::dev::isxander::yacl3::gui::image::r#impl::resource_texture_image::ResourceTextureImage;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public record OptionDescriptionImpl(Component text, CompletableFuture<Optional<ImageRenderer>> image) implements OptionDescription
#[derive(Clone, Debug)]
pub struct OptionDescriptionImpl {
    /// java: Component text
    pub text: MutableComponent,
    /// java: CompletableFuture<Optional<ImageRenderer>> image
    pub image: mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Shared<dyn ImageRenderer>>>,
}

impl OptionDescriptionImpl {
    /// java: the canonical record constructor
    pub fn new(mut text: MutableComponent, mut image: mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Shared<dyn ImageRenderer>>>) -> Self {
        Self { text, image }
    }

    /// java: the record accessor `text()`
    pub fn text(&self) -> MutableComponent {
        self.text.clone()
    }

    /// java: the record accessor `image()`
    pub fn image(&self) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Shared<dyn ImageRenderer>>> {
        self.image.clone()
    }

}

impl OptionDescription for OptionDescriptionImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public static class BuilderImpl implements Builder
#[derive(Clone)]
pub struct BuilderImpl {
    /// java: private final List<Component> descriptionLines = new ArrayList<>();
    pub description_lines: Vec<MutableComponent>,
    /// java: private CompletableFuture<Optional<ImageRenderer>> image = CompletableFuture.completedFuture(Optional.empty());
    pub image: mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Shared<dyn ImageRenderer>>>,
    /// java: private boolean imageUnset = true;
    pub image_unset: bool,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            description_lines: Vec::new(),
            image: CompletableFuture::completed_future(None),
            image_unset: true,
        });
        __this
    }

}

impl Builder for BuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder text(Component... description) (dispatch: the body for this class)
    fn text(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        BuilderImpl::text_dyn
    }

    /// java: @Override public Builder text(Collection<? extends Component> lines) (dispatch: the body for this class)
    fn text_collection(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        BuilderImpl::text_collection_dyn
    }

    /// java: @Override public Builder image(Identifier image, int width, int height) (dispatch: the body for this class)
    fn image(&self) -> fn(&compat::Shared<dyn Builder>, &Identifier, i32, i32) -> compat::Shared<dyn Builder> {
        BuilderImpl::image_dyn
    }

    /// java: @Override public Builder image(Identifier image, float u, float v, int width, int height, int textureWidth, int textureHeight) (dispatch: the body for this class)
    fn image_7(&self) -> fn(&compat::Shared<dyn Builder>, &Identifier, f32, f32, i32, i32, i32, i32) -> compat::Shared<dyn Builder> {
        BuilderImpl::image_7_dyn
    }

    /// java: @Override public Builder image(Path path, Identifier uniqueLocation) (dispatch: the body for this class)
    fn image_2(&self) -> fn(&compat::Shared<dyn Builder>, &std::path::PathBuf, &Identifier) -> compat::Shared<dyn Builder> {
        BuilderImpl::image_2_dyn
    }

    /// java: @Override public Builder gifImage(Identifier image) (dispatch: the body for this class)
    fn gif_image(&self) -> fn(&compat::Shared<dyn Builder>, &Identifier) -> compat::Shared<dyn Builder> {
        BuilderImpl::gif_image_dyn
    }

    /// java: @Override public Builder gifImage(Path path, Identifier uniqueLocation) (dispatch: the body for this class)
    fn gif_image_2(&self) -> fn(&compat::Shared<dyn Builder>, &std::path::PathBuf, &Identifier) -> compat::Shared<dyn Builder> {
        BuilderImpl::gif_image_2_dyn
    }

    /// java: @Override public Builder webpImage(Identifier image) (dispatch: the body for this class)
    fn webp_image(&self) -> fn(&compat::Shared<dyn Builder>, &Identifier) -> compat::Shared<dyn Builder> {
        BuilderImpl::webp_image_dyn
    }

    /// java: @Override public Builder webpImage(Path path, Identifier uniqueLocation) (dispatch: the body for this class)
    fn webp_image_2(&self) -> fn(&compat::Shared<dyn Builder>, &std::path::PathBuf, &Identifier) -> compat::Shared<dyn Builder> {
        BuilderImpl::webp_image_2_dyn
    }

    /// java: @Override public Builder customImage(CompletableFuture<Optional<ImageRenderer>> image) (dispatch: the body for this class)
    fn custom_image(&self) -> fn(&compat::Shared<dyn Builder>, mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Shared<dyn ImageRenderer>>>) -> compat::Shared<dyn Builder> {
        BuilderImpl::custom_image_dyn
    }

    /// java: @Override public OptionDescription build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn OptionDescription> {
        BuilderImpl::build_dyn
    }

}

impl BuilderImpl {
    /// java: @Override public Builder text(Component... description)
    pub fn text_impl(__this: &compat::Shared<Self>, description: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        let __h0 = description.to_vec().into_iter();
        __this.lock().borrow_mut().description_lines.extend(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.text as the dispatch of Builder.text
    pub fn text_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        Self::text_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder text(Collection<? extends Component> lines)
    pub fn text_collection_impl(__this: &compat::Shared<Self>, lines: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        let __h0 = lines.iter().cloned();
        __this.lock().borrow_mut().description_lines.extend(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.text as the dispatch of Builder.text
    pub fn text_collection_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        Self::text_collection_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder image(Identifier image, int width, int height)
    pub fn image_impl(__this: &compat::Shared<Self>, image: &Identifier, mut width: i32, mut height: i32) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(imageUnset, "Image already set!");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(width > 0, "Width must be greater than 0!");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(height > 0, "Height must be greater than 0!");
        { let __v = (ImageRendererManager::register_or_get_image(image.clone(), Arc::new(move || ResourceTextureImage::create_factory(image, ((0) as f32) as f32, ((0) as f32) as f32, width, height, width, height)))).then_apply(|__a0| Some(__a0.clone())); compat::write(__this, |__t| __t.image = __v); }
        { let __v = false; compat::write(__this, |__t| __t.image_unset = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.image as the dispatch of Builder.image
    pub fn image_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Identifier, __p1: i32, __p2: i32) -> compat::Shared<dyn Builder> {
        Self::image_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public Builder image(Identifier image, float u, float v, int width, int height, int textureWidth, int textureHeight)
    pub fn image_7_impl(__this: &compat::Shared<Self>, image: &Identifier, mut u: f32, mut v: f32, mut width: i32, mut height: i32, mut texture_width: i32, mut texture_height: i32) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(imageUnset, "Image already set!");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(width > 0, "Width must be greater than 0!");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(height > 0, "Height must be greater than 0!");
        { let __v = (ImageRendererManager::register_or_get_image(image.clone(), Arc::new(move || ResourceTextureImage::create_factory(image, u, v, width, height, texture_width, texture_height)))).then_apply(|__a0| Some(__a0.clone())); compat::write(__this, |__t| __t.image = __v); }
        { let __v = false; compat::write(__this, |__t| __t.image_unset = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.image as the dispatch of Builder.image
    pub fn image_7_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Identifier, __p1: f32, __p2: f32, __p3: i32, __p4: i32, __p5: i32, __p6: i32) -> compat::Shared<dyn Builder> {
        Self::image_7_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3, __p4, __p5, __p6)
    }

    /// java: @Override public Builder image(Path path, Identifier uniqueLocation)
    pub fn image_2_impl(__this: &compat::Shared<Self>, path: &std::path::PathBuf, unique_location: &Identifier) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(imageUnset, "Image already set!");
        { let __v = (ImageRendererManager::register_or_get_image(unique_location.clone(), Arc::new(move || DynamicTextureImageBase::from_path(path, unique_location, *debug_properties::IMAGE_FILTERING)))).then_apply(|__a0| Some(__a0.clone())); compat::write(__this, |__t| __t.image = __v); }
        { let __v = false; compat::write(__this, |__t| __t.image_unset = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.image as the dispatch of Builder.image
    pub fn image_2_dyn(__this: &compat::Shared<dyn Builder>, __p0: &std::path::PathBuf, __p1: &Identifier) -> compat::Shared<dyn Builder> {
        Self::image_2_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public Builder gifImage(Identifier image)
    pub fn gif_image_impl(__this: &compat::Shared<Self>, image: &Identifier) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(imageUnset, "Image already set!");
        { let __v = (ImageRendererManager::register_or_get_image(image.clone(), Arc::new(move || AnimatedDynamicTextureImage::create_gif_from_texture(image)))).then_apply(|__a0| Some(__a0.clone())); compat::write(__this, |__t| __t.image = __v); }
        { let __v = false; compat::write(__this, |__t| __t.image_unset = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.gifImage as the dispatch of Builder.gifImage
    pub fn gif_image_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Identifier) -> compat::Shared<dyn Builder> {
        Self::gif_image_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder gifImage(Path path, Identifier uniqueLocation)
    pub fn gif_image_2_impl(__this: &compat::Shared<Self>, path: &std::path::PathBuf, unique_location: &Identifier) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(imageUnset, "Image already set!");
        { let __v = (ImageRendererManager::register_or_get_image(unique_location.clone(), Arc::new(move || AnimatedDynamicTextureImage::create_gif_from_path(path, unique_location)))).then_apply(|__a0| Some(__a0.clone())); compat::write(__this, |__t| __t.image = __v); }
        { let __v = false; compat::write(__this, |__t| __t.image_unset = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.gifImage as the dispatch of Builder.gifImage
    pub fn gif_image_2_dyn(__this: &compat::Shared<dyn Builder>, __p0: &std::path::PathBuf, __p1: &Identifier) -> compat::Shared<dyn Builder> {
        Self::gif_image_2_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public Builder webpImage(Identifier image)
    pub fn webp_image_impl(__this: &compat::Shared<Self>, image: &Identifier) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(imageUnset, "Image already set!");
        { let __v = (ImageRendererManager::register_or_get_image(image.clone(), Arc::new(move || AnimatedDynamicTextureImage::create_webp_from_texture(image)))).then_apply(|__a0| Some(__a0.clone())); compat::write(__this, |__t| __t.image = __v); }
        { let __v = false; compat::write(__this, |__t| __t.image_unset = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.webpImage as the dispatch of Builder.webpImage
    pub fn webp_image_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Identifier) -> compat::Shared<dyn Builder> {
        Self::webp_image_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder webpImage(Path path, Identifier uniqueLocation)
    pub fn webp_image_2_impl(__this: &compat::Shared<Self>, path: &std::path::PathBuf, unique_location: &Identifier) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(imageUnset, "Image already set!");
        { let __v = (ImageRendererManager::register_or_get_image(unique_location.clone(), Arc::new(move || AnimatedDynamicTextureImage::create_webp_from_path(path, unique_location)))).then_apply(|__a0| Some(__a0.clone())); compat::write(__this, |__t| __t.image = __v); }
        { let __v = false; compat::write(__this, |__t| __t.image_unset = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.webpImage as the dispatch of Builder.webpImage
    pub fn webp_image_2_dyn(__this: &compat::Shared<dyn Builder>, __p0: &std::path::PathBuf, __p1: &Identifier) -> compat::Shared<dyn Builder> {
        Self::webp_image_2_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public Builder customImage(CompletableFuture<Optional<ImageRenderer>> image)
    pub fn custom_image_impl(__this: &compat::Shared<Self>, mut image: mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Shared<dyn ImageRenderer>>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(image, "Image cannot be null!");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(imageUnset, "Image already set!");
        { let __v = image.clone(); compat::write(__this, |__t| __t.image = __v); }
        { let __v = false; compat::write(__this, |__t| __t.image_unset = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.customImage as the dispatch of Builder.customImage
    pub fn custom_image_dyn(__this: &compat::Shared<dyn Builder>, __p0: mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Shared<dyn ImageRenderer>>>) -> compat::Shared<dyn Builder> {
        Self::custom_image_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public OptionDescription build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn OptionDescription> {
        let mut concatenated_description: MutableComponent = component::empty();
        let mut iter = compat::read(__this, |__t| __t.description_lines.clone()).into_iter();
        // TODO(fabric2seer): the condition: Iterator.hasNext - use a `for` loop / `while let Some(x) = it.next()`
        //   java: iter.hasNext(..)
        while todo!("fabric2seer: the condition (TODO above)") {
            concatenated_description = concatenated_description.append(iter.next().unwrap());
            // TODO(fabric2seer): the condition: Iterator.hasNext - use a `for` loop / `while let Some(x) = it.next()`
            //   java: iter.hasNext(..)
            if todo!("fabric2seer: the condition (TODO above)") {
                concatenated_description = concatenated_description.append_str("\n");
            }
        }
        return (compat::shared(OptionDescriptionImpl::new(concatenated_description.clone(), compat::read(__this, |__t| __t.image.clone()))) as compat::Shared<dyn OptionDescription>);
    }

    /// BuilderImpl.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn OptionDescription> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
