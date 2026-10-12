//! Ported from dev.isxander.yacl3.config.v2.api.autogen.CustomImage by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::empty_custom_image_factory;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ClassRef;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface CustomImage
///
/// Defines a custom image for an option. Without it, the option factory looks for
/// `modid:textures/yacl3/$config_id_path/$fieldName.webp`.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct CustomImage {
    /// java: String value() default "";
    pub value: String,
    /// java: int width() default 0;
    pub width: i32,
    /// java: int height() default 0;
    pub height: i32,
    /// java: Class<? extends CustomImageFactory<?>> factory() default EmptyCustomImageFactory.class;
    /// `new_instance` yields a `compat::RenderThread<Arc<dyn CustomImageFactory<X>>>` (X the type the factory class is
    /// declared for).
    pub factory: ClassRef,
}

impl Default for CustomImage {
    fn default() -> Self {
        CustomImage::of()
    }
}

impl CustomImage {
    /// The annotation as written with no elements: every element takes its declared default.
    pub fn of() -> CustomImage {
        CustomImage { value: "".to_string(), width: 0, height: 0, factory: empty_custom_image_factory::CLASS }
    }

    /// java: String value() default "";
    pub fn value(&self) -> String {
        self.value.clone()
    }

    /// java: int width() default 0;
    pub fn width(&self) -> i32 {
        self.width
    }

    /// java: int height() default 0;
    pub fn height(&self) -> i32 {
        self.height
    }

    /// java: Class<? extends CustomImageFactory<?>> factory() default EmptyCustomImageFactory.class;
    pub fn factory(&self) -> ClassRef {
        self.factory
    }
}

/// java: interface CustomImageFactory<T>
pub trait CustomImageFactory<T: Clone + 'static> {
    /// java: CompletableFuture<ImageRenderer> createImage(T value, ConfigField<T> field, OptionAccess access);
    /// (`compat::LocalFuture`: the port's `CompletableFuture` over the mod's non-`Send` objects)
    fn create_image(&self, value: T, field: &Arc<dyn ConfigField<T>>, access: &Arc<dyn OptionAccess>) -> compat::LocalFuture<Arc<dyn ImageRenderer>>;
}
