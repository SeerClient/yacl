//! Ported from dev.isxander.yacl3.config.v2.api.ConfigClassHandler by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::yet_another_config_lib::YetAnotherConfigLib;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::ConfigSerializer;
use crate::dev::isxander::yacl3::config::v2::r#impl::config_class_handler_impl::BuilderImpl;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public interface ConfigClassHandler<T>
pub trait ConfigClassHandler<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: T instance();
    fn instance(&self) -> T;

    /// java: T defaults();
    fn defaults(&self) -> T;

    /// java: Class<T> configClass();
    fn config_class(&self) -> compat::ClassOf<T>;

    /// java: ConfigField<?>[] fields();
    fn fields(&self) -> Vec<compat::Shared<dyn ConfigField<compat::Object>>>;

    /// java: Identifier id();
    fn id(&self) -> Identifier;

    /// java: YetAnotherConfigLib generateGui();
    fn generate_gui(&self) -> compat::Shared<dyn YetAnotherConfigLib>;

    /// java: boolean supportsAutoGen();
    fn supports_auto_gen(&self) -> bool;

    /// java: boolean load();
    fn load(&mut self) -> bool;

    /// java: void save();
    fn save(&self);

    /// java: @Deprecated ConfigSerializer<T> serializer();
    fn serializer(&self) -> compat::Shared<dyn ConfigSerializer<T>>;

}

/// java: static <T> Builder<T> createBuilder(Class<T> configClass)
pub fn create_builder<T: 'static>(config_class: &compat::ClassOf<T>) -> compat::Shared<dyn Builder<T>> {
    return (compat::shared(BuilderImpl::new(config_class.clone())) as compat::Shared<dyn Builder<compat::Object>>);
}

/// java: interface Builder<T>
pub trait Builder<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Builder<T> id(Identifier id);
    fn id(&mut self, id: Identifier) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> serializer(Function<ConfigClassHandler<T>, ConfigSerializer<T>> serializerFactory);
    fn serializer(&mut self, serializer_factory: Arc<dyn Fn(compat::Shared<dyn ConfigClassHandler<T>>) -> compat::Shared<dyn ConfigSerializer<T>>>) -> compat::Shared<dyn Builder<T>>;

    /// java: ConfigClassHandler<T> build();
    fn build(&self) -> compat::Shared<dyn ConfigClassHandler<T>>;

}
