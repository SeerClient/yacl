//! Ported from dev.isxander.yacl3.config.v2.impl.ConfigClassHandlerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category;
use crate::dev::isxander::yacl3::api::config_category::Builder;
use crate::dev::isxander::yacl3::api::option_addable::OptionAddable;
use crate::dev::isxander::yacl3::api::option_description;
use crate::dev::isxander::yacl3::api::option_group;
use crate::dev::isxander::yacl3::api::yet_another_config_lib;
use crate::dev::isxander::yacl3::api::yet_another_config_lib::YetAnotherConfigLib;
use crate::dev::isxander::yacl3::config::v2::api::autogen::auto_gen_field::AutoGenField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::ConfigSerializer;
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::LoadResult;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::serial_entry::SerialEntry;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::option_access_impl::OptionAccessImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::option_factory_registry::OptionFactoryRegistry;
use crate::dev::isxander::yacl3::config::v2::r#impl::config_field_impl::ConfigFieldImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ReflectionFieldAccess;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::java::lang::throwable::Throwable;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use crate::dev::isxander::yacl3::api::option_group::Builder as _;
use crate::dev::isxander::yacl3::api::yet_another_config_lib::Builder as _;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess as _;

/// java: public class ConfigClassHandlerImpl<T> implements ConfigClassHandler<T>
pub struct ConfigClassHandlerImpl<T> {
    /// java: private final Class<T> configClass;
    pub config_class: compat::ClassOf<T>,
    /// java: private final Identifier id;
    pub id: Option<Identifier>,
    /// java: private final boolean supportsAutoGen;
    pub supports_auto_gen: bool,
    /// java: private final ConfigSerializer<T> serializer;
    pub serializer: Option<compat::Shared<dyn ConfigSerializer<T>>>,
    /// java: private final ConfigFieldImpl<?>[] fields;
    pub fields: Vec<compat::Shared<ConfigFieldImpl<compat::Object>>>,
    /// java: private T instance;
    pub instance: Option<T>,
    /// java: private final T defaults;
    pub defaults: Option<T>,
    /// java: private final Constructor<T> noArgsConstructor;
    pub no_args_constructor: Option<crate::unresolved::Constructor>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl<T: Clone + 'static> ConfigClassHandlerImpl<T> {
    /// java: public ConfigClassHandlerImpl(Class<T> configClass, Identifier id, Function<ConfigClassHandler<T>, ConfigSerializer<T>> serializerFactory)
    pub fn new(mut config_class: compat::ClassOf<T>, mut id: Identifier, serializer_factory: Arc<dyn Fn(compat::Shared<dyn ConfigClassHandler<T>>) -> compat::Shared<dyn ConfigSerializer<T>>>) -> compat::Shared<Self> {
        let __init_config_class = config_class.clone();
        let __init_id = Some(id.clone());
        let __init_supports_auto_gen = (true /* java: id != null: never null here */ && YACLPlatform::get_environment().is_client());
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            config_class: __init_config_class,
            id: __init_id,
            supports_auto_gen: __init_supports_auto_gen,
            serializer: None,
            fields: Default::default(),
            instance: None,
            defaults: None,
            no_args_constructor: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // java: try {..} catch (NoSuchMethodException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            { let __v = Some(config_class); compat::write(&__this, |__t| __t.no_args_constructor = __v); }
        }));
        if let Err(e) = __result {
            panic!("YACLAutoGenException: {}", format!("Failed to find no-args constructor for config class {}.", config_class.get_name()).to_string());
        }
        { let __v = Some(ConfigClassHandlerImpl::create_new_object(&__this)); compat::write(&__this, |__t| __t.instance = __v); }
        { let __v = Some(ConfigClassHandlerImpl::create_new_object(&__this)); compat::write(&__this, |__t| __t.defaults = __v); }
        // TODO(fabric2seer): `.getDeclaredFields(..)` on a Class
        //   java: detectOldAnnotation(configClass.getDeclaredFields());
        { let __v = ConfigClassHandlerImpl::discover_fields(&__this); compat::write(&__this, |__t| __t.fields = __v); }
        { let __v = Some((serializer_factory)(__this)); compat::write(&__this, |__t| __t.serializer = __v); }
        __this
    }

    /// java: private ConfigFieldImpl<?>[] discoverFields()
    pub fn discover_fields(__this: &compat::Shared<Self>) -> Vec<compat::Shared<ConfigFieldImpl<compat::Object>>> {
        // TODO(fabric2seer): `.getAnnotation(..)` on a Class
        //   java: SerialEntry classSerialEntry = configClass.getAnnotation(SerialEntry.class);
        let mut class_serial_entry: SerialEntry = todo!("fabric2seer: the initialiser of `classSerialEntry` (TODO above)");
        let mut class_has_serial_entry: bool = true /* java: class_serial_entry != null: never null here */;
        if class_has_serial_entry {
            if !("" == class_serial_entry.value()) {
                panic!("IllegalArgumentException: {}", format!("SerialEntry on class '{}' must not have a value. Only `required` and `nullable` are permitted parameters on classes.", __this.lock().borrow().config_class.get_name()).to_string());
            }
            if !("" == class_serial_entry.comment()) {
                panic!("IllegalArgumentException: {}", format!("SerialEntry on class '{}' must not have a comment. Only `required` and `nullable` are permitted parameters on classes.", __this.lock().borrow().config_class.get_name()).to_string());
            }
        }
        // TODO(fabric2seer): `.getDeclaredFields(..)` on a Class
        //   java: return Arrays.stream(configClass.getDeclaredFields())
        //   java:                 .peek(field -> field.setAccessible(true))
        //   java:                 .filter(field -> classHasSerialEntry || field.isAnnotationPresent(SerialEntry.class) || field.isAnnotationPresent(AutoGen.class))
        //   java:                 .map(field -> new ConfigFieldImpl<>(
        //   java:                         new ReflectionFieldAccess<>(field, instance),
        //   java:                         new ReflectionFieldAccess<>(field, defaults),
        //   java:                         this,
        //   java:                         field.getAnnotation(SerialEntry.class),
        //   java:                         classSerialEntry,
        //   java:                         field.getAnnotation(AutoGen.class)
        //   java:                 ))
        //   java:                 .toArray(ConfigFieldImpl[]::new);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private <U> Option<U> createOption(ConfigField<U> configField, OptionAccess storage)
    pub fn create_option<U: 'static>(__this: &compat::Shared<Self>, config_field: &compat::Shared<dyn ConfigField<U>>, storage: &compat::Shared<dyn OptionAccess>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<U>> {
        return OptionFactoryRegistry::create_option(&(config_field.lock().borrow().access().lock().borrow().as_any().downcast_ref::<ReflectionFieldAccess<compat::Object>>().unwrap()).field(), config_field, storage).expect("Optional.orElseThrow");
    }

    /// java: private T createNewObject()
    pub fn create_new_object(__this: &compat::Shared<Self>) -> T {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `.newInstance(..)` on a Constructor
            //   java: return noArgsConstructor.newInstance();
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                panic!("YACLAutoGenException: {}", format!("Failed to create instance of config class '{}' with no-args constructor.", __this.lock().borrow().config_class.get_name()).to_string());
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private void detectOldAnnotation(Field[] fields)
    pub fn detect_old_annotation(__this: &compat::Shared<Self>, fields: &Vec<crate::unresolved::Field>) {
        // TODO(fabric2seer): `.isAnnotationPresent(..)` on a Field
        //   java: boolean hasOldConfigEntry = Arrays.stream(fields)
        //   java:                 .anyMatch(field -> field.isAnnotationPresent(ConfigEntry.class));
        let mut has_old_config_entry: bool = todo!("fabric2seer: the initialiser of `hasOldConfigEntry` (TODO above)");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(!hasOldConfigEntry, "At least one field in %s is still annotated with the deprecated @ConfigEntry annotation. This is incorrect. Use @SerialEntry.".formatted(configClass.getName()));
    }

}

impl<T: Clone + 'static> ConfigClassHandler<T> for ConfigClassHandlerImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public T instance()
    fn instance(&self) -> T {
        return self.instance.clone().unwrap();
    }

    /// java: @Override public T defaults()
    fn defaults(&self) -> T {
        return self.defaults.clone().unwrap();
    }

    /// java: @Override public Class<T> configClass()
    fn config_class(&self) -> compat::ClassOf<T> {
        return self.config_class.clone();
    }

    /// java: @Override public ConfigFieldImpl<?>[] fields()
    fn fields(&self) -> Vec<compat::Shared<ConfigFieldImpl<compat::Object>>> {
        return self.fields.clone();
    }

    /// java: @Override public Identifier id()
    fn id(&self) -> Identifier {
        return self.id.clone().unwrap();
    }

    /// java: @Override public boolean supportsAutoGen()
    fn supports_auto_gen(&self) -> bool {
        return self.supports_auto_gen;
    }

    /// java: @Override public YetAnotherConfigLib generateGui()
    fn generate_gui(&self) -> compat::Shared<dyn YetAnotherConfigLib> {
        if !self.supports_auto_gen() {
            panic!("YACLAutoGenException: {}", "Auto GUI generation is not supported for this config class. You either need to enable it in the builder or you are attempting to create a GUI in a dedicated server environment.");
        }
        let mut has_auto_gen_fields: bool = self.fields().iter().cloned().any(|field| field.lock().borrow().auto_gen().is_some());
        if !has_auto_gen_fields {
            panic!("YACLAutoGenException: {}", "No fields in this config class are annotated with @AutoGen. You must annotate at least one field with @AutoGen to generate a GUI.");
        }
        let mut storage: compat::Shared<OptionAccessImpl> = OptionAccessImpl::new();
        let mut categories: std::collections::HashMap<String, CategoryAndGroups> = std::collections::HashMap::new();
        for config_field in self.fields() {
            if let Some(auto_gen) = config_field.lock().borrow().auto_gen() {
                // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
                // rust: let groups = categories.entry(({ let __f = compat::dispatch(&auto_gen, |__t| __t.category()); __f(&auto_gen) })).or_insert_with_key(|k| (|k| CategoryAndGroups::new(({ let __r = &config_category::create_builder(); let __f = compat::dispatch(__r, |__t| __t.name()); __f(__r, component::translatable(&format!("yacl3.config.{}.category.{}", self.id().fmt(), k))) }), std::collections::HashMap::new()))(k.clone()));
                let mut groups: CategoryAndGroups = todo!("fabric2seer: the initialiser of `groups` (TODO above)");
                let mut group: compat::Shared<dyn OptionAddable> = groups.groups().entry(({ let __f = compat::dispatch(&auto_gen, |__t| __t.group()); __f(&auto_gen) }).unwrap_or("".to_string())).or_insert_with_key(|k| (|k| {
                    if k.is_empty() {
                        return groups.category();
                    }
                    // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
                    // rust: let mut group_key: String = format!("yacl3.config.{}.category.{}.group.{}", self.id().fmt(), ({ let __f = compat::dispatch(&auto_gen, |__t| __t.category()); __f(&auto_gen) }), k);
                    let mut group_key: String = todo!("fabric2seer: the initialiser of `groupKey` (TODO above)");
                    let mut group_builder: compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder> = ({ let __r = &option_group::create_builder(); let __f = compat::dispatch(__r, |__t| __t.name()); __f(__r, component::translatable(&group_key)) });
                    let mut description_key: String = format!("{}.desc", group_key);
                    if language::get_instance().has(&description_key) {
                        ({ let __f = compat::dispatch(&group_builder, |__t| __t.description()); __f(&group_builder, option_description::of(&vec![component::translatable(&description_key)])) });
                    }
                    return group_builder;
                })(k.clone())).clone();
                let mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>;
                // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    option = ConfigClassHandlerImpl::create_option(&self.self_ref.upgrade().unwrap(), &config_field, &(storage.clone() as compat::Shared<dyn OptionAccess>));
                }));
                if let Err(e) = __result {
                    panic!("YACLAutoGenException: {}", format!("Failed to create option for field '{}'", config_field.lock().borrow().access().lock().borrow().name()).to_string());
                }
                OptionAccessImpl::put_option(&storage, config_field.lock().borrow().access().lock().borrow().name(), option.clone());
                ({ let __f = compat::dispatch(&group, |__t| __t.option()); __f(&group, &option) });
            }
        }
        OptionAccessImpl::check_bad_operations(&storage);
        for __a0 in categories.values().cloned() {
            __a0.finalise_groups();
        }
        // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // rust: let mut yacl_builder: compat::Shared<dyn crate::dev::isxander::yacl3::api::yet_another_config_lib::Builder> = ({ let __r = &({ let __r = &yet_another_config_lib::create_builder(); let __f = compat::dispatch(__r, |__t| __t.save()); __f(__r, Arc::new(move || { self.serializer().lock().borrow().save(); })) }); let __f = compat::dispatch(__r, |__t| __t.title()); __f(__r, component::translatable(&format!("yacl3.config.{}.title", self.id().fmt()))) });
        let mut yacl_builder: compat::Shared<dyn crate::dev::isxander::yacl3::api::yet_another_config_lib::Builder> = todo!("fabric2seer: the initialiser of `yaclBuilder` (TODO above)");
        for category in categories.values().cloned() {
            ({ let __f = compat::dispatch(&yacl_builder, |__t| __t.category()); __f(&yacl_builder, ({ let __r = &category.category(); let __f = compat::dispatch(__r, |__t| __t.build()); __f(__r) })) });
        }
        return ({ let __f = compat::dispatch(&yacl_builder, |__t| __t.build()); __f(&yacl_builder) });
    }

    /// java: @Override public ConfigSerializer<T> serializer()
    fn serializer(&self) -> compat::Shared<dyn ConfigSerializer<T>> {
        return self.serializer.clone().unwrap();
    }

    /// java: @Override public boolean load()
    fn load(&mut self) -> bool {
        let mut new_instance: T = ConfigClassHandlerImpl::create_new_object(&self.self_ref.upgrade().unwrap());
        // TODO(fabric2seer): `new SimpleImmutableEntry(..)`
        //   java: Map<ConfigFieldImpl<?>, ReflectionFieldAccess<?>> accessBufferImpl = Arrays.stream(fields())
        //   java:                 .map(field -> new AbstractMap.SimpleImmutableEntry<>(
        //   java:                         field,
        //   java:                         new ReflectionFieldAccess<>(field.access().field(), newInstance)
        //   java:                 ))
        //   java:                 .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
        let mut access_buffer_impl: std::collections::HashMap<compat::Shared<ConfigFieldImpl<compat::Object>>, ReflectionFieldAccess<compat::Object>> = todo!("fabric2seer: the initialiser of `accessBufferImpl` (TODO above)");
        // TODO(fabric2seer): static field Map.Entry
        //   java: Map<ConfigField<?>, FieldAccess<?>> accessBuffer = accessBufferImpl.entrySet().stream()
        //   java:                 .collect(Collectors.toMap(Map.Entry::getKey, Map.Entry::getValue));
        let mut access_buffer: std::collections::HashMap<compat::Shared<dyn ConfigField<compat::Object>>, compat::Shared<dyn FieldAccess<compat::Object>>> = todo!("fabric2seer: the initialiser of `accessBuffer` (TODO above)");
        let mut load_result: LoadResult = LoadResult::Failure;
        let mut error: Option<Throwable> = None;
        // java: try {..} catch (Throwable e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            load_result = self.serializer().lock().borrow().load_safely(&access_buffer);
        }));
        if let Err(e) = __result {
            error = Some(e.clone());
        }
        // TODO(fabric2seer): a `case` that falls through into the next one
        //   java: switch (loadResult) {
        //   java:             case DIRTY:
        //   java:             case SUCCESS:
        //   java:                 // replace the instance with the newly created one
        //   java:                 this.instance = newInstance;
        //   java:                 for (ConfigFieldImpl<?> field : fields()) {
        //   java:                     // update the field accesses to point to the correct object
        //   java:                     ((ConfigFieldImpl<Object>) field).setFieldAccess((ReflectionFieldAccess<Object>) accessBufferImpl.get(field));
        //   java:                 }
        //   java:
        //   java:                 if (loadResult == ConfigSerializer.LoadResult.DIRTY) {
        //   java:                     // if the load result is dirty, we need to save the config again
        //   java:                     this.save();
        //   java:                 }
        //   java:             case NO_CHANGE:
        //   java:                 return true;
        //   java:             case FAILURE:
        //   java:                 YACLConstants.LOGGER.error(
        //   java:                         "Unsuccessful load of config class '{}'. The load will be abandoned and config remains unchanged.",
        //   java:                         configClass.getSimpleName(), error
        //   java:                 );
        //   java:         }
        return false;
    }

    /// java: @Override public void save()
    fn save(&self) {
        self.serializer().lock().borrow().save();
    }

}

/// java: public static class BuilderImpl<T> implements Builder<T>
pub struct BuilderImpl<T> {
    /// java: private final Class<T> configClass;
    pub config_class: compat::ClassOf<T>,
    /// java: private Identifier id;
    pub id: Option<Identifier>,
    /// java: private Function<ConfigClassHandler<T>, ConfigSerializer<T>> serializerFactory;
    pub serializer_factory: Option<Arc<dyn Fn(compat::Shared<dyn ConfigClassHandler<T>>) -> compat::Shared<dyn ConfigSerializer<T>>>>,
}

impl<T: Clone + 'static> BuilderImpl<T> {
    /// java: public BuilderImpl(Class<T> configClass)
    pub fn new(mut config_class: compat::ClassOf<T>) -> Self {
        let __init_config_class = config_class.clone();
        let mut this = Self {
            config_class: __init_config_class,
            id: None,
            serializer_factory: None,
        };
        this
    }

}

impl<T: Clone + 'static> crate::dev::isxander::yacl3::config::v2::api::config_class_handler::Builder<T> for BuilderImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder<T> id(Identifier id)
    fn id(&mut self, mut id: Identifier) -> compat::Shared<dyn crate::dev::isxander::yacl3::config::v2::api::config_class_handler::Builder<T>> {
        self.id = Some(id.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> serializer(Function<ConfigClassHandler<T>, ConfigSerializer<T>> serializerFactory)
    fn serializer(&mut self, serializer_factory: Arc<dyn Fn(compat::Shared<dyn ConfigClassHandler<T>>) -> compat::Shared<dyn ConfigSerializer<T>>>) -> compat::Shared<dyn crate::dev::isxander::yacl3::config::v2::api::config_class_handler::Builder<T>> {
        self.serializer_factory = Some(serializer_factory.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ConfigClassHandler<T> build()
    fn build(&self) -> compat::Shared<dyn ConfigClassHandler<T>> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(serializerFactory, "serializerFactory must not be null");
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(configClass, "configClass must not be null");
        return (ConfigClassHandlerImpl::new(self.config_class.clone(), self.id.clone().unwrap(), self.serializer_factory.clone().unwrap()) as compat::Shared<dyn ConfigClassHandler<compat::Object>>);
    }

}

/// java: private record CategoryAndGroups(ConfigCategory.Builder category, Map<String, OptionAddable> groups)
#[derive(Clone, Debug)]
pub struct CategoryAndGroups {
    /// java: ConfigCategory.Builder category
    pub category: compat::Shared<dyn Builder>,
    /// java: Map<String, OptionAddable> groups
    pub groups: std::collections::HashMap<String, compat::Shared<dyn OptionAddable>>,
}

impl CategoryAndGroups {
    /// java: the canonical record constructor
    pub fn new(mut category: compat::Shared<dyn Builder>, mut groups: std::collections::HashMap<String, compat::Shared<dyn OptionAddable>>) -> Self {
        Self { category, groups }
    }

    /// java: the record accessor `category()`
    pub fn category(&self) -> compat::Shared<dyn Builder> {
        self.category.clone()
    }

    /// java: the record accessor `groups()`
    pub fn groups(&self) -> std::collections::HashMap<String, compat::Shared<dyn OptionAddable>> {
        self.groups.clone()
    }

    /// java: private void finaliseGroups()
    pub fn finalise_groups(&self) {
        for (name, group) in self.groups.iter() {
            if let Some(group_builder) = group.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn crate::dev::isxander::yacl3::api::option_group::Builder>>() {
                ({ let __f = compat::dispatch(&self.category, |__t| __t.group()); __f(&self.category, ({ let __f = compat::dispatch(group_builder, |__t| __t.build()); __f(group_builder) })) });
            }
        }
    }

}
