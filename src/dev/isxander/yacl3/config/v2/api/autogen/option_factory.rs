//! Ported from dev.isxander.yacl3.config.v2.api.autogen.OptionFactory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::option_factory_registry::OptionFactoryRegistry;

/// java: public interface OptionFactory<A extends Annotation, T>
///
/// The backing builder for option factories' annotations. `A` is the annotation struct, `T` the option's binding
/// type.
pub trait OptionFactory<A: Clone + 'static, T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess);
    fn create_option(&self, annotation: &A, field: &Arc<dyn ConfigField<T>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<T>>;
}

/// java: static <A extends Annotation, T> void register(Class<A> annotationClass, OptionFactory<A, T> factory)
///
/// `A.class` of an annotation is its `TypeId` (`&std::any::TypeId::of::<A>()`).
pub fn register<A: Clone + Send + Sync + 'static, T: Clone + Send + Sync + 'static>(
    annotation_class: &std::any::TypeId,
    factory: &Arc<dyn OptionFactory<A, T>>,
) {
    OptionFactoryRegistry::register_option_factory(annotation_class, factory.clone());
}
