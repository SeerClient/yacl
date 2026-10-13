//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.ItemController.
//!
//! An `Item` value is a `compat::JavaRef<Item>` (the registry's object, compared by identity).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::{
    AbstractDropdownController, AbstractDropdownControllerBase,
};
use crate::dev::isxander::yacl3::gui::controllers::dropdown::item_controller_element::ItemControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::utils::item_registry_helper::ItemRegistryHelper;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::core::defaulted_registry::DefaultedRegistry as _;
use mcrust::net::minecraft::core::registries::built_in_registries;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::item::Item;

/// java: public class ItemController extends AbstractDropdownController<Item>
pub struct ItemController {
    /// java: extends AbstractDropdownController<Item>
    pub base: AbstractDropdownControllerBase<compat::JavaRef<Item>>,
}

impl ItemController {
    /// java: public ItemController(Option<Item> option) - a simple controller to choose between items
    pub fn new(option: Arc<dyn YOption<compat::JavaRef<Item>>>) -> Arc<Self> {
        Arc::new(Self { base: AbstractDropdownControllerBase::new_1(option) })
    }
}

impl Controller<compat::JavaRef<Item>> for ItemController {
    /// java: AbstractDropdownController.option()
    fn option(&self) -> Arc<dyn YOption<compat::JavaRef<Item>>> {
        self.base.option.clone()
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        component::literal(&self.get_string())
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        ItemControllerElement::new(self, screen, widget_dimension)
    }
}

impl IStringController<compat::JavaRef<Item>> for ItemController {
    /// java: @Override public String getString() - `BuiltInRegistries.ITEM.getKey(option.pendingValue()).toString()`
    fn get_string(&self) -> String {
        let item = self.base.option.pending_value();
        built_in_registries::item().get_key_defaulted(&item.erased_obj()).to_string()
    }

    /// java: @Override public void setFromString(String value)
    fn set_from_string(&self, value: &str) {
        let pending = self.base.option.pending_value();
        let item = ItemRegistryHelper::get_item_from_name(value, Some(&pending)).unwrap_or(pending);
        self.base.option.request_set(item);
    }
}

impl AbstractDropdownController<compat::JavaRef<Item>> for ItemController {
    fn dropdown_controller(&self) -> &AbstractDropdownControllerBase<compat::JavaRef<Item>> {
        &self.base
    }

    /// java: @Override public boolean isValueValid(String value)
    fn is_value_valid(&self, value: &str) -> bool {
        ItemRegistryHelper::is_registered_item(value)
    }

    /// java: @Override protected String getValidValue(String value, int offset)
    fn get_valid_value_2(&self, value: &str, offset: i32) -> String {
        ItemRegistryHelper::get_matching_item_identifiers(value)
            .into_iter()
            .nth(offset.max(0) as usize)
            .map(|identifier| identifier.to_string())
            .unwrap_or_else(|| self.get_string())
    }
}
