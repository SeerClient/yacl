//! Ported from dev.isxander.yacl3.config.v2.api.autogen.OptionFactory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::option_factory_registry::OptionFactoryRegistry;

/// java: public interface OptionFactory<A extends Annotation, T>
pub trait OptionFactory<A: Clone + 'static, T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess);
    fn create_option(&self, annotation: &A, field: &compat::Shared<dyn ConfigField<T>>, option_access: &compat::Shared<dyn OptionAccess>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>;

}

/// java: static <A extends Annotation, T> void register(Class<A> annotationClass, OptionFactory<A, T> factory)
pub fn register<A: 'static, T: 'static>(annotation_class: &compat::ClassOf<A>, factory: &compat::Shared<dyn OptionFactory<A, T>>) {
    OptionFactoryRegistry::register_option_factory(annotation_class.clone(), factory.clone());
}
