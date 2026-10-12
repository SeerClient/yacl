//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.OptionFactoryRegistry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::auto_gen_field::AutoGenField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::boolean::Boolean;
use crate::dev::isxander::yacl3::config::v2::api::autogen::color_field::ColorField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::double_field::DoubleField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::double_slider::DoubleSlider;
use crate::dev::isxander::yacl3::config::v2::api::autogen::dropdown::Dropdown;
use crate::dev::isxander::yacl3::config::v2::api::autogen::enum_cycler::EnumCycler;
use crate::dev::isxander::yacl3::config::v2::api::autogen::float_field::FloatField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::float_slider::FloatSlider;
use crate::dev::isxander::yacl3::config::v2::api::autogen::int_field::IntField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::int_slider::IntSlider;
use crate::dev::isxander::yacl3::config::v2::api::autogen::item_field::ItemField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::label::Label;
use crate::dev::isxander::yacl3::config::v2::api::autogen::list_group::ListGroup;
use crate::dev::isxander::yacl3::config::v2::api::autogen::long_field::LongField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::long_slider::LongSlider;
use crate::dev::isxander::yacl3::config::v2::api::autogen::master_tick_box::MasterTickBox;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::string_field::StringField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::tick_box::TickBox;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::serial_field::SerialField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::boolean_impl::BooleanImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::color_field_impl::ColorFieldImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::double_field_impl::DoubleFieldImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::double_slider_impl::DoubleSliderImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::dropdown_impl::DropdownImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::enum_cycler_impl::EnumCyclerImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::float_field_impl::FloatFieldImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::float_slider_impl::FloatSliderImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::int_field_impl::IntFieldImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::int_slider_impl::IntSliderImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::item_field_impl::ItemFieldImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::label_impl::LabelImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::list_group_impl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::long_field_impl::LongFieldImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::long_slider_impl::LongSliderImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::master_tick_box_impl::MasterTickBoxImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::string_field_impl::StringFieldImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::tick_box_impl::TickBoxImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::yacl_auto_gen_exception::YACLAutoGenException;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::object_from;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::object_to;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::Field;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::FieldType;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ReflectionFieldAccess;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::TypeClass;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER;
use std::any::TypeId;

/// `OptionFactory<?, ?>` as the registry holds it: `factory.createOption(annotation, configField, storage)` with the
/// annotation and the field as Java's erased `Annotation` / `ConfigField<?>`, the option answered as `Option<?>`.
pub type ErasedOptionFactory = Arc<dyn Fn(&compat::Object, &Arc<dyn ConfigField<compat::Object>>, &Arc<dyn OptionAccess>) -> Arc<dyn AnyOption>>;

thread_local! {
    /// java: private static final Map<Class<?>, OptionFactory<?, ?>> factoryMap = new HashMap<>(); - and the
    /// class's `static { registerOptionFactory(..) }` block, which fills it on first use
    static FACTORY_MAP: std::cell::RefCell<std::collections::HashMap<TypeId, ErasedOptionFactory>> =
        std::cell::RefCell::new(OptionFactoryRegistry::static_init());
}

/// java: public class OptionFactoryRegistry
#[derive(Clone, Default)]
pub struct OptionFactoryRegistry {}

impl OptionFactoryRegistry {
    /// java: static { .. } - the 18 built-in factories
    fn static_init() -> std::collections::HashMap<TypeId, ErasedOptionFactory> {
        let mut factory_map: std::collections::HashMap<TypeId, ErasedOptionFactory> = std::collections::HashMap::new();
        factory_map.insert(TypeId::of::<TickBox>(), erase::<TickBox, bool>(Arc::new(TickBoxImpl::new())));
        factory_map.insert(TypeId::of::<Boolean>(), erase::<Boolean, bool>(Arc::new(BooleanImpl::new())));
        factory_map.insert(TypeId::of::<IntSlider>(), erase::<IntSlider, i32>(Arc::new(IntSliderImpl::new())));
        factory_map.insert(TypeId::of::<LongSlider>(), erase::<LongSlider, i64>(Arc::new(LongSliderImpl::new())));
        factory_map.insert(TypeId::of::<FloatSlider>(), erase::<FloatSlider, f32>(Arc::new(FloatSliderImpl::new())));
        factory_map.insert(TypeId::of::<DoubleSlider>(), erase::<DoubleSlider, f64>(Arc::new(DoubleSliderImpl::new())));
        factory_map.insert(TypeId::of::<IntField>(), erase::<IntField, i32>(Arc::new(IntFieldImpl::new())));
        factory_map.insert(TypeId::of::<LongField>(), erase::<LongField, i64>(Arc::new(LongFieldImpl::new())));
        factory_map.insert(TypeId::of::<FloatField>(), erase::<FloatField, f32>(Arc::new(FloatFieldImpl::new())));
        factory_map.insert(TypeId::of::<DoubleField>(), erase::<DoubleField, f64>(Arc::new(DoubleFieldImpl::new())));
        factory_map.insert(
            TypeId::of::<EnumCycler>(),
            erase::<EnumCycler, crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::EnumValue>(Arc::new(EnumCyclerImpl::new())),
        );
        factory_map.insert(TypeId::of::<StringField>(), erase::<StringField, String>(Arc::new(StringFieldImpl::new())));
        factory_map.insert(TypeId::of::<ColorField>(), erase::<ColorField, mcrust::java::awt::color::Color>(Arc::new(ColorFieldImpl::new())));
        factory_map.insert(TypeId::of::<Dropdown>(), erase::<Dropdown, String>(Arc::new(DropdownImpl::new())));
        factory_map.insert(
            TypeId::of::<ItemField>(),
            erase::<ItemField, compat::JavaRef<mcrust::net::minecraft::world::item::item::Item>>(Arc::new(ItemFieldImpl::new())),
        );
        factory_map.insert(
            TypeId::of::<Label>(),
            erase::<Label, mcrust::net::minecraft::network::chat::mutable_component::MutableComponent>(Arc::new(LabelImpl::new())),
        );
        // java: new ListGroupImpl<>() - a raw ListGroupImpl whose element type Java erases; Rust instantiates
        // `ListGroupImpl<X>` per field (the field's generated `list_group`, see list_group_impl::create_erased)
        factory_map.insert(TypeId::of::<ListGroup>(), list_group_impl::erased_factory());
        factory_map.insert(TypeId::of::<MasterTickBox>(), erase::<MasterTickBox, bool>(Arc::new(MasterTickBoxImpl::new())));
        factory_map
    }

    /// java: public static <A extends Annotation, T> void registerOptionFactory(Class<A> annotation, OptionFactory<A, T> factory)
    pub fn register_option_factory<A: Clone + Send + Sync + 'static, T: Clone + Send + Sync + 'static>(
        annotation: &TypeId,
        factory: Arc<dyn OptionFactory<A, T>>,
    ) {
        let erased = erase(factory);
        FACTORY_MAP.with(|factory_map| factory_map.borrow_mut().insert(*annotation, erased));
    }

    /// java: public static <T> Optional<Option<T>> createOption(Field field, ConfigField<T> configField, OptionAccess storage)
    pub fn create_option(
        field: &Field,
        config_field: &Arc<dyn ConfigField<compat::Object>>,
        storage: &Arc<dyn OptionAccess>,
    ) -> Option<Arc<dyn AnyOption>> {
        let annotations: Vec<compat::Object> = FACTORY_MAP.with(|factory_map| {
            let factory_map = factory_map.borrow();
            field
                .get_annotations()
                .into_iter()
                .filter(|annotation| factory_map.contains_key(&std::any::Any::type_id(&**annotation)))
                .collect()
        });

        if annotations.len() != 1 {
            LOGGER.warn(&format!("Found {} option factory annotations on field {}, expected 1", annotations.len(), field));

            if annotations.is_empty() {
                return None;
            }
        }

        let annotation = annotations[0].clone();
        // noinspection unchecked
        let factory: ErasedOptionFactory = FACTORY_MAP.with(|factory_map| {
            factory_map.borrow().get(&std::any::Any::type_id(&*annotation)).cloned().expect("registered above")
        });
        Some(factory(&annotation, config_field, storage))
    }
}

/// `(OptionFactory<Annotation, T>) factory`: the factory over the erased annotation and field. The annotation is
/// cast to `A` (Java's unchecked cast), the field viewed as a `ConfigField<T>`.
fn erase<A: Clone + Send + Sync + 'static, T: Clone + Send + Sync + 'static>(factory: Arc<dyn OptionFactory<A, T>>) -> ErasedOptionFactory {
    Arc::new(move |annotation: &compat::Object, config_field: &Arc<dyn ConfigField<compat::Object>>, storage: &Arc<dyn OptionAccess>| {
        let annotation: A = match (**annotation).downcast_ref::<A>() {
            Some(annotation) => annotation.clone(),
            None => panic!("ClassCastException: an annotation of another class for {}", std::any::type_name::<A>()),
        };
        let typed: Arc<dyn ConfigField<T>> = Arc::new(TypedConfigField::<T>::new(config_field.clone()));
        let option: Arc<dyn YaclOption<T>> = factory.create_option(&annotation, &typed, storage);
        option as Arc<dyn AnyOption>
    })
}

/// `(ConfigField<T>) configField` - a `ConfigField<?>` seen as the `ConfigField<T>` a factory for `T` takes (Java's
/// unchecked generic cast). Every call goes to the field itself (so `access()` follows `setFieldAccess`); values cross
/// as `Object` (`reflection_field_access::object_to` / `object_from`).
pub struct TypedConfigField<T> {
    inner: Arc<dyn ConfigField<compat::Object>>,
    _type: std::marker::PhantomData<fn() -> T>,
}

impl<T> TypedConfigField<T> {
    pub fn new(inner: Arc<dyn ConfigField<compat::Object>>) -> TypedConfigField<T> {
        TypedConfigField { inner, _type: std::marker::PhantomData }
    }

    /// the `ConfigField<?>` this views
    pub fn erased(&self) -> Arc<dyn ConfigField<compat::Object>> {
        self.inner.clone()
    }
}

impl<T: Clone + Send + Sync + 'static> ConfigField<T> for TypedConfigField<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn access(&self) -> Arc<dyn FieldAccess<T>> {
        Arc::new(TypedFieldAccess::<T> { inner: self.inner.access(), _type: std::marker::PhantomData })
    }

    fn default_access(&self) -> Arc<dyn ReadOnlyFieldAccess<T>> {
        Arc::new(TypedReadOnlyFieldAccess::<T> { inner: self.inner.default_access(), _type: std::marker::PhantomData })
    }

    fn parent(&self) -> Arc<dyn ConfigClassHandler<compat::Object>> {
        self.inner.parent()
    }

    fn serial(&self) -> Option<Arc<dyn SerialField>> {
        self.inner.serial()
    }

    fn auto_gen(&self) -> Option<Arc<dyn AutoGenField>> {
        self.inner.auto_gen()
    }
}

/// `(FieldAccess<T>) fieldAccess` (see [`TypedConfigField`])
struct TypedFieldAccess<T> {
    inner: Arc<dyn FieldAccess<compat::Object>>,
    _type: std::marker::PhantomData<fn() -> T>,
}

impl<T: Clone + Send + Sync + 'static> ReadOnlyFieldAccess<T> for TypedFieldAccess<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self.inner.as_any()
    }

    fn get(&self) -> T {
        object_to::<T>(self.inner.get(), self.inner.type_class())
    }

    fn name(&self) -> String {
        self.inner.name()
    }

    fn r#type(&self) -> FieldType {
        self.inner.r#type()
    }

    fn type_class(&self) -> TypeClass {
        self.inner.type_class()
    }

    fn get_annotation(&self, annotation_class: &TypeId) -> Option<compat::Object> {
        self.inner.get_annotation(annotation_class)
    }
}

impl<T: Clone + Send + Sync + 'static> FieldAccess<T> for TypedFieldAccess<T> {
    fn set(&self, value: T) {
        self.inner.set(object_from(value));
    }
}

/// `(ReadOnlyFieldAccess<T>) fieldAccess` (see [`TypedConfigField`])
struct TypedReadOnlyFieldAccess<T> {
    inner: Arc<dyn ReadOnlyFieldAccess<compat::Object>>,
    _type: std::marker::PhantomData<fn() -> T>,
}

impl<T: Clone + Send + Sync + 'static> ReadOnlyFieldAccess<T> for TypedReadOnlyFieldAccess<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self.inner.as_any()
    }

    fn get(&self) -> T {
        object_to::<T>(self.inner.get(), self.inner.type_class())
    }

    fn name(&self) -> String {
        self.inner.name()
    }

    fn r#type(&self) -> FieldType {
        self.inner.r#type()
    }

    fn type_class(&self) -> TypeClass {
        self.inner.type_class()
    }

    fn get_annotation(&self, annotation_class: &TypeId) -> Option<compat::Object> {
        self.inner.get_annotation(annotation_class)
    }
}

/// the field behind a `ConfigField<?>`'s access (`((ReflectionFieldAccess<?>) configField.access()).field()`)
pub(crate) fn reflected_field(config_field: &Arc<dyn ConfigField<compat::Object>>) -> Field {
    let access = config_field.access();
    match access.as_any().downcast_ref::<ReflectionFieldAccess<compat::Object>>() {
        Some(reflection) => reflection.field(),
        None => panic!("ClassCastException: the field access of '{}' is not a ReflectionFieldAccess", access.name()),
    }
}

/// Keeps `YACLAutoGenException` in this module's vocabulary for `ListGroup`'s missing instantiation.
pub(crate) fn no_list_group_instantiation(field: &Field) -> ! {
    YACLAutoGenException::new(format!(
        "'{}': @ListGroup needs the field's element type, which the generated reflection did not record (DeclaredField.list_group)",
        field.get_name()
    ))
    .throw()
}
