//! Ported from dev.isxander.yacl3.config.v3.CodecConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v3::entry_addable::EntryAddable;
use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry;
use mcrust::com::mojang::datafixers::util::pair::Pair;
use mcrust::com::mojang::serialization::data_result::DataResult;
use mcrust::com::mojang::serialization::dynamic_ops::DynamicOps;
use mcrust::com::mojang::serialization::record_builder::RecordBuilder;

/// java: @ApiStatus.Experimental public abstract class CodecConfig<S extends CodecConfig<S>> implements EntryAddable, Codec<S>
#[derive(Clone)]
pub struct CodecConfigBase<S> {
    /// java: private final List<ReadonlyConfigEntry<?>> entries = new ArrayList<>();
    pub entries: Vec<compat::Shared<dyn ReadonlyConfigEntry<compat::Object>>>,
}

impl<S: Clone + 'static> CodecConfigBase<S> {
    /// java: public CodecConfig()
    pub fn new() -> Self {
        let mut this = Self {
            entries: Vec::new(),
        };
        let ignored = &this;
        this
    }

}

/// java: the virtual methods of CodecConfig (implemented by CodecConfigBase and every subclass)
pub trait CodecConfig<S: Clone + 'static>: EntryAddable {
    fn codec_config_base(&self) -> &CodecConfigBase<S>;
    fn codec_config_base_mut(&mut self) -> &mut CodecConfigBase<S>;

    /// java: protected void onFinishedDecode(boolean successful)
    fn on_finished_decode(&self, mut successful: bool) {
    }

    /// java: @Override public <R> DataResult<R> encode(S input, DynamicOps<R> ops, R prefix)
    fn encode<R: 'static>(&self, input: &S, ops: &Arc<dyn DynamicOps<R>>, prefix: &R) -> DataResult<R> where Self: Sized {
        if true /* java: input != null: never null here */ && input != self {
            panic!("IllegalArgumentException: {}", "`input` is ignored. It must be null or equal to `this`.");
        }
        return self.encode_2(ops, prefix);
    }

    /// java: public final <R> DataResult<R> encode(DynamicOps<R> ops, R prefix)
    fn encode_2<R: 'static>(&self, ops: &Arc<dyn DynamicOps<R>>, prefix: &R) -> DataResult<R> where Self: Sized {
        // TODO(fabric2seer): `map_builder` takes 1 argument(s) in the port (`ops: &Ops`), Java passes 0
        // rust: let mut builder: Box<dyn RecordBuilder<R>> = ops.map_builder();
        let mut builder: Box<dyn RecordBuilder<R>> = todo!("fabric2seer: the initialiser of `builder` (TODO above)");
        for entry in self.codec_config_base().entries.iter() {
            builder = entry.lock().borrow().encode(ops, &builder);
        }
        return builder.build(prefix);
    }

    /// java: @Override public <R> DataResult<Pair<S, R>> decode(DynamicOps<R> ops, R input)
    fn decode<R: Clone + 'static>(&self, ops: &Arc<dyn DynamicOps<R>>, input: &R) -> DataResult<(S, R)> where Self: Sized {
        self.decode_r_dynamic_ops(input, ops);
        return DataResult::success(Pair::of(self.clone(), input.clone()));
    }

    /// java: public final <R> boolean decode(R encoded, DynamicOps<R> ops)
    fn decode_r_dynamic_ops<R: 'static>(&self, encoded: &R, ops: &Arc<dyn DynamicOps<R>>) -> bool where Self: Sized {
        let mut success: bool = true;
        for entry in self.codec_config_base().entries.iter() {
            success = ((success & entry.lock().borrow_mut().decode(encoded, ops))) as bool;
        }
        self.on_finished_decode(success);
        return success;
    }

    /// java: public final <R> DataResult<R> encodeStart(DynamicOps<R> ops)
    fn encode_start<R: 'static>(&self, ops: &Arc<dyn DynamicOps<R>>) -> DataResult<R> where Self: Sized {
        return self.encode_2(ops, &ops.empty());
    }

}

// CodecConfig.register implements EntryAddable.register: it is emitted in each concrete subclass's `impl EntryAddable` (Rust has no inherited impls)
// CodecConfig.register implements EntryAddable.register: it is emitted in each concrete subclass's `impl EntryAddable` (Rust has no inherited impls)
