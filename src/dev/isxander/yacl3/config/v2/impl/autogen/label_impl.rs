//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.LabelImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::label_option;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::label::Label;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class LabelImpl implements OptionFactory<Label, Component>
#[derive(Clone, Default)]
pub struct LabelImpl {}

impl LabelImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl OptionFactory<Label, MutableComponent> for LabelImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<Component> createOption(Label annotation, ConfigField<Component> field, OptionAccess optionAccess)
    fn create_option(&self, annotation: &Label, field: &Arc<dyn ConfigField<MutableComponent>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<MutableComponent>> {
        label_option::create(field.access().get())
    }
}
