//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.OptionFactoryRegistry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
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
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
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
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::list_group_impl::ListGroupImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::long_field_impl::LongFieldImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::long_slider_impl::LongSliderImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::master_tick_box_impl::MasterTickBoxImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::string_field_impl::StringFieldImpl;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::tick_box_impl::TickBoxImpl;

/// java: private static final Map<Class<?>, OptionFactory<?, ?>> factoryMap = new HashMap<>();
pub static FACTORY_MAP: compat::MainThread<std::collections::HashMap<compat::ClassOf<compat::Object>, compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>>> = compat::MainThread::new(|| std::collections::HashMap::new());

/// java: public class OptionFactoryRegistry
#[derive(Clone)]
pub struct OptionFactoryRegistry {}

impl OptionFactoryRegistry {
    /// java: static { .. } - Java runs it when the class is first used; every static method and
    /// constructor calls this first.
    fn clinit() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<TickBox>(), (compat::shared(TickBoxImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<bool>(), (compat::shared(BooleanImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<IntSlider>(), (compat::shared(IntSliderImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<LongSlider>(), (compat::shared(LongSliderImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<FloatSlider>(), (compat::shared(FloatSliderImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<DoubleSlider>(), (compat::shared(DoubleSliderImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<IntField>(), (compat::shared(IntFieldImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<LongField>(), (compat::shared(LongFieldImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<FloatField>(), (compat::shared(FloatFieldImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<DoubleField>(), (compat::shared(DoubleFieldImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<EnumCycler>(), (compat::shared(EnumCyclerImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<StringField>(), (compat::shared(StringFieldImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<ColorField>(), (compat::shared(ColorFieldImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<Dropdown>(), (compat::shared(DropdownImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<ItemField>(), (compat::shared(ItemFieldImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<Label>(), (compat::shared(LabelImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<ListGroup>(), (compat::shared(ListGroupImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
            OptionFactoryRegistry::register_option_factory(std::any::TypeId::of::<MasterTickBox>(), (compat::shared(MasterTickBoxImpl::new()) as compat::Shared<dyn OptionFactory<compat::Object, compat::Object>>));
        });
    }

    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self::clinit();
        let mut this = Self {
        };
        this
    }

    /// java: public static <A extends Annotation, T> void registerOptionFactory(Class<A> annotation, OptionFactory<A, T> factory)
    pub fn register_option_factory<A: 'static, T: 'static>(mut annotation: compat::ClassOf<A>, mut factory: compat::Shared<dyn OptionFactory<A, T>>) {
        Self::clinit();
        FACTORY_MAP.get_mut().insert(annotation.clone(), factory.clone());
    }

    /// java: public static <T> Optional<Option<T>> createOption(Field field, ConfigField<T> configField, OptionAccess storage)
    pub fn create_option<T: 'static>(field: &crate::unresolved::Field, config_field: &compat::Shared<dyn ConfigField<T>>, storage: &compat::Shared<dyn OptionAccess>) -> Option<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>> {
        Self::clinit();
        // TODO(fabric2seer): `.getAnnotations(..)` on a Field
        //   java: Annotation[] annotations = Arrays.stream(field.getAnnotations())
        //   java:                 .filter(annotation -> factoryMap.containsKey(annotation.annotationType()))
        //   java:                 .toArray(Annotation[]::new);
        let mut annotations: Vec<crate::unresolved::Annotation> = todo!("fabric2seer: the initialiser of `annotations` (TODO above)");
        if (annotations.len() as i32) != 1 {
            logger().warn(&format!("Found {} option factory annotations on field {}, expected 1", (annotations.len() as i32), compat::jstring(&field)));
            if (annotations.len() as i32) == 0 {
                return None;
            }
        }
        let annotation = &annotations[(0) as usize];
        // TODO(fabric2seer): `.annotationType(..)` on a Annotation
        //   java: OptionFactory<Annotation, T> factory = (OptionFactory<Annotation, T>) factoryMap.get(annotation.annotationType());
        let mut factory: compat::Shared<dyn OptionFactory<crate::unresolved::Annotation, T>> = todo!("fabric2seer: the initialiser of `factory` (TODO above)");
        return Some(factory.lock().borrow().create_option(annotation, config_field, storage));
    }

}
