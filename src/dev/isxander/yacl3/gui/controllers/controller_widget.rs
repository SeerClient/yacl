//! Ported from dev.isxander.yacl3.gui.controllers.ControllerWidget.
//!
//! `abstract class ControllerWidget<T extends Controller<?>> extends AbstractWidget`. A subclass is a struct holding a
//! `ControllerWidgetBase` (the class's fields) that implements this trait; every overridable method of the class AND of
//! `AbstractWidget` is declared here with Java's body as the default, so a base body that calls `this.getValueText()`
//! reaches the subclass's override (Java's virtual dispatch). `compat::GuiEventListener` and `AbstractWidget` are then
//! implemented for every `ControllerWidget` (the blanket impls at the bottom). The base bodies are the free functions
//! of this module - an override calls them for `super.x(..)`.
//!
//! Call the class's methods as `ControllerWidget::x(self, ..)` in a module that also has `AbstractWidget` or
//! `compat::GuiEventListener` in scope: both carry methods of the same names (PATTERNS_Y2.md P4).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::gui::abstract_widget::{self as abstract_widget, AbstractWidgetBase};
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use compat::Graphics;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::components::multi_line_label::MultiLineLabel;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarrationPriority;
use mcrust::net::minecraft::client::gui::narration::narrated_element_type::NarratedElementType;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::cell::{Cell, RefCell};
use std::sync::Weak;

/// `MultiLineLabel.create(textRenderer, text, maxWidth)`
pub fn multi_line_label(text: &MutableComponent, max_width: i32) -> MultiLineLabel {
    let font = compat::font();
    MultiLineLabel::create_single(&font.splitter(), &FormattedText::from(text.clone()), max_width)
}

/// java: the fields of `ControllerWidget<T extends Controller<?>>` (and of `AbstractWidget` under it)
pub struct ControllerWidgetBase {
    /// java: extends AbstractWidget
    pub base: AbstractWidgetBase,
    /// `control.option()` - the controller's option (the class reaches it through `control` at every use; it is the
    /// same object for the controller's life). The subclass keeps its own typed `control`.
    pub option: Arc<dyn AnyOption>,
    /// `control.formatValue()`
    format_value: Box<dyn Fn() -> MutableComponent>,
    /// java: protected MultiLineLabel wrappedTooltip;
    pub wrapped_tooltip: Rc<RefCell<MultiLineLabel>>,
    /// java: protected final YACLScreen screen; (held weakly: the screen holds its widgets)
    pub screen: Weak<YACLScreen>,
    /// java: protected boolean focused = false;
    pub focused: Cell<bool>,
    /// java: protected boolean hovered = false;
    pub hovered: Cell<bool>,
    /// java: protected final Component modifiedOptionName;
    pub modified_option_name: MutableComponent,
    /// java: protected final String optionNameString;
    pub option_name_string: String,
}

impl ControllerWidgetBase {
    /// java: public ControllerWidget(T control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new<T: Clone + 'static, C: Controller<T> + ?Sized + 'static>(control: &Arc<C>, screen: &Arc<YACLScreen>, dim: Arc<dyn Dimension<i32>>) -> ControllerWidgetBase {
        let option: Arc<dyn AnyOption> = control.option();
        let format_control = control.clone();
        let wrapped_tooltip = Rc::new(RefCell::new(MultiLineLabel::empty()));
        // java: control.option().addListener((opt, pending) -> updateTooltip());
        let tooltip_cell = Rc::downgrade(&wrapped_tooltip);
        let tooltip_screen = Arc::downgrade(screen);
        option.add_any_listener(Arc::new(move |opt: &dyn AnyOption| {
            if let (Some(cell), Some(screen)) = (tooltip_cell.upgrade(), tooltip_screen.upgrade()) {
                *cell.borrow_mut() = multi_line_label(&opt.tooltip(), screen.width() / 3 * 2 - 10);
            }
        }));
        // java: updateTooltip();
        *wrapped_tooltip.borrow_mut() = multi_line_label(&option.tooltip(), screen.width() / 3 * 2 - 10);
        let name = option.name();
        ControllerWidgetBase {
            base: AbstractWidgetBase::new(dim),
            modified_option_name: name.copy().with_style_format(ChatFormatting::Italic),
            option_name_string: name.get_string().to_lowercase(),
            option,
            format_value: Box::new(move || format_control.format_value()),
            wrapped_tooltip,
            screen: Arc::downgrade(screen),
            focused: Cell::new(false),
            hovered: Cell::new(false),
        }
    }

    /// `screen`
    pub fn screen(&self) -> Arc<YACLScreen> {
        self.screen.upgrade().expect("a YACL widget outlived its screen")
    }

    /// `control.formatValue()`
    pub fn control_format_value(&self) -> MutableComponent {
        (self.format_value)()
    }

    /// java: private void updateTooltip()
    pub fn update_tooltip(&self) {
        let width = self.screen().width();
        *self.wrapped_tooltip.borrow_mut() = multi_line_label(&self.option.tooltip(), width / 3 * 2 - 10);
    }
}

/// java: the overridable methods of `ControllerWidget` and of the `AbstractWidget` / `GuiEventListener` under it.
pub trait ControllerWidget {
    /// The class's fields.
    fn controller_widget(&self) -> &ControllerWidgetBase;

    // ---- GuiEventListener / Renderable --------------------------------------------------------------------------

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        extract_render_state(self, graphics, mouse_x, mouse_y, a)
    }

    /// `GuiEventListener.mouseClicked` - `false`.
    fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        let _ = (event, double_click);
        false
    }

    /// `GuiEventListener.mouseReleased` - `false`.
    fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
        let _ = event;
        false
    }

    /// `GuiEventListener.mouseDragged` - `false`.
    fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        let _ = (event, dx, dy);
        false
    }

    /// `GuiEventListener.mouseScrolled` - `false`.
    fn mouse_scrolled(&self, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        let _ = (x, y, scroll_x, scroll_y);
        false
    }

    /// `GuiEventListener.mouseMoved` - empty.
    fn mouse_moved(&self, x: f64, y: f64) {
        let _ = (x, y);
    }

    /// `GuiEventListener.keyPressed` - `false`.
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        let _ = event;
        false
    }

    /// `GuiEventListener.charTyped` - `false`.
    fn char_typed(&self, event: &CharacterEvent) -> bool {
        let _ = event;
        false
    }

    /// java: AbstractWidget.isMouseOver(double mouseX, double mouseY)
    fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        self.controller_widget().base.is_mouse_over(mouse_x, mouse_y)
    }

    /// java: @Override public void setFocused(boolean focused)
    fn set_focused(&self, focused: bool) {
        self.controller_widget().focused.set(focused);
    }

    /// java: @Override public boolean isFocused()
    fn is_focused(&self) -> bool {
        self.controller_widget().focused.get()
    }

    /// `GuiEventListener.shouldTakeFocusAfterInteraction` - `true`.
    fn should_take_focus_after_interaction(&self) -> bool {
        true
    }

    // ---- AbstractWidget -----------------------------------------------------------------------------------------

    /// java: @Override public boolean canReset()
    fn can_reset(&self) -> bool {
        true
    }

    /// java: AbstractWidget.setDimension(Dimension<Integer> dim)
    fn set_dimension(&self, dim: Arc<dyn Dimension<i32>>) {
        self.controller_widget().base.set_dimension(dim);
    }

    /// java: AbstractWidget.getDimension()
    fn get_dimension(&self) -> Arc<dyn Dimension<i32>> {
        self.controller_widget().base.get_dimension()
    }

    /// java: @Override public NarrationPriority narrationPriority()
    fn narration_priority(&self) -> NarrationPriority {
        narration_priority(self)
    }

    /// java: @Override public void unfocus()
    fn unfocus(&self) {
        self.controller_widget().focused.set(false);
    }

    /// java: @Override public boolean matchesSearch(String query)
    fn matches_search(&self, query: &str) -> bool {
        matches_search(self, query)
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder)
    fn update_narration(&self, builder: &mut dyn NarrationElementOutput) {
        update_narration(self, builder)
    }

    /// java: AbstractWidget.playDownSound()
    fn play_down_sound(&self) {
        compat::play_down_sound();
    }

    // ---- ControllerWidget ---------------------------------------------------------------------------------------

    /// java: protected void extractHoveredControl(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_hovered_control(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = (graphics, mouse_x, mouse_y, a);
    }

    /// java: protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_value_text(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        extract_value_text(self, graphics, mouse_x, mouse_y, a)
    }

    /// java: protected int getControlWidth()
    fn get_control_width(&self) -> i32 {
        get_control_width(self)
    }

    /// java: public boolean isHovered()
    fn is_hovered(&self) -> bool {
        is_hovered(self)
    }

    /// java: protected abstract int getHoveredControlWidth();
    fn get_hovered_control_width(&self) -> i32;

    /// java: protected int getUnhoveredControlWidth()
    fn get_unhovered_control_width(&self) -> i32 {
        get_unhovered_control_width(self)
    }

    /// java: protected int getXPadding()
    fn get_x_padding(&self) -> i32 {
        5
    }

    /// java: protected int getYPadding()
    fn get_y_padding(&self) -> i32 {
        2
    }

    /// java: protected Component getValueText()
    fn get_value_text(&self) -> MutableComponent {
        self.controller_widget().control_format_value()
    }

    /// java: protected boolean isAvailable()
    fn is_available(&self) -> bool {
        self.controller_widget().option.available()
    }

    /// java: protected int getValueColor()
    fn get_value_color(&self) -> i32 {
        get_value_color(self)
    }

    /// java: protected int getTextY()
    fn get_text_y(&self) -> i32 {
        get_text_y(self)
    }
}

/// java: ControllerWidget.extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
pub fn extract_render_state<W: ControllerWidget + ?Sized>(this: &W, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
    let cw = this.controller_widget();
    cw.hovered.set(ControllerWidget::is_mouse_over(this, mouse_x as f64, mouse_y as f64));

    let name = if cw.option.changed() { cw.modified_option_name.clone() } else { cw.option.name() };
    let dim = ControllerWidget::get_dimension(this);
    let font = compat::font();
    let shortened_name = component::literal(&GuiUtils::shorten_string(
        name.get_string(),
        &font,
        dim.width() - this.get_control_width() - this.get_x_padding() - 7,
        "...",
    ))
    .set_style(name.get_style().clone());

    let available = this.is_available();
    abstract_widget::draw_button_rect(
        graphics,
        dim.x(),
        dim.y(),
        dim.x_limit(),
        dim.y_limit(),
        (cw.hovered.get() && available) || cw.focused.get(),
        available,
    );
    graphics.text_component(&shortened_name, dim.x() + this.get_x_padding(), this.get_text_y(), this.get_value_color(), true);

    this.extract_value_text(graphics, mouse_x, mouse_y, a);
    if this.is_hovered() {
        this.extract_hovered_control(graphics, mouse_x, mouse_y, a);
    }
}

/// java: protected void extractValueText(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
pub fn extract_value_text<W: ControllerWidget + ?Sized>(this: &W, graphics: &Graphics<'_>, _mouse_x: i32, _mouse_y: i32, _a: f32) {
    let value_text = this.get_value_text();
    let font = compat::font();
    let dim = ControllerWidget::get_dimension(this);
    let text = FormattedText::from(value_text);
    graphics.text(&text, dim.x_limit() - font.width_of(&text) - this.get_x_padding(), this.get_text_y(), this.get_value_color(), true);
}

/// java: protected int getControlWidth()
pub fn get_control_width<W: ControllerWidget + ?Sized>(this: &W) -> i32 {
    if this.is_hovered() { this.get_hovered_control_width() } else { this.get_unhovered_control_width() }
}

/// java: public boolean isHovered()
pub fn is_hovered<W: ControllerWidget + ?Sized>(this: &W) -> bool {
    let cw = this.controller_widget();
    this.is_available() && (cw.hovered.get() || cw.focused.get())
}

/// java: protected int getUnhoveredControlWidth()
pub fn get_unhovered_control_width<W: ControllerWidget + ?Sized>(this: &W) -> i32 {
    compat::font().width_of(&FormattedText::from(this.get_value_text()))
}

/// java: protected int getValueColor()
pub fn get_value_color<W: ControllerWidget + ?Sized>(this: &W) -> i32 {
    if this.is_available() { -1 } else { AbstractWidgetBase::INACTIVE_COLOR }
}

/// java: protected int getTextY()
pub fn get_text_y<W: ControllerWidget + ?Sized>(this: &W) -> i32 {
    let dim = ControllerWidget::get_dimension(this);
    (dim.y() as f32 + dim.height() as f32 / 2.0 - mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32 / 2.0) as i32
}

/// java: @Override public NarrationPriority narrationPriority()
pub fn narration_priority<W: ControllerWidget + ?Sized>(this: &W) -> NarrationPriority {
    if this.controller_widget().focused.get() {
        NarrationPriority::Focused
    } else if this.is_hovered() {
        NarrationPriority::Hovered
    } else {
        NarrationPriority::None
    }
}

/// java: @Override public boolean matchesSearch(String query)
pub fn matches_search<W: ControllerWidget + ?Sized>(this: &W, query: &str) -> bool {
    this.controller_widget().option_name_string.contains(&query.to_lowercase())
}

/// java: @Override public void updateNarration(NarrationElementOutput builder)
pub fn update_narration<W: ControllerWidget + ?Sized>(this: &W, builder: &mut dyn NarrationElementOutput) {
    let option = &this.controller_widget().option;
    builder.add_component(NarratedElementType::Title, FormattedText::from(option.name()));
    builder.add_component(NarratedElementType::Hint, FormattedText::from(option.tooltip()));
}

/// Every `ControllerWidget` is a `GuiEventListener` and an `AbstractWidget` (Java's `extends`): the class's methods.
mod dispatch {
    use super::ControllerWidget;
    use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
    use crate::compat;
    use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarrationPriority;
    use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
    use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
    use mcrust::net::minecraft::client::input::key_event::KeyEvent;
    use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
    use std::sync::Arc;

    impl<W: ControllerWidget + compat::DynAny + ?Sized> compat::GuiEventListener for W {
        fn extract_render_state(&self, graphics: &compat::Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
            ControllerWidget::extract_render_state(self, graphics, mouse_x, mouse_y, a)
        }
        fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
            ControllerWidget::mouse_clicked(self, event, double_click)
        }
        fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
            ControllerWidget::mouse_released(self, event)
        }
        fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
            ControllerWidget::mouse_dragged(self, event, dx, dy)
        }
        fn mouse_scrolled(&self, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
            ControllerWidget::mouse_scrolled(self, x, y, scroll_x, scroll_y)
        }
        fn mouse_moved(&self, x: f64, y: f64) {
            ControllerWidget::mouse_moved(self, x, y)
        }
        fn key_pressed(&self, event: &KeyEvent) -> bool {
            ControllerWidget::key_pressed(self, event)
        }
        fn char_typed(&self, event: &CharacterEvent) -> bool {
            ControllerWidget::char_typed(self, event)
        }
        fn is_mouse_over(&self, x: f64, y: f64) -> bool {
            ControllerWidget::is_mouse_over(self, x, y)
        }
        fn set_focused(&self, focused: bool) {
            ControllerWidget::set_focused(self, focused)
        }
        fn is_focused(&self) -> bool {
            ControllerWidget::is_focused(self)
        }
        fn should_take_focus_after_interaction(&self) -> bool {
            ControllerWidget::should_take_focus_after_interaction(self)
        }
    }

    impl<W: ControllerWidget + compat::DynAny + ?Sized> crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget for W {
        fn can_reset(&self) -> bool {
            ControllerWidget::can_reset(self)
        }
        fn set_dimension(&self, dim: Arc<dyn Dimension<i32>>) {
            ControllerWidget::set_dimension(self, dim)
        }
        fn get_dimension(&self) -> Arc<dyn Dimension<i32>> {
            ControllerWidget::get_dimension(self)
        }
        fn narration_priority(&self) -> NarrationPriority {
            ControllerWidget::narration_priority(self)
        }
        fn unfocus(&self) {
            ControllerWidget::unfocus(self)
        }
        fn matches_search(&self, query: &str) -> bool {
            ControllerWidget::matches_search(self, query)
        }
        fn update_narration(&self, builder: &mut dyn NarrationElementOutput) {
            ControllerWidget::update_narration(self, builder)
        }
        fn play_down_sound(&self) {
            ControllerWidget::play_down_sound(self)
        }
    }
}
