//! Ported from dev.isxander.yacl3.config.v2.impl.ConfigClassHandlerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category;
use crate::dev::isxander::yacl3::api::config_category::Builder as _;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::option_addable::OptionAddable;
use crate::dev::isxander::yacl3::api::option_description;
use crate::dev::isxander::yacl3::api::option_group;
use crate::dev::isxander::yacl3::api::option_group::Builder as _;
use crate::dev::isxander::yacl3::api::yet_another_config_lib;
use crate::dev::isxander::yacl3::api::yet_another_config_lib::Builder as _;
use crate::dev::isxander::yacl3::api::yet_another_config_lib::YetAnotherConfigLib;
use crate::dev::isxander::yacl3::config::config_entry::ConfigEntry;
use crate::dev::isxander::yacl3::config::v2::api::autogen::auto_gen::AutoGen;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::Builder;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::BufferAccessMap;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::ConfigSerializer;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::ConfigSerializerBase;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::LoadResult;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::serial_entry::SerialEntry;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::option_access_impl::OptionAccessImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::option_factory_registry;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::option_factory_registry::OptionFactoryRegistry;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::yacl_auto_gen_exception::YACLAutoGenException;
use crate::dev::isxander::yacl3::config::v2::r#impl::config_field_impl::ConfigFieldImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::annotation_as;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::class_annotation;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::Field;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ReflectedClass;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ReflectionFieldAccess;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use crate::dev::isxander::yacl3::r#impl::config_category_impl::BuilderImpl as ConfigCategoryBuilder;
use crate::dev::isxander::yacl3::r#impl::option_group_impl::BuilderImpl as OptionGroupBuilder;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::locale::language::LanguageObj as _;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use std::any::TypeId;

/// java: public class ConfigClassHandlerImpl<T> implements ConfigClassHandler<T>
///
/// `T` is the user's config class (`ReflectedClass`: its fields are the generated reflection). Shared by reference
/// (`Arc`); the fields Java assigns after construction or in `load` are cells.
pub struct ConfigClassHandlerImpl<T: 'static> {
    /// java: private final Class<T> configClass;
    pub config_class: compat::ClassOf<T>,
    /// java: private final Identifier id;
    pub id: Option<Identifier>,
    /// java: private final boolean supportsAutoGen;
    pub supports_auto_gen: bool,
    /// java: private final ConfigSerializer<T> serializer; (assigned at the end of the constructor, from `this`)
    pub serializer: std::cell::OnceCell<Arc<dyn ConfigSerializer<T>>>,
    /// java: private final ConfigFieldImpl<?>[] fields; (assigned in the constructor, from `this`)
    pub fields: std::cell::OnceCell<Vec<Arc<ConfigFieldImpl<compat::Object>>>>,
    /// java: private T instance;
    pub instance: std::cell::RefCell<compat::Shared<T>>,
    /// java: private final T defaults;
    pub defaults: compat::Shared<T>,
    /// java: private final Constructor<T> noArgsConstructor;
    pub no_args_constructor: fn() -> T,
    /// Java's `this` (`parent` of every field, the serializer's `config`)
    pub self_ref: std::sync::Weak<Self>,
}

impl<T: ReflectedClass> ConfigClassHandlerImpl<T> {
    /// java: public ConfigClassHandlerImpl(Class<T> configClass, Identifier id, Function<ConfigClassHandler<T>, ConfigSerializer<T>> serializerFactory)
    pub fn new(
        config_class: compat::ClassOf<T>,
        id: Option<Identifier>,
        serializer_factory: Arc<dyn Fn(Arc<dyn ConfigClassHandler<T>>) -> Arc<dyn ConfigSerializer<T>>>,
    ) -> Arc<Self> {
        let supports_auto_gen = id.is_some() && YACLPlatform::get_environment().is_client();

        // java: noArgsConstructor = configClass.getDeclaredConstructor(); - a generated class always has one
        let no_args_constructor = config_class.new;

        let this = Arc::new_cyclic(|self_ref| Self {
            config_class,
            id,
            supports_auto_gen,
            serializer: std::cell::OnceCell::new(),
            fields: std::cell::OnceCell::new(),
            instance: std::cell::RefCell::new(compat::shared(Self::create_new_object_of(&config_class, no_args_constructor))),
            defaults: compat::shared(Self::create_new_object_of(&config_class, no_args_constructor)),
            no_args_constructor,
            self_ref: self_ref.clone(),
        });

        this.detect_old_annotation(&T::declared_fields());
        let fields = this.discover_fields();
        let _ = this.fields.set(fields);
        let serializer = serializer_factory(this.clone());
        let _ = this.serializer.set(serializer);
        this
    }

    fn this(&self) -> Arc<Self> {
        self.self_ref.upgrade().expect("ConfigClassHandlerImpl used after it was dropped")
    }

    /// java: private ConfigFieldImpl<?>[] discoverFields()
    fn discover_fields(&self) -> Vec<Arc<ConfigFieldImpl<compat::Object>>> {
        let class_serial_entry: Option<SerialEntry> = class_annotation(&T::class_annotations());
        let class_has_serial_entry = class_serial_entry.is_some();
        if let Some(class_serial_entry) = &class_serial_entry {
            if !("" == class_serial_entry.value()) {
                panic!(
                    "IllegalArgumentException: {}",
                    format!(
                        "SerialEntry on class '{}' must not have a value. Only `required` and `nullable` are permitted parameters on classes.",
                        self.config_class.get_name()
                    )
                );
            }
            if !("" == class_serial_entry.comment()) {
                panic!(
                    "IllegalArgumentException: {}",
                    format!(
                        "SerialEntry on class '{}' must not have a comment. Only `required` and `nullable` are permitted parameters on classes.",
                        self.config_class.get_name()
                    )
                );
            }
        }

        let instance: compat::Object = Arc::new(self.instance.borrow().clone());
        let defaults: compat::Object = Arc::new(self.defaults.clone());
        let parent: Arc<dyn ConfigClassHandler<compat::Object>> = Arc::new(WildcardClassHandler::<T> { handler: self.self_ref.clone() });

        T::declared_fields()
            .into_iter()
            .inspect(|field| field.set_accessible(true))
            .filter(|field| {
                class_has_serial_entry
                    || field.is_annotation_present(&TypeId::of::<SerialEntry>())
                    || field.is_annotation_present(&TypeId::of::<AutoGen>())
            })
            .map(|field| {
                ConfigFieldImpl::<compat::Object>::new(
                    ReflectionFieldAccess::new(field.clone(), instance.clone()),
                    ReflectionFieldAccess::new(field.clone(), defaults.clone()),
                    parent.clone(),
                    annotation_as(field.get_annotation(&TypeId::of::<SerialEntry>())),
                    class_serial_entry.clone(),
                    annotation_as(field.get_annotation(&TypeId::of::<AutoGen>())),
                )
            })
            .collect()
    }

    /// java: private <U> Option<U> createOption(ConfigField<U> configField, OptionAccess storage)
    fn create_option(&self, config_field: &Arc<dyn ConfigField<compat::Object>>, storage: &Arc<dyn OptionAccess>) -> Arc<dyn AnyOption> {
        let field: Field = option_factory_registry::reflected_field(config_field);
        match OptionFactoryRegistry::create_option(&field, config_field, storage) {
            Some(option) => option,
            None => YACLAutoGenException::new(format!("Failed to create option for field {}", config_field.access().name())).throw(),
        }
    }

    /// java: private T createNewObject()
    fn create_new_object(&self) -> T {
        Self::create_new_object_of(&self.config_class, self.no_args_constructor)
    }

    /// `createNewObject()`'s body, which the constructor also runs before `this` exists
    fn create_new_object_of(config_class: &compat::ClassOf<T>, no_args_constructor: fn() -> T) -> T {
        // java: try { return noArgsConstructor.newInstance(); } catch (Exception e)
        match std::panic::catch_unwind(no_args_constructor) {
            Ok(object) => object,
            Err(_) => YACLAutoGenException::new(format!(
                "Failed to create instance of config class '{}' with no-args constructor.",
                config_class.get_name()
            ))
            .throw(),
        }
    }

    /// java: private void detectOldAnnotation(Field[] fields)
    fn detect_old_annotation(&self, fields: &[Field]) {
        let has_old_config_entry = fields.iter().any(|field| field.is_annotation_present(&TypeId::of::<ConfigEntry>()));
        // java: Validate.isTrue(!hasOldConfigEntry, ..)
        if has_old_config_entry {
            panic!(
                "IllegalArgumentException: {}",
                format!(
                    "At least one field in {} is still annotated with the deprecated @ConfigEntry annotation. This is incorrect. Use @SerialEntry.",
                    self.config_class.get_name()
                )
            );
        }
    }
}

impl<T: ReflectedClass> ConfigClassHandler<T> for ConfigClassHandlerImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public T instance()
    fn instance(&self) -> compat::Shared<T> {
        self.instance.borrow().clone()
    }

    /// java: @Override public T defaults()
    fn defaults(&self) -> compat::Shared<T> {
        self.defaults.clone()
    }

    /// java: @Override public Class<T> configClass()
    fn config_class(&self) -> compat::ClassOf<T> {
        self.config_class
    }

    /// java: @Override public ConfigFieldImpl<?>[] fields()
    fn fields(&self) -> Vec<Arc<dyn ConfigField<compat::Object>>> {
        self.fields
            .get()
            .map(|fields| fields.iter().map(|field| field.clone() as Arc<dyn ConfigField<compat::Object>>).collect())
            .unwrap_or_default()
    }

    /// java: @Override public Identifier id()
    fn id(&self) -> Identifier {
        self.id.clone().expect("NullPointerException: the config class handler was built without an id")
    }

    /// java: @Override public boolean supportsAutoGen()
    fn supports_auto_gen(&self) -> bool {
        self.supports_auto_gen
    }

    /// java: @Override public YetAnotherConfigLib generateGui()
    fn generate_gui(&self) -> Arc<dyn YetAnotherConfigLib> {
        if !self.supports_auto_gen() {
            YACLAutoGenException::new(
                "Auto GUI generation is not supported for this config class. You either need to enable it in the builder or you are attempting to create a GUI in a dedicated server environment."
                    .to_string(),
            )
            .throw();
        }

        let has_auto_gen_fields = self.fields().iter().any(|field| field.auto_gen().is_some());
        if !has_auto_gen_fields {
            YACLAutoGenException::new(
                "No fields in this config class are annotated with @AutoGen. You must annotate at least one field with @AutoGen to generate a GUI."
                    .to_string(),
            )
            .throw();
        }

        let storage_impl: Arc<OptionAccessImpl> = Arc::new(OptionAccessImpl::new());
        let storage: Arc<dyn OptionAccess> = storage_impl.clone();

        // java: Map<String, CategoryAndGroups> categories = new LinkedHashMap<>();
        let mut categories: Vec<(String, CategoryAndGroups)> = Vec::new();
        for config_field in self.fields() {
            let Some(auto_gen) = config_field.auto_gen() else {
                continue;
            };

            // java: categories.computeIfAbsent(autoGen.category(), k -> new CategoryAndGroups(
            //     ConfigCategory.createBuilder().name(Component.translatable("yacl3.config.%s.category.%s".formatted(id().toString(), k))),
            //     new LinkedHashMap<>()))
            let category_key = auto_gen.category();
            let category_index = match categories.iter().position(|(k, _)| *k == category_key) {
                Some(index) => index,
                None => {
                    let category = config_category::create_builder()
                        .name(component::translatable(&format!("yacl3.config.{}.category.{}", self.id(), category_key)));
                    categories.push((category_key.clone(), CategoryAndGroups::new(category, Vec::new())));
                    categories.len() - 1
                }
            };
            let groups = &mut categories[category_index].1;

            // java: groups.groups().computeIfAbsent(autoGen.group().orElse(""), k -> { .. })
            let group_key = auto_gen.group().unwrap_or("".to_string());
            let group_index = match groups.groups.iter().position(|(k, _)| *k == group_key) {
                Some(index) => index,
                None => {
                    let group = if group_key.is_empty() {
                        // java: return groups.category();
                        OptionAddableRef::Category
                    } else {
                        let group_translation_key =
                            format!("yacl3.config.{}.category.{}.group.{}", self.id(), auto_gen.category(), group_key);
                        let mut group_builder = option_group::create_builder().name(component::translatable(&group_translation_key));

                        let description_key = format!("{}.desc", group_translation_key);
                        if language::get_instance().has(&description_key) {
                            group_builder = group_builder.description(option_description::of(vec![component::translatable(&description_key)]));
                        }

                        OptionAddableRef::Group(group_builder)
                    };
                    groups.groups.push((group_key.clone(), group));
                    groups.groups.len() - 1
                }
            };

            // java: try { option = createOption(configField, storage); } catch (Exception e)
            let option = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.create_option(&config_field, &storage))) {
                Ok(option) => option,
                Err(e) => YACLAutoGenException::new(format!(
                    "Failed to create option for field '{}' (caused by: {})",
                    config_field.access().name(),
                    compat::panic_message(&e)
                ))
                .throw(),
            };

            storage_impl.put_option(&config_field.access().name(), option.clone());
            groups.option(group_index, option);
        }
        storage_impl.check_bad_operations();
        let categories: Vec<CategoryAndGroups> = categories.into_iter().map(|(_, category)| category.finalise_groups()).collect();

        // java: this.serializer()::save - the serializer is read here, once
        let serializer = self.serializer();
        let mut yacl_builder = yet_another_config_lib::create_builder()
            .save(move || serializer.save())
            .title(component::translatable(&format!("yacl3.config.{}.title", self.id())));

        for category in categories {
            yacl_builder = yacl_builder.category(category.category.build());
        }

        yacl_builder.build()
    }

    /// java: @Override public ConfigSerializer<T> serializer()
    fn serializer(&self) -> Arc<dyn ConfigSerializer<T>> {
        self.serializer.get().cloned().expect("ConfigClassHandlerImpl: the serializer is assigned at the end of its constructor")
    }

    /// java: @Override public boolean load()
    fn load(&self) -> bool {
        // create a new instance to load into
        let new_instance: compat::Shared<T> = compat::shared(self.create_new_object());
        let new_instance_object: compat::Object = Arc::new(new_instance.clone());

        // create field accesses for the new object
        let fields: Vec<Arc<ConfigFieldImpl<compat::Object>>> = self.fields.get().cloned().unwrap_or_default();
        let access_buffer_impl: Vec<(Arc<ConfigFieldImpl<compat::Object>>, Arc<ReflectionFieldAccess<compat::Object>>)> = fields
            .iter()
            .map(|field| {
                let access = Arc::new(ReflectionFieldAccess::new(field.reflection_access().field(), new_instance_object.clone()));
                (field.clone(), access)
            })
            .collect();

        // convert the map into API safe field accesses
        let access_buffer = BufferAccessMap::new(
            access_buffer_impl
                .iter()
                .map(|(field, access)| {
                    (field.clone() as Arc<dyn ConfigField<compat::Object>>, access.clone() as Arc<dyn FieldAccess<compat::Object>>)
                })
                .collect(),
        );

        // attempt to load the config
        let mut load_result = LoadResult::Failure;
        let mut error: Option<String> = None;
        let serializer = self.serializer();
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| serializer.load_safely(&access_buffer))) {
            Ok(result) => load_result = result,
            // handle any errors later in the loadResult switch case
            Err(e) => error = Some(compat::panic_message(&e)),
        }

        match load_result {
            LoadResult::Dirty | LoadResult::Success => {
                // replace the instance with the newly created one
                *self.instance.borrow_mut() = new_instance;
                for (field, access) in access_buffer_impl.iter() {
                    // update the field accesses to point to the correct object
                    field.set_field_access(access.clone());
                }

                if load_result == LoadResult::Dirty {
                    // if the load result is dirty, we need to save the config again
                    self.save();
                }
                // falls through to NO_CHANGE
                true
            }
            LoadResult::NoChange => true,
            LoadResult::Failure => {
                LOGGER.error(&format!(
                    "Unsuccessful load of config class '{}'. The load will be abandoned and config remains unchanged.{}",
                    self.config_class.get_simple_name(),
                    error.map(|e| format!(" {}", e)).unwrap_or_default()
                ));
                false
            }
        }
    }

    /// java: @Override public void save()
    fn save(&self) {
        self.serializer().save();
    }
}

/// java: public static class BuilderImpl<T> implements Builder<T>
pub struct BuilderImpl<T: 'static> {
    /// java: private final Class<T> configClass;
    pub config_class: compat::ClassOf<T>,
    /// java: private Identifier id;
    pub id: std::cell::RefCell<Option<Identifier>>,
    /// java: private Function<ConfigClassHandler<T>, ConfigSerializer<T>> serializerFactory;
    pub serializer_factory: std::cell::RefCell<Option<Arc<dyn Fn(Arc<dyn ConfigClassHandler<T>>) -> Arc<dyn ConfigSerializer<T>>>>>,
    /// Java's `this`, handed back by every setter
    pub self_ref: std::sync::Weak<Self>,
}

impl<T: ReflectedClass> BuilderImpl<T> {
    /// java: public BuilderImpl(Class<T> configClass)
    pub fn new(config_class: compat::ClassOf<T>) -> Arc<Self> {
        Arc::new_cyclic(|self_ref| Self {
            config_class,
            id: std::cell::RefCell::new(None),
            serializer_factory: std::cell::RefCell::new(None),
            self_ref: self_ref.clone(),
        })
    }

    fn this(&self) -> Arc<dyn Builder<T>> {
        self.self_ref.upgrade().expect("ConfigClassHandler.Builder used after it was dropped")
    }
}

impl<T: ReflectedClass> Builder<T> for BuilderImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder<T> id(Identifier id)
    fn id(&self, id: Identifier) -> Arc<dyn Builder<T>> {
        *self.id.borrow_mut() = Some(id);
        self.this()
    }

    /// java: @Override public Builder<T> serializer(Function<ConfigClassHandler<T>, ConfigSerializer<T>> serializerFactory)
    fn serializer(
        &self,
        serializer_factory: Arc<dyn Fn(Arc<dyn ConfigClassHandler<T>>) -> Arc<dyn ConfigSerializer<T>>>,
    ) -> Arc<dyn Builder<T>> {
        *self.serializer_factory.borrow_mut() = Some(serializer_factory);
        self.this()
    }

    /// java: @Override public ConfigClassHandler<T> build()
    fn build(&self) -> Arc<dyn ConfigClassHandler<T>> {
        // java: Validate.notNull(serializerFactory, "serializerFactory must not be null"); - configClass is never null
        let serializer_factory = self
            .serializer_factory
            .borrow()
            .clone()
            .unwrap_or_else(|| panic!("NullPointerException: {}", "serializerFactory must not be null"));

        ConfigClassHandlerImpl::new(self.config_class, self.id.borrow().clone(), serializer_factory)
    }
}

/// java: an `OptionAddable` of `CategoryAndGroups.groups`: the category builder itself (the root group, `""`) or a
/// group's builder. Java's map holds the category builder object under `""` and `finaliseGroups` tells them apart with
/// `instanceof OptionGroup.Builder`; the builders here are values, so the root entry names the category.
enum OptionAddableRef {
    Category,
    Group(OptionGroupBuilder),
}

/// java: private record CategoryAndGroups(ConfigCategory.Builder category, Map<String, OptionAddable> groups)
struct CategoryAndGroups {
    /// java: ConfigCategory.Builder category
    category: ConfigCategoryBuilder,
    /// java: Map<String, OptionAddable> groups (a LinkedHashMap)
    groups: Vec<(String, OptionAddableRef)>,
}

impl CategoryAndGroups {
    /// java: the canonical record constructor
    fn new(category: ConfigCategoryBuilder, groups: Vec<(String, OptionAddableRef)>) -> Self {
        Self { category, groups }
    }

    /// java: group.option(option) - on the entry at `index`
    fn option(&mut self, index: usize, option: Arc<dyn AnyOption>) {
        let (_, group) = &mut self.groups[index];
        match group {
            OptionAddableRef::Category => {
                let category = std::mem::replace(&mut self.category, config_category::create_builder());
                self.category = category.option(option);
            }
            OptionAddableRef::Group(group_builder) => {
                let builder = std::mem::replace(group_builder, option_group::create_builder());
                *group_builder = builder.option(option);
            }
        }
    }

    /// java: private void finaliseGroups()
    fn finalise_groups(mut self) -> Self {
        let groups = std::mem::take(&mut self.groups);
        for (name, group) in groups {
            // java: if (group instanceof OptionGroup.Builder groupBuilder) category.group(groupBuilder.build());
            if let OptionAddableRef::Group(group_builder) = group {
                self.category = self.category.group(group_builder.build());
            }
        }
        self
    }
}

/// java: `this` as `ConfigClassHandler<?>` (each field's `parent()`): the handler with its config objects seen as
/// Java's `Object` (`Arc<compat::Shared<T>>`). It holds the handler weakly (the handler holds the fields).
pub struct WildcardClassHandler<T: 'static> {
    handler: std::sync::Weak<ConfigClassHandlerImpl<T>>,
}

impl<T: ReflectedClass> WildcardClassHandler<T> {
    fn handler(&self) -> Arc<ConfigClassHandlerImpl<T>> {
        self.handler.upgrade().expect("ConfigField.parent() used after its handler was dropped")
    }
}

/// `Class<?>` of a reflected class: its constructor and Gson forms over the erased object
fn wildcard_new<T: ReflectedClass>() -> compat::Object {
    Arc::new(compat::shared((T::class_of().new)()))
}

fn wildcard_handle<T: ReflectedClass>(object: &compat::Object) -> &compat::Shared<T> {
    match (**object).downcast_ref::<compat::Shared<T>>() {
        Some(handle) => handle,
        None => panic!("ClassCastException: not an object of class {}", T::class_of().name),
    }
}

fn wildcard_to_gson<T: ReflectedClass>(object: &compat::Object) -> compat::GsonValue {
    compat::read(wildcard_handle::<T>(object), |config| (T::class_of().to_gson)(config))
}

fn wildcard_read_gson<T: ReflectedClass>(json: &serde_json::Value) -> Option<compat::Object> {
    (T::class_of().read_gson)(json).map(|config| Arc::new(compat::shared(config)) as compat::Object)
}

fn wildcard_patch<T: ReflectedClass>(object: &mut compat::Object, path: &[&str], value: &compat::PatchValue) -> bool {
    compat::write(wildcard_handle::<T>(object), |config| (T::class_of().patch)(config, path, value))
}

impl<T: ReflectedClass> ConfigClassHandler<compat::Object> for WildcardClassHandler<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn instance(&self) -> compat::Shared<compat::Object> {
        compat::shared(Arc::new(self.handler().instance()) as compat::Object)
    }

    fn defaults(&self) -> compat::Shared<compat::Object> {
        compat::shared(Arc::new(self.handler().defaults()) as compat::Object)
    }

    fn config_class(&self) -> compat::ClassOf<compat::Object> {
        let class = self.handler().config_class();
        compat::ClassOf {
            name: class.name,
            new: wildcard_new::<T>,
            to_gson: wildcard_to_gson::<T>,
            read_gson: wildcard_read_gson::<T>,
            patch: wildcard_patch::<T>,
            is_enum: class.is_enum,
            enum_constants: class.enum_constants,
        }
    }

    fn fields(&self) -> Vec<Arc<dyn ConfigField<compat::Object>>> {
        self.handler().fields()
    }

    fn id(&self) -> Identifier {
        self.handler().id()
    }

    fn generate_gui(&self) -> Arc<dyn YetAnotherConfigLib> {
        self.handler().generate_gui()
    }

    fn supports_auto_gen(&self) -> bool {
        self.handler().supports_auto_gen()
    }

    fn load(&self) -> bool {
        self.handler().load()
    }

    fn save(&self) {
        self.handler().save()
    }

    fn serializer(&self) -> Arc<dyn ConfigSerializer<compat::Object>> {
        Arc::new(WildcardSerializer::<T> {
            inner: self.handler().serializer(),
            base: ConfigSerializerBase::new(Arc::new(WildcardClassHandler::<T> { handler: self.handler.clone() })),
        })
    }
}

/// java: the handler's `ConfigSerializer<T>` as `ConfigSerializer<?>` (`ConfigClassHandler<?>.serializer()`)
struct WildcardSerializer<T: 'static> {
    inner: Arc<dyn ConfigSerializer<T>>,
    base: ConfigSerializerBase<compat::Object>,
}

impl<T: ReflectedClass> ConfigSerializer<compat::Object> for WildcardSerializer<T> {
    fn config_serializer_base(&self) -> &ConfigSerializerBase<compat::Object> {
        &self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self.inner.as_any()
    }

    fn save(&self) {
        self.inner.save()
    }

    fn load_safely(&self, buffer_access_map: &BufferAccessMap) -> LoadResult {
        self.inner.load_safely(buffer_access_map)
    }

    fn load(&self) {
        self.inner.load()
    }
}
