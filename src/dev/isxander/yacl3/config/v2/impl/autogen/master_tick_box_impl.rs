//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.MasterTickBoxImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::tick_box_controller_builder;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::master_tick_box::MasterTickBox;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::ControllerBuilderRef;
use crate::dev::isxander::yacl3::api::option::AnyOption;

/// java: public class MasterTickBoxImpl extends SimpleOptionFactory<MasterTickBox, Boolean>
#[derive(Clone, Default)]
pub struct MasterTickBoxImpl {}

impl MasterTickBoxImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleOptionFactory<MasterTickBox, bool> for MasterTickBoxImpl {
    /// java: @Override protected ControllerBuilder<Boolean> createController(MasterTickBox annotation, ConfigField<Boolean> field, OptionAccess storage, Option<Boolean> option)
    fn create_controller(
        &self,
        annotation: &MasterTickBox,
        field: &Arc<dyn ConfigField<bool>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn YaclOption<bool>>,
    ) -> ControllerBuilderRef<bool> {
        ControllerBuilderRef::plain(tick_box_controller_builder::create(option.clone()))
    }

    /// java: @Override protected void listener(MasterTickBox annotation, ConfigField<Boolean> field, OptionAccess storage, Option<Boolean> option, Boolean value)
    fn listener(&self, annotation: &MasterTickBox, field: &Arc<dyn ConfigField<bool>>, storage: &Arc<dyn OptionAccess>, option: &dyn YaclOption<bool>, value: bool) {
        for child in annotation.value() {
            let invert = annotation.invert();
            storage.schedule_option_operation(
                &child,
                Arc::new(move |child_opt: Arc<dyn AnyOption>| {
                    child_opt.set_available(invert != value);
                }),
            );
        }
    }
}

impl OptionFactory<MasterTickBox, bool> for MasterTickBoxImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess) (SimpleOptionFactory)
    fn create_option(&self, annotation: &MasterTickBox, field: &Arc<dyn ConfigField<bool>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<bool>> {
        simple_option_factory::create_option(self, annotation, field, option_access)
    }
}
