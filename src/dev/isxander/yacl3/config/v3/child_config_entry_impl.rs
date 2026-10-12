//! Ported from dev.isxander.yacl3.config.v3.ChildConfigEntryImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER;
use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntry;
use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntryBase;
use crate::dev::isxander::yacl3::config::v3::codec_config;
use crate::dev::isxander::yacl3::config::v3::codec_config::CodecConfig;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::AnyReadonlyConfigEntry;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::dynamic_ops::Ops;
use mcrust::com::mojang::serialization::map_codec::MapCodecRef;
use mcrust::com::mojang::serialization::map_decoder::MapDecoderDecoder;
use mcrust::com::mojang::serialization::map_encoder::MapEncoder as _;
use mcrust::com::mojang::serialization::record_builder::RecordBuilder;
use mcrust::com::mojang::serialization::Obj;

/// java: @ApiStatus.Experimental public class ChildConfigEntryImpl<T extends CodecConfig<T>> extends AbstractReadonlyConfigEntry<T>
///
/// The child config is the mod's object, held as `compat::Shared<T>`.
pub struct ChildConfigEntryImpl<T: 'static> {
    /// java: extends AbstractReadonlyConfigEntry
    pub base: AbstractReadonlyConfigEntryBase<compat::Shared<T>>,
    /// java: private final T config;
    pub config: compat::Shared<T>,
    /// java: private final MapCodec<T> mapCodec;
    pub map_codec: MapCodecRef<compat::Shared<T>>,
    /// Java's `this`, handed back by `modifyGet`
    pub self_ref: std::sync::Weak<Self>,
}

impl<T: 'static> std::ops::Deref for ChildConfigEntryImpl<T> {
    type Target = AbstractReadonlyConfigEntryBase<compat::Shared<T>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: CodecConfig<T> + 'static> ChildConfigEntryImpl<T> {
    /// java: public ChildConfigEntryImpl(String fieldName, T config)
    pub fn new(field_name: String, config: compat::Shared<T>) -> Arc<Self> {
        Arc::new_cyclic(|self_ref| {
            let base = AbstractReadonlyConfigEntryBase::new(field_name);
            // java: this.mapCodec = config.fieldOf(this.fieldName());
            let map_codec = codec::field_of(codec_config::codec(&config), &base.field_name());
            Self { base, config, map_codec, self_ref: self_ref.clone() }
        })
    }
}

impl<T: CodecConfig<T> + 'static> AnyReadonlyConfigEntry for ChildConfigEntryImpl<T> {
    /// java: @Override public String fieldName() (AbstractReadonlyConfigEntry)
    fn field_name(&self) -> String {
        self.base.field_name()
    }

    /// java: @Override public <R> RecordBuilder<R> encode(DynamicOps<R> ops, RecordBuilder<R> recordBuilder)
    fn encode(&self, ops: &Ops, record_builder: &mut dyn RecordBuilder) {
        self.map_codec.encode(&self.config, ops, record_builder);
    }

    /// java: @Override public <R> boolean decode(R encoded, DynamicOps<R> ops)
    fn decode(&self, encoded: &Obj, ops: &Ops) -> bool {
        // java: mapCodec.decoder().parse(ops, encoded)
        let result: DataResult<compat::Shared<T>> = MapDecoderDecoder { inner: self.map_codec.clone() }.parse(ops, encoded);
        if let Some(error) = result.error_ref() {
            LOGGER.error(&format!("Failed to decode entry {}: {}", self.field_name(), error.message()));
            return false;
        }

        true
    }
}

impl<T: CodecConfig<T> + 'static> ReadonlyConfigEntry<compat::Shared<T>> for ChildConfigEntryImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public T get() (AbstractReadonlyConfigEntry)
    fn get(&self) -> compat::Shared<T> {
        self.base.get(self.inner_get())
    }

    /// java: @Override public ReadonlyConfigEntry<T> modifyGet(UnaryOperator<T> modifier) (AbstractReadonlyConfigEntry)
    fn modify_get(&self, modifier: Arc<dyn Fn(compat::Shared<T>) -> compat::Shared<T>>) -> Arc<dyn ReadonlyConfigEntry<compat::Shared<T>>> {
        self.base.modify_get(modifier);
        self.self_ref.upgrade().expect("ChildConfigEntryImpl used after it was dropped")
    }
}

impl<T: CodecConfig<T> + 'static> AbstractReadonlyConfigEntry<compat::Shared<T>> for ChildConfigEntryImpl<T> {
    fn abstract_readonly_config_entry_base(&self) -> &AbstractReadonlyConfigEntryBase<compat::Shared<T>> {
        &self.base
    }

    /// java: @Override protected T innerGet()
    fn inner_get(&self) -> compat::Shared<T> {
        self.config.clone()
    }
}
