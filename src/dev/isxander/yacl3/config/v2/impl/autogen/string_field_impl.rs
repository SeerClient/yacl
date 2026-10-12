//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.StringFieldImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::string_controller_builder;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::string_field::StringField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::ControllerBuilderRef;

/// java: public class StringFieldImpl extends SimpleOptionFactory<StringField, String>
#[derive(Clone, Default)]
pub struct StringFieldImpl {}

impl StringFieldImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleOptionFactory<StringField, String> for StringFieldImpl {
    /// java: @Override protected ControllerBuilder<String> createController(StringField annotation, ConfigField<String> field, OptionAccess storage, Option<String> option)
    fn create_controller(
        &self,
        annotation: &StringField,
        field: &Arc<dyn ConfigField<String>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn YaclOption<String>>,
    ) -> ControllerBuilderRef<String> {
        ControllerBuilderRef::plain(string_controller_builder::create(option.clone()))
    }
}

impl OptionFactory<StringField, String> for StringFieldImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess) (SimpleOptionFactory)
    fn create_option(&self, annotation: &StringField, field: &Arc<dyn ConfigField<String>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<String>> {
        simple_option_factory::create_option(self, annotation, field, option_access)
    }
}
