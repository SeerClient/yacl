//! Ported from dev.isxander.yacl3.config.v3.CodecConfigEntryImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER;
use crate::dev::isxander::yacl3::config::v3::abstract_config_entry::AbstractConfigEntry;
use crate::dev::isxander::yacl3::config::v3::abstract_config_entry::AbstractConfigEntryBase;
use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntry;
use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntryBase;
use crate::dev::isxander::yacl3::config::v3::config_entry::ConfigEntry;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::AnyReadonlyConfigEntry;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::dynamic_ops::Ops;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::com::mojang::serialization::map_decoder::MapDecoderDecoder;
use mcrust::com::mojang::serialization::map_encoder::MapEncoder as _;
use mcrust::com::mojang::serialization::record_builder::RecordBuilder;
use mcrust::com::mojang::serialization::Obj;

/// java: @ApiStatus.Experimental public class CodecConfigEntryImpl<T> extends AbstractConfigEntry<T>
pub struct CodecConfigEntryImpl<T: 'static> {
    /// java: extends AbstractConfigEntry
    pub base: AbstractConfigEntryBase<T>,
    /// java: private final MapCodec<T> mapCodec;
    pub map_codec: MapCodecRef<T>,
    /// Java's `this`, handed back by `modifyGet` / `modifySet`
    pub self_ref: std::sync::Weak<Self>,
}

impl<T: 'static> std::ops::Deref for CodecConfigEntryImpl<T> {
    type Target = AbstractConfigEntryBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: Clone + 'static> CodecConfigEntryImpl<T> {
    /// java: public CodecConfigEntryImpl(String fieldName, T defaultValue, Codec<T> codec)
    pub fn new(field_name: String, default_value: T, codec: CodecRef<T>) -> Arc<Self> {
        Arc::new_cyclic(|self_ref| {
            let base = AbstractConfigEntryBase::new(field_name, default_value);
            // java: this.mapCodec = codec.fieldOf(this.fieldName());
            let map_codec = codec::field_of(codec, &base.field_name());
            Self { base, map_codec, self_ref: self_ref.clone() }
        })
    }

    fn this(&self) -> Arc<Self> {
        self.self_ref.upgrade().expect("CodecConfigEntryImpl used after it was dropped")
    }
}

impl<T: Clone + 'static> AnyReadonlyConfigEntry for CodecConfigEntryImpl<T> {
    /// java: @Override public String fieldName() (AbstractReadonlyConfigEntry)
    fn field_name(&self) -> String {
        self.base.field_name()
    }

    /// java: @Override public <R> RecordBuilder<R> encode(DynamicOps<R> ops, RecordBuilder<R> recordBuilder)
    fn encode(&self, ops: &Ops, record_builder: &mut dyn RecordBuilder) {
        self.map_codec.encode(&self.get(), ops, record_builder);
    }

    /// java: @Override public <R> boolean decode(R encoded, DynamicOps<R> ops)
    fn decode(&self, encoded: &Obj, ops: &Ops) -> bool {
        // java: mapCodec.decoder().parse(ops, encoded)
        let result: DataResult<T> = MapDecoderDecoder { inner: self.map_codec.clone() }.parse(ops, encoded);
        if let Some(error) = result.error_ref() {
            LOGGER.error(&format!("Failed to decode entry {}: {}", self.field_name(), error.message()));
            return false;
        }

        let value: T = result.result().expect("NoSuchElementException: No value present");
        self.set(value);
        true
    }
}

impl<T: Clone + 'static> ReadonlyConfigEntry<T> for CodecConfigEntryImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public T get() (AbstractReadonlyConfigEntry)
    fn get(&self) -> T {
        self.base.base.get(self.inner_get())
    }

    /// java: @Override public ConfigEntry<T> modifyGet(UnaryOperator<T> modifier) (AbstractConfigEntry)
    fn modify_get(&self, modifier: Arc<dyn Fn(T) -> T>) -> Arc<dyn ReadonlyConfigEntry<T>> {
        self.base.modify_get(modifier);
        self.this()
    }
}

impl<T: Clone + 'static> AbstractReadonlyConfigEntry<T> for CodecConfigEntryImpl<T> {
    fn abstract_readonly_config_entry_base(&self) -> &AbstractReadonlyConfigEntryBase<T> {
        &self.base.base
    }

    /// java: @Override protected T innerGet() (AbstractConfigEntry)
    fn inner_get(&self) -> T {
        self.base.inner_get()
    }
}

impl<T: Clone + 'static> ConfigEntry<T> for CodecConfigEntryImpl<T> {
    /// java: @Override public void set(T value) (AbstractConfigEntry)
    fn set(&self, value: T) {
        self.base.set(value);
    }

    /// java: @Override public T defaultValue() (AbstractConfigEntry)
    fn default_value(&self) -> T {
        self.base.default_value()
    }

    /// java: @Override public ConfigEntry<T> modifyGet(UnaryOperator<T> modifier) (AbstractConfigEntry)
    fn modify_get_entry(&self, modifier: Arc<dyn Fn(T) -> T>) -> Arc<dyn ConfigEntry<T>> {
        self.base.modify_get(modifier);
        self.this()
    }

    /// java: @Override public ConfigEntry<T> modifySet(UnaryOperator<T> modifier) (AbstractConfigEntry)
    fn modify_set(&self, modifier: Arc<dyn Fn(T) -> T>) -> Arc<dyn ConfigEntry<T>> {
        self.base.modify_set(modifier);
        self.this()
    }
}

impl<T: Clone + 'static> AbstractConfigEntry<T> for CodecConfigEntryImpl<T> {
    fn abstract_config_entry_base(&self) -> &AbstractConfigEntryBase<T> {
        &self.base
    }
}
