//! Ported from dev.isxander.yacl3.config.v3.CodecConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v3::child_config_entry_impl::ChildConfigEntryImpl;
use crate::dev::isxander::yacl3::config::v3::codec_config_entry_impl::CodecConfigEntryImpl;
use crate::dev::isxander::yacl3::config::v3::config_entry::ConfigEntry;
use crate::dev::isxander::yacl3::config::v3::entry_addable::EntryAddable;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::AnyReadonlyConfigEntry;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::com::mojang::serialization::decoder::Decoder;
use mcrust::com::mojang::serialization::dynamic_ops::DynamicOps as _;
use mcrust::com::mojang::serialization::dynamic_ops::Ops;
use mcrust::com::mojang::serialization::encoder::Encoder;
use mcrust::com::mojang::serialization::record_builder::RecordBuilder as _;
use mcrust::com::mojang::serialization::Obj;

/// java: @ApiStatus.Experimental public abstract class CodecConfig<S extends CodecConfig<S>> implements EntryAddable, Codec<S>
/// - its field, and the bodies of `register` every subclass's `impl EntryAddable` calls (Rust has no inherited impls).
///
/// A config object is the mod's object, `compat::Shared<S>`. Its `Codec<S>` half is [`codec`] (a `Codec` must be
/// `Send + Sync` in the port, and a config holds the mod's non-`Send` objects).
pub struct CodecConfigBase<S> {
    /// java: private final List<ReadonlyConfigEntry<?>> entries = new ArrayList<>();
    pub entries: std::cell::RefCell<Vec<Arc<dyn AnyReadonlyConfigEntry>>>,
    _self_type: std::marker::PhantomData<fn() -> S>,
}

impl<S: 'static> CodecConfigBase<S> {
    /// java: public CodecConfig()
    pub fn new() -> Self {
        // java: var ignored = (S) this; - the cast that throws on a wrong `S` is the type system's check here
        Self { entries: std::cell::RefCell::new(Vec::new()), _self_type: std::marker::PhantomData }
    }

    /// java: @Override public <T> ConfigEntry<T> register(String fieldName, T defaultValue, Codec<T> codec)
    pub fn register<T: Clone + 'static>(&self, field_name: &str, default_value: T, codec: CodecRef<T>) -> Arc<dyn ConfigEntry<T>> {
        let entry: Arc<dyn ConfigEntry<T>> = CodecConfigEntryImpl::new(field_name.to_string(), default_value, codec);
        self.entries.borrow_mut().push(entry.clone());
        entry
    }

    /// java: @Override public <T extends CodecConfig<T>> ReadonlyConfigEntry<T> register(String fieldName, T configInstance)
    pub fn register_2<T: CodecConfig<T> + 'static>(&self, field_name: &str, config_instance: compat::Shared<T>) -> Arc<dyn ReadonlyConfigEntry<compat::Shared<T>>> {
        let entry: Arc<dyn ReadonlyConfigEntry<compat::Shared<T>>> = ChildConfigEntryImpl::new(field_name.to_string(), config_instance);
        self.entries.borrow_mut().push(entry.clone());
        entry
    }
}

impl<S: 'static> Default for CodecConfigBase<S> {
    fn default() -> Self {
        Self::new()
    }
}

/// java: the virtual methods of CodecConfig (implemented by every subclass)
pub trait CodecConfig<S: 'static>: EntryAddable {
    fn codec_config_base(&self) -> &CodecConfigBase<S>;

    /// java: protected void onFinishedDecode(boolean successful)
    fn on_finished_decode(&self, successful: bool) {}

    /// java: public final <R> DataResult<R> encode(DynamicOps<R> ops, R prefix)
    fn encode_2(&self, ops: &Ops, prefix: &Obj) -> DataResult<Obj> {
        let mut builder = ops.map_builder(ops);
        let entries = self.codec_config_base().entries.borrow().clone();
        for entry in entries.iter() {
            entry.encode(ops, builder.as_mut());
        }
        builder.build(prefix)
    }

    /// java: public final <R> DataResult<R> encodeStart(DynamicOps<R> ops)
    fn encode_start(&self, ops: &Ops) -> DataResult<Obj> {
        self.encode_2(ops, &ops.empty())
    }

    /// java: public final <R> boolean decode(R encoded, DynamicOps<R> ops) - true if decoding of all entries was
    /// successful
    fn decode_r_dynamic_ops(&self, encoded: &Obj, ops: &Ops) -> bool {
        let mut success = true;
        let entries = self.codec_config_base().entries.borrow().clone();
        for entry in entries.iter() {
            success &= entry.decode(encoded, ops);
        }
        self.on_finished_decode(success);
        success
    }
}

/// java: `CodecConfig implements Codec<S>` - the config object as its `Codec<S>`: `Encoder.encode(S input, ops,
/// prefix)` and `Decoder.decode(ops, input)` of `this`. The port's `Codec` is `Send + Sync`; the object is the mod's,
/// reached only on the render thread (`compat::RenderThread` checks it).
pub struct CodecConfigCodec<S: 'static> {
    this: compat::RenderThread<compat::Shared<S>>,
}

/// `(Codec<S>) this`
pub fn codec<S: CodecConfig<S> + 'static>(this: &compat::Shared<S>) -> CodecRef<compat::Shared<S>> {
    Arc::new(CodecConfigCodec { this: compat::RenderThread::new(this.clone()) })
}

impl<S: CodecConfig<S> + 'static> Encoder<compat::Shared<S>> for CodecConfigCodec<S> {
    /// java: @Override public <R> DataResult<R> encode(S input, DynamicOps<R> ops, R prefix)
    fn encode(&self, input: &compat::Shared<S>, ops: &Ops, prefix: &Obj) -> DataResult<Obj> {
        let this = self.this.get();
        // java: if (input != null && input != this) - a Rust reference is never null
        if !Arc::ptr_eq(input, this) {
            panic!("IllegalArgumentException: {}", "`input` is ignored. It must be null or equal to `this`.");
        }
        compat::read(this, |config| config.encode_2(ops, prefix))
    }
}

impl<S: CodecConfig<S> + 'static> Decoder<compat::Shared<S>> for CodecConfigCodec<S> {
    /// java: @Override public <R> DataResult<Pair<S, R>> decode(DynamicOps<R> ops, R input)
    fn decode(&self, ops: &Ops, input: &Obj) -> DataResult<(compat::Shared<S>, Obj)> {
        let this = self.this.get();
        compat::read(this, |config| config.decode_r_dynamic_ops(input, ops));
        DataResult::success((this.clone(), input.clone()))
    }
}
