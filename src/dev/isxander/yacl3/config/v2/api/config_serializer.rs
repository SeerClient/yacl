//! Ported from dev.isxander.yacl3.config.v2.api.ConfigSerializer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;

/// java: public abstract class ConfigSerializer<T>
#[derive(Clone)]
pub struct ConfigSerializerBase<T> {
    /// java: protected final ConfigClassHandler<T> config;
    pub config: compat::Shared<dyn ConfigClassHandler<T>>,
}

impl<T: Clone + 'static> ConfigSerializerBase<T> {
    /// java: public ConfigSerializer(ConfigClassHandler<T> config)
    pub fn new(mut config: compat::Shared<dyn ConfigClassHandler<T>>) -> Self {
        let __init_config = config.clone();
        let mut this = Self {
            config: __init_config,
        };
        this
    }

}

/// java: the virtual methods of ConfigSerializer (implemented by ConfigSerializerBase and every subclass)
pub trait ConfigSerializer<T: Clone + 'static> {
    fn config_serializer_base(&self) -> &ConfigSerializerBase<T>;
    fn config_serializer_base_mut(&mut self) -> &mut ConfigSerializerBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public abstract void save();
    fn save(&self);

    /// java: public LoadResult loadSafely(Map<ConfigField<?>, FieldAccess<?>> bufferAccessMap)
    fn load_safely(&self, buffer_access_map: &std::collections::HashMap<compat::Shared<dyn ConfigField<compat::Object>>, compat::Shared<dyn FieldAccess<compat::Object>>>) -> LoadResult {
        self.load();
        return LoadResult::NoChange;
    }

    /// java: @Deprecated public void load()
    fn load(&mut self) {
        panic!("IllegalArgumentException: {}", "load() is deprecated, use loadSafely() instead.");
    }

}

/// java: public enum LoadResult
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LoadResult {
    Success,
    Failure,
    NoChange,
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
