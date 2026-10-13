//! Ported from dev.isxander.yacl3.config.v2.impl.serializer.GsonConfigSerializer by fabric2seer.
//!
//! The file is written and read through quilt-parsers' `JsonWriter` / `JsonReader` (`org.quiltmc.parsers.json`,
//! 0.2.1, the library YACL bundles), wrapped for Gson by `GsonWriter` / `GsonReader`. Their behaviour is ported here
//! as the private `JsonWriter` / `JsonReader` (read off the 0.2.1 bytecode): tab indent, `": "` separator,
//! `//` comments (JSON5/JSONC only), unquoted identifier names in JSON5, not HTML-safe, nulls serialized.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::BufferAccessMap;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::ConfigSerializer;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::ConfigSerializerBase;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::LoadResult;
use crate::dev::isxander::yacl3::config::v2::api::serializer::gson_config_serializer_builder::GsonConfigSerializerBuilder;
use crate::dev::isxander::yacl3::gui::utils::item_registry_helper::ItemRegistryHelper;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::core::defaulted_registry::DefaultedRegistry as _;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::world::item::item::Item;

/// java: public class GsonConfigSerializer<T> extends ConfigSerializer<T>
pub struct GsonConfigSerializer<T: 'static> {
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

impl<T: 'static> GsonConfigSerializer<T> {
    /// java: private GsonConfigSerializer(ConfigClassHandler<T> config, Path path, Gson gson, boolean json5)
    fn new(config: Arc<dyn ConfigClassHandler<T>>, path: std::path::PathBuf, gson: compat::Gson, json5: bool) -> Self {
        Self { base: ConfigSerializerBase::new(config), gson, path, json5 }
    }

    /// The body of `save()`'s `try (StringWriter ..) { .. }`: `Err` is the `IOException` its catch logs; `Ok` also
    /// covers the abandoned save (the inner `return`).
    fn write_config(&self) -> std::io::Result<()> {
        let mut json_writer = JsonWriter::new(if self.json5 { JsonFormat::Json5 } else { JsonFormat::Json });
        json_writer.begin_object().unwrap_or_else(|e| panic!("{}", e));
        for field in self.config.fields() {
            let Some(serial) = field.serial() else {
                continue;
            };

            if !self.json5 && serial.comment().is_some() && YACLPlatform::is_development_env() {
                LOGGER.warn(&format!(
                    "Found comment in config field '{}', but json5 is not enabled. Enable it with `.setJson5(true)` on the `GsonConfigSerializerBuilder`. Comments will not be serialized. This warning is only visible in development environments.",
                    serial.serial_name()
                ));
            }
            json_writer.comment(serial.comment());

            json_writer.name(&serial.serial_name()).unwrap_or_else(|e| panic!("{}", e));

            // java: element = gson.toJsonTree(field.access().get(), field.access().type());
            let element = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let access = field.access();
                access.r#type().to_json_tree(&access.get())
            }));
            let element = match element {
                Ok(element) => element,
                Err(e) => {
                    LOGGER.error(&format!(
                        "Failed to serialize config field '{}'. Serializing as null. {}",
                        serial.serial_name(),
                        compat::panic_message(&e)
                    ));
                    json_writer.null_value().unwrap_or_else(|e| panic!("{}", e));
                    continue;
                }
            };

            // java: gson.toJson(element, gsonWriter);
            if let Err(e) = json_writer.element(&element) {
                LOGGER.error(&format!(
                    "Failed to serialize config field '{}'. Due to the error state this JSON writer cannot continue safely and the save will be abandoned. {}",
                    serial.serial_name(),
                    e
                ));
                return Ok(());
            }
        }
        json_writer.end_object().unwrap_or_else(|e| panic!("{}", e));
        // java: jsonWriter.flush() - the text is already in the StringWriter

        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        // java: Files.writeString(path, stringWriter.toString(), StandardOpenOption.TRUNCATE_EXISTING, StandardOpenOption.CREATE);
        std::fs::write(&self.path, json_writer.out)?;
        Ok(())
    }

    /// The body of `loadSafely`'s `try (JsonReader ..) { .. }`: `Err` is the `IOException` its catch logs (a
    /// malformed document outside a field's value); `Ok(Some(..))` is a `return` from inside it.
    fn read_config(
        &self,
        json_reader: &mut JsonReader,
        field_map: &mut Vec<(String, Arc<dyn ConfigField<compat::Object>>)>,
        buffer_access_map: &BufferAccessMap,
        dirty: &mut bool,
    ) -> Result<Option<LoadResult>, String> {
        json_reader.begin_object()?;

        while json_reader.has_next()? {
            let name = json_reader.next_name()?;
            // java: fieldMap.get(name); missingFields.remove(name); - `missingFields` is fieldMap's key set, so the
            // remove takes the entry out of `fieldMap` itself
            let field = match field_map.iter().position(|(k, _)| *k == name) {
                Some(i) => Some(field_map.remove(i).1),
                None => None,
            };

            let Some(field) = field else {
                LOGGER.warn(&format!("Found unknown config field '{}'.", name));
                json_reader.skip_value()?;
                continue;
            };

            let buffer_access = buffer_access_map.get(&field);
            let Some(serial) = field.serial() else {
                continue;
            };

            // java: element = gson.fromJson(gsonReader, JsonElement.class);
            let element = match json_reader.read_value() {
                Ok(element) => element,
                Err(e) => {
                    LOGGER.error(&format!(
                        "Failed to deserialize config field '{}'. Due to the error state this JSON reader cannot be re-used and loading will be aborted. {}",
                        name, e
                    ));
                    return Ok(Some(LoadResult::Failure));
                }
            };

            if element.is_null() && !serial.nullable() {
                LOGGER.warn(&format!(
                    "Found null value in non-nullable config field '{}'. Leaving field as default and marking as dirty.",
                    name
                ));
                *dirty = true;
                continue;
            }

            // java: try { bufferAccess.set(gson.fromJson(element, bufferAccess.type())); } catch (Exception e)
            let set = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let buffer_access = buffer_access.clone().expect("NullPointerException: bufferAccess");
                match buffer_access.r#type().from_json_tree(&element) {
                    Some(value) => {
                        buffer_access.set(value);
                        true
                    }
                    None => false,
                }
            }));
            match set {
                Ok(true) => {}
                Ok(false) => LOGGER.error(&format!("Failed to deserialize config field '{}'. Leaving as default.", name)),
                Err(e) => LOGGER.error(&format!(
                    "Failed to deserialize config field '{}'. Leaving as default. {}",
                    name,
                    compat::panic_message(&e)
                )),
            }
        }

        json_reader.end_object()?;
        Ok(None)
    }
}

impl<T: 'static> ConfigSerializer<T> for GsonConfigSerializer<T> {
    fn config_serializer_base(&self) -> &ConfigSerializerBase<T> {
        &self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void save()
    fn save(&self) {
        LOGGER.info(&format!("Serializing {} to '{}'", self.config.config_class(), self.path.display()));

        if let Err(e) = self.write_config() {
            LOGGER.error(&format!("Failed to serialize config class '{}'. {}", self.config.config_class().get_simple_name(), e));
        }
    }

    /// java: @Override public LoadResult loadSafely(Map<ConfigField<?>, FieldAccess<?>> bufferAccessMap)
    fn load_safely(&self, buffer_access_map: &BufferAccessMap) -> LoadResult {
        if !self.path.exists() {
            LOGGER.info(&format!("Config file '{}' does not exist. Creating it with default values.", self.path.display()));
            self.save();
            return LoadResult::NoChange;
        }

        LOGGER.info(&format!("Deserializing {} from '{}'", self.config.config_class().get_simple_name(), self.path.display()));

        // java: Arrays.stream(config.fields()).filter(field -> field.serial().isPresent())
        //           .collect(Collectors.toMap(f -> f.serial().orElseThrow().serialName(), Function.identity()));
        let mut field_map: Vec<(String, Arc<dyn ConfigField<compat::Object>>)> = Vec::new();
        for field in self.config.fields() {
            let Some(serial) = field.serial() else {
                continue;
            };
            let key = serial.serial_name();
            if field_map.iter().any(|(k, _)| *k == key) {
                // Collectors.toMap on a repeated key
                panic!("IllegalStateException: Duplicate key {} (attempted merging values)", key);
            }
            field_map.push((key, field));
        }
        let mut dirty = false;

        // java: try (JsonReader jsonReader = json5 ? JsonReader.json5(path) : JsonReader.json(path))
        let text = match std::fs::read_to_string(&self.path) {
            Ok(text) => text,
            Err(e) => {
                LOGGER.error(&format!("Failed to deserialize config class. {}", e));
                return LoadResult::Failure;
            }
        };
        let mut json_reader = JsonReader::new(text, if self.json5 { JsonFormat::Json5 } else { JsonFormat::Json });
        match self.read_config(&mut json_reader, &mut field_map, buffer_access_map, &mut dirty) {
            Ok(Some(result)) => return result,
            Ok(None) => {}
            Err(e) => {
                LOGGER.error(&format!("Failed to deserialize config class. {}", e));
                return LoadResult::Failure;
            }
        }

        // java: missingFields - the serial names never read (still in fieldMap)
        for (missing_field, field) in field_map.iter() {
            if field.serial().expect("NoSuchElementException: No value present").required() {
                dirty = true;
                LOGGER.warn(&format!("Missing required config field '{}''. Re-saving as default.", missing_field));
            }
        }

        if dirty { LoadResult::Dirty } else { LoadResult::Success }
    }

    /// java: @Override @Deprecated public void load()
    fn load(&self) {
        LOGGER.warn("Calling ConfigSerializer#load() directly is deprecated. Please use ConfigClassHandler#load() instead.");
        self.config.load();
    }
}

/// java: public static class StyleTypeAdapter implements JsonSerializer<Style>, JsonDeserializer<Style>
#[derive(Clone, Default)]
pub struct StyleTypeAdapter {}

impl StyleTypeAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: @Override public Style deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context)
    pub fn deserialize(&self, json: &serde_json::Value) -> Style {
        compat::gson_parse_codec(&mcrust::net::minecraft::network::chat::style::serializer::codec(), json).unwrap_or(Style::EMPTY)
    }

    /// java: @Override public JsonElement serialize(Style src, Type typeOfSrc, JsonSerializationContext context)
    pub fn serialize(&self, src: &Style) -> compat::GsonValue {
        // java: CODEC.encodeStart(..).result().orElse(JsonNull.INSTANCE)
        compat::gson_by_codec(&mcrust::net::minecraft::network::chat::style::serializer::codec(), src)
    }
}

/// java: public static class ColorTypeAdapter implements JsonSerializer<Color>, JsonDeserializer<Color>
#[derive(Clone, Default)]
pub struct ColorTypeAdapter {}

impl ColorTypeAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: @Override public Color deserialize(JsonElement jsonElement, Type type, JsonDeserializationContext jsonDeserializationContext)
    pub fn deserialize(&self, json_element: &serde_json::Value) -> Color {
        Color::from_rgba(compat::json::get_as_int(json_element), true)
    }

    /// java: @Override public JsonElement serialize(Color color, Type type, JsonSerializationContext jsonSerializationContext)
    pub fn serialize(&self, color: &Color) -> compat::GsonValue {
        compat::GsonValue::int(color.get_rgb() as i64)
    }
}

/// java: public static class ItemTypeAdapter implements JsonSerializer<Item>, JsonDeserializer<Item>
#[derive(Clone, Default)]
pub struct ItemTypeAdapter {}

impl ItemTypeAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: @Override public Item deserialize(JsonElement jsonElement, Type type, JsonDeserializationContext jsonDeserializationContext)
    pub fn deserialize(&self, json_element: &serde_json::Value) -> compat::JavaRef<Item> {
        ItemRegistryHelper::get_item_from_name_1(&compat::json::get_as_string(json_element))
    }

    /// java: @Override public JsonElement serialize(Item item, Type type, JsonSerializationContext jsonSerializationContext)
    pub fn serialize(&self, item: &compat::JavaRef<Item>) -> compat::GsonValue {
        compat::GsonValue::str(
            &mcrust::net::minecraft::core::registries::built_in_registries::item().get_key_defaulted(&item.erased_obj()).to_string(),
        )
    }
}

/// java: @ApiStatus.Internal public static class Builder<T> implements GsonConfigSerializerBuilder<T>
pub struct Builder<T: 'static> {
    /// java: private final ConfigClassHandler<T> config;
    pub config: Arc<dyn ConfigClassHandler<T>>,
    /// java: private Path path;
    pub path: std::cell::RefCell<Option<std::path::PathBuf>>,
    /// java: private boolean json5;
    pub json5: std::cell::Cell<bool>,
    /// java: private UnaryOperator<GsonBuilder> gsonBuilder = builder -> builder
    ///   .setFieldNamingPolicy(FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES).serializeNulls()
    ///   .registerTypeHierarchyAdapter(Component.class, new CodecSerializerAdapter<>(ComponentSerialization.CODEC))
    ///   .registerTypeHierarchyAdapter(Style.class, new StyleTypeAdapter())
    ///   .registerTypeHierarchyAdapter(Color.class, new ColorTypeAdapter())
    ///   .registerTypeHierarchyAdapter(Item.class, new ItemTypeAdapter()).setPrettyPrinting();
    pub gson_builder: std::cell::RefCell<Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>>,
    /// Java's `this`, handed back by every setter
    pub self_ref: std::sync::Weak<Self>,
}

impl<T: 'static> Builder<T> {
    /// java: public Builder(ConfigClassHandler<T> config)
    pub fn new(config: Arc<dyn ConfigClassHandler<T>>) -> Arc<Self> {
        Arc::new_cyclic(|self_ref| Self {
            config,
            path: std::cell::RefCell::new(None),
            json5: std::cell::Cell::new(false),
            gson_builder: std::cell::RefCell::new(Arc::new(|builder: compat::GsonBuilder| {
                // the five settings that change no generated form are recorded by `compat::GsonBuilder` as no-ops:
                // field naming is the generated forms' (see GAPS_Y1.md), the four type adapters are built into
                // each generated form (`StyleTypeAdapter` & co above are their bodies)
                builder
                    .set_field_naming_policy(compat::FieldNamingPolicy::Identity)
                    .serialize_nulls()
                    .register_type_hierarchy_adapter()
                    .register_type_hierarchy_adapter()
                    .register_type_hierarchy_adapter()
                    .register_type_hierarchy_adapter()
                    .set_pretty_printing()
            })),
            self_ref: self_ref.clone(),
        })
    }

    fn this(&self) -> Arc<dyn GsonConfigSerializerBuilder<T>> {
        self.self_ref.upgrade().expect("GsonConfigSerializer.Builder used after it was dropped")
    }
}

impl<T: 'static> GsonConfigSerializerBuilder<T> for Builder<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder<T> setPath(Path path)
    fn set_path(&self, path: std::path::PathBuf) -> Arc<dyn GsonConfigSerializerBuilder<T>> {
        *self.path.borrow_mut() = Some(path);
        self.this()
    }

    /// java: @Override public Builder<T> overrideGsonBuilder(GsonBuilder gsonBuilder)
    fn override_gson_builder(&self, gson_builder: &compat::GsonBuilder) -> Arc<dyn GsonConfigSerializerBuilder<T>> {
        let gson_builder = gson_builder.clone();
        *self.gson_builder.borrow_mut() = Arc::new(move |_builder| gson_builder.clone());
        self.this()
    }

    /// java: @Override public Builder<T> overrideGsonBuilder(Gson gson)
    fn override_gson_builder_gson(&self, gson: &compat::Gson) -> Arc<dyn GsonConfigSerializerBuilder<T>> {
        // java: gson.newBuilder() - a builder with the instance's settings
        let mut builder = compat::GsonBuilder::new();
        builder.pretty = gson.pretty;
        self.override_gson_builder(&builder)
    }

    /// java: @Override public Builder<T> appendGsonBuilder(UnaryOperator<GsonBuilder> gsonBuilder)
    fn append_gson_builder(&self, gson_builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>) -> Arc<dyn GsonConfigSerializerBuilder<T>> {
        let prev = self.gson_builder.borrow().clone();
        *self.gson_builder.borrow_mut() = Arc::new(move |builder| gson_builder(prev(builder)));
        self.this()
    }

    /// java: @Override public Builder<T> setJson5(boolean json5)
    fn set_json5(&self, json5: bool) -> Arc<dyn GsonConfigSerializerBuilder<T>> {
        self.json5.set(json5);
        self.this()
    }

    /// java: @Override public GsonConfigSerializer<T> build()
    fn build(&self) -> Arc<dyn ConfigSerializer<T>> {
        let gson = (self.gson_builder.borrow().clone())(compat::GsonBuilder::new()).create();
        let path = self.path.borrow().clone().expect("NullPointerException: path");
        Arc::new(GsonConfigSerializer::new(self.config.clone(), path, gson, self.json5.get()))
    }
}

// ---------------------------------------------------------------------------------------------------------------
// org.quiltmc.parsers.json (0.2.1): the writer and reader the file goes through
// ---------------------------------------------------------------------------------------------------------------

/// `JsonParser.parseString(json)` (`JsonFileCodecConfig.loadFromFile`, `GsonConfigInstance.load`): Gson's LENIENT
/// parse of one element that must be the whole document - comments, unquoted names, single-quoted strings and
/// trailing commas are accepted, as the JSON5 reader below accepts them. `Err` is the `JsonParseException`.
pub(crate) fn parse_lenient(json: &str) -> Result<serde_json::Value, String> {
    let mut reader = JsonReader::new(json.to_string(), JsonFormat::Json5);
    let element = reader.read_value()?;
    reader.skip_whitespace()?;
    if !element.is_null() && reader.peek_char().is_some() {
        return Err("JsonSyntaxException: Did not consume the entire document.".to_string());
    }
    Ok(element)
}

/// java: org.quiltmc.parsers.json.JsonFormat
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum JsonFormat {
    Json,
    Json5,
}

/// `JsonWriter.REPLACEMENT_CHARS['"']`: quilt-parsers 0.2.1's `string(value, quote, escapeQuotes = false)` (how a
/// comment is written) sets the STATIC table's quote entry to null, so every string written after the first comment
/// in the process no longer escapes `"`. Reproduced: it is what the Java writes.
static QUOTES_UNESCAPED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// JsonWriter's scope stack codes
const EMPTY_ARRAY: u8 = 1;
const NONEMPTY_ARRAY: u8 = 2;
const EMPTY_OBJECT: u8 = 3;
const DANGLING_NAME: u8 = 4;
const NONEMPTY_OBJECT: u8 = 5;
const EMPTY_DOCUMENT: u8 = 6;
const NONEMPTY_DOCUMENT: u8 = 7;

/// java: org.quiltmc.parsers.json.JsonWriter over a `StringWriter` (indent `\t`, separator `": "`, not HTML-safe,
/// nulls serialized), and `GsonWriter`'s forwarding of `gson.toJson(element, gsonWriter)` onto it (`element`).
/// An `Err` is the exception the Java method throws.
struct JsonWriter {
    out: String,
    format: JsonFormat,
    stack: Vec<u8>,
    indent: &'static str,
    separator: &'static str,
    deferred_name: Option<String>,
    deferred_comment: Option<String>,
    serialize_nulls: bool,
}

impl JsonWriter {
    fn new(format: JsonFormat) -> JsonWriter {
        JsonWriter {
            out: String::new(),
            format,
            stack: vec![EMPTY_DOCUMENT],
            indent: "\t",
            separator: ": ",
            deferred_name: None,
            deferred_comment: None,
            serialize_nulls: true,
        }
    }

    fn peek(&self) -> Result<u8, String> {
        self.stack.last().copied().ok_or_else(|| "IllegalStateException: JsonWriter is closed.".to_string())
    }

    fn replace_top(&mut self, top: u8) {
        if let Some(last) = self.stack.last_mut() {
            *last = top;
        }
    }

    fn name(&mut self, name: &str) -> Result<(), String> {
        if self.deferred_name.is_some() {
            return Err("IllegalStateException".to_string());
        }
        if self.stack.is_empty() {
            return Err("IllegalStateException: JsonWriter is closed.".to_string());
        }
        self.deferred_name = Some(name.to_string());
        Ok(())
    }

    /// `comment(String)`: a `null` comment, the JSON format and a compact writer write nothing
    fn comment(&mut self, comment: Option<String>) {
        let Some(comment) = comment else {
            return;
        };
        if self.format == JsonFormat::Json {
            return;
        }
        self.deferred_comment = Some(match self.deferred_comment.take() {
            None => comment,
            Some(prev) => format!("{}\n{}", prev, comment),
        });
        if self.stack.len() == 1 && self.stack[0] == NONEMPTY_DOCUMENT {
            self.out.push('\n');
            self.write_deferred_comment();
        }
    }

    fn begin_object(&mut self) -> Result<(), String> {
        self.write_deferred_name()?;
        self.open(EMPTY_OBJECT, '{')
    }

    fn end_object(&mut self) -> Result<(), String> {
        self.close(EMPTY_OBJECT, NONEMPTY_OBJECT, '}')
    }

    fn begin_array(&mut self) -> Result<(), String> {
        self.write_deferred_name()?;
        self.open(EMPTY_ARRAY, '[')
    }

    fn end_array(&mut self) -> Result<(), String> {
        self.close(EMPTY_ARRAY, NONEMPTY_ARRAY, ']')
    }

    fn value_string(&mut self, value: &str) -> Result<(), String> {
        self.write_deferred_name()?;
        self.before_value()?;
        self.string(value, true, true);
        Ok(())
    }

    fn value_bool(&mut self, value: bool) -> Result<(), String> {
        self.write_deferred_name()?;
        self.before_value()?;
        self.out.push_str(if value { "true" } else { "false" });
        Ok(())
    }

    /// `value(Number)`: the number's `toString()`, which a non-JSON5 writer refuses for the non-finite three
    fn value_number(&mut self, text: &str) -> Result<(), String> {
        self.write_deferred_name()?;
        if self.format != JsonFormat::Json5 && (text == "-Infinity" || text == "Infinity" || text == "NaN") {
            return Err(format!("IllegalArgumentException: Numeric values must be finite, but was {}", text));
        }
        self.before_value()?;
        self.out.push_str(text);
        Ok(())
    }

    fn null_value(&mut self) -> Result<(), String> {
        if self.deferred_name.is_some() {
            if self.serialize_nulls {
                self.write_deferred_name()?;
            } else {
                self.deferred_name = None;
                return Ok(());
            }
        }
        self.before_value()?;
        self.out.push_str("null");
        Ok(())
    }

    /// `gson.toJson(element, gsonWriter)`: Gson's `JsonElement` adapter writing through `GsonWriter`
    fn element(&mut self, element: &compat::GsonValue) -> Result<(), String> {
        match element {
            compat::GsonValue::Null => self.null_value(),
            compat::GsonValue::Bool(b) => self.value_bool(*b),
            compat::GsonValue::Num(n) => self.value_number(n),
            compat::GsonValue::Str(s) => self.value_string(s),
            compat::GsonValue::Arr(values) => {
                self.begin_array()?;
                for value in values {
                    self.element(value)?;
                }
                self.end_array()
            }
            compat::GsonValue::Obj(members) => {
                self.begin_object()?;
                for (name, value) in members {
                    self.name(name)?;
                    self.element(value)?;
                }
                self.end_object()
            }
        }
    }

    fn open(&mut self, empty: u8, bracket: char) -> Result<(), String> {
        self.before_value()?;
        self.stack.push(empty);
        self.out.push(bracket);
        Ok(())
    }

    fn close(&mut self, empty: u8, nonempty: u8, bracket: char) -> Result<(), String> {
        let context = self.peek()?;
        if context != nonempty && context != empty {
            return Err("IllegalStateException: Nesting problem.".to_string());
        }
        if let Some(name) = &self.deferred_name {
            return Err(format!("IllegalStateException: Dangling name: {}", name));
        }
        self.stack.pop();
        if context == nonempty {
            self.comment_and_newline();
        }
        self.out.push(bracket);
        Ok(())
    }

    fn write_deferred_name(&mut self) -> Result<(), String> {
        if let Some(name) = self.deferred_name.clone() {
            self.before_name()?;
            let mut quote = true;
            let mut chars = name.chars();
            if self.format == JsonFormat::Json5 {
                if let Some(first) = chars.next() {
                    if is_java_identifier_start(first) {
                        quote = false;
                        if chars.any(|c| !is_java_identifier_part(c)) {
                            quote = true;
                        }
                    }
                }
            }
            self.string(&name, quote, true);
            self.deferred_name = None;
        }
        Ok(())
    }

    fn write_deferred_comment(&mut self) {
        let Some(comment) = self.deferred_comment.take() else {
            return;
        };
        for line in java_split_newline(&comment) {
            for _ in 1..self.stack.len() {
                self.out.push_str(self.indent);
            }
            self.out.push_str("//");
            self.string(&line, false, false);
            self.out.push('\n');
        }
    }

    fn string(&mut self, value: &str, quote: bool, escape_quotes: bool) {
        if quote {
            self.out.push('"');
        }
        if !escape_quotes {
            QUOTES_UNESCAPED.store(true, std::sync::atomic::Ordering::Relaxed);
        }
        let quotes_unescaped = QUOTES_UNESCAPED.load(std::sync::atomic::Ordering::Relaxed);
        for c in value.chars() {
            match c {
                '"' if !quotes_unescaped => self.out.push_str("\\\""),
                '\\' => self.out.push_str("\\\\"),
                '\t' => self.out.push_str("\\t"),
                '\u{8}' => self.out.push_str("\\b"),
                '\n' => self.out.push_str("\\n"),
                '\r' => self.out.push_str("\\r"),
                '\u{c}' => self.out.push_str("\\f"),
                '\u{2028}' => self.out.push_str("\\u2028"),
                '\u{2029}' => self.out.push_str("\\u2029"),
                c if (c as u32) < 0x20 => self.out.push_str(&format!("\\u{:04x}", c as u32)),
                c => self.out.push(c),
            }
        }
        if quote {
            self.out.push('"');
        }
    }

    fn comment_and_newline(&mut self) {
        self.out.push('\n');
        self.write_deferred_comment();
        for _ in 1..self.stack.len() {
            self.out.push_str(self.indent);
        }
    }

    fn before_name(&mut self) -> Result<(), String> {
        let context = self.peek()?;
        if context == NONEMPTY_OBJECT {
            self.out.push(',');
        } else if context != EMPTY_OBJECT {
            return Err("IllegalStateException: Nesting problem.".to_string());
        }
        self.comment_and_newline();
        self.replace_top(DANGLING_NAME);
        Ok(())
    }

    fn before_value(&mut self) -> Result<(), String> {
        match self.peek()? {
            NONEMPTY_DOCUMENT => Err("IllegalStateException: JSON must have only one top-level value.".to_string()),
            EMPTY_DOCUMENT => {
                self.write_deferred_comment();
                self.replace_top(NONEMPTY_DOCUMENT);
                Ok(())
            }
            EMPTY_ARRAY => {
                self.replace_top(NONEMPTY_ARRAY);
                self.comment_and_newline();
                Ok(())
            }
            NONEMPTY_ARRAY => {
                self.out.push(',');
                self.comment_and_newline();
                Ok(())
            }
            DANGLING_NAME => {
                self.out.push_str(self.separator);
                self.replace_top(NONEMPTY_OBJECT);
                Ok(())
            }
            _ => Err("IllegalStateException: Nesting problem.".to_string()),
        }
    }
}

/// `String.split("\n")`: trailing empty strings removed
fn java_split_newline(s: &str) -> Vec<String> {
    let mut parts: Vec<String> = s.split('\n').map(|p| p.to_string()).collect();
    while parts.len() > 1 && parts.last().is_some_and(|p| p.is_empty()) {
        parts.pop();
    }
    if parts.len() == 1 && parts[0].is_empty() && !s.is_empty() {
        parts.clear();
    }
    parts
}

/// `Character.isJavaIdentifierStart`
fn is_java_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c == '$'
}

/// `Character.isJavaIdentifierPart`
fn is_java_identifier_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

/// java: org.quiltmc.parsers.json.JsonReader over the config file, and `GsonReader`'s `gson.fromJson(gsonReader,
/// JsonElement.class)` (`read_value`). JSON is strict; JSON5 takes comments, unquoted identifier names, single-quoted
/// strings, trailing commas, hexadecimal, `Infinity` / `NaN` and leading / trailing decimal points. An `Err` is the
/// (`Malformed`)`IOException` the Java reader throws.
struct JsonReader {
    chars: Vec<char>,
    pos: usize,
    format: JsonFormat,
    /// the top-level object has had a member (a `,` is due before the next)
    members_read: bool,
}

impl JsonReader {
    fn new(text: String, format: JsonFormat) -> JsonReader {
        JsonReader { chars: text.chars().collect(), pos: 0, format, members_read: false }
    }

    fn json5(&self) -> bool {
        self.format == JsonFormat::Json5
    }

    fn error<R>(&self, message: &str) -> Result<R, String> {
        let line = self.chars[..self.pos.min(self.chars.len())].iter().filter(|c| **c == '\n').count() + 1;
        Err(format!("MalformedJsonException: {} at line {} (offset {})", message, line, self.pos))
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    /// whitespace, and in JSON5 comments
    fn skip_whitespace(&mut self) -> Result<(), String> {
        loop {
            match self.peek_char() {
                Some(' ' | '\t' | '\n' | '\r') => self.pos += 1,
                Some('\u{a0}' | '\u{feff}' | '\u{2028}' | '\u{2029}' | '\u{b}' | '\u{c}') if self.json5() => self.pos += 1,
                Some(c) if self.json5() && c.is_whitespace() => self.pos += 1,
                Some('/') if self.json5() => match self.chars.get(self.pos + 1) {
                    Some('/') => {
                        while let Some(c) = self.peek_char() {
                            if c == '\n' || c == '\r' {
                                break;
                            }
                            self.pos += 1;
                        }
                    }
                    Some('*') => {
                        self.pos += 2;
                        loop {
                            match self.peek_char() {
                                None => return self.error("Unterminated comment"),
                                Some('*') if self.chars.get(self.pos + 1) == Some(&'/') => {
                                    self.pos += 2;
                                    break;
                                }
                                Some(_) => self.pos += 1,
                            }
                        }
                    }
                    _ => return self.error("Unexpected character '/'"),
                },
                _ => return Ok(()),
            }
        }
    }

    fn expect(&mut self, c: char) -> Result<(), String> {
        self.skip_whitespace()?;
        if self.peek_char() == Some(c) {
            self.pos += 1;
            Ok(())
        } else {
            self.error(&format!("Expected '{}'", c))
        }
    }

    /// java: beginObject()
    fn begin_object(&mut self) -> Result<(), String> {
        self.expect('{')?;
        self.members_read = false;
        Ok(())
    }

    /// java: hasNext()
    fn has_next(&mut self) -> Result<bool, String> {
        self.skip_whitespace()?;
        match self.peek_char() {
            Some('}') => return Ok(false),
            None => return self.error("End of input"),
            _ => {}
        }
        if self.members_read {
            if self.peek_char() != Some(',') {
                return self.error("Unterminated object");
            }
            self.pos += 1;
            self.members_read = false;
            self.skip_whitespace()?;
            if self.peek_char() == Some('}') {
                return if self.json5() { Ok(false) } else { self.error("Expected name") };
            }
        }
        Ok(true)
    }

    /// java: nextName()
    fn next_name(&mut self) -> Result<String, String> {
        self.skip_whitespace()?;
        let name = self.read_name()?;
        self.expect(':')?;
        self.members_read = true;
        Ok(name)
    }

    /// java: endObject()
    fn end_object(&mut self) -> Result<(), String> {
        self.expect('}')
    }

    /// java: skipValue()
    fn skip_value(&mut self) -> Result<(), String> {
        self.read_value().map(|_| ())
    }

    fn read_name(&mut self) -> Result<String, String> {
        match self.peek_char() {
            Some('"') => self.read_string('"'),
            Some('\'') if self.json5() => self.read_string('\''),
            Some(c) if self.json5() && (is_java_identifier_start(c)) => {
                let start = self.pos;
                while let Some(c) = self.peek_char() {
                    if !is_java_identifier_part(c) {
                        break;
                    }
                    self.pos += 1;
                }
                Ok(self.chars[start..self.pos].iter().collect())
            }
            _ => self.error("Expected name"),
        }
    }

    /// java: gson.fromJson(gsonReader, JsonElement.class) - the next value as a tree
    fn read_value(&mut self) -> Result<serde_json::Value, String> {
        self.skip_whitespace()?;
        match self.peek_char() {
            None => self.error("End of input"),
            Some('{') => {
                self.pos += 1;
                let mut map = serde_json::Map::new();
                let mut first = true;
                loop {
                    self.skip_whitespace()?;
                    if self.peek_char() == Some('}') {
                        if !first && !self.json5() {
                            return self.error("Expected name");
                        }
                        self.pos += 1;
                        break;
                    }
                    let name = self.read_name()?;
                    self.expect(':')?;
                    let value = self.read_value()?;
                    map.insert(name, value);
                    first = false;
                    self.skip_whitespace()?;
                    match self.peek_char() {
                        Some(',') => {
                            self.pos += 1;
                            self.skip_whitespace()?;
                            if self.peek_char() == Some('}') && !self.json5() {
                                return self.error("Expected name");
                            }
                            first = true;
                        }
                        Some('}') => {
                            self.pos += 1;
                            break;
                        }
                        _ => return self.error("Unterminated object"),
                    }
                }
                Ok(serde_json::Value::Object(map))
            }
            Some('[') => {
                self.pos += 1;
                let mut values = Vec::new();
                loop {
                    self.skip_whitespace()?;
                    if self.peek_char() == Some(']') {
                        if !values.is_empty() && !self.json5() {
                            return self.error("Expected value");
                        }
                        self.pos += 1;
                        break;
                    }
                    values.push(self.read_value()?);
                    self.skip_whitespace()?;
                    match self.peek_char() {
                        Some(',') => {
                            self.pos += 1;
                            self.skip_whitespace()?;
                            if self.peek_char() == Some(']') {
                                if !self.json5() {
                                    return self.error("Expected value");
                                }
                                self.pos += 1;
                                break;
                            }
                        }
                        Some(']') => {
                            self.pos += 1;
                            break;
                        }
                        _ => return self.error("Unterminated array"),
                    }
                }
                Ok(serde_json::Value::Array(values))
            }
            Some('"') => self.read_string('"').map(serde_json::Value::String),
            Some('\'') if self.json5() => self.read_string('\'').map(serde_json::Value::String),
            Some(_) => self.read_literal(),
        }
    }

    fn read_string(&mut self, quote: char) -> Result<String, String> {
        self.pos += 1;
        let mut out = String::new();
        loop {
            let Some(c) = self.peek_char() else {
                return self.error("Unterminated string");
            };
            self.pos += 1;
            if c == quote {
                return Ok(out);
            }
            if c == '\\' {
                let Some(e) = self.peek_char() else {
                    return self.error("Unterminated escape sequence");
                };
                self.pos += 1;
                match e {
                    '"' => out.push('"'),
                    '\'' => out.push('\''),
                    '\\' => out.push('\\'),
                    '/' => out.push('/'),
                    'b' => out.push('\u{8}'),
                    'f' => out.push('\u{c}'),
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    'v' if self.json5() => out.push('\u{b}'),
                    '0' if self.json5() => out.push('\0'),
                    '\n' if self.json5() => {}
                    '\r' if self.json5() => {
                        if self.peek_char() == Some('\n') {
                            self.pos += 1;
                        }
                    }
                    'x' if self.json5() => {
                        let hex: String = self.chars.get(self.pos..self.pos + 2).unwrap_or(&[]).iter().collect();
                        match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                            Some(ch) if hex.len() == 2 => out.push(ch),
                            _ => return self.error("Invalid escape sequence"),
                        }
                        self.pos += 2;
                    }
                    'u' => {
                        let hex: String = self.chars.get(self.pos..self.pos + 4).unwrap_or(&[]).iter().collect();
                        let Some(unit) = u16::from_str_radix(&hex, 16).ok().filter(|_| hex.len() == 4) else {
                            return self.error("Invalid escape sequence");
                        };
                        self.pos += 4;
                        // a surrogate pair is two escapes
                        if (0xD800..0xDC00).contains(&unit)
                            && self.chars.get(self.pos) == Some(&'\\')
                            && self.chars.get(self.pos + 1) == Some(&'u')
                        {
                            let low_hex: String = self.chars.get(self.pos + 2..self.pos + 6).unwrap_or(&[]).iter().collect();
                            if let Ok(low) = u16::from_str_radix(&low_hex, 16) {
                                if (0xDC00..0xE000).contains(&low) {
                                    self.pos += 6;
                                    out.push_str(&String::from_utf16_lossy(&[unit, low]));
                                    continue;
                                }
                            }
                        }
                        out.push_str(&String::from_utf16_lossy(&[unit]));
                    }
                    _ if self.json5() => out.push(e),
                    _ => return self.error("Invalid escape sequence"),
                }
            } else if (c == '\n' || c == '\r') && !self.json5() {
                return self.error("Unterminated string");
            } else {
                out.push(c);
            }
        }
    }

    /// `true` / `false` / `null` / a number (JSON5: `Infinity`, `NaN`, hexadecimal, a leading `+`, a bare `.`)
    fn read_literal(&mut self) -> Result<serde_json::Value, String> {
        let start = self.pos;
        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || matches!(c, '+' | '-' | '.' | '_') {
                self.pos += 1;
            } else {
                break;
            }
        }
        let text: String = self.chars[start..self.pos].iter().collect();
        match text.as_str() {
            "true" => return Ok(serde_json::Value::Bool(true)),
            "false" => return Ok(serde_json::Value::Bool(false)),
            "null" => return Ok(serde_json::Value::Null),
            _ => {}
        }
        if self.json5() {
            let (sign, body) = match text.strip_prefix('-') {
                Some(rest) => (-1.0, rest),
                None => (1.0, text.strip_prefix('+').unwrap_or(&text)),
            };
            if body == "Infinity" || body == "NaN" {
                // a JSON tree cannot hold them: Gson's lenient number reads parse the text
                let text = if body == "NaN" { "NaN".to_string() } else if sign < 0.0 { "-Infinity".to_string() } else { "Infinity".to_string() };
                return Ok(serde_json::Value::String(text));
            }
            if let Some(hex) = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X")) {
                return match i64::from_str_radix(hex, 16) {
                    Ok(v) => Ok(serde_json::Value::from(if sign < 0.0 { -v } else { v })),
                    Err(_) => self.error("Invalid number"),
                };
            }
            return self.number(&format!("{}{}", if sign < 0.0 { "-" } else { "" }, body), true);
        }
        self.number(&text, false)
    }

    fn number(&self, text: &str, lenient_points: bool) -> Result<serde_json::Value, String> {
        let digits_ok = {
            let t = text.strip_prefix('-').unwrap_or(text);
            let mut chars = t.chars().peekable();
            let mut seen_digit = false;
            let mut valid = !t.is_empty();
            if !lenient_points && (t.starts_with('.') || t.ends_with('.') || (t.len() > 1 && t.starts_with('0') && t.as_bytes()[1].is_ascii_digit())) {
                valid = false;
            }
            while let Some(c) = chars.next() {
                match c {
                    '0'..='9' => seen_digit = true,
                    '.' | 'e' | 'E' | '+' | '-' => {}
                    _ => valid = false,
                }
            }
            valid && seen_digit
        };
        if !digits_ok {
            return self.error(&format!("Unexpected value '{}'", text));
        }
        if !text.contains(['.', 'e', 'E']) {
            if let Ok(v) = text.parse::<i64>() {
                return Ok(serde_json::Value::from(v));
            }
            if let Ok(v) = text.parse::<u64>() {
                return Ok(serde_json::Value::from(v));
            }
        }
        match text.parse::<f64>() {
            Ok(v) => match serde_json::Number::from_f64(v) {
                Some(n) => Ok(serde_json::Value::Number(n)),
                None => Ok(serde_json::Value::String(text.to_string())),
            },
            Err(_) => self.error(&format!("Unexpected value '{}'", text)),
        }
    }
}
