//! Ported from dev.isxander.yacl3.gui.controllers.ControllerWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::client::gui::component_path::ComponentPath;
use mcrust::net::minecraft::client::gui::components::multi_line_label::MultiLineLabel;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::focus_navigation_event::FocusNavigationEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public abstract class ControllerWidget<T extends Controller<?>> extends AbstractWidget
pub struct ControllerWidgetBase<T> {
    /// java: extends AbstractWidget
    pub base: AbstractWidgetBase,
    /// java: protected final T control;
    pub control: T,
    /// java: protected MultiLineLabel wrappedTooltip;
    pub wrapped_tooltip: Option<MultiLineLabel>,
    /// java: protected final YACLScreen screen;
    pub screen: compat::Shared<YACLScreen>,
    /// java: protected boolean focused = false;
    pub focused: bool,
    /// java: protected boolean hovered = false;
    pub hovered: bool,
    /// java: protected final Component modifiedOptionName;
    pub modified_option_name: Option<MutableComponent>,
    /// java: protected final String optionNameString;
    pub option_name_string: String,
}

impl<T: 'static> std::ops::Deref for ControllerWidgetBase<T> {
    type Target = AbstractWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for ControllerWidgetBase<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> ControllerWidgetBase<T> {
    /// java: public ControllerWidget(T control, YACLScreen screen, Dimension<Integer> dim) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut control: T, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn ControllerWidget>, __R)) -> (compat::Shared<dyn ControllerWidget>, __R) {
        let __init_control = control.clone();
        let __init_screen = screen.clone();
        let (_, (__this, __r)) = AbstractWidgetBase::new_in(dim.clone(), move |__base| {
            let this = Self {
                base: __base,
                control: __init_control,
                wrapped_tooltip: None,
                screen: __init_screen,
                focused: false,
                hovered: false,
                modified_option_name: None,
                option_name_string: Default::default(),
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn AbstractWidget>), (__h, __r))
        });
        // TODO(fabric2seer): `.option(..)` on a T
        //   java: control.option().addListener((opt, pending) -> updateTooltip());
        __this.lock().borrow_mut().update_tooltip();
        // TODO(fabric2seer): `.option(..)` on a T
        //   java: this.modifiedOptionName = control.option().name().copy().withStyle(ChatFormatting.ITALIC);
        // TODO(fabric2seer): `.option(..)` on a T
        //   java: this.optionNameString = control.option().name().getString().toLowerCase();
        (__this, __r)
    }

}

/// java: the virtual methods of ControllerWidget (implemented by ControllerWidgetBase and every subclass)
pub trait ControllerWidget<T: Clone + 'static>: AbstractWidget {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<T>;
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<T>;

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `isMouseOver(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: hovered = isMouseOver(mouseX, mouseY);
        // TODO(fabric2seer): `.option(..)` on a T
        //   java: Component name = control.option().changed() ? modifiedOptionName : control.option().name();
        let mut name: MutableComponent = todo!("fabric2seer: the initialiser of `name` (TODO above)");
        // TODO(fabric2seer): `getDimension(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: Component shortenedName = Component.literal(GuiUtils.shortenString(name.getString(), textRenderer, getDimension().width() - getControlWidth() - getXPadding() - 7, "...")).setStyle(name.getStyle());
        let mut shortened_name: MutableComponent = todo!("fabric2seer: the initialiser of `shortenedName` (TODO above)");
        // TODO(fabric2seer): `getDimension(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: drawButtonRect(graphics, getDimension().x(), getDimension().y(), getDimension().xLimit(), getDimension().yLimit(), (hovered && isAvailable()) || focused, isAvailable());
        // TODO(fabric2seer): `getDimension(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: graphics.text(textRenderer, shortenedName, getDimension().x() + getXPadding(), getTextY(), getValueColor(), true);
        self.extract_value_text(graphics, mouse_x, mouse_y, a);
        if self.is_hovered() {
            self.extract_hovered_control(graphics, mouse_x, mouse_y, a);
        }
    }

    /// java: protected void extractHoveredControl(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_hovered_control(&self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
    }

    /// java: protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&mut self, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        let mut value_text: MutableComponent = self.get_value_text();
        // TODO(fabric2seer): `getDimension(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: graphics.text(textRenderer, valueText, getDimension().xLimit() - textRenderer.width(valueText) - getXPadding(), getTextY(), getValueColor(), true);
    }

    /// java: private void updateTooltip()
    fn update_tooltip(&mut self) {
        // TODO(fabric2seer): `.option(..)` on a T
        //   java: this.wrappedTooltip = MultiLineLabel.create(textRenderer, control.option().tooltip(), screen.width / 3 * 2 - 10);
    }

    /// java: protected int getControlWidth()
    fn get_control_width(&self) -> i32 {
        return (if self.is_hovered() { self.get_hovered_control_width() } else { self.get_unhovered_control_width() });
    }

    /// java: public boolean isHovered()
    fn is_hovered(&self) -> bool {
        return (self.is_available() && (self.controller_widget_base().hovered || self.controller_widget_base().focused));
    }

    /// java: protected abstract int getHoveredControlWidth();
    fn get_hovered_control_width(&self) -> i32;

    /// java: protected int getUnhoveredControlWidth()
    fn get_unhovered_control_width(&self) -> i32 {
        return self.controller_widget_base().text_renderer.as_ref().unwrap().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(self.get_value_text()));
    }

    /// java: protected int getXPadding()
    fn get_x_padding(&self) -> i32 {
        return 5;
    }

    /// java: protected int getYPadding()
    fn get_y_padding(&self) -> i32 {
        return 2;
    }

    /// java: protected Component getValueText()
    fn get_value_text(&self) -> MutableComponent {
        // TODO(fabric2seer): `.formatValue(..)` on a T
        //   java: return control.formatValue();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: protected boolean isAvailable()
    fn is_available(&self) -> bool {
        // TODO(fabric2seer): `.option(..)` on a T
        //   java: return control.option().available();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: protected int getValueColor()
    fn get_value_color(&self) -> i32 {
        return (if self.is_available() { -1 } else { self.controller_widget_base().inactive_color });
    }

    /// java: protected int getTextY()
    fn get_text_y(&self) -> i32 {
        // TODO(fabric2seer): `getDimension(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
        //   java: return (int)(getDimension().y() + getDimension().height() / 2f - textRenderer.lineHeight / 2f);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Nullable @Override public ComponentPath nextFocusPath(FocusNavigationEvent focusNavigationEvent)
    fn next_focus_path(&self, focus_navigation_event: &FocusNavigationEvent) -> Option<ComponentPath> {
        // TODO(fabric2seer): static `ComponentPath.leaf` has no Rust counterpart in the port
        // rust: return (if !self.is_focused() { Some(ComponentPath::leaf(self.clone())) } else { None });
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean isFocused()
    fn is_focused(&self) -> bool {
        return self.controller_widget_base().focused;
    }

    /// java: @Override public void setFocused(boolean focused)
    fn set_focused(&mut self, mut focused: bool) {
        self.controller_widget_base_mut().focused = focused;
    }

}

// ControllerWidget.canReset implements AbstractWidget.canReset: it is emitted in each concrete subclass's `impl AbstractWidget` (Rust has no inherited impls)
// ControllerWidget.unfocus implements AbstractWidget.unfocus: it is emitted in each concrete subclass's `impl AbstractWidget` (Rust has no inherited impls)
// ControllerWidget.matchesSearch implements AbstractWidget.matchesSearch: it is emitted in each concrete subclass's `impl AbstractWidget` (Rust has no inherited impls)
// ControllerWidget.narrationPriority implements AbstractWidget.narrationPriority: it is emitted in each concrete subclass's `impl AbstractWidget` (Rust has no inherited impls)
// ControllerWidget.updateNarration implements AbstractWidget.updateNarration: it is emitted in each concrete subclass's `impl AbstractWidget` (Rust has no inherited impls)
