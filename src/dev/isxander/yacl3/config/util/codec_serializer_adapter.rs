//! Ported from dev.isxander.yacl3.config.util.CodecSerializerAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;

/// java: public class CodecSerializerAdapter<T> implements JsonSerializer<T>, JsonDeserializer<T>
///
/// Gson's type adapter over a `Codec`. Registered on a `GsonBuilder` for `Component` (`GsonConfigSerializer.Builder`,
/// `GsonConfigInstance`), it is built into each `Component` field's generated Gson form; these are its bodies.
pub struct CodecSerializerAdapter<T: 'static> {
    /// java: private final Codec<T> codec;
    pub codec: CodecRef<T>,
}

impl<T: 'static> CodecSerializerAdapter<T> {
    /// java: public CodecSerializerAdapter(Codec<T> codec)
    pub fn new(codec: CodecRef<T>) -> Self {
        Self { codec }
    }

    /// java: @Override public JsonElement serialize(T src, Type typeOfSrc, JsonSerializationContext context)
    pub fn serialize(&self, src: &T) -> serde_json::Value {
        // java: codec.encodeStart(JsonOps.INSTANCE, src).getOrThrow()
        match self.codec.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), src).get_or_throw() {
            Ok(value) => compat::json_from_port(&value),
            Err(message) => panic!("IllegalStateException: {}", message),
        }
    }

    /// java: @Override public T deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(&self, json: &serde_json::Value) -> T {
        // java: codec.parse(JsonOps.INSTANCE, json).getOrThrow()
        match self.codec.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(json)).get_or_throw() {
            Ok(value) => value,
            Err(message) => panic!("IllegalStateException: {}", message),
        }
    }
}
