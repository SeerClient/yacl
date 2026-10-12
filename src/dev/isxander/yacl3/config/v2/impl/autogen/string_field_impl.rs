//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.StringFieldImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::string_controller_builder;
use crate::dev::isxander::yacl3::api::option;
use crate::dev::isxander::yacl3::api::option_description::Builder;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactoryBase;
use crate::dev::isxander::yacl3::config::v2::api::autogen::string_field::StringField;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::AutoGenUtils;
use crate::dev::isxander::yacl3::config::v2::r#impl::field_backed_binding::FieldBackedBinding;
use crate::dev::isxander::yacl3::api::option::Builder as _;

/// java: public class StringFieldImpl extends SimpleOptionFactory<StringField, String>
#[derive(Clone)]
pub struct StringFieldImpl {
    /// java: extends SimpleOptionFactory
    pub base: SimpleOptionFactoryBase<StringField, String>,
}

impl std::ops::Deref for StringFieldImpl {
    type Target = SimpleOptionFactoryBase<StringField, String>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for StringFieldImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl StringFieldImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: SimpleOptionFactoryBase::<StringField, String>::new(),
        };
        this
    }

}

impl SimpleOptionFactory<crate::dev::isxander::yacl3::config::v2::api::autogen::string_field::StringField, String> for StringFieldImpl {
    fn simple_option_factory_base(&self) -> &SimpleOptionFactoryBase<crate::dev::isxander::yacl3::config::v2::api::autogen::string_field::StringField, String> {
        &self.base
    }
    fn simple_option_factory_base_mut(&mut self) -> &mut SimpleOptionFactoryBase<crate::dev::isxander::yacl3::config::v2::api::autogen::string_field::StringField, String> {
        &mut self.base
    }

    /// java: @Override protected ControllerBuilder<String> createController(StringField annotation, ConfigField<String> field, OptionAccess storage, Option<String> option)
    fn create_controller(&self, annotation: &StringField, field: &compat::Shared<dyn ConfigField<String>>, storage: &compat::Shared<dyn OptionAccess>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<String>>) -> compat::Shared<dyn ControllerBuilder<String>> {
        return (string_controller_builder::create(option) as compat::Shared<dyn ControllerBuilder<compat::Object>>);
    }

}

impl OptionFactory<compat::Object, compat::Object> for StringFieldImpl {
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
