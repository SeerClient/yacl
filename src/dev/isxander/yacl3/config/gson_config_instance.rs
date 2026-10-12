//! Ported from dev.isxander.yacl3.config.GsonConfigInstance by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::config_instance::ConfigInstance;
use crate::dev::isxander::yacl3::config::config_instance::ConfigInstanceBase;
use crate::dev::isxander::yacl3::gui::utils::item_registry_helper::ItemRegistryHelper;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::core::registry::Registry as _;

/// java: @Deprecated public class GsonConfigInstance<T> extends ConfigInstance<T>
pub struct GsonConfigInstance<T> {
    /// java: extends ConfigInstance
    pub base: ConfigInstanceBase<T>,
    /// java: private final Gson gson;
    pub gson: Option<compat::Gson>,
    /// java: private final Path path;
    pub path: Option<std::path::PathBuf>,
}

impl<T: 'static> std::ops::Deref for GsonConfigInstance<T> {
    type Target = ConfigInstanceBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for GsonConfigInstance<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> GsonConfigInstance<T> {
    /// java: @Deprecated public GsonConfigInstance(Class<T> configClass, Path path)
    pub fn new(mut config_class: compat::ClassOf<T>, mut path: std::path::PathBuf) -> Self {
        let mut this = Self::new4(config_class.clone(), path.clone(), compat::GsonBuilder::new());
        this
    }

    /// java: @Deprecated public GsonConfigInstance(Class<T> configClass, Path path, Gson gson)
    pub fn new2(mut config_class: compat::ClassOf<T>, mut path: std::path::PathBuf, mut gson: compat::Gson) -> Self {
        // TODO(fabric2seer): `Gson.newBuilder` (Gson over the generated per-class forms)
        //   java: this(configClass, path, gson.newBuilder());
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Deprecated public GsonConfigInstance(Class<T> configClass, Path path, UnaryOperator<GsonBuilder> builder)
    pub fn new3(mut config_class: compat::ClassOf<T>, mut path: std::path::PathBuf, builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>) -> Self {
        let mut this = Self::new4(config_class.clone(), path.clone(), (builder)(compat::GsonBuilder::new()));
        this
    }

    /// java: @Deprecated public GsonConfigInstance(Class<T> configClass, Path path, GsonBuilder builder)
    pub fn new4(mut config_class: compat::ClassOf<T>, mut path: std::path::PathBuf, mut builder: compat::GsonBuilder) -> Self {
        let __init_path = Some(path.clone());
        // TODO(fabric2seer): `GsonBuilder.setExclusionStrategies` (Gson over the generated per-class forms)
        //   java: this.gson = builder
        //   java:                 .setExclusionStrategies(new ConfigExclusionStrategy())
        //   java:                 .registerTypeHierarchyAdapter(Component.class, new CodecSerializerAdapter<>(ComponentSerialization.CODEC))
        //   java:                 .registerTypeHierarchyAdapter(Style.class, new GsonConfigSerializer.StyleTypeAdapter())
        //   java:                 .registerTypeHierarchyAdapter(Color.class, new ColorTypeAdapter())
        //   java:                 .registerTypeHierarchyAdapter(Item.class, new ItemTypeAdapter())
        //   java:                 .serializeNulls()
        //   java:                 .setFieldNamingPolicy(FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES)
        //   java:                 .create();
        let mut this = Self {
            base: ConfigInstanceBase::<T>::new(config_class.clone()),
            gson: todo!("fabric2seer: the field value (TODO above)"),
            path: __init_path,
        };
        this
    }

    /// java: private GsonConfigInstance(Class<T> configClass, Path path, Gson gson, boolean fromBuilder)
    pub fn new_4(mut config_class: compat::ClassOf<T>, mut path: std::path::PathBuf, mut gson: compat::Gson, mut from_builder: bool) -> Self {
        let __init_path = Some(path.clone());
        let __init_gson = Some(gson.clone());
        let mut this = Self {
            base: ConfigInstanceBase::<T>::new(config_class.clone()),
            gson: __init_gson,
            path: __init_path,
        };
        this
    }

    /// java: public static <T> Builder<T> createBuilder(Class<T> configClass)
    pub fn create_builder<T: 'static>(config_class: &compat::ClassOf<T>) -> Builder<T> {
        return Builder::new(config_class.clone());
    }

    /// java: public Path getPath()
    pub fn get_path(&self) -> std::path::PathBuf {
        return self.path.clone().unwrap();
    }

}

impl<T: Clone + 'static> ConfigInstance<T> for GsonConfigInstance<T> {
    fn config_instance_base(&self) -> &ConfigInstanceBase<T> {
        &self.base
    }
    fn config_instance_base_mut(&mut self) -> &mut ConfigInstanceBase<T> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void save()
    fn save(&self) {
        // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            logger().info(&format!("Saving {}...", self.get_config_class().get_simple_name()));
            // TODO(fabric2seer): `Gson.toJson` (Gson over the generated per-class forms)
            //   java: Files.writeString(path, gson.toJson(getConfig()), StandardOpenOption.TRUNCATE_EXISTING, StandardOpenOption.CREATE);
        }));
        if let Err(e) = __result {
            logger().error(&compat::panic_message(&e));
        }
    }

    /// java: @Override public void load()
    fn load(&mut self) {
        // java: try {..} catch (IOException e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            if !self.path.as_ref().unwrap().exists() {
                self.save();
                return;
            }
            logger().info(&format!("Loading {}...", self.get_config_class().get_simple_name()));
            { let __s0 = self.gson.as_ref().unwrap().from_json(&std::fs::read_to_string(self.path.as_ref().unwrap()).expect("IOException"), &self.get_config_class()).unwrap(); self.set_config(__s0) };
        }
        // TODO(fabric2seer): the `catch (IOException e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java:             e.printStackTrace();
        //   java:         }
    }

}

/// java: private static class ConfigExclusionStrategy implements ExclusionStrategy
#[derive(Clone)]
pub struct ConfigExclusionStrategy {}

impl ConfigExclusionStrategy {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public boolean shouldSkipField(FieldAttributes fieldAttributes)
    pub fn should_skip_field(&self, field_attributes: &crate::unresolved::FieldAttributes) -> bool {
        // TODO(fabric2seer): `.getAnnotation(..)` on a FieldAttributes
        //   java: return fieldAttributes.getAnnotation(ConfigEntry.class) == null;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public boolean shouldSkipClass(Class<?> aClass)
    pub fn should_skip_class(&self, a_class: &compat::ClassOf<compat::Object>) -> bool {
        return false;
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
    pub fn deserialize(&self, json_element: &serde_json::Value, r#type: &crate::unresolved::Type, json_deserialization_context: &crate::unresolved::JsonDeserializationContext) -> crate::unresolved::Color {
        // TODO(fabric2seer): `new Color(..)`
        //   java: return new Color(jsonElement.getAsInt(), true);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public JsonElement serialize(Color color, Type type, JsonSerializationContext jsonSerializationContext)
    pub fn serialize(&self, color: &crate::unresolved::Color, r#type: &crate::unresolved::Type, json_serialization_context: &crate::unresolved::JsonSerializationContext) -> serde_json::Value {
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

/// java: public static class Builder<T>
pub struct Builder<T> {
    /// java: private final Class<T> configClass;
    pub config_class: compat::ClassOf<T>,
    /// java: private Path path;
    pub path: Option<std::path::PathBuf>,
    /// java: private UnaryOperator<GsonBuilder> gsonBuilder = builder -> builder .setFieldNamingPolicy(FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES) .serializeNulls() .regi
    pub gson_builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>,
}

impl<T: Clone + 'static> Builder<T> {
    /// java: private Builder(Class<T> configClass)
    pub fn new(mut config_class: compat::ClassOf<T>) -> Self {
        let __init_config_class = config_class.clone();
        let mut this = Self {
            config_class: __init_config_class,
            path: None,
            gson_builder: Arc::new(move |builder| builder.clone().set_field_naming_policy(compat::FieldNamingPolicy::Identity).serialize_nulls().register_type_hierarchy_adapter().register_type_hierarchy_adapter().register_type_hierarchy_adapter().register_type_hierarchy_adapter()),
        };
        this
    }

    /// java: public Builder<T> setPath(Path path)
    pub fn set_path(&mut self, mut path: std::path::PathBuf) -> Builder<T> {
        self.path = Some(path.clone());
        return self.clone();
    }

    /// java: public Builder<T> overrideGsonBuilder(GsonBuilder gsonBuilder)
    pub fn override_gson_builder(&mut self, gson_builder: &compat::GsonBuilder) -> Builder<T> {
        self.gson_builder = Arc::new(move |builder| gson_builder.clone());
        return self.clone();
    }

    /// java: public Builder<T> overrideGsonBuilder(Gson gson)
    pub fn override_gson_builder_gson(&mut self, gson: &compat::Gson) -> Builder<T> {
        // TODO(fabric2seer): `Gson.newBuilder` (Gson over the generated per-class forms)
        //   java: return this.overrideGsonBuilder(gson.newBuilder());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public Builder<T> appendGsonBuilder(UnaryOperator<GsonBuilder> gsonBuilder)
    pub fn append_gson_builder(&mut self, gson_builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>) -> Builder<T> {
        let mut prev: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder> = self.gson_builder.clone();
        self.gson_builder = Arc::new(move |builder| (gson_builder)((prev)(builder.clone())));
        return self.clone();
    }

    /// java: public GsonConfigInstance<T> build()
    pub fn build(&self) -> GsonConfigInstance<T> {
        // TODO(fabric2seer): `GsonBuilder.addSerializationExclusionStrategy` (Gson over the generated per-class forms)
        //   java: UnaryOperator<GsonBuilder> gsonBuilder = builder -> this.gsonBuilder.apply(builder)
        //   java:                     .addSerializationExclusionStrategy(new ConfigExclusionStrategy())
        //   java:                     .addDeserializationExclusionStrategy(new ConfigExclusionStrategy());
        let mut gson_builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder> = todo!("fabric2seer: the initialiser of `gsonBuilder` (TODO above)");
        return GsonConfigInstance::new_4(self.config_class.clone(), self.path.clone().unwrap(), (gson_builder)(compat::GsonBuilder::new()).create(), true);
    }

}
