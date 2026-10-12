//! Ported from dev.isxander.yacl3.config.v2.api.ConfigSerializer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;

/// java: public abstract class ConfigSerializer<T> - its fields
pub struct ConfigSerializerBase<T: 'static> {
    /// java: protected final ConfigClassHandler<T> config;
    pub config: Arc<dyn ConfigClassHandler<T>>,
}

impl<T: 'static> Clone for ConfigSerializerBase<T> {
    fn clone(&self) -> Self {
        Self { config: self.config.clone() }
    }
}

impl<T: 'static> ConfigSerializerBase<T> {
    /// java: public ConfigSerializer(ConfigClassHandler<T> config)
    pub fn new(config: Arc<dyn ConfigClassHandler<T>>) -> Self {
        Self { config }
    }
}

/// java: the virtual methods of ConfigSerializer (implemented by every subclass)
///
/// Every method takes `&self`: a serializer is reached through the handler (`config.serializer().save()`) while the
/// handler itself is in use (`ConfigClassHandler.load` -> `loadSafely` -> `config.fields()`).
pub trait ConfigSerializer<T: 'static> {
    fn config_serializer_base(&self) -> &ConfigSerializerBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public abstract void save();
    fn save(&self);

    /// java: public LoadResult loadSafely(Map<ConfigField<?>, FieldAccess<?>> bufferAccessMap)
    fn load_safely(&self, buffer_access_map: &BufferAccessMap) -> LoadResult {
        self.load();
        LoadResult::NoChange
    }

    /// java: @Deprecated public void load()
    fn load(&self) {
        panic!("IllegalArgumentException: {}", "load() is deprecated, use loadSafely() instead.");
    }
}

/// java: `Map<ConfigField<?>, FieldAccess<?>>` - `ConfigFieldImpl` has no `equals`/`hashCode`, so the map is keyed by
/// the field object's identity (`Arc::ptr_eq`).
#[derive(Clone, Default)]
pub struct BufferAccessMap {
    pub entries: Vec<(Arc<dyn ConfigField<compat::Object>>, Arc<dyn FieldAccess<compat::Object>>)>,
}

impl BufferAccessMap {
    pub fn new(entries: Vec<(Arc<dyn ConfigField<compat::Object>>, Arc<dyn FieldAccess<compat::Object>>)>) -> BufferAccessMap {
        BufferAccessMap { entries }
    }

    /// java: Map.get(field) - `None` is Java's `null`
    pub fn get(&self, field: &Arc<dyn ConfigField<compat::Object>>) -> Option<Arc<dyn FieldAccess<compat::Object>>> {
        self.entries
            .iter()
            .find(|(k, _)| std::ptr::addr_eq(Arc::as_ptr(k), Arc::as_ptr(field)))
            .map(|(_, v)| v.clone())
    }

    /// java: Map.size()
    pub fn size(&self) -> i32 {
        self.entries.len() as i32
    }
}

/// java: public enum LoadResult
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LoadResult {
    /// The config was loaded successfully and the temporary object should be applied.
    Success,
    /// The config was not loaded successfully and the load should be abandoned.
    Failure,
    /// The config has not changed after a load and the temporary object should be ignored.
    NoChange,
    /// The config was loaded successfully, but should be re-saved straight away.
    Dirty,
}

impl LoadResult {
    pub const VALUES: [LoadResult; 4] = [LoadResult::Success, LoadResult::Failure, LoadResult::NoChange, LoadResult::Dirty];

    /// java: values()
    pub fn values() -> Vec<LoadResult> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            LoadResult::Success => "SUCCESS",
            LoadResult::Failure => "FAILURE",
            LoadResult::NoChange => "NO_CHANGE",
            LoadResult::Dirty => "DIRTY",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> LoadResult {
        match name {
            "SUCCESS" => LoadResult::Success,
            "FAILURE" => LoadResult::Failure,
            "NO_CHANGE" => LoadResult::NoChange,
            "DIRTY" => LoadResult::Dirty,
            _ => panic!("IllegalArgumentException: No enum constant LoadResult.{name}"),
        }
    }
}

impl std::fmt::Display for LoadResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
