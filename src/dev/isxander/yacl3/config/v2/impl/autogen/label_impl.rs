//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.LabelImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::label_option;
use crate::dev::isxander::yacl3::config::v2::api::autogen::label::Label;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess as _;

/// java: public class LabelImpl implements OptionFactory<Label, Component>
#[derive(Clone)]
pub struct LabelImpl {}

impl LabelImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

impl OptionFactory<crate::dev::isxander::yacl3::config::v2::api::autogen::label::Label, mcrust::net::minecraft::network::chat::mutable_component::MutableComponent> for LabelImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<Component> createOption(Label annotation, ConfigField<Component> field, OptionAccess optionAccess)
    fn create_option(&self, annotation: &Label, field: &compat::Shared<dyn ConfigField<MutableComponent>>, option_access: &compat::Shared<dyn OptionAccess>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<MutableComponent>> {
        return (label_option::create(&field.lock().borrow().access().lock().borrow().get()) as compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>);
    }

}
