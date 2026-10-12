//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.EnumCyclerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::cycling_list_controller_builder;
use crate::dev::isxander::yacl3::api::nameable_enum::NameableEnum;
use crate::dev::isxander::yacl3::api::option;
use crate::dev::isxander::yacl3::api::option_description::Builder;
use crate::dev::isxander::yacl3::config::v2::api::autogen::enum_cycler::CyclableEnum;
use crate::dev::isxander::yacl3::config::v2::api::autogen::enum_cycler::EnumCycler;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactoryBase;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::AutoGenUtils;
use crate::dev::isxander::yacl3::config::v2::r#impl::field_backed_binding::FieldBackedBinding;
use crate::dev::isxander::yacl3::api::controller::cycling_list_controller_builder::CyclingListControllerBuilder as _;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController as _;
use crate::dev::isxander::yacl3::api::option::Builder as _;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public class EnumCyclerImpl extends SimpleOptionFactory<EnumCycler, Enum<?>>
#[derive(Clone)]
pub struct EnumCyclerImpl {
    /// java: extends SimpleOptionFactory
    pub base: SimpleOptionFactoryBase<EnumCycler, crate::unresolved::Enum>,
}

impl std::ops::Deref for EnumCyclerImpl {
    type Target = SimpleOptionFactoryBase<EnumCycler, crate::unresolved::Enum>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EnumCyclerImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EnumCyclerImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: SimpleOptionFactoryBase::<EnumCycler, crate::unresolved::Enum>::new(),
        };
        this
    }

}

impl SimpleOptionFactory<crate::dev::isxander::yacl3::config::v2::api::autogen::enum_cycler::EnumCycler, crate::unresolved::Enum> for EnumCyclerImpl {
    fn simple_option_factory_base(&self) -> &SimpleOptionFactoryBase<crate::dev::isxander::yacl3::config::v2::api::autogen::enum_cycler::EnumCycler, crate::unresolved::Enum> {
        &self.base
    }
    fn simple_option_factory_base_mut(&mut self) -> &mut SimpleOptionFactoryBase<crate::dev::isxander::yacl3::config::v2::api::autogen::enum_cycler::EnumCycler, crate::unresolved::Enum> {
        &mut self.base
    }

    /// java: @Override protected ControllerBuilder<Enum<?>> createController(EnumCycler annotation, ConfigField<Enum<?>> field, OptionAccess storage, Option<Enum<?>> option)
    fn create_controller(&self, annotation: &EnumCycler, field: &compat::Shared<dyn ConfigField<crate::unresolved::Enum>>, storage: &compat::Shared<dyn OptionAccess>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<crate::unresolved::Enum>>) -> compat::Shared<dyn ControllerBuilder<crate::unresolved::Enum>> {
        let mut values: Vec<crate::unresolved::Enum>;
        if let Some(cyclable_enum) = option.lock().borrow().pending_value().as_any().downcast_ref::<compat::Shared<dyn CyclableEnum<compat::Object>>>() {
            values = cyclable_enum.lock().borrow().allowed_values().to_vec();
        } else {
            // TODO(fabric2seer): `.getEnumConstants(..)` on a Class
            //   java: Enum<?>[] constants = field.access().typeClass().getEnumConstants();
            let mut constants: Vec<crate::unresolved::Enum> = todo!("fabric2seer: the initialiser of `constants` (TODO above)");
            values = (0..(constants.len() as i32)).filter(|ordinal| ((annotation.allowed_ordinals().len() as i32) == 0 || !annotation.allowed_ordinals().iter().copied().any(|allowed| (allowed == *ordinal)))).map(|ordinal| constants[(ordinal) as usize].clone()).collect::<Vec<_>>();
        }
        let mut builder = { let (__a0,) = (values.clone(),); cycling_list_controller_builder::create(option).lock().borrow_mut().values_iterable(__a0) };
        // TODO(fabric2seer): the condition: `.isAssignableFrom(..)` on a Class
        //   java: NameableEnum.class.isAssignableFrom(..)
        if todo!("fabric2seer: the condition (TODO above)") {
            { let (__a0,) = (Arc::new(move |v| (v)()),); builder.lock().borrow_mut().format_value(__a0) };
        } else {
            // TODO(fabric2seer): `.getSimpleName(..)` on a Class
            //   java: builder.formatValue(v -> Component.translatable("yacl3.config.enum.%s.%s".formatted(field.access().typeClass().getSimpleName(), v.name().toLowerCase())));
        }
        return (builder as compat::Shared<dyn ControllerBuilder<compat::Object>>);
    }

}

impl OptionFactory<compat::Object, compat::Object> for EnumCyclerImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess)
    fn create_option(&self, annotation: &A, field: &compat::Shared<dyn ConfigField<T>>, option_access: &compat::Shared<dyn OptionAccess>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>> {
        // TODO(fabric2seer): `.pendingValue(..)` on a value of unknown type
        //   java: Option<T> option = Option.<T>createBuilder()
        //   java:                 .name(this.name(annotation, field, optionAccess))
        //   java:                 .description(v -> this.description(v, annotation, field, optionAccess).build())
        //   java:                 .binding(new FieldBackedBinding<>(field.access(), field.defaultAccess()))
        //   java:                 .controller(opt -> {
        //   java:                     ControllerBuilder<T> builder = this.createController(annotation, field, optionAccess, opt);
        //   java:
        //   java:                     AutoGenUtils.addCustomFormatterToController(builder, field.access());
        //   java:
        //   java:                     return builder;
        //   java:                 })
        //   java:                 .available(this.available(annotation, field, optionAccess))
        //   java:                 .flags(this.flags(annotation, field, optionAccess))
        //   java:                 .addListener((opt, event) -> this.listener(annotation, field, optionAccess, opt, opt.pendingValue()))
        //   java:                 .build();
        let mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>> = todo!("fabric2seer: the initialiser of `option` (TODO above)");
        self.post_init(annotation, field, option_access, &option);
        return option;
    }

}
