//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.EmptyCustomImageFactory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_image::CustomImageFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ClassRef;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;

/// java: EmptyCustomImageFactory.class - `@CustomImage.factory()`'s default
pub const CLASS: ClassRef = ClassRef {
    name: "dev.isxander.yacl3.config.v2.impl.autogen.EmptyCustomImageFactory",
    new_instance: Some(new_instance),
};

/// `getConstructor().newInstance()`: the factory as the `CustomImageFactory<Object>` it implements
fn new_instance() -> compat::Object {
    let factory: Arc<dyn CustomImageFactory<compat::Object>> = Arc::new(EmptyCustomImageFactory::new());
    Arc::new(compat::RenderThread::new(factory))
}

/// java: public class EmptyCustomImageFactory implements CustomImage.CustomImageFactory<Object>
#[derive(Clone, Default)]
pub struct EmptyCustomImageFactory {}

impl EmptyCustomImageFactory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl CustomImageFactory<compat::Object> for EmptyCustomImageFactory {
    /// java: @Override public CompletableFuture<ImageRenderer> createImage(Object value, ConfigField<Object> field, OptionAccess access)
    fn create_image(
        &self,
        value: compat::Object,
        field: &Arc<dyn ConfigField<compat::Object>>,
        access: &Arc<dyn OptionAccess>,
    ) -> compat::LocalFuture<Arc<dyn ImageRenderer>> {
        panic!("IllegalStateException");
    }
}
