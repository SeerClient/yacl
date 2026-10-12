//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.ItemFieldImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::item_controller_builder;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::item_field::ItemField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::ControllerBuilderRef;
use mcrust::net::minecraft::world::item::item::Item;

/// java: public class ItemFieldImpl extends SimpleOptionFactory<ItemField, Item>
#[derive(Clone, Default)]
pub struct ItemFieldImpl {}

impl ItemFieldImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleOptionFactory<ItemField, compat::JavaRef<Item>> for ItemFieldImpl {
    /// java: @Override protected ControllerBuilder<Item> createController(ItemField annotation, ConfigField<Item> field, OptionAccess storage, Option<Item> option)
    fn create_controller(
        &self,
        annotation: &ItemField,
        field: &Arc<dyn ConfigField<compat::JavaRef<Item>>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn YaclOption<compat::JavaRef<Item>>>,
    ) -> ControllerBuilderRef<compat::JavaRef<Item>> {
        ControllerBuilderRef::plain(item_controller_builder::create(option.clone()))
    }
}

impl OptionFactory<ItemField, compat::JavaRef<Item>> for ItemFieldImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess) (SimpleOptionFactory)
    fn create_option(&self, annotation: &ItemField, field: &Arc<dyn ConfigField<compat::JavaRef<Item>>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<compat::JavaRef<Item>>> {
        simple_option_factory::create_option(self, annotation, field, option_access)
    }
}
