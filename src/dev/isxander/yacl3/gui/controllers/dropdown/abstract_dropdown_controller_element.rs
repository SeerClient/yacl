//! Ported from dev.isxander.yacl3.gui.controllers.dropdown.AbstractDropdownControllerElement by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::ControllerPopupWidget;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::dropdown_widget::DropdownWidget;
use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElement;
use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElementBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::dev::isxander::yacl3::api::option::Option as _;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget as _;

/// java: public abstract class AbstractDropdownControllerElement<T, U> extends StringControllerElement
pub struct AbstractDropdownControllerElementBase<T, U> {
    /// java: extends StringControllerElement
    pub base: StringControllerElementBase,
    /// java: private final AbstractDropdownController<T> dropdownController;
    pub dropdown_controller: Option<compat::Shared<dyn AbstractDropdownController<T>>>,
    /// java: protected DropdownWidget<T> dropdownWidget;
    pub dropdown_widget: Option<compat::Shared<DropdownWidget<T>>>,
    /// java: protected boolean dropdownVisible = false;
    pub dropdown_visible: bool,
    /// java: protected List<U> matchingValues = null;
    pub matching_values: Option<Vec<U>>,
}

impl<T: 'static, U: 'static> std::ops::Deref for AbstractDropdownControllerElementBase<T, U> {
    type Target = StringControllerElementBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static, U: 'static> std::ops::DerefMut for AbstractDropdownControllerElementBase<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static, U: Clone + 'static> AbstractDropdownControllerElementBase<T, U> {
    /// java: public AbstractDropdownControllerElement(AbstractDropdownController<T> control, YACLScreen screen, Dimension<Integer> dim) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut control: compat::Shared<dyn AbstractDropdownController<T>>, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn AbstractDropdownControllerElement>, __R)) -> (compat::Shared<dyn AbstractDropdownControllerElement>, __R) {
        // NOTE(fabric2seer): `dropdownWidget` is read by `setDimension`, which StringControllerElement's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `dropdownVisible` is read by `setDimension`, which StringControllerElement's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `dropdownController` is read by `setDimension`, which StringControllerElement's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        // NOTE(fabric2seer): `matchingValues` is read by `setDimension`, which StringControllerElement's constructor calls before this constructor assigns it: Java sees its default there (0 / false / null), and so does this port
        let (_, (__this, __r)) = StringControllerElementBase::new_in((control.clone() as compat::Shared<dyn IStringController<compat::Object>>), screen.clone(), dim.clone(), false, move |__base| {
            let this = Self {
                base: __base,
                dropdown_controller: None,
                dropdown_widget: None,
                dropdown_visible: false,
                matching_values: None,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn StringControllerElement>), (__h, __r))
        });
        { let __v = Some(control.clone()); compat::write(&__this, |__t| __t.abstract_dropdown_controller_element_base_mut().dropdown_controller = __v); }
        let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(&__this));
        { let (__a0,) = (Arc::new(move |opt, val| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; { { let __v = Some(__this_1.lock().borrow_mut().compute_matching_values()); compat::write(&__this_1, |__t| __t.abstract_dropdown_controller_element_base_mut().matching_values = __v); } } }),); compat::read(&__this, |__t| __t.abstract_dropdown_controller_element_base().dropdown_controller.clone()).unwrap().lock().borrow_mut().abstract_dropdown_controller_base_mut().option.as_mut().unwrap().add_listener(__a0) };
        (__this, __r)
    }

}

/// java: the virtual methods of AbstractDropdownControllerElement (implemented by AbstractDropdownControllerElementBase and every subclass)
pub trait AbstractDropdownControllerElement<T: Clone + 'static, U: Clone + 'static>: StringControllerElement {
    fn abstract_dropdown_controller_element_base(&self) -> &AbstractDropdownControllerElementBase<T, U>;
    fn abstract_dropdown_controller_element_base_mut(&mut self) -> &mut AbstractDropdownControllerElementBase<T, U>;

    /// java: public void ensureValidValue()
    fn ensure_valid_value(&mut self) {
        if !({ let (__a0,) = (self.abstract_dropdown_controller_element_base().input_field.clone(),); self.abstract_dropdown_controller_element_base().dropdown_controller.as_ref().unwrap().lock().borrow().is_value_valid(__a0) }) {
            if self.abstract_dropdown_controller_element_base().dropdown_widget.is_none() {
                self.abstract_dropdown_controller_element_base_mut().input_field = { let (__a0,) = (&self.abstract_dropdown_controller_element_base().input_field,); self.abstract_dropdown_controller_element_base().dropdown_controller.as_ref().unwrap().lock().borrow().get_valid_value(__a0) };
            } else {
                self.abstract_dropdown_controller_element_base_mut().input_field = { let (__a0, __a1,) = (&self.abstract_dropdown_controller_element_base().input_field, DropdownWidget::selected_index(self.abstract_dropdown_controller_element_base().dropdown_widget.as_ref().unwrap()),); self.abstract_dropdown_controller_element_base().dropdown_controller.as_ref().unwrap().lock().borrow().get_valid_value_2(__a0, __a1) };
                DropdownWidget::reset_selected_index(self.abstract_dropdown_controller_element_base().dropdown_widget.as_ref().unwrap());
            }
            // TODO(fabric2seer): `getDefaultCaretPos(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
            //   java: caretPos = getDefaultCaretPos();
            self.abstract_dropdown_controller_element_base_mut().matching_values = Some(self.compute_matching_values());
        }
    }

    /// java: public abstract List<U> computeMatchingValues();
    fn compute_matching_values(&mut self) -> Vec<U>;

    /// java: public boolean matchingValue(String value)
    fn matching_value(&self, value: &str) -> bool {
        return value.to_lowercase().contains(&self.abstract_dropdown_controller_element_base().input_field.to_lowercase());
    }

    /// java: void extractDropdownEntry(GuiGraphicsExtractor graphics, Dimension<Integer> entryDimension, int index)
    fn extract_dropdown_entry(&self, graphics: &GuiGraphicsExtractor, entry_dimension: &compat::Shared<dyn Dimension<i32>>, mut index: i32) {
        self.extract_dropdown_entry_gui_graphics_extractor_dimension_u(graphics, entry_dimension, &self.abstract_dropdown_controller_element_base().matching_values.as_ref().unwrap()[(index) as usize]);
    }

    /// java: protected void extractDropdownEntry(GuiGraphicsExtractor graphics, Dimension<Integer> entryDimension, U value)
    fn extract_dropdown_entry_gui_graphics_extractor_dimension_u(&self, graphics: &GuiGraphicsExtractor, entry_dimension: &compat::Shared<dyn Dimension<i32>>, value: &U) {
        let mut entry: String = self.get_string(value);
        let mut text: MutableComponent;
        if compat::is_blank(&entry) {
            text = component::translatable("yacl.control.text.blank").with_style_format(ChatFormatting::Gray);
        } else {
            text = self.shorten_string(&entry);
        }
        gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone()), ((entry_dimension.lock().borrow().x_limit()).wrapping_sub(self.abstract_dropdown_controller_element_base().text_renderer.as_ref().unwrap().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone())))).wrapping_sub(self.get_dropdown_entry_padding()), self.get_text_y(entry_dimension), -1, true);
    }

    /// java: protected int getTextY(Dimension<Integer> dim)
    fn get_text_y(&self, dim: &compat::Shared<dyn Dimension<i32>>) -> i32 {
        return ((((dim.lock().borrow().y() + (dim.lock().borrow().height() / 2.0f32)) - ((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32) / 2.0f32))) as i32);
    }

    /// java: public abstract String getString(U object);
    fn get_string(&self, object: &U) -> String;

    /// java: public Component shortenString(String value)
    fn shorten_string(&self, value: &str) -> MutableComponent {
        // TODO(fabric2seer): `getDimension(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: return Component.literal(GuiUtils.shortenString(value, textRenderer, getDimension().width() - 20, "..."));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: protected int getDecorationPadding()
    fn get_decoration_padding(&self) -> i32 {
        return self.base.get_x_padding();
    }

    /// java: protected int getDropdownEntryPadding()
    fn get_dropdown_entry_padding(&self) -> i32 {
        return 0;
    }

    /// java: public void createDropdownWidget()
    fn create_dropdown_widget(&mut self) {
        self.abstract_dropdown_controller_element_base_mut().dropdown_visible = true;
        // TODO(fabric2seer): `getDimension(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: dropdownWidget = new DropdownWidget<>(dropdownController, screen, getDimension(), this);
        YACLScreen::add_popup_controller_widget(&self.abstract_dropdown_controller_element_base().screen, (self.abstract_dropdown_controller_element_base().dropdown_widget.clone() as compat::Shared<dyn ControllerPopupWidget<compat::Object>>));
    }

    /// java: public DropdownWidget<T> dropdownWidget()
    fn dropdown_widget(&self) -> compat::Shared<DropdownWidget<T>> {
        return self.abstract_dropdown_controller_element_base().dropdown_widget.clone().unwrap();
    }

    /// java: public boolean isDropdownVisible()
    fn is_dropdown_visible(&self) -> bool {
        return self.abstract_dropdown_controller_element_base().dropdown_visible;
    }

    /// java: public void removeDropdownWidget()
    fn remove_dropdown_widget(&mut self) {
        self.ensure_valid_value();
        YACLScreen::clear_popup_controller_widget(&self.abstract_dropdown_controller_element_base().screen);
        self.abstract_dropdown_controller_element_base_mut().dropdown_visible = false;
        self.abstract_dropdown_controller_element_base_mut().dropdown_widget = None;
    }

}

// AbstractDropdownControllerElement.mouseClicked implements StringControllerElement.mouseClicked: it is emitted in each concrete subclass's `impl StringControllerElement` (Rust has no inherited impls)
// AbstractDropdownControllerElement.keyPressed implements StringControllerElement.keyPressed: it is emitted in each concrete subclass's `impl StringControllerElement` (Rust has no inherited impls)
// AbstractDropdownControllerElement.charTyped implements StringControllerElement.charTyped: it is emitted in each concrete subclass's `impl StringControllerElement` (Rust has no inherited impls)
// AbstractDropdownControllerElement.modifyInput implements StringControllerElement.modifyInput: it is emitted in each concrete subclass's `impl StringControllerElement` (Rust has no inherited impls)
// AbstractDropdownControllerElement.setFocused implements ControllerWidget.setFocused: it is emitted in each concrete subclass's `impl ControllerWidget` (Rust has no inherited impls)
// AbstractDropdownControllerElement.getValueColor implements ControllerWidget.getValueColor: it is emitted in each concrete subclass's `impl ControllerWidget` (Rust has no inherited impls)
// AbstractDropdownControllerElement.extractRenderState implements ControllerWidget.extractRenderState: it is emitted in each concrete subclass's `impl ControllerWidget` (Rust has no inherited impls)
// AbstractDropdownControllerElement.unfocus implements AbstractWidget.unfocus: it is emitted in each concrete subclass's `impl AbstractWidget` (Rust has no inherited impls)
// AbstractDropdownControllerElement.setDimension implements AbstractWidget.setDimension: it is emitted in each concrete subclass's `impl AbstractWidget` (Rust has no inherited impls)
