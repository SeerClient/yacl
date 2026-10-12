//! Ported from dev.isxander.yacl3.config.v3.CodecConfigEntryImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v3::abstract_config_entry::AbstractConfigEntry;
use crate::dev::isxander::yacl3::config::v3::abstract_config_entry::AbstractConfigEntryBase;
use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntry;
use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntryBase;
use crate::dev::isxander::yacl3::config::v3::config_entry::ConfigEntry;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::com::mojang::serialization::dynamic_ops::DynamicOps;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::com::mojang::serialization::record_builder::RecordBuilder;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::map_encoder::MapEncoder as _;

/// java: @ApiStatus.Experimental public class CodecConfigEntryImpl<T> extends AbstractConfigEntry<T>
pub struct CodecConfigEntryImpl<T> {
    /// java: extends AbstractConfigEntry
    pub base: AbstractConfigEntryBase<T>,
    /// java: private final MapCodec<T> mapCodec;
    pub map_codec: Option<MapCodecRef<T>>,
}

impl<T: 'static> std::ops::Deref for CodecConfigEntryImpl<T> {
    type Target = AbstractConfigEntryBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for CodecConfigEntryImpl<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> CodecConfigEntryImpl<T> {
    /// java: public CodecConfigEntryImpl(String fieldName, T defaultValue, Codec<T> codec)
    pub fn new(mut field_name: String, mut default_value: T, mut codec: CodecRef<T>) -> Self {
        let mut this = Self {
            base: AbstractConfigEntryBase::<T>::new(field_name.clone(), default_value.clone()),
            map_codec: None,
        };
        this.map_codec = Some(codec::field_of(codec.clone(), &this.field_name()));
        this
    }

}

impl<T: Clone + 'static> AbstractConfigEntry<T> for CodecConfigEntryImpl<T> {
    fn abstract_config_entry_base(&self) -> &AbstractConfigEntryBase<T> {
        &self.base
    }
    fn abstract_config_entry_base_mut(&mut self) -> &mut AbstractConfigEntryBase<T> {
        &mut self.base
    }

}

impl<T: Clone + 'static> AbstractReadonlyConfigEntry<compat::Object> for CodecConfigEntryImpl<T> {
    fn abstract_readonly_config_entry_base(&self) -> &AbstractReadonlyConfigEntryBase<compat::Object> {
        &self.base.base
    }
    fn abstract_readonly_config_entry_base_mut(&mut self) -> &mut AbstractReadonlyConfigEntryBase<compat::Object> {
        &mut self.base.base
    }

    /// java: @Override protected T innerGet()
    fn inner_get(&self) -> T {
        return self.value.clone();
    }

}

impl<T: Clone + 'static> ReadonlyConfigEntry<compat::Object> for CodecConfigEntryImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public <R> RecordBuilder<R> encode(DynamicOps<R> ops, RecordBuilder<R> recordBuilder)
    fn encode<R: 'static>(&self, ops: &Arc<dyn DynamicOps<R>>, record_builder: &dyn RecordBuilder<R>) -> Box<dyn RecordBuilder<R>> where Self: Sized {
        return self.map_codec.as_ref().unwrap().encode(&self.get(), ops, &mut record_builder);
    }

    /// java: @Override public <R> boolean decode(R encoded, DynamicOps<R> ops)
    fn decode<R: 'static>(&mut self, encoded: &R, ops: &Arc<dyn DynamicOps<R>>) -> bool where Self: Sized {
        // TODO(fabric2seer): `MapCodec.decoder` has no Rust method in the port (looked for `decoder` in MapCodec and its supertypes)
        // rust: let mut result: DataResult<T> = self.map_codec.as_ref().unwrap().decoder().parse(ops, encoded);
        let mut result: DataResult<T> = todo!("fabric2seer: the initialiser of `result` (TODO above)");
        let mut error: Option<mcrust::com::mojang::serialization::data_result::Error<T>> = result.error_ref().unwrap();
        if error.is_some() {
            logger().error(&format!("Failed to decode entry {}: {}", self.field_name(), error.clone().unwrap().message()));
            return false;
        }
        let mut value: T = result.result().unwrap();
        self.set(&value);
        return true;
    }

    /// java: @Override public String fieldName()
    fn field_name(&self) -> String {
        return self.field_name.clone();
    }

    /// java: @Override public T get()
    fn get(&self) -> T {
        return (self.get_modifier)(self.inner_get());
    }

    /// java: @Override public ConfigEntry<T> modifyGet(UnaryOperator<T> modifier)
    fn modify_get(&mut self, modifier: Arc<dyn Fn(T) -> T>) -> compat::Shared<dyn ConfigEntry<T>> {
        self.base.modify_get(modifier.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl<T: Clone + 'static> ConfigEntry<compat::Object> for CodecConfigEntryImpl<T> {
    /// java: @Override public void set(T value)
    fn set(&mut self, value: &T) {
        self.value = (self.set_modifier)(value.clone());
    }

    /// java: @Override public T defaultValue()
    fn default_value(&self) -> T {
        return self.default_value.clone();
    }

    /// java: @Override public ConfigEntry<T> modifySet(UnaryOperator<T> modifier)
    fn modify_set(&mut self, modifier: Arc<dyn Fn(T) -> T>) -> compat::Shared<dyn ConfigEntry<T>> {
        // TODO(fabric2seer): Function.andThen
        //   java: this.setModifier = this.setModifier.andThen(modifier);
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
