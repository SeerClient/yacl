//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.DropdownImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::dropdown_string_controller_builder;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::dropdown::Dropdown;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::ControllerBuilderRef;

/// java: public class DropdownImpl extends SimpleOptionFactory<Dropdown, String>
#[derive(Clone, Default)]
pub struct DropdownImpl {}

impl DropdownImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleOptionFactory<Dropdown, String> for DropdownImpl {
    /// java: @Override protected ControllerBuilder<String> createController(Dropdown annotation, ConfigField<String> field, OptionAccess storage, Option<String> option)
    fn create_controller(
        &self,
        annotation: &Dropdown,
        field: &Arc<dyn ConfigField<String>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn YaclOption<String>>,
    ) -> ControllerBuilderRef<String> {
        ControllerBuilderRef::plain(
            dropdown_string_controller_builder::create(option.clone())
                // java: .values(annotation.values()) - the String... overload
                .values_string(annotation.values())
                .allow_empty_value(annotation.allow_empty_value())
                .allow_any_value(annotation.allow_any_value()),
        )
    }
}

impl OptionFactory<Dropdown, String> for DropdownImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess) (SimpleOptionFactory)
    fn create_option(&self, annotation: &Dropdown, field: &Arc<dyn ConfigField<String>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<String>> {
        simple_option_factory::create_option(self, annotation, field, option_access)
    }
}
