//! Ported from dev.isxander.yacl3.api.OptionDescription by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::r#impl::option_description_impl::BuilderImpl;
use crate::dev::isxander::yacl3::r#impl::option_description_impl::OptionDescriptionImpl;
use mcrust::java::util::concurrent::completable_future::CompletableFuture;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: OptionDescription EMPTY = new OptionDescriptionImpl(CommonComponents.EMPTY, CompletableFuture.completedFuture(Optional.empty()));
pub static EMPTY: compat::MainThread<compat::Shared<dyn OptionDescription>> = compat::MainThread::new(|| (compat::shared(OptionDescriptionImpl::new(mcrust::net::minecraft::network::chat::common_components::empty(), CompletableFuture::completed_future(None))) as compat::Shared<dyn OptionDescription>));

/// java: public interface OptionDescription
pub trait OptionDescription: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Component text(); (dispatch: the body for this class)
    fn text(&self) -> fn(&compat::Shared<dyn OptionDescription>) -> MutableComponent;

    /// java: CompletableFuture<Optional<ImageRenderer>> image(); (dispatch: the body for this class)
    fn image(&self) -> fn(&compat::Shared<dyn OptionDescription>) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Shared<dyn ImageRenderer>>>;

}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (BuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: static OptionDescription of(Component... description)
pub fn of(description: &Vec<MutableComponent>) -> compat::Shared<dyn OptionDescription> {
    return ({ let __r = &({ let __r = &create_builder(); let __f = compat::dispatch(__r, |__t| __t.text()); __f(__r, description) }); let __f = compat::dispatch(__r, |__t| __t.build()); __f(__r) });
}

/// java: interface Builder
pub trait Builder: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Builder text(Component... description); (dispatch: the body for this class)
    fn text(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder>;

    /// java: Builder text(Collection<? extends Component> lines); (dispatch: the body for this class)
    fn text_collection(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder>;

    /// java: Builder image(Identifier image, int width, int height); (dispatch: the body for this class)
    fn image(&self) -> fn(&compat::Shared<dyn Builder>, &Identifier, i32, i32) -> compat::Shared<dyn Builder>;

    /// java: Builder image(Identifier image, float u, float v, int width, int height, int textureWidth, int textureHeight); (dispatch: the body for this class)
    fn image_7(&self) -> fn(&compat::Shared<dyn Builder>, &Identifier, f32, f32, i32, i32, i32, i32) -> compat::Shared<dyn Builder>;

    /// java: Builder image(Path path, Identifier uniqueLocation); (dispatch: the body for this class)
    fn image_2(&self) -> fn(&compat::Shared<dyn Builder>, &std::path::PathBuf, &Identifier) -> compat::Shared<dyn Builder>;

    /// java: Builder webpImage(Identifier image); (dispatch: the body for this class)
    fn webp_image(&self) -> fn(&compat::Shared<dyn Builder>, &Identifier) -> compat::Shared<dyn Builder>;

    /// java: Builder webpImage(Path path, Identifier uniqueLocation); (dispatch: the body for this class)
    fn webp_image_2(&self) -> fn(&compat::Shared<dyn Builder>, &std::path::PathBuf, &Identifier) -> compat::Shared<dyn Builder>;

    /// java: Builder customImage(CompletableFuture<Optional<ImageRenderer>> image); (dispatch: the body for this class)
    fn custom_image(&self) -> fn(&compat::Shared<dyn Builder>, mcrust::java::util::concurrent::completable_future::CompletableFuture<Option<compat::Shared<dyn ImageRenderer>>>) -> compat::Shared<dyn Builder>;

    /// java: default Builder customImage(ImageRenderer image) (dispatch: the body for this class)
    fn custom_image_image_renderer(&self) -> fn(&compat::Shared<dyn Builder>, &compat::Shared<dyn ImageRenderer>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::custom_image_image_renderer_body
    }

    /// java: @Deprecated Builder gifImage(Identifier image); (dispatch: the body for this class)
    fn gif_image(&self) -> fn(&compat::Shared<dyn Builder>, &Identifier) -> compat::Shared<dyn Builder>;

    /// java: @Deprecated Builder gifImage(Path path, Identifier uniqueLocation); (dispatch: the body for this class)
    fn gif_image_2(&self) -> fn(&compat::Shared<dyn Builder>, &std::path::PathBuf, &Identifier) -> compat::Shared<dyn Builder>;

    /// java: OptionDescription build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn OptionDescription>;

}

impl dyn Builder {
    /// java: default Builder customImage(ImageRenderer image)
    pub fn custom_image_image_renderer_body(__this: &compat::Shared<dyn Builder>, image: &compat::Shared<dyn ImageRenderer>) -> compat::Shared<dyn Builder> {
        return ({ let __f = compat::dispatch(__this, |__t| __t.custom_image()); __f(__this, CompletableFuture::completed_future(Some(image.clone()))) });
    }

}
