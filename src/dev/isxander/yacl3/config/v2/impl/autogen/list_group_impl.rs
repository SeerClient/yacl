//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.ListGroupImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::list_option;
use crate::dev::isxander::yacl3::api::list_option::Builder as _;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::api::option_description;
use crate::dev::isxander::yacl3::api::option_description::Builder as _;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::config::v2::api::autogen::list_group::ControllerFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::list_group::ListGroup;
use crate::dev::isxander::yacl3::config::v2::api::autogen::list_group::ValueFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::option_factory_registry;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::option_factory_registry::ErasedOptionFactory;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::option_factory_registry::TypedConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::yacl_auto_gen_exception::YACLAutoGenException;
use crate::dev::isxander::yacl3::config::v2::r#impl::field_backed_binding::FieldBackedBinding;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ClassRef;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::locale::language::LanguageObj as _;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class ListGroupImpl<T> implements OptionFactory<ListGroup, List<T>>
pub struct ListGroupImpl<T> {
    _type: std::marker::PhantomData<fn() -> T>,
}

impl<T> Clone for ListGroupImpl<T> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<T> Default for ListGroupImpl<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ListGroupImpl<T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { _type: std::marker::PhantomData }
    }
}

impl<T: Clone + PartialEq + Send + Sync + 'static> ListGroupImpl<T> {
    /// java: private OptionDescription description(ConfigField<List<T>> field)
    fn description(&self, field: &Arc<dyn ConfigField<Vec<T>>>) -> Arc<dyn OptionDescription> {
        let mut builder = option_description::create_builder();

        let mut key: String = self.get_translation_key(field, Some("desc"));
        if language::get_instance().has(&key) {
            builder = builder.text(vec![component::translatable(&key)]);
        } else {
            key.push('.');
            // java: int i = 0; while (Language.getInstance().has(key + i++)) builder.text(Component.translatable(key + i));
            // - the key tested is `i`, the line added `i + 1`, as written
            let mut i: i32 = 0;
            loop {
                let tested = format!("{}{}", key, i);
                i += 1;
                if !language::get_instance().has(&tested) {
                    break;
                }
                builder = builder.text(vec![component::translatable(&format!("{}{}", key, i))]);
            }
        }

        let access = field.access();
        let mut image_path: String = format!("textures/yacl3/{}/{}.webp", field.parent().id().path(), access.name());
        image_path = compat::regex::replace_all(&image_path.to_lowercase(), "[^a-z0-9/._:-]", "_");
        let image_location: Identifier = YACLPlatform::rl_2(field.parent().id().namespace(), &image_path);
        if compat::minecraft().get_resource_manager().get_resource(&image_location).is_some() {
            builder = builder.webp_image(image_location);
        }

        builder.build()
    }

    /// java: private ListGroup.ValueFactory<T> createValueFactory(Class<? extends ListGroup.ValueFactory<T>> clazz)
    fn create_value_factory(&self, clazz: &ClassRef) -> Arc<dyn ValueFactory<T>> {
        // java: constructor = clazz.getConstructor(); - NoSuchMethodException
        let Some(constructor) = clazz.new_instance else {
            YACLAutoGenException::new(format!(
                "Could not find no-args constructor for `valueFactory` on '{}' for @ListGroup field.",
                clazz.get_name()
            ))
            .throw();
        };
        // java: return constructor.newInstance(); - InvocationTargetException | InstantiationException | IllegalAccessException
        let instance = match std::panic::catch_unwind(constructor) {
            Ok(instance) => instance,
            Err(_) => YACLAutoGenException::new(format!(
                "Couldn't invoke no-args constructor for `valueFactory` on '{}' for @ListGroup field.",
                clazz.get_name()
            ))
            .throw(),
        };
        match (*instance).downcast_ref::<compat::RenderThread<Arc<dyn ValueFactory<T>>>>() {
            Some(factory) => factory.get().clone(),
            None => panic!("ClassCastException: {} is not a ListGroup.ValueFactory of the field's element type", clazz.get_name()),
        }
    }

    /// java: private ListGroup.ControllerFactory<T> createControllerFactory(Class<? extends ListGroup.ControllerFactory<T>> clazz)
    fn create_controller_factory(&self, clazz: &ClassRef) -> Arc<dyn ControllerFactory<T>> {
        let Some(constructor) = clazz.new_instance else {
            YACLAutoGenException::new(format!(
                "Could not find no-args constructor on `controllerFactory`, '{}' for @ListGroup field.",
                clazz.get_name()
            ))
            .throw();
        };
        let instance = match std::panic::catch_unwind(constructor) {
            Ok(instance) => instance,
            Err(_) => YACLAutoGenException::new(format!(
                "Couldn't invoke no-args constructor on `controllerFactory`, '{}' for @ListGroup field.",
                clazz.get_name()
            ))
            .throw(),
        };
        match (*instance).downcast_ref::<compat::RenderThread<Arc<dyn ControllerFactory<T>>>>() {
            Some(factory) => factory.get().clone(),
            None => panic!("ClassCastException: {} is not a ListGroup.ControllerFactory of the field's element type", clazz.get_name()),
        }
    }

    /// java: private String getTranslationKey(ConfigField<List<T>> field, @Nullable String suffix)
    fn get_translation_key(&self, field: &Arc<dyn ConfigField<Vec<T>>>, suffix: Option<&str>) -> String {
        let mut key: String = format!("yacl3.config.{}.{}", field.parent().id(), field.access().name());
        if let Some(suffix) = suffix {
            key.push_str(&format!(".{}", suffix));
        }
        key
    }
}

impl<T: Clone + PartialEq + Send + Sync + 'static> OptionFactory<ListGroup, Vec<T>> for ListGroupImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<List<T>> createOption(ListGroup annotation, ConfigField<List<T>> field, OptionAccess optionAccess)
    fn create_option(&self, annotation: &ListGroup, field: &Arc<dyn ConfigField<Vec<T>>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<Vec<T>>> {
        if field.auto_gen().expect("NoSuchElementException: No value present").group().is_some() {
            YACLAutoGenException::new(format!(
                "@ListGroup fields ('{}') cannot be inside a group as lists act as groups.",
                field.access().name()
            ))
            .throw();
        }

        let value_factory: Arc<dyn ValueFactory<T>> = self.create_value_factory(&annotation.value_factory());
        let controller_factory: Arc<dyn ControllerFactory<T>> = self.create_controller_factory(&annotation.controller_factory());

        let option = list_option::create_builder::<T>()
            .name(component::translatable(&self.get_translation_key(field, None)))
            .description(self.description(field))
            .initial(move || value_factory.provide_new_value())
            .controller({
                let (annotation, field, option_access) = (annotation.clone(), field.clone(), option_access.clone());
                move |opt: Arc<dyn YaclOption<T>>| controller_factory.create_controller(&annotation, &field, &option_access, &opt)
            })
            .binding(Arc::new(FieldBackedBinding::new(field.access(), field.default_access())))
            .minimum_number_of_entries(annotation.min_entries())
            .maximum_number_of_entries(if annotation.max_entries() == 0 { i32::MAX } else { annotation.max_entries() })
            .insert_entries_at_end(annotation.add_entries_to_bottom())
            .build();
        option
    }
}

/// `ListGroupImpl<X>.createOption` over the erased field: what a `List<X>` field annotated `@ListGroup` records as
/// its `DeclaredField::list_group` (`Some(list_group_impl::create_erased::<X>)`). Java registers one raw
/// `ListGroupImpl` and erases `X`; Rust needs `X` to read the list, so the field names its instantiation.
pub fn create_erased<T: Clone + PartialEq + Send + Sync + 'static>(
    annotation: &compat::Object,
    config_field: &Arc<dyn ConfigField<compat::Object>>,
    storage: &Arc<dyn OptionAccess>,
) -> Arc<dyn AnyOption> {
    let annotation: ListGroup = match (**annotation).downcast_ref::<ListGroup>() {
        Some(annotation) => annotation.clone(),
        None => panic!("ClassCastException: not a ListGroup annotation"),
    };
    let field: Arc<dyn ConfigField<Vec<T>>> = Arc::new(TypedConfigField::<Vec<T>>::new(config_field.clone()));
    let option: Arc<dyn YaclOption<Vec<T>>> = ListGroupImpl::<T>::new().create_option(&annotation, &field, storage);
    option
}

/// `registerOptionFactory(ListGroup.class, new ListGroupImpl<>())`: the registry's entry, which runs the field's own
/// instantiation of `ListGroupImpl`.
pub fn erased_factory() -> ErasedOptionFactory {
    Arc::new(|annotation: &compat::Object, config_field: &Arc<dyn ConfigField<compat::Object>>, storage: &Arc<dyn OptionAccess>| {
        let field = option_factory_registry::reflected_field(config_field);
        match field.list_group() {
            Some(create_option) => create_option(annotation, config_field, storage),
            None => option_factory_registry::no_list_group_instantiation(&field),
        }
    })
}
