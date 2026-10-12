//! Ported from dev.isxander.yacl3.config.v3.EntryAddable by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v3::codec_config::CodecConfig;
use crate::dev::isxander::yacl3::config::v3::config_entry::ConfigEntry;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

/// java: @ApiStatus.Experimental public interface EntryAddable
pub trait EntryAddable: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: <T> ConfigEntry<T> register(String fieldName, T defaultValue, Codec<T> codec);
    fn register<T: 'static>(&mut self, field_name: &str, default_value: &T, codec: &CodecRef<T>) -> compat::Shared<dyn ConfigEntry<T>> where Self: Sized;

    /// java: <T extends CodecConfig<T>> ReadonlyConfigEntry<T> register(String fieldName, T configInstance);
    fn register_2(&mut self, field_name: &str, config_instance: &compat::Shared<dyn CodecConfig<compat::Object>>) -> compat::Shared<dyn ReadonlyConfigEntry<compat::Shared<dyn CodecConfig<compat::Object>>>>;

}
