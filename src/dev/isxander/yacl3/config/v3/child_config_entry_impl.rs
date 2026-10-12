//! Ported from dev.isxander.yacl3.config.v3.ChildConfigEntryImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntry;
use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntryBase;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::com::mojang::serialization::dynamic_ops::DynamicOps;
use mcrust::com::mojang::serialization::map_codec::MapCodec;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::com::mojang::serialization::record_builder::RecordBuilder;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::map_encoder::MapEncoder as _;

/// java: @ApiStatus.Experimental public class ChildConfigEntryImpl<T extends CodecConfig<T>> extends AbstractReadonlyConfigEntry<T>
pub struct ChildConfigEntryImpl<T> {
    /// java: extends AbstractReadonlyConfigEntry
    pub base: AbstractReadonlyConfigEntryBase<T>,
    /// java: private final T config;
    pub config: T,
    /// java: private final MapCodec<T> mapCodec;
    pub map_codec: Option<MapCodecRef<T>>,
}

impl<T: 'static> std::ops::Deref for ChildConfigEntryImpl<T> {
    type Target = AbstractReadonlyConfigEntryBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for ChildConfigEntryImpl<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> ChildConfigEntryImpl<T> {
    /// java: public ChildConfigEntryImpl(String fieldName, T config)
    pub fn new(mut field_name: String, mut config: T) -> Self {
        let __init_config = config.clone();
        let mut this = Self {
            base: AbstractReadonlyConfigEntryBase::<T>::new(field_name.clone()),
            config: __init_config,
            map_codec: None,
        };
        // TODO(fabric2seer): `.fieldOf(..)` on a T
        //   java: this.mapCodec = config.fieldOf(this.fieldName());
        this
    }

}

impl<T: Clone + 'static> AbstractReadonlyConfigEntry<T> for ChildConfigEntryImpl<T> {
    fn abstract_readonly_config_entry_base(&self) -> &AbstractReadonlyConfigEntryBase<T> {
        &self.base
    }
    fn abstract_readonly_config_entry_base_mut(&mut self) -> &mut AbstractReadonlyConfigEntryBase<T> {
        &mut self.base
    }

    /// java: @Override protected T innerGet()
    fn inner_get(&self) -> T {
        return self.config.clone();
    }

}

impl<T: Clone + 'static> ReadonlyConfigEntry<compat::Object> for ChildConfigEntryImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public <R> RecordBuilder<R> encode(DynamicOps<R> ops, RecordBuilder<R> recordBuilder)
    fn encode<R: 'static>(&self, ops: &Arc<dyn DynamicOps<R>>, record_builder: &dyn RecordBuilder<R>) -> Box<dyn RecordBuilder<R>> where Self: Sized {
        return self.map_codec.as_ref().unwrap().encode(&self.config, ops, &mut record_builder);
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

    /// java: @Override public ReadonlyConfigEntry<T> modifyGet(UnaryOperator<T> modifier)
    fn modify_get(&mut self, modifier: Arc<dyn Fn(T) -> T>) -> compat::Shared<dyn ReadonlyConfigEntry<T>> {
        // TODO(fabric2seer): Function.andThen
        //   java: this.getModifier = this.getModifier.andThen(modifier);
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
