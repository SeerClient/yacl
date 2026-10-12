//! Ported from dev.isxander.yacl3.config.v3.EntryAddable by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v3::codec_config::CodecConfig;
use crate::dev::isxander::yacl3::config::v3::config_entry::ConfigEntry;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry;
use mcrust::com::mojang::serialization::codec::CodecRef;

/// java: @ApiStatus.Experimental public interface EntryAddable
///
/// `CodecConfig`'s implementation is `CodecConfigBase::register` / `register_2` (each subclass's `impl EntryAddable`
/// calls them on its `codec_config_base()`).
pub trait EntryAddable {
    /// java: <T> ConfigEntry<T> register(String fieldName, T defaultValue, Codec<T> codec);
    fn register<T: Clone + 'static>(&self, field_name: &str, default_value: T, codec: CodecRef<T>) -> Arc<dyn ConfigEntry<T>>
    where
        Self: Sized;

    /// java: <T extends CodecConfig<T>> ReadonlyConfigEntry<T> register(String fieldName, T configInstance);
    fn register_2<T: CodecConfig<T> + 'static>(&self, field_name: &str, config_instance: compat::Shared<T>) -> Arc<dyn ReadonlyConfigEntry<compat::Shared<T>>>
    where
        Self: Sized;
}
