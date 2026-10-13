//! Ported from dev.isxander.yacl3.gui.controllers.LabelController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::{AbstractWidget, AbstractWidgetBase};
use crate::dev::isxander::yacl3::gui::controllers::controller_widget::multi_line_label;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::{CursorKind, Graphics, GuiEventListener};
use mcrust::net::minecraft::client::gui::components::multi_line_label::MultiLineLabel;
use mcrust::net::minecraft::client::gui::font::LINE_HEIGHT;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarrationPriority;
use mcrust::net::minecraft::client::gui::narration::narrated_element_type::NarratedElementType;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::style::Style;
use std::cell::{Cell, RefCell};
use std::sync::Weak;

/// java: public record LabelController(Option<Component> option) implements Controller<Component>
///
/// Simply renders some text as a label.
pub struct LabelController {
    /// java: Option<Component> option
    pub option: Arc<dyn YOption<MutableComponent>>,
}

impl LabelController {
    /// java: public LabelController (the canonical record constructor) - constructs a label controller
    pub fn new(option: Arc<dyn YOption<MutableComponent>>) -> Self {
        Self { option }
    }
}

impl Controller<MutableComponent> for LabelController {
    /// java: @Override public Option<Component> option()
    fn option(&self) -> Arc<dyn YOption<MutableComponent>> {
        self.option.clone()
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        self.option.pending_value()
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        Arc::new(LabelControllerElement::new(self, screen, widget_dimension))
    }
}

/// java: public class LabelControllerElement extends AbstractWidget (an inner class: `control` is its
/// `LabelController.this`)
pub struct LabelControllerElement {
    /// java: extends AbstractWidget
    pub base: AbstractWidgetBase,
    /// `LabelController.this`
    control: Arc<LabelController>,
    /// java: private List<FormattedCharSequence> wrappedText;
    wrapped_text: RefCell<Vec<FormattedText>>,
    /// java: protected MultiLineLabel wrappedTooltip; (shared with the option listener that rebuilds it)
    pub wrapped_tooltip: Rc<RefCell<MultiLineLabel>>,
    /// java: protected boolean focused;
    pub focused: Cell<bool>,
    /// java: protected final YACLScreen screen; (held weakly: the screen holds its widgets)
    pub screen: Weak<YACLScreen>,
}

impl LabelControllerElement {
    /// java: public LabelControllerElement(YACLScreen screen, Dimension<Integer> dim)
    pub fn new(control: Arc<LabelController>, screen: &Arc<YACLScreen>, dim: Arc<dyn Dimension<i32>>) -> Self {
        let wrapped_tooltip = Rc::new(RefCell::new(MultiLineLabel::empty()));
        // java: option().addListener((opt, pending) -> updateTooltip());
        let tooltip_cell = Rc::downgrade(&wrapped_tooltip);
        let tooltip_screen = Arc::downgrade(screen);
        control.option.add_listener(Arc::new(move |opt: &dyn YOption<MutableComponent>, _pending: MutableComponent| {
            if let (Some(cell), Some(screen)) = (tooltip_cell.upgrade(), tooltip_screen.upgrade()) {
                *cell.borrow_mut() = multi_line_label(&opt.tooltip(), screen.width() / 3 * 2 - 10);
            }
        }));
        let element = LabelControllerElement {
            base: AbstractWidgetBase::new(dim),
            control,
            wrapped_text: RefCell::new(Vec::new()),
            wrapped_tooltip,
            focused: Cell::new(false),
            screen: Arc::downgrade(screen),
        };
        element.update_tooltip();
        element.update_text();
        element
    }

    /// `option()` - the outer controller's
    fn option(&self) -> Arc<dyn YOption<MutableComponent>> {
        self.control.option()
    }

    /// `formatValue()` - the outer controller's
    fn format_value(&self) -> MutableComponent {
        self.control.format_value()
    }

    /// java: @Nullable protected Style getStyle(int mouseX, int mouseY)
    pub fn get_style(&self, mouse_x: i32, mouse_y: i32) -> Option<Style> {
        let dim = self.base.get_dimension();
        if !dim.is_point_inside(mouse_x, mouse_y) {
            return None;
        }

        let x = mouse_x - dim.x() - self.get_x_padding();
        let y = mouse_y - dim.y() - self.get_y_padding();
        let line = y / LINE_HEIGHT;

        if x < 0 || x > dim.x_limit() {
            return None;
        }
        if y < 0 || y > dim.y_limit() {
            return None;
        }
        if line < 0 || line >= self.wrapped_text.borrow().len() as i32 {
            return None;
        }

        // TODO reimplement (YACL's own TODO: the style lookup is not implemented upstream)
        None
    }

    /// java: private int getXPadding()
    fn get_x_padding(&self) -> i32 {
        4
    }

    /// java: private int getYPadding()
    fn get_y_padding(&self) -> i32 {
        3
    }

    /// java: private void updateText()
    fn update_text(&self) {
        let font = compat::font();
        let dim = self.base.get_dimension();
        let wrapped = font.split(&FormattedText::from(self.format_value()), dim.width() - self.get_x_padding() * 2);
        let height = wrapped.len() as i32 * LINE_HEIGHT + self.get_y_padding() * 2;
        *self.wrapped_text.borrow_mut() = wrapped;
        AbstractWidget::set_dimension(self, dim.with_height(height));
    }

    /// java: private void updateTooltip()
    fn update_tooltip(&self) {
        let width = self.screen.upgrade().map(|screen| screen.width()).unwrap_or(0);
        *self.wrapped_tooltip.borrow_mut() = multi_line_label(&self.option().tooltip(), width / 3 * 2 - 10);
    }
}

impl GuiEventListener for LabelControllerElement {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        self.update_text();

        let dim = self.base.get_dimension();
        let mut y = dim.y();
        // graphics.textRenderer(HoveredTextEffects.TOOLTIP_AND_CURSOR): white, drop shadow; every char of a line
        // carries `textStyle` (GuiUtils.overrideStyle), so the line is its string in that style.
        let text_style = Style::EMPTY.with_color(if self.option().available() { -1 } else { 0xFFA0A0A0u32 as i32 });
        for text in self.wrapped_text.borrow().iter() {
            let styled = FormattedText::Styled(text.get_string(), text_style.clone());
            graphics.text(&styled, dim.x() + self.get_x_padding(), y + self.get_y_padding(), -1, true);
            y += LINE_HEIGHT;
        }

        if self.is_focused() {
            graphics.fill(dim.x() - 1, dim.y() - 1, dim.x_limit() + 1, dim.y(), -1);
            graphics.fill(dim.x() - 1, dim.y() - 1, dim.x(), dim.y_limit() + 1, -1);
            graphics.fill(dim.x() - 1, dim.y_limit(), dim.x_limit() + 1, dim.y_limit() + 1, -1);
            graphics.fill(dim.x_limit(), dim.y() - 1, dim.x_limit() + 1, dim.y_limit() + 1, -1);
        }

        graphics.push_pose();
        if self.is_mouse_over(mouse_x as f64, mouse_y as f64) {
            let style = self.get_style(mouse_x, mouse_y);

            if style.is_some_and(|style| style.get_click_event().is_some()) {
                graphics.request_cursor(CursorKind::PointingHand);
            }
        }
        graphics.pop_pose();
    }

    /// java: @Override public boolean mouseClicked(@NonNull MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked(&self, event: &MouseButtonEvent, _double_click: bool) -> bool {
        if !self.is_mouse_over(event.x(), event.y()) {
            return false;
        }

        let style = self.get_style(event.x() as i32, event.y() as i32);

        if style.is_none() {
            return false;
        }

        // TODO: reimplement (YACL's own TODO)
        false
    }

    /// java: AbstractWidget.isMouseOver(double mouseX, double mouseY)
    fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.base.is_mouse_over(mouse_x, mouse_y)
    }

    /// java: @Override public void setFocused(boolean focused)
    fn set_focused(&self, focused: bool) {
        self.focused.set(focused);
    }

    /// java: @Override public boolean isFocused()
    fn is_focused(&self) -> bool {
        self.focused.get()
    }
}

impl AbstractWidget for LabelControllerElement {
    fn set_dimension(&self, dim: Arc<dyn Dimension<i32>>) {
        self.base.set_dimension(dim);
    }

    fn get_dimension(&self) -> Arc<dyn Dimension<i32>> {
        self.base.get_dimension()
    }

    /// java: @Override public boolean matchesSearch(String query)
    fn matches_search(&self, query: &str) -> bool {
        self.format_value().get_string().to_lowercase().contains(&query.to_lowercase())
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder)
    fn update_narration(&self, builder: &mut dyn NarrationElementOutput) {
        builder.add_component(NarratedElementType::Title, FormattedText::from(self.format_value()));
    }

    /// java: @Override public NarrationPriority narrationPriority()
    fn narration_priority(&self) -> NarrationPriority {
        NarrationPriority::Focused
    }

}

impl LabelControllerElement {
    /// java: @Nullable @Override public ComponentPath nextFocusPath(FocusNavigationEvent focusNavigationEvent) -
    /// `ComponentPath.leaf(this)` when this answers `true`, `null` otherwise.
    ///
    /// rust: port gap (GAPS_Y2.md G7) - the YACL screen has no keyboard focus navigation to ask it yet.
    pub fn next_focus_path_is_leaf(&self) -> bool {
        if !self.option().available() {
            return false;
        }
        !self.is_focused()
    }
}
