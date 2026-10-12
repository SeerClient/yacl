//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.EnumCyclerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::cycling_list_controller_builder;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::enum_cycler::EnumCycler;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::ControllerBuilderRef;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::EnumValue;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class EnumCyclerImpl extends SimpleOptionFactory<EnumCycler, Enum<?>>
///
/// `Enum<?>` is [`EnumValue`]: the field's enum is the user's, which YACL reaches through the field's `TypeClass`.
#[derive(Clone, Default)]
pub struct EnumCyclerImpl {}

impl EnumCyclerImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleOptionFactory<EnumCycler, EnumValue> for EnumCyclerImpl {
    /// java: @Override protected ControllerBuilder<Enum<?>> createController(EnumCycler annotation, ConfigField<Enum<?>> field, OptionAccess storage, Option<Enum<?>> option)
    fn create_controller(
        &self,
        annotation: &EnumCycler,
        field: &Arc<dyn ConfigField<EnumValue>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn YaclOption<EnumValue>>,
    ) -> ControllerBuilderRef<EnumValue> {
        let values: Vec<EnumValue>;
        let pending_value: EnumValue = option.pending_value();
        // java: if (option.pendingValue() instanceof EnumCycler.CyclableEnum<?> cyclableEnum)
        if let Some(allowed_values) = pending_value.class.cyclable {
            // java: values = Arrays.asList(cyclableEnum.allowedValues());
            values = allowed_values(&pending_value.value).into_iter().map(|v| EnumValue::of(v, pending_value.class)).collect();
        } else {
            let type_class = field.access().type_class();
            let constants: Vec<compat::Object> = type_class.get_enum_constants().unwrap_or_default();
            let allowed_ordinals: Vec<i32> = annotation.allowed_ordinals();
            // java: .filter(ordinal -> annotation.allowedOrdinals().length == 0 || Arrays.stream(annotation.allowedOrdinals()).noneMatch(allowed -> allowed == ordinal))
            values = (0..constants.len() as i32)
                .filter(|ordinal| allowed_ordinals.is_empty() || allowed_ordinals.iter().all(|allowed| *allowed != *ordinal))
                .map(|ordinal| EnumValue::of(constants[ordinal as usize].clone(), type_class))
                .collect();
        }

        // EnumController doesn't support filtering
        // java: CyclingListControllerBuilder.create(option).values(values) - the Iterable overload
        let mut builder = cycling_list_controller_builder::create(option.clone()).values_iterable(values);

        let type_class = field.access().type_class();
        if let Some(display_name) = type_class.nameable {
            // java: builder.formatValue(v -> ((NameableEnum) v).getDisplayName());
            builder = builder.format_value(Arc::new(move |v: EnumValue| -> MutableComponent { display_name(&v.value) }));
        } else {
            builder = builder.format_value(Arc::new(move |v: EnumValue| -> MutableComponent {
                component::translatable(&format!("yacl3.config.enum.{}.{}", type_class.get_simple_name(), v.name().to_lowercase()))
            }));
        }

        ControllerBuilderRef::formattable(builder)
    }
}

impl OptionFactory<EnumCycler, EnumValue> for EnumCyclerImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess) (SimpleOptionFactory)
    fn create_option(&self, annotation: &EnumCycler, field: &Arc<dyn ConfigField<EnumValue>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<EnumValue>> {
        simple_option_factory::create_option(self, annotation, field, option_access)
    }
}
