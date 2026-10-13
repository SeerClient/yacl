//! Ported from dev.isxander.yacl3.gui.DescriptionWithName.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option_description::OptionDescription;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public record DescriptionWithName(Component name, OptionDescription description)
#[derive(Clone)]
pub struct DescriptionWithName {
    /// java: Component name
    pub name: MutableComponent,
    /// java: OptionDescription description
    pub description: Arc<dyn OptionDescription>,
}

impl DescriptionWithName {
    /// java: the canonical record constructor
    pub fn new(name: MutableComponent, description: Arc<dyn OptionDescription>) -> Self {
        Self { name, description }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: the record accessor `description()`
    pub fn description(&self) -> Arc<dyn OptionDescription> {
        self.description.clone()
    }

    /// java: public static DescriptionWithName of(Component name, OptionDescription description)
    pub fn of(name: &MutableComponent, description: Arc<dyn OptionDescription>) -> DescriptionWithName {
        DescriptionWithName::new(name.copy().with_style_format(ChatFormatting::Bold), description)
    }
}
