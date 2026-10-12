//! Ported from dev.isxander.yacl3.config.v2.impl.serializer.GsonConfigSerializer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::ConfigSerializer;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::ConfigSerializerBase;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::LoadResult;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::serial_field::SerialField;
use crate::dev::isxander::yacl3::config::v2::api::serializer::gson_config_serializer_builder::GsonConfigSerializerBuilder;
use crate::dev::isxander::yacl3::gui::utils::item_registry_helper::ItemRegistryHelper;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::world::item::item::Item;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess as _;
use mcrust::net::minecraft::core::registry::Registry as _;

/// java: public class GsonConfigSerializer<T> extends ConfigSerializer<T>
pub struct GsonConfigSerializer<T> {
    /// java: extends ConfigSerializer
    pub base: ConfigSerializerBase<T>,
    /// java: private final Gson gson;
    pub gson: compat::Gson,
    /// java: private final Path path;
    pub path: std::path::PathBuf,
    /// java: private final boolean json5;
    pub json5: bool,
}

impl<T: 'static> std::ops::Deref for GsonConfigSerializer<T> {
    type Target = ConfigSerializerBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for GsonConfigSerializer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> GsonConfigSerializer<T> {
    /// java: private GsonConfigSerializer(ConfigClassHandler<T> config, Path path, Gson gson, boolean json5)
    pub fn new(mut config: compat::Shared<dyn ConfigClassHandler<T>>, mut path: std::path::PathBuf, mut gson: compat::Gson, mut json5: bool) -> Self {
        let __init_gson = gson.clone();
        let __init_path = path.clone();
        let __init_json5 = json5;
        let mut this = Self {
            base: ConfigSerializerBase::<T>::new(config.clone()),
            gson: __init_gson,
            path: __init_path,
            json5: __init_json5,
        };
        this
    }

}

impl<T: Clone + 'static> ConfigSerializer<T> for GsonConfigSerializer<T> {
    fn config_serializer_base(&self) -> &ConfigSerializerBase<T> {
        &self.base
    }
    fn config_serializer_base_mut(&mut self) -> &mut ConfigSerializerBase<T> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void save()
    fn save(&self) {
        logger().info(&format!("Serializing {} to '{}'", compat::jstring(&self.config.lock().borrow().config_class()), self.path.display()));
        {
            // TODO(fabric2seer): `new StringWriter(..)`
            //   java: StringWriter stringWriter = new StringWriter();
            let mut string_writer: crate::unresolved::StringWriter = todo!("fabric2seer: the initialiser of `stringWriter` (TODO above)");
            // java: try {..} catch (IOException e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
            {
                // TODO(fabric2seer): static call JsonWriter.json5(..)
                //   java: JsonWriter jsonWriter = json5 ? JsonWriter.json5(stringWriter) : JsonWriter.json(stringWriter);
                let mut json_writer: crate::unresolved::JsonWriter = todo!("fabric2seer: the initialiser of `jsonWriter` (TODO above)");
                // TODO(fabric2seer): `new GsonWriter(..)`
                //   java: GsonWriter gsonWriter = new GsonWriter(jsonWriter);
                let mut gson_writer: crate::unresolved::GsonWriter = todo!("fabric2seer: the initialiser of `gsonWriter` (TODO above)");
                // TODO(fabric2seer): `.beginObject(..)` on a JsonWriter
                //   java: jsonWriter.beginObject();
                for field in self.config.lock().borrow().fields() {
                    let Some(mut serial) = field.lock().borrow().serial() else {
                        continue;
                    };
                    if !self.json5 && ({ let __f = compat::dispatch(&serial, |__t| __t.comment()); __f(&serial) }).is_some() && YACLPlatform::is_development_env() {
                        logger().warn(&format!("Found comment in config field '{}', but json5 is not enabled. Enable it with `.setJson5(true)` on the `GsonConfigSerializerBuilder`. Comments will not be serialized. This warning is only visible in development environments.", ({ let __f = compat::dispatch(&serial, |__t| __t.serial_name()); __f(&serial) })));
                    }
                    // TODO(fabric2seer): `.comment(..)` on a JsonWriter
                    //   java: jsonWriter.comment(serial.comment().orElse(null));
                    // TODO(fabric2seer): `.name(..)` on a JsonWriter
                    //   java: jsonWriter.name(serial.serialName());
                    let mut element: serde_json::Value;
                    // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // TODO(fabric2seer): `Gson.toJsonTree` (Gson over the generated per-class forms)
                        //   java: element = gson.toJsonTree(field.access().get(), field.access().type());
                    }));
                    if let Err(e) = __result {
                        logger().error(&format!("Failed to serialize config field '{}'. Serializing as null.", ({ let __f = compat::dispatch(&serial, |__t| __t.serial_name()); __f(&serial) })));
                        // TODO(fabric2seer): `.nullValue(..)` on a JsonWriter
                        //   java: jsonWriter.nullValue();
                        continue;
                    }
                    // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        self.gson.to_json(&element, &gson_writer);
                    }));
                    if let Err(e) = __result {
                        logger().error(&format!("Failed to serialize config field '{}'. Due to the error state this JSON writer cannot continue safely and the save will be abandoned.", ({ let __f = compat::dispatch(&serial, |__t| __t.serial_name()); __f(&serial) })));
                        return;
                    }
                }
                // TODO(fabric2seer): `.endObject(..)` on a JsonWriter
                //   java: jsonWriter.endObject();
                // TODO(fabric2seer): `.flush(..)` on a JsonWriter
                //   java: jsonWriter.flush();
                std::fs::create_dir_all(&self.path.parent().map(|x| x.to_path_buf()).unwrap()).expect("IOException");
                std::fs::write(&self.path, &compat::jstring(&string_writer)).expect("IOException");
            }
            // TODO(fabric2seer): the `catch (IOException e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
            //   java: {
            //   java:             YACLConstants.LOGGER.error("Failed to serialize config class '{}'.", config.configClass().getSimpleName(), e);
            //   java:         }
        }
    }

    /// java: @Override public LoadResult loadSafely(Map<ConfigField<?>, FieldAccess<?>> bufferAccessMap)
    fn load_safely(&self, buffer_access_map: &std::collections::HashMap<compat::Shared<dyn ConfigField<compat::Object>>, compat::Shared<dyn FieldAccess<compat::Object>>>) -> LoadResult {
        if !self.path.exists() {
            logger().info(&format!("Config file '{}' does not exist. Creating it with default values.", self.path.display()));
            self.save();
            return LoadResult::NoChange;
        }
        logger().info(&format!("Deserializing {} from '{}'", self.config.lock().borrow().config_class().get_simple_name(), self.path.display()));
        // TODO(fabric2seer): not a lambda
        //   java: Map<String, ConfigField<?>> fieldMap = Arrays.stream(config.fields())
        //   java:                 .filter(field -> field.serial().isPresent())
        //   java:                 .collect(Collectors.toMap(f -> f.serial().orElseThrow().serialName(), Function.identity()));
        let mut field_map: std::collections::HashMap<String, compat::Shared<dyn ConfigField<compat::Object>>> = todo!("fabric2seer: the initialiser of `fieldMap` (TODO above)");
        let mut missing_fields: std::collections::HashSet<String> = field_map.keys().cloned().collect::<std::collections::HashSet<_>>();
        let mut dirty: bool = false;
        {
            // TODO(fabric2seer): static call JsonReader.json5(..)
            //   java: JsonReader jsonReader = json5 ? JsonReader.json5(path) : JsonReader.json(path);
            let mut json_reader: crate::unresolved::JsonReader = todo!("fabric2seer: the initialiser of `jsonReader` (TODO above)");
            // java: try {..} catch (IOException e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
            {
                // TODO(fabric2seer): `new GsonReader(..)`
                //   java: GsonReader gsonReader = new GsonReader(jsonReader);
                let mut gson_reader: crate::unresolved::GsonReader = todo!("fabric2seer: the initialiser of `gsonReader` (TODO above)");
                // TODO(fabric2seer): `.beginObject(..)` on a JsonReader
                //   java: jsonReader.beginObject();
                // TODO(fabric2seer): the condition: `.hasNext(..)` on a JsonReader
                //   java: jsonReader.hasNext(..)
                while todo!("fabric2seer: the condition (TODO above)") {
                    // TODO(fabric2seer): `.nextName(..)` on a JsonReader
                    //   java: String name = jsonReader.nextName();
                    let mut name: String = todo!("fabric2seer: the initialiser of `name` (TODO above)");
                    let mut field: Option<compat::Shared<dyn ConfigField<compat::Object>>> = field_map.get(&name).cloned();
                    missing_fields.remove(&name);
                    let Some(mut field) = field.clone() else {
                        logger().warn(&format!("Found unknown config field '{}'.", name));
                        // TODO(fabric2seer): `.skipValue(..)` on a JsonReader
                        //   java: jsonReader.skipValue();
                        continue;
                    };
                    let mut buffer_access: compat::Shared<dyn FieldAccess<compat::Object>> = buffer_access_map.get(&field).cloned().unwrap();
                    let Some(mut serial) = field.lock().borrow().serial() else {
                        continue;
                    };
                    let mut element: serde_json::Value;
                    // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        element = self.gson.from_json_tree(&gson_reader, &(std::any::TypeId::of::<serde_json::Value>())).unwrap();
                    }));
                    if let Err(e) = __result {
                        logger().error(&format!("Failed to deserialize config field '{}'. Due to the error state this JSON reader cannot be re-used and loading will be aborted.", name));
                        return LoadResult::Failure;
                    }
                    if compat::json::is_null(&element) && !({ let __f = compat::dispatch(&serial, |__t| __t.nullable()); __f(&serial) }) {
                        logger().warn(&format!("Found null value in non-nullable config field '{}'. Leaving field as default and marking as dirty.", name));
                        dirty = true;
                        continue;
                    }
                    // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                    let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        { let (__a0,) = (self.gson.from_json_tree(&element, &(buffer_access.lock().borrow().r#type())).unwrap(),); buffer_access.lock().borrow_mut().set(__a0) };
                    }));
                    if let Err(e) = __result {
                        logger().error(&format!("Failed to deserialize config field '{}'. Leaving as default.", name));
                    }
                }
                // TODO(fabric2seer): `.endObject(..)` on a JsonReader
                //   java: jsonReader.endObject();
            }
            // TODO(fabric2seer): the `catch (IOException e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
            //   java: {
            //   java:             YACLConstants.LOGGER.error("Failed to deserialize config class.", e);
            //   java:             return LoadResult.FAILURE;
            //   java:         }
        }
        if !missing_fields.is_empty() {
            for missing_field in missing_fields.iter() {
                if { let __r = &field_map.get(missing_field).cloned().unwrap().lock().borrow().serial().unwrap(); let __f = compat::dispatch(__r, |__t| __t.required()); __f(__r) } {
                    dirty = true;
                    logger().warn(&format!("Missing required config field '{}''. Re-saving as default.", missing_field));
                }
            }
        }
        return (if dirty { LoadResult::Dirty } else { LoadResult::Success });
    }

    /// java: @Override @Deprecated @SuppressWarnings("deprecation") public void load()
    fn load(&mut self) {
        logger().warn("Calling ConfigSerializer#load() directly is deprecated. Please use ConfigClassHandler#load() instead.");
        self.config.lock().borrow_mut().load();
    }

}

/// java: public static class StyleTypeAdapter implements JsonSerializer<Style>, JsonDeserializer<Style>
#[derive(Clone)]
pub struct StyleTypeAdapter {}

impl StyleTypeAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public Style deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    pub fn deserialize(&self, json: &serde_json::Value, type_of_t: &crate::unresolved::Type, context: &crate::unresolved::JsonDeserializationContext) -> Style {
        // TODO(fabric2seer): field `CODEC` of a value of unknown type
        //   java: return Style.Serializer.CODEC.parse(JsonOps.INSTANCE, json).result().orElse(Style.EMPTY);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public JsonElement serialize(Style src, Type typeOfSrc, JsonSerializationContext context)
    pub fn serialize(&self, src: &Style, type_of_src: &crate::unresolved::Type, context: &crate::unresolved::JsonSerializationContext) -> serde_json::Value {
        // TODO(fabric2seer): field `CODEC` of a value of unknown type
        //   java: return Style.Serializer.CODEC.encodeStart(JsonOps.INSTANCE, src).result().orElse(JsonNull.INSTANCE);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: public static class ColorTypeAdapter implements JsonSerializer<Color>, JsonDeserializer<Color>
#[derive(Clone)]
pub struct ColorTypeAdapter {}

impl ColorTypeAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public Color deserialize(JsonElement jsonElement, Type type, JsonDeserializationContext jsonDeserializationContext) throws JsonParseException
    pub fn deserialize(&self, json_element: &serde_json::Value, r#type: &crate::unresolved::Type, json_deserialization_context: &crate::unresolved::JsonDeserializationContext) -> Color {
        // TODO(fabric2seer): `new` takes 3 argument(s) in the port (`r: i32, g: i32, b: i32`), Java passes 2
        // rust: return Color::new(compat::json::get_as_int(&json_element), true);
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public JsonElement serialize(Color color, Type type, JsonSerializationContext jsonSerializationContext)
    pub fn serialize(&self, color: &Color, r#type: &crate::unresolved::Type, json_serialization_context: &crate::unresolved::JsonSerializationContext) -> serde_json::Value {
        // TODO(fabric2seer): `.getRGB(..)` on a Color
        //   java: return new JsonPrimitive(color.getRGB());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: public static class ItemTypeAdapter implements JsonSerializer<Item>, JsonDeserializer<Item>
#[derive(Clone)]
pub struct ItemTypeAdapter {}

impl ItemTypeAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public Item deserialize(JsonElement jsonElement, Type type, JsonDeserializationContext jsonDeserializationContext) throws JsonParseException
    pub fn deserialize(&self, json_element: &serde_json::Value, r#type: &crate::unresolved::Type, json_deserialization_context: &crate::unresolved::JsonDeserializationContext) -> Arc<Item> {
        return ItemRegistryHelper::get_item_from_name_1(&compat::json::get_as_string(&json_element));
    }

    /// java: @Override public JsonElement serialize(Item item, Type type, JsonSerializationContext jsonSerializationContext)
    pub fn serialize(&self, item: &Arc<Item>, r#type: &crate::unresolved::Type, json_serialization_context: &crate::unresolved::JsonSerializationContext) -> serde_json::Value {
        // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // rust: return serde_json::json!(mcrust::net::minecraft::core::registries::built_in_registries::item().get_key(item).unwrap().fmt());
        return todo!("fabric2seer: see the TODO above");
    }

}

/// java: @ApiStatus.Internal public static class Builder<T> implements GsonConfigSerializerBuilder<T>
#[derive(Clone)]
pub struct Builder<T> {
    /// java: private final ConfigClassHandler<T> config;
    pub config: compat::Shared<dyn ConfigClassHandler<T>>,
    /// java: private Path path;
    pub path: Option<std::path::PathBuf>,
    /// java: private boolean json5;
    pub json5: bool,
    /// java: private UnaryOperator<GsonBuilder> gsonBuilder = builder -> builder .setFieldNamingPolicy(FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES) .serializeNulls() .regi
    pub gson_builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>,
}

impl<T: Clone + 'static> Builder<T> {
    /// java: public Builder(ConfigClassHandler<T> config)
    pub fn new(mut config: compat::Shared<dyn ConfigClassHandler<T>>) -> Self {
        let __init_config = config.clone();
        let mut this = Self {
            config: __init_config,
            path: None,
            json5: false,
            gson_builder: Arc::new(move |builder| builder.clone().set_field_naming_policy(compat::FieldNamingPolicy::Identity).serialize_nulls().register_type_hierarchy_adapter().register_type_hierarchy_adapter().register_type_hierarchy_adapter().register_type_hierarchy_adapter().set_pretty_printing()),
        };
        this
    }

}

impl<T: Clone + 'static> GsonConfigSerializerBuilder<T> for Builder<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder<T> setPath(Path path)
    fn set_path(&mut self, mut path: std::path::PathBuf) -> Builder<T> {
        self.path = Some(path.clone());
        return self.clone();
    }

    /// java: @Override public Builder<T> overrideGsonBuilder(GsonBuilder gsonBuilder)
    fn override_gson_builder(&mut self, gson_builder: &compat::GsonBuilder) -> Builder<T> {
        self.gson_builder = Arc::new(move |builder| gson_builder.clone());
        return self.clone();
    }

    /// java: @Override public Builder<T> overrideGsonBuilder(Gson gson)
    fn override_gson_builder_gson(&mut self, gson: &compat::Gson) -> Builder<T> {
        // TODO(fabric2seer): `Gson.newBuilder` (Gson over the generated per-class forms)
        //   java: return this.overrideGsonBuilder(gson.newBuilder());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> appendGsonBuilder(UnaryOperator<GsonBuilder> gsonBuilder)
    fn append_gson_builder(&mut self, gson_builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>) -> Builder<T> {
        let mut prev: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder> = self.gson_builder.clone();
        self.gson_builder = Arc::new(move |builder| (gson_builder)((prev)(builder.clone())));
        return self.clone();
    }

    /// java: @Override public Builder<T> setJson5(boolean json5)
    fn set_json5(&mut self, mut json5: bool) -> Builder<T> {
        self.json5 = json5;
        return self.clone();
    }

    /// java: @Override public GsonConfigSerializer<T> build()
    fn build(&self) -> GsonConfigSerializer<T> {
        return GsonConfigSerializer::new(self.config.clone(), self.path.clone().unwrap(), (self.gson_builder)(compat::GsonBuilder::new()).create(), self.json5);
    }

}
