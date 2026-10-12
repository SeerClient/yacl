//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.TickBoxImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::tick_box_controller_builder;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::tick_box::TickBox;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::ControllerBuilderRef;

/// java: public class TickBoxImpl extends SimpleOptionFactory<TickBox, Boolean>
#[derive(Clone, Default)]
pub struct TickBoxImpl {}

impl TickBoxImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleOptionFactory<TickBox, bool> for TickBoxImpl {
    /// java: @Override protected ControllerBuilder<Boolean> createController(TickBox annotation, ConfigField<Boolean> field, OptionAccess storage, Option<Boolean> option)
    fn create_controller(
        &self,
        annotation: &TickBox,
        field: &Arc<dyn ConfigField<bool>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn YaclOption<bool>>,
    ) -> ControllerBuilderRef<bool> {
        ControllerBuilderRef::plain(tick_box_controller_builder::create(option.clone()))
    }
}

impl OptionFactory<TickBox, bool> for TickBoxImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess) (SimpleOptionFactory)
    fn create_option(&self, annotation: &TickBox, field: &Arc<dyn ConfigField<bool>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<bool>> {
        simple_option_factory::create_option(self, annotation, field, option_access)
    }
}
