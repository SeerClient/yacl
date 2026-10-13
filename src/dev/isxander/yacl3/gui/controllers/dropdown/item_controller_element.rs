//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.ItemControllerElement.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::{self, ControllerWidget};
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::{
    self as dropdown_element, AbstractDropdownControllerElement, AbstractDropdownControllerElementClass, DropdownElement,
};
use crate::dev::isxander::yacl3::gui::controllers::dropdown::item_controller::ItemController;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::{
    self as string_element, Base, StringControllerElement, StringControllerElementClass,
};
use crate::dev::isxander::yacl3::gui::utils::item_registry_helper::ItemRegistryHelper;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::Graphics;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::world::item::item_stack::ItemStack;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Weak;

/// java: public class ItemControllerElement extends AbstractDropdownControllerElement<Item, Identifier>
pub struct ItemControllerElement {
    /// java: extends AbstractDropdownControllerElement<Item, Identifier>
    pub dropdown: AbstractDropdownControllerElement<Identifier>,
    /// java: private final ItemController itemController;
    item_controller: Arc<ItemController>,
    /// java: protected Item currentItem = null;
    pub current_item: RefCell<Option<compat::JavaRef<Item>>>,
    /// java: protected Map<Identifier, Item> matchingItems = new HashMap<>();
    pub matching_items: RefCell<HashMap<Identifier, Option<compat::JavaRef<Item>>>>,
    /// `this`
    this: Weak<ItemControllerElement>,
}

impl ItemControllerElement {
    /// java: public ItemControllerElement(ItemController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(control: Arc<ItemController>, screen: &Arc<YACLScreen>, dim: Arc<dyn Dimension<i32>>) -> Arc<Self> {
        let element = Arc::new_cyclic(|this| Self {
            dropdown: AbstractDropdownControllerElement::new::<compat::JavaRef<Item>, ItemController>(control.clone(), screen, dim),
            item_controller: control,
            current_item: RefCell::new(None),
            matching_items: RefCell::new(HashMap::new()),
            this: this.clone(),
        });
        dropdown_element::register_matching_values_listener(&element);
        element
    }

    /// java: private void extractFakeItem(GuiGraphicsExtractor graphics, Item item, int x, int y)
    fn extract_fake_item(&self, graphics: &Graphics<'_>, item: Option<&compat::JavaRef<Item>>, x: i32, y: i32) {
        // `new ItemStack(null)` throws the NullPointerException the Java catches ("ItemStacks no longer exist until
        // dynamic registries have been loaded"): no item, no draw
        let Some(item) = item else {
            return;
        };
        let stack = ItemStack::new(&item.0);
        graphics.item(&stack, x, y);
    }
}

impl StringControllerElementClass for ItemControllerElement {
    fn string_element(&self) -> &StringControllerElement {
        &self.dropdown.string
    }

    crate::abstract_dropdown_controller_element_overrides!();

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let old_dimension = StringControllerElementClass::get_dimension(self);
        StringControllerElementClass::set_dimension(self, old_dimension.with_width(old_dimension.width() - self.get_decoration_padding()));
        string_element::extract_value_text(self, graphics, mouse_x, mouse_y, a);
        StringControllerElementClass::set_dimension(self, old_dimension);

        let current_item = self.current_item.borrow().clone();
        if let Some(current_item) = current_item {
            let dim = StringControllerElementClass::get_dimension(self);
            self.extract_fake_item(
                graphics,
                Some(&current_item),
                dim.x_limit() - StringControllerElementClass::get_x_padding(self) - self.get_decoration_padding() + 2,
                dim.y() + 2,
            );
        }
    }

    /// java: @Override protected int getControlWidth()
    fn get_control_width(&self) -> i32 {
        controller_widget::get_control_width(&Base(self)) + self.get_decoration_padding()
    }

    /// java: @Override protected Component getValueText()
    fn get_value_text(&self) -> MutableComponent {
        let s = &self.dropdown.string;
        if s.input_field.borrow().is_empty() {
            return string_element::get_value_text(self);
        }

        if s.input_field_focused.get() {
            return component::literal(&s.input_field());
        }

        let item = self.item_controller.option().pending_value();
        // `item.getDefaultInstance()` cannot throw here (YACL guards a Fabric API NPE)
        let stack = item.0.get_default_instance();
        compat::component_of(item.get_name(&stack))
    }
}

impl AbstractDropdownControllerElementClass for ItemControllerElement {
    type Value = Identifier;

    fn dropdown_element(&self) -> &AbstractDropdownControllerElement<Identifier> {
        &self.dropdown
    }

    fn this_dropdown(&self) -> (Weak<dyn DropdownElement>, Weak<dyn ControllerWidget>) {
        (self.this.clone(), self.this.clone())
    }

    /// java: @Override public List<Identifier> computeMatchingValues()
    fn compute_matching_values(&self) -> Vec<Identifier> {
        let input_field = self.dropdown.string.input_field();
        let identifiers = ItemRegistryHelper::get_matching_item_identifiers(&input_field);
        *self.current_item.borrow_mut() = ItemRegistryHelper::get_item_from_name(&input_field, None);
        let mut matching_items = self.matching_items.borrow_mut();
        for identifier in &identifiers {
            matching_items.insert(identifier.clone(), ItemRegistryHelper::get_from_registry(identifier));
        }
        identifiers
    }

    /// java: @Override protected void extractDropdownEntry(GuiGraphicsExtractor graphics, Dimension<Integer>
    /// entryDimension, Identifier identifier)
    fn extract_dropdown_entry(&self, graphics: &Graphics<'_>, entry_dimension: &dyn Dimension<i32>, identifier: &Identifier) {
        dropdown_element::extract_dropdown_entry(self, graphics, entry_dimension, identifier);
        let item = self.matching_items.borrow().get(identifier).cloned().flatten();
        self.extract_fake_item(graphics, item.as_ref(), entry_dimension.x_limit() - 2, entry_dimension.y() + 1);
    }

    /// java: @Override public String getString(Identifier identifier)
    fn get_string_of(&self, identifier: &Identifier) -> String {
        identifier.to_string()
    }

    /// java: @Override protected int getDecorationPadding()
    fn get_decoration_padding(&self) -> i32 {
        16
    }

    /// java: @Override protected int getDropdownEntryPadding()
    fn get_dropdown_entry_padding(&self) -> i32 {
        4
    }
}
