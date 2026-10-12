//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.LongFieldImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::long_field_controller_builder;
use crate::dev::isxander::yacl3::api::option;
use crate::dev::isxander::yacl3::api::option_description::Builder;
use crate::dev::isxander::yacl3::config::v2::api::autogen::long_field::LongField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactoryBase;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::AutoGenUtils;
use crate::dev::isxander::yacl3::config::v2::r#impl::field_backed_binding::FieldBackedBinding;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::network::chat::component;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController as _;
use crate::dev::isxander::yacl3::api::option::Builder as _;

/// java: public class LongFieldImpl extends SimpleOptionFactory<LongField, Long>
#[derive(Clone)]
pub struct LongFieldImpl {
    /// java: extends SimpleOptionFactory
    pub base: SimpleOptionFactoryBase<LongField, i64>,
}

impl std::ops::Deref for LongFieldImpl {
    type Target = SimpleOptionFactoryBase<LongField, i64>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LongFieldImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LongFieldImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: SimpleOptionFactoryBase::<LongField, i64>::new(),
        };
        this
    }

}

impl SimpleOptionFactory<crate::dev::isxander::yacl3::config::v2::api::autogen::long_field::LongField, i64> for LongFieldImpl {
    fn simple_option_factory_base(&self) -> &SimpleOptionFactoryBase<crate::dev::isxander::yacl3::config::v2::api::autogen::long_field::LongField, i64> {
        &self.base
    }
    fn simple_option_factory_base_mut(&mut self) -> &mut SimpleOptionFactoryBase<crate::dev::isxander::yacl3::config::v2::api::autogen::long_field::LongField, i64> {
        &mut self.base
    }

    /// java: @Override protected ControllerBuilder<Long> createController(LongField annotation, ConfigField<Long> field, OptionAccess storage, Option<Long> option)
    fn create_controller(&self, annotation: &LongField, field: &compat::Shared<dyn ConfigField<i64>>, storage: &compat::Shared<dyn OptionAccess>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<i64>>) -> compat::Shared<dyn ControllerBuilder<i64>> {
        // TODO(fabric2seer): `.range(..)` on a B
        //   java: return LongFieldControllerBuilder.create(option)
        //   java:                 .formatValue(v -> {
        //   java:                     String key = getTranslationKey(field, "fmt." + v);
        //   java:                     if (Language.getInstance().has(key))
        //   java:                         return Component.translatable(key);
        //   java:                     key = getTranslationKey(field, "fmt");
        //   java:                     if (Language.getInstance().has(key))
        //   java:                         return Component.translatable(key, v);
        //   java:                     return Component.literal(String.format(annotation.format(), v));
        //   java:                 })
        //   java:                 .range(annotation.min(), annotation.max());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl OptionFactory<compat::Object, compat::Object> for LongFieldImpl {
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
