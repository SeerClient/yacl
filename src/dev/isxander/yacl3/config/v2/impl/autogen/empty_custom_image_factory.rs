//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.EmptyCustomImageFactory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_image::CustomImageFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;

/// java: public class EmptyCustomImageFactory implements CustomImage.CustomImageFactory<Object>
#[derive(Clone)]
pub struct EmptyCustomImageFactory {}

impl EmptyCustomImageFactory {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

impl CustomImageFactory<compat::Object> for EmptyCustomImageFactory {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public CompletableFuture<ImageRenderer> createImage(Object value, ConfigField<Object> field, OptionAccess access)
    fn create_image(&self, value: &compat::Object, field: &compat::Shared<dyn ConfigField<compat::Object>>, access: &compat::Shared<dyn OptionAccess>) -> mcrust::java::util::concurrent::completable_future::CompletableFuture<compat::Shared<dyn ImageRenderer>> {
        panic!("IllegalStateException");
    }

}
