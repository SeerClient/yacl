//! Ported from dev.isxander.yacl3.config.v2.api.serializer.GsonConfigSerializerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::ConfigSerializer;
use crate::dev::isxander::yacl3::config::v2::r#impl::serializer::gson_config_serializer::Builder;

/// java: public interface GsonConfigSerializerBuilder<T>
///
/// `&self` methods (state in cells): a builder chain reaches the same object again (see `ConfigClassHandler.Builder`).
pub trait GsonConfigSerializerBuilder<T: 'static> {
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: GsonConfigSerializerBuilder<T> setPath(Path path);
    fn set_path(&self, path: std::path::PathBuf) -> Arc<dyn GsonConfigSerializerBuilder<T>>;

    /// java: GsonConfigSerializerBuilder<T> overrideGsonBuilder(GsonBuilder gsonBuilder);
    fn override_gson_builder(&self, gson_builder: &compat::GsonBuilder) -> Arc<dyn GsonConfigSerializerBuilder<T>>;

    /// java: GsonConfigSerializerBuilder<T> overrideGsonBuilder(Gson gson);
    fn override_gson_builder_gson(&self, gson: &compat::Gson) -> Arc<dyn GsonConfigSerializerBuilder<T>>;

    /// java: GsonConfigSerializerBuilder<T> appendGsonBuilder(UnaryOperator<GsonBuilder> gsonBuilder);
    fn append_gson_builder(&self, gson_builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>) -> Arc<dyn GsonConfigSerializerBuilder<T>>;

    /// java: GsonConfigSerializerBuilder<T> setJson5(boolean json5);
    fn set_json5(&self, json5: bool) -> Arc<dyn GsonConfigSerializerBuilder<T>>;

    /// java: ConfigSerializer<T> build();
    fn build(&self) -> Arc<dyn ConfigSerializer<T>>;
}

/// java: static <T> GsonConfigSerializerBuilder<T> create(ConfigClassHandler<T> config)
pub fn create<T: 'static>(config: &Arc<dyn ConfigClassHandler<T>>) -> Arc<dyn GsonConfigSerializerBuilder<T>> {
    Builder::<T>::new(config.clone())
}
