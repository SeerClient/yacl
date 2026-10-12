//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.ItemController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownControllerBase;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::item_controller_element::ItemControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::utils::item_registry_helper::ItemRegistryHelper;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::item::item::Item;
use crate::dev::isxander::yacl3::api::option::Option as _;
use mcrust::net::minecraft::core::registry::Registry as _;

/// java: public class ItemController extends AbstractDropdownController<Item>
#[derive(Clone)]
pub struct ItemController {
    /// java: extends AbstractDropdownController
    pub base: AbstractDropdownControllerBase<Arc<Item>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for ItemController {
    type Target = AbstractDropdownControllerBase<Arc<Item>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemController {
    /// java: public ItemController(Option<Item> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Arc<Item>>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractDropdownControllerBase::<Arc<Item>>::new_1(option.clone()),
            self_ref: std::sync::Weak::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl AbstractDropdownController<Arc<mcrust::net::minecraft::world::item::item::Item>> for ItemController {
    fn abstract_dropdown_controller_base(&self) -> &AbstractDropdownControllerBase<Arc<mcrust::net::minecraft::world::item::item::Item>> {
        &self.base
    }
    fn abstract_dropdown_controller_base_mut(&mut self) -> &mut AbstractDropdownControllerBase<Arc<mcrust::net::minecraft::world::item::item::Item>> {
        &mut self.base
    }

    /// java: @Override public boolean isValueValid(String value)
    fn is_value_valid(&self, mut value: String) -> bool {
        return ItemRegistryHelper::is_registered_item(&value);
    }

    /// java: @Override protected String getValidValue(String value, int offset)
    fn get_valid_value_2(&self, value: &str, mut offset: i32) -> String {
        // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // rust: return ItemRegistryHelper::get_matching_item_identifiers(value).skip((offset) as usize).next().map(|__a0| __a0.fmt()).unwrap_or_else(|| self.get_string());
        return todo!("fabric2seer: see the TODO above");
    }

}

impl IStringController<compat::Object> for ItemController {
    /// java: @Override public String getString()
    fn get_string(&self) -> String {
        // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // rust: return mcrust::net::minecraft::core::registries::built_in_registries::item().get_key(&self.option.as_ref().unwrap().lock().borrow().pending_value()).unwrap().fmt();
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public void setFromString(String value)
    fn set_from_string(&self, mut value: String) {
        { let __s0 = ItemRegistryHelper::get_item_from_name(&value, &self.option.as_ref().unwrap().lock().borrow().pending_value()); self.option.as_mut().unwrap().lock().borrow_mut().request_set(__s0) };
    }

}

impl Controller<compat::Object> for ItemController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        return component::literal(&self.get_string());
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        return (ItemControllerElement::new(self.self_ref.upgrade().unwrap(), screen.clone(), widget_dimension.clone()) as compat::Shared<dyn AbstractWidget>);
    }

    /// java: @Override public Option<T> option()
    fn option(&self) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>> {
        return self.option.clone().unwrap();
    }

}
