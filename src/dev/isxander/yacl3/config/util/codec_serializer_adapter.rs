//! Ported from dev.isxander.yacl3.config.util.CodecSerializerAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::decoder::Decoder as _;
use mcrust::com::mojang::serialization::encoder::Encoder as _;

/// java: public class CodecSerializerAdapter<T> implements JsonSerializer<T>, JsonDeserializer<T>
pub struct CodecSerializerAdapter<T> {
    /// java: private final Codec<T> codec;
    pub codec: CodecRef<T>,
}

impl<T: Clone + 'static> CodecSerializerAdapter<T> {
    /// java: public CodecSerializerAdapter(Codec<T> codec)
    pub fn new(mut codec: CodecRef<T>) -> Self {
        let __init_codec = codec.clone();
        let mut this = Self {
            codec: __init_codec,
        };
        this
    }

    /// java: @Override public JsonElement serialize(T src, Type typeOfSrc, JsonSerializationContext context)
    pub fn serialize(&self, src: &T, type_of_src: &crate::unresolved::Type, context: &crate::unresolved::JsonSerializationContext) -> serde_json::Value {
        return (self.codec.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), src)).get_or_throw().unwrap();
    }

    /// java: @Override public T deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(&self, json: &serde_json::Value, type_of_t: &crate::unresolved::Type, context: &crate::unresolved::JsonDeserializationContext) -> T {
        return (self.codec.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(json))).get_or_throw().unwrap();
    }

}
