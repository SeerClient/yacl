//! Ported from dev.isxander.yacl3.config.GsonConfigInstance by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER;
use crate::dev::isxander::yacl3::config::config_entry::ConfigEntry;
use crate::dev::isxander::yacl3::config::config_instance::ConfigInstance;
use crate::dev::isxander::yacl3::config::config_instance::ConfigInstanceBase;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::Field;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ReflectedClass;
use crate::dev::isxander::yacl3::config::v2::r#impl::serializer::gson_config_serializer;
use crate::dev::isxander::yacl3::gui::utils::item_registry_helper::ItemRegistryHelper;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::core::defaulted_registry::DefaultedRegistry as _;
use mcrust::net::minecraft::world::item::item::Item;

/// java: @Deprecated public class GsonConfigInstance<T> extends ConfigInstance<T>
///
/// Uses GSON to serialize and deserialize config data from JSON to a file. Only fields annotated with
/// `@ConfigEntry` are included (`ConfigExclusionStrategy`); names are `lower_case_with_underscores`.
///
/// Gson's reflective walk over the config class is the generated reflection (`T: ReflectedClass`): the fields the
/// exclusion strategy keeps, each written / read as its generated Gson form.
pub struct GsonConfigInstance<T: 'static> {
    /// java: extends ConfigInstance
    pub base: ConfigInstanceBase<T>,
    /// java: private final Gson gson;
    pub gson: compat::Gson,
    /// java: private final Path path;
    pub path: std::path::PathBuf,
}

impl<T: 'static> std::ops::Deref for GsonConfigInstance<T> {
    type Target = ConfigInstanceBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: ReflectedClass> GsonConfigInstance<T> {
    /// java: @Deprecated public GsonConfigInstance(Class<T> configClass, Path path)
    pub fn new(config_class: compat::ClassOf<T>, path: std::path::PathBuf) -> Self {
        Self::new4(config_class, path, compat::GsonBuilder::new())
    }

    /// java: @Deprecated public GsonConfigInstance(Class<T> configClass, Path path, Gson gson)
    pub fn new2(config_class: compat::ClassOf<T>, path: std::path::PathBuf, gson: compat::Gson) -> Self {
        // java: gson.newBuilder() - a builder with the instance's settings
        let mut builder = compat::GsonBuilder::new();
        builder.pretty = gson.pretty;
        Self::new4(config_class, path, builder)
    }

    /// java: @Deprecated public GsonConfigInstance(Class<T> configClass, Path path, UnaryOperator<GsonBuilder> builder)
    pub fn new3(config_class: compat::ClassOf<T>, path: std::path::PathBuf, builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>) -> Self {
        Self::new4(config_class, path, builder(compat::GsonBuilder::new()))
    }

    /// java: @Deprecated public GsonConfigInstance(Class<T> configClass, Path path, GsonBuilder builder)
    pub fn new4(config_class: compat::ClassOf<T>, path: std::path::PathBuf, builder: compat::GsonBuilder) -> Self {
        // java: builder.setExclusionStrategies(new ConfigExclusionStrategy())
        //   .registerTypeHierarchyAdapter(Component.class, new CodecSerializerAdapter<>(ComponentSerialization.CODEC))
        //   .registerTypeHierarchyAdapter(Style.class, new GsonConfigSerializer.StyleTypeAdapter())
        //   .registerTypeHierarchyAdapter(Color.class, new ColorTypeAdapter())
        //   .registerTypeHierarchyAdapter(Item.class, new ItemTypeAdapter())
        //   .serializeNulls().setFieldNamingPolicy(FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES).create();
        // The exclusion strategy and the naming are applied by `save`/`load` (the field walk is YACL's here); the
        // four adapters are built into each field's generated Gson form.
        let gson = builder
            .register_type_hierarchy_adapter()
            .register_type_hierarchy_adapter()
            .register_type_hierarchy_adapter()
            .register_type_hierarchy_adapter()
            .serialize_nulls()
            .create();
        Self { base: ConfigInstanceBase::new(config_class), path, gson }
    }

    /// java: private GsonConfigInstance(Class<T> configClass, Path path, Gson gson, boolean fromBuilder)
    pub fn new_4(config_class: compat::ClassOf<T>, path: std::path::PathBuf, gson: compat::Gson, from_builder: bool) -> Self {
        Self { base: ConfigInstanceBase::new(config_class), path, gson }
    }

    /// java: public Path getPath()
    pub fn get_path(&self) -> std::path::PathBuf {
        self.path.clone()
    }

    /// java: public static <T> Builder<T> createBuilder(Class<T> configClass)
    pub fn create_builder(config_class: &compat::ClassOf<T>) -> Builder<T> {
        Builder::new(*config_class)
    }

    /// The fields Gson's reflective adapter walks for this class: those `ConfigExclusionStrategy` keeps, each with
    /// its serialized name (`FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES`).
    fn serialized_fields() -> Vec<(String, Field)> {
        let strategy = ConfigExclusionStrategy::new();
        T::declared_fields()
            .into_iter()
            .filter(|field| !strategy.should_skip_field(field))
            .map(|field| (lower_case_with_underscores(&field.get_name()), field))
            .collect()
    }

    /// java: gson.toJson(getConfig())
    fn to_json(&self, config: &compat::Shared<T>) -> String {
        let instance: compat::Object = Arc::new(config.clone());
        let members: Vec<(String, compat::GsonValue)> = Self::serialized_fields()
            .into_iter()
            .map(|(name, field)| {
                let value = field.get(&instance);
                (name, field.get_generic_type().to_json_tree(&value))
            })
            .collect();
        compat::gson_text(&compat::GsonValue::Obj(members), self.gson.pretty)
    }

    /// java: gson.fromJson(json, getConfigClass()) - Gson's reflective read: a new object from the no-args
    /// constructor, then each serialized field present in the JSON set (unknown keys ignored, a JSON `null` leaves a
    /// field Gson cannot set to null as it was)
    fn from_json(&self, json: &str) -> compat::Shared<T> {
        let tree = match gson_config_serializer::parse_lenient(json) {
            Ok(tree) => tree,
            Err(e) => panic!("JsonSyntaxException: {}", e),
        };
        let serde_json::Value::Object(members) = tree else {
            panic!("JsonSyntaxException: java.lang.IllegalStateException: Expected BEGIN_OBJECT");
        };
        let config = compat::shared(self.get_config_class().new_instance());
        let instance: compat::Object = Arc::new(config.clone());
        for (name, field) in Self::serialized_fields() {
            let Some(value) = members.get(&name) else {
                continue;
            };
            match field.get_generic_type().from_json_tree(value) {
                Some(value) => field.set(&instance, value),
                None if value.is_null() => {}
                None => panic!("JsonSyntaxException: field {} cannot hold {}", name, value),
            }
        }
        config
    }
}

impl<T: ReflectedClass> ConfigInstance<T> for GsonConfigInstance<T> {
    fn config_instance_base(&self) -> &ConfigInstanceBase<T> {
        &self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void save()
    fn save(&self) {
        LOGGER.info(&format!("Saving {}...", self.get_config_class().get_simple_name()));
        // java: Files.writeString(path, gson.toJson(getConfig()), StandardOpenOption.TRUNCATE_EXISTING, StandardOpenOption.CREATE);
        if let Err(e) = std::fs::write(&self.path, self.to_json(&self.get_config())) {
            // java: e.printStackTrace();
            LOGGER.error(&format!("java.io.IOException: {}", e));
        }
    }

    /// java: @Override public void load()
    fn load(&self) {
        if !self.path.exists() {
            self.save();
            return;
        }

        LOGGER.info(&format!("Loading {}...", self.get_config_class().get_simple_name()));
        match std::fs::read_to_string(&self.path) {
            Ok(json) => self.set_config(self.from_json(&json)),
            // java: e.printStackTrace();
            Err(e) => LOGGER.error(&format!("java.io.IOException: {}", e)),
        }
    }
}

/// Gson's `FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES`: `separateCamelCase(name, '_').toLowerCase(Locale.ENGLISH)`
fn lower_case_with_underscores(name: &str) -> String {
    let mut translation = String::new();
    for character in name.chars() {
        if character.is_uppercase() && !translation.is_empty() {
            translation.push('_');
        }
        translation.push(character);
    }
    translation.to_lowercase()
}

/// java: private static class ConfigExclusionStrategy implements ExclusionStrategy
#[derive(Clone, Default)]
pub struct ConfigExclusionStrategy {}

impl ConfigExclusionStrategy {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }

    /// java: @Override public boolean shouldSkipField(FieldAttributes fieldAttributes)
    pub fn should_skip_field(&self, field_attributes: &Field) -> bool {
        field_attributes.get_annotation(&std::any::TypeId::of::<ConfigEntry>()).is_none()
    }

    /// java: @Override public boolean shouldSkipClass(Class<?> aClass)
    pub fn should_skip_class(&self, a_class: &str) -> bool {
        false
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
    pub fn deserialize(&self, json_element: &serde_json::Value) -> Arc<Item> {
        ItemRegistryHelper::get_item_from_name_1(&compat::json::get_as_string(json_element))
    }

    /// java: @Override public JsonElement serialize(Item item, Type type, JsonSerializationContext jsonSerializationContext)
    pub fn serialize(&self, item: &Arc<Item>) -> compat::GsonValue {
        compat::GsonValue::str(
            &mcrust::net::minecraft::core::registries::built_in_registries::item().get_key_defaulted(&item.erased_obj()).to_string(),
        )
    }
}

/// java: public static class Builder<T>
///
/// Every setter takes the builder BY VALUE and answers it (Java's `return this`).
pub struct Builder<T: 'static> {
    /// java: private final Class<T> configClass;
    pub config_class: compat::ClassOf<T>,
    /// java: private Path path;
    pub path: Option<std::path::PathBuf>,
    /// java: private UnaryOperator<GsonBuilder> gsonBuilder = builder -> builder
    ///   .setFieldNamingPolicy(FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES).serializeNulls()
    ///   .registerTypeHierarchyAdapter(Component.class, ..).registerTypeHierarchyAdapter(Style.class, ..)
    ///   .registerTypeHierarchyAdapter(Color.class, ..).registerTypeHierarchyAdapter(Item.class, ..);
    pub gson_builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>,
}

impl<T: ReflectedClass> Builder<T> {
    /// java: private Builder(Class<T> configClass)
    fn new(config_class: compat::ClassOf<T>) -> Self {
        Self {
            config_class,
            path: None,
            gson_builder: Arc::new(|builder: compat::GsonBuilder| {
                // the naming policy is applied by GsonConfigInstance's field walk; the four adapters are built into
                // each field's generated Gson form
                builder
                    .serialize_nulls()
                    .register_type_hierarchy_adapter()
                    .register_type_hierarchy_adapter()
                    .register_type_hierarchy_adapter()
                    .register_type_hierarchy_adapter()
            }),
        }
    }

    /// java: public Builder<T> setPath(Path path)
    pub fn set_path(mut self, path: std::path::PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    /// java: public Builder<T> overrideGsonBuilder(GsonBuilder gsonBuilder)
    pub fn override_gson_builder(mut self, gson_builder: &compat::GsonBuilder) -> Self {
        let gson_builder = gson_builder.clone();
        self.gson_builder = Arc::new(move |_builder| gson_builder.clone());
        self
    }

    /// java: public Builder<T> overrideGsonBuilder(Gson gson)
    pub fn override_gson_builder_gson(self, gson: &compat::Gson) -> Self {
        // java: gson.newBuilder()
        let mut builder = compat::GsonBuilder::new();
        builder.pretty = gson.pretty;
        self.override_gson_builder(&builder)
    }

    /// java: public Builder<T> appendGsonBuilder(UnaryOperator<GsonBuilder> gsonBuilder)
    pub fn append_gson_builder(mut self, gson_builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>) -> Self {
        let prev = self.gson_builder.clone();
        self.gson_builder = Arc::new(move |builder| gson_builder(prev(builder)));
        self
    }

    /// java: public GsonConfigInstance<T> build()
    pub fn build(&self) -> GsonConfigInstance<T> {
        // java: builder -> this.gsonBuilder.apply(builder).addSerializationExclusionStrategy(new ConfigExclusionStrategy())
        //   .addDeserializationExclusionStrategy(new ConfigExclusionStrategy()) - the strategy is GsonConfigInstance's
        //   field walk
        let gson = (self.gson_builder)(compat::GsonBuilder::new()).create();
        GsonConfigInstance::new_4(self.config_class, self.path.clone().expect("NullPointerException: path"), gson, true)
    }
}
