//! Ported from dev.isxander.yacl3.config.v2.api.serializer.GsonConfigSerializerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::ConfigSerializer;
use crate::dev::isxander::yacl3::config::v2::r#impl::serializer::gson_config_serializer::Builder;

/// java: public interface GsonConfigSerializerBuilder<T>
pub trait GsonConfigSerializerBuilder<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: GsonConfigSerializerBuilder<T> setPath(Path path);
    fn set_path(&mut self, path: std::path::PathBuf) -> compat::Shared<dyn GsonConfigSerializerBuilder<T>>;

    /// java: GsonConfigSerializerBuilder<T> overrideGsonBuilder(GsonBuilder gsonBuilder);
    fn override_gson_builder(&mut self, gson_builder: &compat::GsonBuilder) -> compat::Shared<dyn GsonConfigSerializerBuilder<T>>;

    /// java: GsonConfigSerializerBuilder<T> overrideGsonBuilder(Gson gson);
    fn override_gson_builder_gson(&mut self, gson: &compat::Gson) -> compat::Shared<dyn GsonConfigSerializerBuilder<T>>;

    /// java: GsonConfigSerializerBuilder<T> appendGsonBuilder(UnaryOperator<GsonBuilder> gsonBuilder);
    fn append_gson_builder(&mut self, gson_builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>) -> compat::Shared<dyn GsonConfigSerializerBuilder<T>>;

    /// java: GsonConfigSerializerBuilder<T> setJson5(boolean json5);
    fn set_json5(&mut self, json5: bool) -> compat::Shared<dyn GsonConfigSerializerBuilder<T>>;

    /// java: ConfigSerializer<T> build();
    fn build(&self) -> compat::Shared<dyn ConfigSerializer<T>>;

}

/// java: static <T> GsonConfigSerializerBuilder<T> create(ConfigClassHandler<T> config)
pub fn create<T: 'static>(config: &compat::Shared<dyn ConfigClassHandler<T>>) -> compat::Shared<dyn GsonConfigSerializerBuilder<T>> {
    return (compat::shared(Builder::new(config.clone())) as compat::Shared<dyn GsonConfigSerializerBuilder<compat::Object>>);
}
