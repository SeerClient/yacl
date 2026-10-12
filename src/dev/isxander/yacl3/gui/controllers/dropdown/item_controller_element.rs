//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.ItemControllerElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidgetBase;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::AbstractDropdownControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::AbstractDropdownControllerElementBase;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::item_controller::ItemController;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElementBase;
use crate::dev::isxander::yacl3::gui::utils::item_registry_helper::ItemRegistryHelper;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::item::Item;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public class ItemControllerElement extends AbstractDropdownControllerElement<Item, Identifier>
pub struct ItemControllerElement {
    /// java: extends AbstractDropdownControllerElement
    pub base: AbstractDropdownControllerElementBase<Arc<Item>, Identifier>,
    /// java: private final ItemController itemController;
    pub item_controller: Option<compat::Shared<ItemController>>,
    /// java: protected Item currentItem = null;
    pub current_item: Option<Arc<Item>>,
    /// java: protected Map<Identifier, Item> matchingItems = new HashMap<>();
    pub matching_items: std::collections::HashMap<Identifier, Arc<Item>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for ItemControllerElement {
    type Target = AbstractDropdownControllerElementBase<Arc<Item>, Identifier>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemControllerElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemControllerElement {
    /// java: public ItemControllerElement(ItemController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(mut control: compat::Shared<ItemController>, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractDropdownControllerElementBase::<Arc<Item>, Identifier>::new((control.clone() as compat::Shared<dyn AbstractDropdownController<compat::Object>>), screen.clone(), dim.clone()),
            self_ref: std::sync::Weak::new(),
            item_controller: None,
            current_item: None,
            matching_items: std::collections::HashMap::new(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(control.clone()); compat::write(&__this, |__t| __t.item_controller = __v); }
        __this
    }

    /// java: private void extractFakeItem(GuiGraphicsExtractor graphics, Item item, int x, int y)
    pub fn extract_fake_item(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, item: &Arc<Item>, mut x: i32, mut y: i32) {
        let mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = None;
        // java: try {..} catch (NullPointerException ignored) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            stack = Some(mcrust::net::minecraft::world::item::item_stack::ItemStack::of_one(item.clone()));
        }));
        if let Err(ignored) = __result {
        }
        if let Some(stack) = &stack {
        } else {
            return;
        }
        graphics.fake_item(stack.as_ref().unwrap(), x, y);
    }

}

impl AbstractDropdownControllerElement<Arc<mcrust::net::minecraft::world::item::item::Item>, mcrust::net::minecraft::resources::identifier::Identifier> for ItemControllerElement {
    fn abstract_dropdown_controller_element_base(&self) -> &AbstractDropdownControllerElementBase<Arc<mcrust::net::minecraft::world::item::item::Item>, mcrust::net::minecraft::resources::identifier::Identifier> {
        &self.base
    }
    fn abstract_dropdown_controller_element_base_mut(&mut self) -> &mut AbstractDropdownControllerElementBase<Arc<mcrust::net::minecraft::world::item::item::Item>, mcrust::net::minecraft::resources::identifier::Identifier> {
        &mut self.base
    }

    /// java: @Override public List<Identifier> computeMatchingValues()
    fn compute_matching_values(&mut self) -> Vec<Identifier> {
        let mut identifiers: Vec<Identifier> = ItemRegistryHelper::get_matching_item_identifiers(&self.input_field).collect::<Vec<_>>();
        self.current_item = Some(ItemRegistryHelper::get_item_from_name(&self.input_field, None /* java: null */));
        for identifier in identifiers.iter() {
            self.matching_items.insert(identifier.clone(), ItemRegistryHelper::get_from_registry(mcrust::net::minecraft::core::registries::built_in_registries::item(), identifier));
        }
        return identifiers;
    }

    /// java: @Override protected void extractDropdownEntry(GuiGraphicsExtractor graphics, Dimension<Integer> entryDimension, Identifier identifier)
    fn extract_dropdown_entry_gui_graphics_extractor_dimension_u(&self, graphics: &GuiGraphicsExtractor, entry_dimension: &compat::Shared<dyn Dimension<i32>>, identifier: &Identifier) {
        self.base.extract_dropdown_entry_gui_graphics_extractor_dimension_u(graphics, entry_dimension, identifier);
        ItemControllerElement::extract_fake_item(&self.self_ref.upgrade().unwrap(), graphics, &self.matching_items.get(identifier).cloned().unwrap(), (entry_dimension.lock().borrow().x_limit()).wrapping_sub(2), (entry_dimension.lock().borrow().y()).wrapping_add(1));
    }

    /// java: @Override public String getString(Identifier identifier)
    fn get_string(&self, identifier: &Identifier) -> String {
        // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // rust: return identifier.fmt();
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override protected int getDecorationPadding()
    fn get_decoration_padding(&self) -> i32 {
        return 16;
    }

    /// java: @Override protected int getDropdownEntryPadding()
    fn get_dropdown_entry_padding(&self) -> i32 {
        return 4;
    }

}

impl StringControllerElement for ItemControllerElement {
    fn string_controller_element_base(&self) -> &StringControllerElementBase {
        &self.base.base
    }
    fn string_controller_element_base_mut(&mut self) -> &mut StringControllerElementBase {
        &mut self.base.base
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick) (dispatch: the body for this class)
    fn mouse_clicked(&self) -> fn(&compat::Shared<dyn StringControllerElement>, &MouseButtonEvent, bool) -> bool {
        <dyn AbstractDropdownControllerElement>::mouse_clicked_from::<Self>
    }

    /// java: @Override public boolean keyPressed(@NonNull KeyEvent event) (dispatch: the body for this class)
    fn key_pressed(&self) -> fn(&compat::Shared<dyn StringControllerElement>, &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        <dyn AbstractDropdownControllerElement>::key_pressed_from::<Self>
    }

    /// java: @Override public boolean charTyped(@NonNull CharacterEvent event) (dispatch: the body for this class)
    fn char_typed(&self) -> fn(&compat::Shared<dyn StringControllerElement>, &CharacterEvent) -> bool {
        <dyn AbstractDropdownControllerElement>::char_typed_from::<Self>
    }

    /// java: @Override public boolean modifyInput(Consumer<StringBuilder> builder) (dispatch: the body for this class)
    fn modify_input(&self) -> fn(&compat::Shared<dyn StringControllerElement>, Arc<dyn Fn(String)>) -> bool {
        <dyn AbstractDropdownControllerElement>::modify_input_from::<Self>
    }

}

impl ControllerWidget<compat::Object> for ItemControllerElement {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<compat::Object> {
        &self.base.base.base
    }
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<compat::Object> {
        &mut self.base.base.base
    }

    /// java: @Override protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut old_dimension = ({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>)) });
        ({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>), { let (__a0,) = (&((({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).width()).wrapping_sub(self.get_decoration_padding()),); (({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).with_width(__a0) }) });
        self.base.extract_value_text(graphics, mouse_x, mouse_y, a);
        ({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.set_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>), old_dimension.clone()) });
        if self.current_item.is_some() {
            ItemControllerElement::extract_fake_item(&self.self_ref.upgrade().unwrap(), graphics, self.current_item.as_ref().unwrap(), ((((({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x_limit()).wrapping_sub(self.get_x_padding())).wrapping_sub(self.get_decoration_padding())).wrapping_add(2), ((({ let __r = &self.self_ref.upgrade().unwrap(); let __f = compat::dispatch(__r, |__t| __t.get_dimension()); __f(&compat::view(__r, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y()).wrapping_add(2));
        }
    }

    /// java: @Override protected int getControlWidth()
    fn get_control_width(&self) -> i32 {
        return self.base.get_control_width().wrapping_add(self.get_decoration_padding());
    }

    /// java: @Override protected Component getValueText()
    fn get_value_text(&self) -> MutableComponent {
        if self.input_field.is_empty() || self.item_controller.is_none() {
            return self.base.get_value_text();
        }
        if self.input_field_focused {
            return component::literal(&self.input_field);
        }
        let mut item: Arc<Item> = self.item_controller.as_ref().unwrap().lock().borrow().option().lock().borrow().pending_value();
        let mut stack: Option<mcrust::net::minecraft::world::item::item_stack::ItemStack> = None;
        // java: try {..} catch (NullPointerException ignored) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            stack = Some(item.get_default_instance());
        }));
        if let Err(ignored) = __result {
        }
        if let Some(stack) = &stack {
            return compat::component_of(item.get_name(stack));
        } else {
            // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut std::fmt::Formatter<'_>`), Java passes 0
            // rust: return component::literal(&item.fmt());
            return todo!("fabric2seer: see the TODO above");
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        // TODO(fabric2seer): `getUnshiftedLength(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: return Math.min(textRenderer.width(getValueText()), getUnshiftedLength());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl AbstractWidget for ItemControllerElement {
    fn abstract_widget_base(&self) -> &AbstractWidgetBase {
        &self.base.base.base.base
    }
    fn abstract_widget_base_mut(&mut self) -> &mut AbstractWidgetBase {
        &mut self.base.base.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean canReset() (dispatch: the body for this class)
    fn can_reset(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> bool {
        <dyn ControllerWidget>::can_reset_from::<Self>
    }

    /// java: @Override public void setDimension(Dimension<Integer> dim) (dispatch: the body for this class)
    fn set_dimension(&self) -> fn(&compat::Shared<dyn AbstractWidget>, compat::Shared<dyn Dimension<i32>>) {
        <dyn AbstractDropdownControllerElement>::set_dimension_from::<Self>
    }

    /// java: @Override public NarrationPriority narrationPriority() (dispatch: the body for this class)
    fn narration_priority(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> crate::unresolved::NarrationPriority {
        <dyn ControllerWidget>::narration_priority_from::<Self>
    }

    /// java: @Override public void unfocus() (dispatch: the body for this class)
    fn unfocus(&self) -> fn(&compat::Shared<dyn AbstractWidget>) {
        <dyn AbstractDropdownControllerElement>::unfocus_from::<Self>
    }

    /// java: @Override public boolean matchesSearch(String query) (dispatch: the body for this class)
    fn matches_search(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &str) -> bool {
        <dyn ControllerWidget>::matches_search_from::<Self>
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_narration(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &mut dyn NarrationElementOutput) {
        <dyn ControllerWidget>::update_narration_from::<Self>
    }

}
