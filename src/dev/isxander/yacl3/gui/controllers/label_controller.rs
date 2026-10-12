//! Ported from dev.isxander.yacl3.gui.controllers.LabelController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::net::minecraft::client::gui::active_text_collector::ActiveTextCollector;
use mcrust::net::minecraft::client::gui::component_path::ComponentPath;
use mcrust::net::minecraft::client::gui::components::multi_line_label::MultiLineLabel;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narrated_element_type::NarratedElementType;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::focus_navigation_event::FocusNavigationEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public record LabelController(Option<Component> option) implements Controller<Component>
#[derive(Clone, Debug)]
pub struct LabelController {
    /// java: Option<Component> option
    pub option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<MutableComponent>>,
}

impl LabelController {
    /// java: the canonical record constructor
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<MutableComponent>>) -> Self {
        Self { option }
    }

}

impl Controller<mcrust::net::minecraft::network::chat::mutable_component::MutableComponent> for LabelController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<Component> option()
    fn option(&self) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<MutableComponent>> {
        return self.option.clone();
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        return self.option().lock().borrow().pending_value();
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        return (LabelControllerElement::new(screen.clone(), widget_dimension.clone()) as compat::Shared<dyn AbstractWidget>);
    }

}

/// java: public class LabelControllerElement extends AbstractWidget
pub struct LabelControllerElement {
    /// java: extends AbstractWidget
    pub base: AbstractWidgetBase,
    /// java: private List<FormattedCharSequence> wrappedText;
    pub wrapped_text: Vec<Seq>,
    /// java: protected MultiLineLabel wrappedTooltip;
    pub wrapped_tooltip: Option<MultiLineLabel>,
    /// java: protected boolean focused;
    pub focused: bool,
    /// java: protected final YACLScreen screen;
    pub screen: compat::Shared<YACLScreen>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for LabelControllerElement {
    type Target = AbstractWidgetBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for LabelControllerElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl LabelControllerElement {
    /// java: public LabelControllerElement(YACLScreen screen, Dimension<Integer> dim)
    pub fn new(mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>) -> compat::Shared<Self> {
        let __init_screen = screen.clone();
        let (_, __this) = AbstractWidgetBase::new_in(dim.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                wrapped_text: Default::default(),
                wrapped_tooltip: None,
                focused: false,
                screen: __init_screen,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn AbstractWidget>), __h)
        });
        // TODO(fabric2seer): `option(..)` is a method of the enclosing instance (LabelController.this) - an inner class has no outer reference in Rust
        //   java: option().addListener((opt, pending) -> updateTooltip());
        LabelControllerElement::update_tooltip(&__this);
        LabelControllerElement::update_text(&__this);
        __this
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        LabelControllerElement::update_text(__this);
        let mut y: i32 = (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y();
        // TODO(fabric2seer): field `TOOLTIP_AND_CURSOR` of a value of unknown type
        //   java: ActiveTextCollector textCollector = graphics.textRenderer(GuiGraphicsExtractor.HoveredTextEffects.TOOLTIP_AND_CURSOR);
        let mut text_collector: Box<dyn ActiveTextCollector> = todo!("fabric2seer: the initialiser of `textCollector` (TODO above)");
        // TODO(fabric2seer): `option(..)` is a method of the enclosing instance (LabelController.this) - an inner class has no outer reference in Rust
        //   java: Style textStyle = Style.EMPTY.withColor(option().available() ? -1 : 0xFFA0A0A0);
        let mut text_style: Style = todo!("fabric2seer: the initialiser of `textStyle` (TODO above)");
        for text in compat::read(__this, |__t| __t.wrapped_text.clone()) {
            // TODO(fabric2seer): the port's `accept` ports this Java method with other parameters (left, y, text, advance for Java's x, y, text)
            // TODO(fabric2seer): `accept` takes 4 argument(s) in the port (`left: i32, y: i32, text: &crate::net::minecraft::network::chat::formatted_text::FormattedText, advance: &dyn Fn(char, &Style) -> f32`), Java passes 3
            // rust: text_collector.accept(((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x()).wrapping_add(LabelControllerElement::get_x_padding(__this)), y.wrapping_add(LabelControllerElement::get_y_padding(__this)), &GuiUtils::override_style(&text, &text_style));
            y = y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT);
        }
        if LabelControllerElement::is_focused(__this) {
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x()).wrapping_sub(1), ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y()).wrapping_sub(1), ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x_limit()).wrapping_add(1), (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y(), -1);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x()).wrapping_sub(1), ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y()).wrapping_sub(1), (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x(), ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y_limit()).wrapping_add(1), -1);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x()).wrapping_sub(1), (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y_limit(), ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x_limit()).wrapping_add(1), ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y_limit()).wrapping_add(1), -1);
            graphics.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x_limit(), ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y()).wrapping_sub(1), ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x_limit()).wrapping_add(1), ((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y_limit()).wrapping_add(1), -1);
        }
        let __pose_1 = gui_graphics_extractor::push_pose();
        if { let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), (mouse_x) as f64, (mouse_y) as f64) } {
            let mut style: Option<Style> = LabelControllerElement::get_style(__this, mouse_x, mouse_y);
            if style.is_some() && style.as_ref().unwrap().get_click_event().is_some() {
                // TODO(fabric2seer): static `CursorTypes.POINTING_HAND` has no Rust counterpart in the port's CursorTypes file
                // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
                // rust: graphics.request_cursor(&CursorTypes::POINTING_HAND);
            }
        }
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), event.x(), event.y()) }) {
            return false;
        }
        let Some(mut style) = LabelControllerElement::get_style(__this, ((event.x()) as i32), ((event.y()) as i32)) else {
            return false;
        };
        return false;
    }

    /// java: @Nullable protected Style getStyle(int mouseX, int mouseY)
    pub fn get_style(__this: &compat::Shared<Self>, mut mouse_x: i32, mut mouse_y: i32) -> Option<Style> {
        if !((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).is_point_inside(&mouse_x, &mouse_y)) {
            return None;
        }
        let mut x: i32 = (mouse_x.wrapping_sub((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x())).wrapping_sub(LabelControllerElement::get_x_padding(__this));
        let mut y: i32 = (mouse_y.wrapping_sub((({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y())).wrapping_sub(LabelControllerElement::get_y_padding(__this));
        let mut line: i32 = y.wrapping_div(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT);
        if x < 0 || x > (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).x_limit() {
            return None;
        }
        if y < 0 || y > (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).y_limit() {
            return None;
        }
        if line < 0 || line >= (__this.lock().borrow().wrapped_text.len() as i32) {
            return None;
        }
        return None;
    }

    /// java: private int getXPadding()
    pub fn get_x_padding(__this: &compat::Shared<Self>) -> i32 {
        return 4;
    }

    /// java: private int getYPadding()
    pub fn get_y_padding(__this: &compat::Shared<Self>) -> i32 {
        return 3;
    }

    /// java: private void updateText()
    pub fn update_text(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `formatValue(..)` is a method of the enclosing instance (LabelController.this) - an inner class has no outer reference in Rust
        //   java: wrappedText = textRenderer.split(formatValue(), getDimension().width() - getXPadding() * 2);
        ({ let __f = compat::dispatch(__this, |__t| __t.set_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), { let (__a0,) = (&((__this.lock().borrow().wrapped_text.len() as i32).wrapping_mul(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT)).wrapping_add((LabelControllerElement::get_y_padding(__this)).wrapping_mul(2)),); (({ let __f = compat::dispatch(__this, |__t| __t.get_dimension()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>)) }).lock().borrow()).with_height(__a0) }) });
    }

    /// java: private void updateTooltip()
    pub fn update_tooltip(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `option(..)` is a method of the enclosing instance (LabelController.this) - an inner class has no outer reference in Rust
        //   java: this.wrappedTooltip = MultiLineLabel.create(textRenderer, option().tooltip(), screen.width / 3 * 2 - 10);
    }

    /// java: @Nullable @Override public ComponentPath nextFocusPath(FocusNavigationEvent focusNavigationEvent)
    pub fn next_focus_path(__this: &compat::Shared<Self>, focus_navigation_event: &FocusNavigationEvent) -> Option<ComponentPath> {
        // TODO(fabric2seer): the condition: `option(..)` is a method of the enclosing instance (LabelController.this) - an inner class has no outer reference in Rust
        //   java: !option().available()
        if todo!("fabric2seer: the condition (TODO above)") {
            return None;
        }
        // TODO(fabric2seer): static `ComponentPath.leaf` has no Rust counterpart in the port
        // rust: return (if !(LabelControllerElement::is_focused(__this)) { Some(ComponentPath::leaf(__this.clone())) } else { None });
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public boolean isFocused()
    pub fn is_focused(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.focused);
    }

    /// java: @Override public void setFocused(boolean focused)
    pub fn set_focused(__this: &compat::Shared<Self>, mut focused: bool) {
        { let __v = focused; compat::write(__this, |__t| __t.focused = __v); }
    }

}

impl AbstractWidget for LabelControllerElement {
    fn abstract_widget_base(&self) -> &AbstractWidgetBase {
        &self.base
    }
    fn abstract_widget_base_mut(&mut self) -> &mut AbstractWidgetBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean matchesSearch(String query) (dispatch: the body for this class)
    fn matches_search(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &str) -> bool {
        LabelControllerElement::matches_search_dyn
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_narration(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &mut dyn NarrationElementOutput) {
        LabelControllerElement::update_narration_dyn
    }

    /// java: @Override public NarrationPriority narrationPriority() (dispatch: the body for this class)
    fn narration_priority(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> crate::unresolved::NarrationPriority {
        LabelControllerElement::narration_priority_dyn
    }

}

impl LabelControllerElement {
    /// java: @Override public boolean matchesSearch(String query)
    pub fn matches_search_impl(__this: &compat::Shared<Self>, query: &str) -> bool {
        // TODO(fabric2seer): `formatValue(..)` is a method of the enclosing instance (LabelController.this) - an inner class has no outer reference in Rust
        //   java: return formatValue().getString().toLowerCase().contains(query.toLowerCase());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// LabelControllerElement.matchesSearch as the dispatch of AbstractWidget.matchesSearch
    pub fn matches_search_dyn(__this: &compat::Shared<dyn AbstractWidget>, __p0: &str) -> bool {
        Self::matches_search_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder)
    pub fn update_narration_impl(__this: &compat::Shared<Self>, builder: &mut dyn NarrationElementOutput) {
        // TODO(fabric2seer): `formatValue(..)` is a method of the enclosing instance (LabelController.this) - an inner class has no outer reference in Rust
        //   java: builder.add(NarratedElementType.TITLE, formatValue());
    }

    /// LabelControllerElement.updateNarration as the dispatch of AbstractWidget.updateNarration
    pub fn update_narration_dyn(__this: &compat::Shared<dyn AbstractWidget>, __p0: &mut dyn NarrationElementOutput) {
        Self::update_narration_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public NarrationPriority narrationPriority()
    pub fn narration_priority_impl(__this: &compat::Shared<Self>) -> crate::unresolved::NarrationPriority {
        // TODO(fabric2seer): static field NarrationPriority.FOCUSED
        //   java: return NarrationPriority.FOCUSED;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// LabelControllerElement.narrationPriority as the dispatch of AbstractWidget.narrationPriority
    pub fn narration_priority_dyn(__this: &compat::Shared<dyn AbstractWidget>) -> crate::unresolved::NarrationPriority {
        Self::narration_priority_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
