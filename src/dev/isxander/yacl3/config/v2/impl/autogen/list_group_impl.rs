//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.ListGroupImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::list_option;
use crate::dev::isxander::yacl3::api::option_description;
use crate::dev::isxander::yacl3::api::option_description::Builder;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use crate::dev::isxander::yacl3::config::v2::api::autogen::auto_gen_field::AutoGenField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::list_group::ControllerFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::list_group::ListGroup;
use crate::dev::isxander::yacl3::config::v2::api::autogen::list_group::ValueFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::field_backed_binding::FieldBackedBinding;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;
use crate::dev::isxander::yacl3::api::list_option::Builder as _;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler as _;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess as _;

/// java: public class ListGroupImpl<T> implements OptionFactory<ListGroup, List<T>>
#[derive(Clone)]
pub struct ListGroupImpl<T> {}

impl<T: Clone + 'static> ListGroupImpl<T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private OptionDescription description(ConfigField<List<T>> field)
    pub fn description(&self, field: &compat::Shared<dyn ConfigField<Vec<T>>>) -> compat::Shared<dyn OptionDescription> {
        let mut builder: compat::Shared<dyn Builder> = option_description::create_builder();
        let mut key: String = self.get_translation_key(field, Some("desc".to_string()));
        if language::get_instance().has(&key) {
            ({ let __f = compat::dispatch(&builder, |__t| __t.text_collection()); __f(&builder, &component::translatable(&key)) });
        } else {
            key.push_str(".");
            let mut i: i32 = 0;
            while language::get_instance().has(&format!("{}{}", key, { let __v = i; i += 1; __v })) {
                ({ let __f = compat::dispatch(&builder, |__t| __t.text_collection()); __f(&builder, &component::translatable(&format!("{}{}", key, i))) });
            }
        }
        let mut image_path: String = format!("textures/yacl3/{}/{}.webp", field.lock().borrow().parent().lock().borrow().id().path(), field.lock().borrow().access().lock().borrow().name());
        image_path = compat::regex::replace_all(&image_path.to_lowercase(), "[^a-z0-9/._:-]", "_");
        let mut image_location: Identifier = YACLPlatform::rl_2(field.lock().borrow().parent().lock().borrow().id().namespace(), &image_path);
        if compat::minecraft().get_resource_manager().get_resource(&image_location).is_some() {
            ({ let __f = compat::dispatch(&builder, |__t| __t.webp_image()); __f(&builder, &image_location) });
        }
        return ({ let __f = compat::dispatch(&builder, |__t| __t.build()); __f(&builder) });
    }

    /// java: private ListGroup.ValueFactory<T> createValueFactory(Class<? extends ListGroup.ValueFactory<T>> clazz)
    pub fn create_value_factory(&self, clazz: &compat::ClassOf<compat::Shared<dyn ValueFactory<T>>>) -> compat::Shared<dyn ValueFactory<T>> {
        let mut constructor: crate::unresolved::Constructor;
        // java: try {..} catch (NoSuchMethodException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `.getConstructor(..)` on a Class
            //   java: constructor = clazz.getConstructor();
        }));
        if let Err(e) = __result {
            panic!("YACLAutoGenException: {}", format!("Could not find no-args constructor for `valueFactory` on '{}' for @ListGroup field.", clazz.get_name()).to_string());
        }
        // java: try {..} catch (InvocationTargetException | InstantiationException | IllegalAccessException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `.newInstance(..)` on a Constructor
            //   java: return constructor.newInstance();
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                panic!("YACLAutoGenException: {}", format!("Couldn't invoke no-args constructor for `valueFactory` on '{}' for @ListGroup field.", clazz.get_name()).to_string());
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private ListGroup.ControllerFactory<T> createControllerFactory(Class<? extends ListGroup.ControllerFactory<T>> clazz)
    pub fn create_controller_factory(&self, clazz: &compat::ClassOf<compat::Shared<dyn ControllerFactory<T>>>) -> compat::Shared<dyn ControllerFactory<T>> {
        let mut constructor: crate::unresolved::Constructor;
        // java: try {..} catch (NoSuchMethodException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `.getConstructor(..)` on a Class
            //   java: constructor = clazz.getConstructor();
        }));
        if let Err(e) = __result {
            panic!("YACLAutoGenException: {}", format!("Could not find no-args constructor on `controllerFactory`, '{}' for @ListGroup field.", clazz.get_name()).to_string());
        }
        // java: try {..} catch (InvocationTargetException | InstantiationException | IllegalAccessException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `.newInstance(..)` on a Constructor
            //   java: return constructor.newInstance();
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                panic!("YACLAutoGenException: {}", format!("Couldn't invoke no-args constructor on `controllerFactory`, '{}' for @ListGroup field.", clazz.get_name()).to_string());
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private String getTranslationKey(ConfigField<List<T>> field, @Nullable String suffix)
    pub fn get_translation_key(&self, field: &compat::Shared<dyn ConfigField<Vec<T>>>, mut suffix: Option<String>) -> String {
        // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // rust: let mut key: String = format!("yacl3.config.{}.{}", field.lock().borrow().parent().lock().borrow().id().fmt(), field.lock().borrow().access().lock().borrow().name());
        let mut key: String = todo!("fabric2seer: the initialiser of `key` (TODO above)");
        if let Some(suffix) = &suffix {
            key.push_str(&format!(".{}", suffix).to_string());
        }
        return key;
    }

}

impl<T: Clone + 'static> OptionFactory<crate::dev::isxander::yacl3::config::v2::api::autogen::list_group::ListGroup, Vec<T>> for ListGroupImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<List<T>> createOption(ListGroup annotation, ConfigField<List<T>> field, OptionAccess optionAccess)
    fn create_option(&self, annotation: &ListGroup, field: &compat::Shared<dyn ConfigField<Vec<T>>>, option_access: &compat::Shared<dyn OptionAccess>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Vec<T>>> {
        if ({ let __r = &field.lock().borrow().auto_gen().unwrap(); let __f = compat::dispatch(__r, |__t| __t.group()); __f(__r) }).is_some() {
            panic!("YACLAutoGenException: {}", format!("@ListGroup fields ('{}') cannot be inside a group as lists act as groups.", field.lock().borrow().access().lock().borrow().name()).to_string());
        }
        let mut value_factory: compat::Shared<dyn ValueFactory<T>> = self.create_value_factory(&annotation.value_factory());
        let mut controller_factory: compat::Shared<dyn ControllerFactory<T>> = self.create_controller_factory(&annotation.controller_factory());
        return (({ let (__a0,) = (annotation.add_entries_to_bottom(),); ({ let (__a0,) = ((if annotation.max_entries() == 0 { i32::MAX } else { annotation.max_entries() }),); ({ let (__a0,) = (annotation.min_entries(),); ({ let (__a0,) = ((compat::shared(FieldBackedBinding::new(field.lock().borrow().access(), field.lock().borrow().default_access())) as compat::Shared<dyn Binding<compat::Object>>),); ({ let (__a0,) = (Arc::new(move |opt| controller_factory.lock().borrow().create_controller(annotation, field, option_access, &opt)),); ({ let (__a0,) = (Arc::new(move || value_factory.lock().borrow().provide_new_value()),); ({ let (__a0,) = (self.description(field),); ({ let (__a0,) = (component::translatable(&self.get_translation_key(field, None)),); list_option::create_builder().lock().borrow_mut().name(__a0) }.lock().borrow_mut()).description(__a0) }.lock().borrow_mut()).initial(__a0) }.lock().borrow_mut()).controller(__a0) }.lock().borrow_mut()).binding(__a0) }.lock().borrow_mut()).minimum_number_of_entries(__a0) }.lock().borrow_mut()).maximum_number_of_entries(__a0) }.lock().borrow_mut()).insert_entries_at_end(__a0) }.lock().borrow_mut()).build() as compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>);
    }

}
