//! Ported from dev.isxander.yacl3.gui.YACLSelectionList, with the game classes it extends
//! (`ContainerObjectSelectionList` -> `AbstractSelectionList` -> `AbstractContainerWidget` -> `AbstractScrollArea`).
//!
//! The YACL screen draws and dispatches its own widgets (PATTERNS_Y2.md P3), so the list is not one of the game's
//! (`abstract_selection_list.rs` keeps its rows as indices into a screen's widget tree): `YACLSelectionList<E>` holds
//! the `AbstractSelectionList` / `AbstractContainerWidget` / `AbstractScrollArea` fields (the scroll state is the
//! game's own `ScrollArea`), and a subclass implements `SelectionListClass`, whose defaults are these classes' bodies
//! for the methods `OptionListWidget` overrides. Entries implement `Entry` (`AbstractSelectionList.Entry` +
//! `ContainerObjectSelectionList.Entry`).
//!
//! The entry list is walked by index with a borrow per step: an entry's handler may add or remove entries
//! (`OptionListWidget.refreshListEntries`), and walking a copy would allocate every frame.

#[allow(unused_imports)]
use crate::prelude::*;
use compat::{ContainerEventHandler, Graphics, GuiEventListener, WidgetFields};
use mcrust::net::minecraft::client::gui::components::abstract_scroll_area::{self, ScrollArea, ScrollbarSettings};
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarrationPriority;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::gui::navigation::screen_direction::ScreenDirection;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::mth;
use std::cell::{Cell, RefCell};

/// `a == b` for two list entries (identity).
pub fn same<E: ?Sized>(a: &Arc<E>, b: &Arc<E>) -> bool {
    std::ptr::eq(Arc::as_ptr(a) as *const (), Arc::as_ptr(b) as *const ())
}

/// `AbstractSelectionList.Entry`'s fields (`x`, `y`, `width`, `height`) and `ContainerObjectSelectionList.Entry`'s
/// (`focused`, `dragging`).
pub struct EntryBase {
    x: Cell<i32>,
    y: Cell<i32>,
    width: Cell<i32>,
    height: Cell<i32>,
    /// `ContainerObjectSelectionList.Entry`'s `focused` / `dragging` (its `setFocused(GuiEventListener)` tells the
    /// old and new child, as `AbstractContainerEventHandler`'s does)
    pub container: ContainerEventHandler,
}

impl Default for EntryBase {
    fn default() -> Self {
        Self::new()
    }
}

impl EntryBase {
    /// `public Entry()` - every field zero
    pub fn new() -> EntryBase {
        EntryBase { x: Cell::new(0), y: Cell::new(0), width: Cell::new(0), height: Cell::new(0), container: ContainerEventHandler::new() }
    }

    /// `getX()`
    pub fn get_x(&self) -> i32 {
        self.x.get()
    }

    /// `getY()`
    pub fn get_y(&self) -> i32 {
        self.y.get()
    }

    /// `getWidth()`
    pub fn get_width(&self) -> i32 {
        self.width.get()
    }

    /// `setX(int)`
    pub fn set_x(&self, x: i32) {
        self.x.set(x);
    }

    /// `setY(int)`
    pub fn set_y(&self, y: i32) {
        self.y.set(y);
    }

    /// `setWidth(int)`
    pub fn set_width(&self, width: i32) {
        self.width.set(width);
    }

    /// `setHeight(int)`
    pub fn set_height(&self, height: i32) {
        self.height.set(height);
    }

    /// the `height` field (`AbstractSelectionList.Entry.getHeight()`'s body)
    pub fn height(&self) -> i32 {
        self.height.get()
    }
}

/// java: public static abstract class Entry<E extends YACLSelectionList.Entry<E>> extends ContainerObjectSelectionList.Entry<E>
/// - with `ContainerObjectSelectionList.Entry`'s and `AbstractSelectionList.Entry`'s overridable methods.
pub trait Entry: compat::DynAny {
    /// The class's fields.
    fn entry_base(&self) -> &EntryBase;

    /// `AbstractSelectionList.Entry.getHeight()`
    fn get_height(&self) -> i32 {
        self.entry_base().height()
    }

    /// `public abstract void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float a);`
    fn extract_content(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32);

    /// `AbstractSelectionList.Entry.isMouseOver` - `getRectangle().containsPoint(x, y)`
    fn is_mouse_over(&self, mouse_x: f64, mouse_y: f64) -> bool {
        let base = self.entry_base();
        ScreenRectangle::new_at(base.get_x(), base.get_y(), base.get_width(), self.get_height()).contains_point(mouse_x as i32, mouse_y as i32)
    }

    /// `AbstractSelectionList.Entry.setFocused(boolean)` - empty
    fn set_focused(&self, focused: bool) {
        let _ = focused;
    }

    /// `public abstract List<? extends GuiEventListener> children();`
    fn children(&self) -> Vec<Arc<dyn GuiEventListener>>;

    /// `ContainerObjectSelectionList.Entry.mouseClicked` - `ContainerEventHandler.super.mouseClicked(..)`
    fn mouse_clicked(&self, event: &MouseButtonEvent, double_click: bool) -> bool {
        self.entry_base().container.mouse_clicked(&self.children(), event, double_click)
    }

    /// `ContainerEventHandler.mouseReleased`
    fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
        self.entry_base().container.mouse_released(event)
    }

    /// `ContainerEventHandler.mouseDragged`
    fn mouse_dragged(&self, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
        self.entry_base().container.mouse_dragged(event, dx, dy)
    }

    /// `ContainerEventHandler.mouseScrolled`
    fn mouse_scrolled(&self, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        ContainerEventHandler::mouse_scrolled(&self.children(), x, y, scroll_x, scroll_y)
    }

    /// `ContainerEventHandler.keyPressed`
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        self.entry_base().container.key_pressed(event)
    }

    /// `ContainerEventHandler.charTyped`
    fn char_typed(&self, event: &CharacterEvent) -> bool {
        self.entry_base().container.char_typed(event)
    }

    /// `GuiEventListener.shouldTakeFocusAfterInteraction()`
    fn should_take_focus_after_interaction(&self) -> bool {
        true
    }

    /// `ContainerObjectSelectionList.Entry.updateNarration(NarrationElementOutput)` over `narratables()` - each entry
    /// narrates its own (`narratables()`'s widgets).
    fn update_narration(&self, output: &mut dyn NarrationElementOutput) {
        let _ = output;
    }
}

/// The list's own state: `AbstractScrollArea`'s, `AbstractContainerWidget`'s, `AbstractSelectionList`'s,
/// `YACLSelectionList`'s.
pub struct YACLSelectionList<E: ?Sized + Entry> {
    /// the `AbstractWidget` fields (`x`, `y`, `width`, `height`, `visible`, ..)
    pub fields: WidgetFields,
    /// `AbstractScrollArea.scrollAmount` / `scrolling` / `scrollbarSettings` (the game's)
    scroll: RefCell<ScrollArea>,
    /// java: protected final int defaultEntryHeight;
    pub default_entry_height: i32,
    /// java: private final List<E> children
    children: RefCell<Vec<Arc<E>>>,
    /// java: @Nullable private E selected;
    selected: RefCell<Option<Arc<E>>>,
    /// java: @Nullable private E hovered;
    hovered: RefCell<Option<Arc<E>>>,
    /// `AbstractContainerWidget.focused`
    focused: RefCell<Option<Arc<E>>>,
    /// `AbstractContainerWidget.isDragging`
    dragging: Cell<bool>,
    /// java: private boolean doneRefresh;
    done_refresh: Cell<bool>,
}

impl<E: ?Sized + Entry> YACLSelectionList<E> {
    /// java: public YACLSelectionList(Minecraft minecraft, int width, int height, int y) -
    /// `super(minecraft, width, height, y, 20)` -> `AbstractSelectionList(minecraft, width, height, y, defaultEntryHeight)`
    /// = `super(0, y, width, height, CommonComponents.EMPTY, AbstractScrollArea.defaultSettings(defaultEntryHeight / 2))`
    pub fn new(width: i32, height: i32, y: i32) -> YACLSelectionList<E> {
        let default_entry_height = 20;
        YACLSelectionList {
            fields: WidgetFields::new(0, y, width, height, mcrust::net::minecraft::network::chat::component::empty()),
            scroll: RefCell::new(ScrollArea::new(y, height, ScrollbarSettings::default_settings((default_entry_height / 2) as f64))),
            default_entry_height,
            children: RefCell::new(Vec::new()),
            selected: RefCell::new(None),
            hovered: RefCell::new(None),
            focused: RefCell::new(None),
            dragging: Cell::new(false),
            done_refresh: Cell::new(false),
        }
    }

    /// The entry at `index` (a borrow for the one step).
    pub fn child(&self, index: usize) -> Option<Arc<E>> {
        self.children.borrow().get(index).cloned()
    }

    /// `children().size()`
    pub fn child_count(&self) -> usize {
        self.children.borrow().len()
    }

    /// `public final List<E> children()` - a copy (an entry's handler may change the list)
    pub fn children(&self) -> Vec<Arc<E>> {
        self.children.borrow().clone()
    }

    /// `children.indexOf(entry)`
    pub fn index_of(&self, entry: &Arc<E>) -> Option<usize> {
        self.children.borrow().iter().position(|child| same(child, entry))
    }

    /// `superModifiableChildren().add(index, entry)` (`AbstractSelectionListAccessor.getChildren()`)
    pub fn insert_child(&self, index: usize, entry: Arc<E>) {
        self.children.borrow_mut().insert(index, entry);
    }

    /// `@Nullable public E getSelected()`
    pub fn get_selected(&self) -> Option<Arc<E>> {
        self.selected.borrow().clone()
    }

    /// `@Nullable public E getFocused()`
    pub fn get_focused(&self) -> Option<Arc<E>> {
        self.focused.borrow().clone()
    }

    /// `@Nullable protected E getHovered()`
    pub fn get_hovered(&self) -> Option<Arc<E>> {
        self.hovered.borrow().clone()
    }

    /// `isDragging()`
    pub fn is_dragging(&self) -> bool {
        self.dragging.get()
    }

    /// `setDragging(boolean)`
    pub fn set_dragging(&self, dragging: bool) {
        self.dragging.set(dragging);
    }

    /// `scrollAmount()`
    pub fn scroll_amount(&self) -> f64 {
        self.scroll.borrow().scroll_amount()
    }

    /// `getX()` / `getY()` / `getWidth()` / `getHeight()` / `getRight()` / `getBottom()`
    pub fn get_x(&self) -> i32 {
        self.fields.get_x()
    }
    pub fn get_y(&self) -> i32 {
        self.fields.get_y()
    }
    pub fn get_width(&self) -> i32 {
        self.fields.get_width()
    }
    pub fn get_height(&self) -> i32 {
        self.fields.get_height()
    }
    pub fn get_right(&self) -> i32 {
        self.fields.get_right()
    }
    pub fn get_bottom(&self) -> i32 {
        self.fields.get_bottom()
    }

    /// `setX(int)`
    pub fn set_x(&self, x: i32) {
        self.fields.set_x(x);
    }

    /// `setY(int)` - the scroll area's box moves with it
    pub fn set_y(&self, y: i32) {
        self.fields.set_y(y);
        self.scroll.borrow_mut().y = y;
    }

    /// `setWidth(int)`
    pub fn set_width(&self, width: i32) {
        self.fields.set_width(width);
    }

    /// `setHeight(int)` - the scroll area's box with it
    pub fn set_height(&self, height: i32) {
        self.fields.set_height(height);
        self.scroll.borrow_mut().height = height;
    }

    /// `protected int contentHeight()` - every entry's (virtual) height, plus 4
    pub fn content_height(&self) -> i32 {
        let mut total_height = 0;
        let mut index = 0;
        while let Some(child) = self.child(index) {
            total_height += child.get_height();
            index += 1;
        }
        total_height + 4
    }

    /// `public int maxScrollAmount()`
    pub fn max_scroll_amount(&self) -> i32 {
        self.scroll.borrow().max_scroll_amount(self.content_height())
    }

    /// `private int getFirstEntryY()`
    fn get_first_entry_y(&self) -> i32 {
        self.get_y() + 2
    }

    /// `protected void clearEntries()`
    pub fn clear_entries(&self) {
        self.children.borrow_mut().clear();
        *self.selected.borrow_mut() = None;
    }

    /// `@Nullable protected final E getEntryAtPosition(double posX, double posY)`
    pub fn get_entry_at_position(&self, pos_x: f64, pos_y: f64) -> Option<Arc<E>> {
        let mut index = 0;
        while let Some(child) = self.child(index) {
            if child.is_mouse_over(pos_x, pos_y) {
                return Some(child);
            }
            index += 1;
        }
        None
    }

    /// `isOverScrollbar` / `updateScrolling` / the scrollbar's x, through the game's `ScrollArea`
    fn update_scrolling(&self, scroll_bar_x: i32, event: &MouseButtonEvent) -> bool {
        let content_height = self.content_height();
        self.scroll.borrow_mut().update_scrolling(content_height, scroll_bar_x, event.button(), event.x(), event.y())
    }
}

/// The overridable members of `AbstractSelectionList` / `AbstractScrollArea` a subclass (`OptionListWidget`) replaces,
/// each defaulting to the game class's body; the list's bodies that call them are the functions below.
pub trait SelectionListClass {
    /// `E`
    type E: ?Sized + Entry;

    /// The list's fields.
    fn list(&self) -> &YACLSelectionList<Self::E>;

    /// `public int getRowLeft()` - `getX() + width / 2 - getRowWidth() / 2`
    fn get_row_left(&self) -> i32 {
        let list = self.list();
        list.get_x() + list.get_width() / 2 - self.get_row_width() / 2
    }

    /// `public int getRowWidth()` - 220
    fn get_row_width(&self) -> i32 {
        220
    }

    /// `protected void extractListBackground(GuiGraphicsExtractor graphics)`
    fn extract_list_background(&self, graphics: &Graphics<'_>) {
        extract_list_background(self, graphics)
    }

    /// `protected boolean entriesCanBeSelected()` - `ContainerObjectSelectionList`'s `false`
    fn entries_can_be_selected(&self) -> bool {
        false
    }
}

/// `public int getRowRight()`
pub fn get_row_right<L: SelectionListClass + ?Sized>(this: &L) -> i32 {
    this.get_row_left() + this.get_row_width()
}

/// `protected int scrollBarX()` - `getRowRight() + scrollbarWidth() + 2`
pub fn scroll_bar_x<L: SelectionListClass + ?Sized>(this: &L) -> i32 {
    get_row_right(this) + abstract_scroll_area::SCROLLBAR_WIDTH + 2
}

/// `public int getNextY()`
pub fn get_next_y<L: SelectionListClass + ?Sized>(this: &L) -> i32 {
    let list = this.list();
    let mut y = list.get_first_entry_y() - list.scroll_amount() as i32;
    let mut index = 0;
    while let Some(child) = list.child(index) {
        y += child.get_height();
        index += 1;
    }
    y
}

/// `protected int addEntry(E entry, int height)`
pub fn add_entry<L: SelectionListClass + ?Sized>(this: &L, entry: Arc<L::E>, height: i32) -> i32 {
    let base = entry.entry_base();
    base.set_x(this.get_row_left());
    base.set_width(this.get_row_width());
    base.set_y(get_next_y(this));
    base.set_height(height);
    let list = this.list();
    list.children.borrow_mut().push(entry);
    list.child_count() as i32 - 1
}

/// `private void repositionEntries()`
pub fn reposition_entries<L: SelectionListClass + ?Sized>(this: &L) {
    let list = this.list();
    let mut y = list.get_first_entry_y() - list.scroll_amount() as i32;
    let (row_left, row_width) = (this.get_row_left(), this.get_row_width());
    let mut index = 0;
    while let Some(child) = list.child(index) {
        let base = child.entry_base();
        base.set_y(y);
        y += child.get_height();
        base.set_x(row_left);
        base.set_width(row_width);
        index += 1;
    }
}

/// `protected void removeEntry(E entry)`
pub fn remove_entry<L: SelectionListClass + ?Sized>(this: &L, entry: &Arc<L::E>) {
    let list = this.list();
    let removed = {
        let mut children = list.children.borrow_mut();
        match children.iter().position(|child| same(child, entry)) {
            Some(index) => {
                children.remove(index);
                true
            }
            None => false,
        }
    };
    if removed {
        reposition_entries(this);
        if list.get_selected().is_some_and(|selected| same(&selected, entry)) {
            set_selected(this, None);
        }
    }
}

/// `public void setSelected(@Nullable E selected)` - the last input is the mouse here, so only a clipped entry scrolls
pub fn set_selected<L: SelectionListClass + ?Sized>(this: &L, selected: Option<Arc<L::E>>) {
    let list = this.list();
    *list.selected.borrow_mut() = selected.clone();
    if let Some(selected) = selected {
        let content_y = selected.entry_base().get_y() + 2;
        let content_bottom = content_y + selected.get_height() - 4;
        let top_clipped = content_y < list.get_y();
        let bottom_clipped = content_bottom > list.get_bottom();
        if top_clipped || bottom_clipped {
            scroll_to_entry(this, &selected);
        }
    }
}

/// `protected void scrollToEntry(E entry)`
pub fn scroll_to_entry<L: SelectionListClass + ?Sized>(this: &L, entry: &Arc<L::E>) {
    let list = this.list();
    let top_delta = entry.entry_base().get_y() - list.get_y() - 2;
    if top_delta < 0 {
        set_scroll_amount(this, list.scroll_amount() + top_delta as f64);
    }

    let bottom_delta = list.get_bottom() - entry.entry_base().get_y() - entry.get_height() - 2;
    if bottom_delta < 0 {
        set_scroll_amount(this, list.scroll_amount() - bottom_delta as f64);
    }
}

/// `public void setScrollAmount(double scrollAmount)` - `super.setScrollAmount(..)` (clamped) then `repositionEntries()`
pub fn set_scroll_amount<L: SelectionListClass + ?Sized>(this: &L, scroll_amount: f64) {
    let list = this.list();
    let content_height = list.content_height();
    list.scroll.borrow_mut().set_scroll_amount(content_height, scroll_amount);
    reposition_entries(this);
}

/// `public void refreshScrollAmount()`
pub fn refresh_scroll_amount<L: SelectionListClass + ?Sized>(this: &L) {
    let scroll_amount = this.list().scroll_amount();
    set_scroll_amount(this, scroll_amount);
}

/// `AbstractSelectionList.setFocused(GuiEventListener)` under `ContainerObjectSelectionList.setFocused`:
/// `if (getFocused() != focused) { oldFocus.setFocused(null); AbstractContainerWidget.setFocused(focused);
/// setSelected(..); if (focused == null) setSelected(null); }`
pub fn set_focused_child<L: SelectionListClass + ?Sized>(this: &L, focused: Option<Arc<L::E>>) {
    let list = this.list();
    let old_focus = list.get_focused();
    let unchanged = match (&old_focus, &focused) {
        (Some(a), Some(b)) => same(a, b),
        (None, None) => true,
        _ => false,
    };
    if unchanged {
        return;
    }

    // AbstractSelectionList.setFocused: the old focus (a ContainerEventHandler) lets go of its child
    if let Some(old_focus) = &old_focus {
        old_focus.entry_base().container.set_focused(None);
    }
    // AbstractContainerWidget.setFocused
    if let Some(old_focus) = &old_focus {
        old_focus.set_focused(false);
    }
    if let Some(focused) = &focused {
        focused.set_focused(true);
    }
    *list.focused.borrow_mut() = focused.clone();
    match focused {
        Some(focused) => {
            if list.index_of(&focused).is_some() {
                set_selected(this, Some(focused));
            }
        }
        // ContainerObjectSelectionList.setFocused - `if (focused == null) setSelected(null)`
        None => set_selected(this, None),
    }
}

/// `AbstractSelectionList.extractWidgetRenderState` (under `YACLSelectionList`'s: the first frame repositions the
/// entries - "idk why but the scroll is broken initially and this fixes it")
pub fn extract_render_state<L: SelectionListClass + ?Sized>(this: &L, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
    let list = this.list();
    // AbstractWidget.extractRenderState
    if !list.fields.visible.get() {
        return;
    }
    list.fields.update_hover(mouse_x, mouse_y);

    // YACLSelectionList.extractWidgetRenderState
    if !list.done_refresh.get() {
        reposition_entries(this);
        list.done_refresh.set(true);
    }

    // AbstractSelectionList.extractWidgetRenderState
    let hovered = if is_mouse_over(this, mouse_x as f64, mouse_y as f64) {
        list.get_entry_at_position(mouse_x as f64, mouse_y as f64)
    } else {
        None
    };
    *list.hovered.borrow_mut() = hovered.clone();
    this.extract_list_background(graphics);
    enable_scissor(this, graphics);
    extract_list_items(this, graphics, mouse_x, mouse_y, a, hovered.as_ref());
    graphics.disable_scissor();
    extract_list_separators(this, graphics);
    extract_scrollbar(this, graphics, mouse_x, mouse_y);
}

/// `protected void extractListItems(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)`
fn extract_list_items<L: SelectionListClass + ?Sized>(
    this: &L,
    graphics: &Graphics<'_>,
    mouse_x: i32,
    mouse_y: i32,
    a: f32,
    hovered: Option<&Arc<L::E>>,
) {
    let list = this.list();
    let mut index = 0;
    while let Some(child) = list.child(index) {
        let base = child.entry_base();
        if base.get_y() + child.get_height() >= list.get_y() && base.get_y() <= list.get_bottom() {
            // extractItem: no selection outline (`entriesCanBeSelected()` is false), then the content
            if this.entries_can_be_selected() && list.get_selected().is_some_and(|selected| same(&selected, &child)) {
                let outline_color = if is_focused(this) { -1 } else { -8355712 };
                extract_selection(graphics, &*child, outline_color);
            }
            child.extract_content(graphics, mouse_x, mouse_y, hovered.is_some_and(|hovered| same(hovered, &child)), a);
        }
        index += 1;
    }
}

/// `protected void extractSelection(GuiGraphicsExtractor graphics, E entry, int outlineColor)`
fn extract_selection<E: ?Sized + Entry>(graphics: &Graphics<'_>, entry: &E, outline_color: i32) {
    let base = entry.entry_base();
    let (x0, y0) = (base.get_x(), base.get_y());
    let (x1, y1) = (x0 + base.get_width(), y0 + entry.get_height());
    graphics.fill(x0, y0, x1, y1, outline_color);
    graphics.fill(x0 + 1, y0 + 1, x1 - 1, y1 - 1, -16777216);
}

/// `protected void enableScissor(GuiGraphicsExtractor graphics)`
fn enable_scissor<L: SelectionListClass + ?Sized>(this: &L, graphics: &Graphics<'_>) {
    let list = this.list();
    graphics.enable_scissor(
        mth::clamp_i32(list.get_x(), 0, graphics.gui_width()),
        mth::clamp_i32(list.get_y(), 0, graphics.gui_height()),
        mth::clamp_i32(list.get_right(), 0, graphics.gui_width()),
        mth::clamp_i32(list.get_bottom(), 0, graphics.gui_height()),
    );
}

/// `minecraft.level == null` - the separators and the list background have in-world variants
fn in_world() -> bool {
    mcrust::mcrust::modding::game::level().is_some()
}

/// `protected void extractListSeparators(GuiGraphicsExtractor graphics)`
fn extract_list_separators<L: SelectionListClass + ?Sized>(this: &L, graphics: &Graphics<'_>) {
    let list = this.list();
    let (header, footer) = if in_world() {
        ("textures/gui/inworld_header_separator.png", "textures/gui/inworld_footer_separator.png")
    } else {
        ("textures/gui/header_separator.png", "textures/gui/footer_separator.png")
    };
    graphics.blit(&Identifier::with_default_namespace(header), list.get_x(), list.get_y() - 2, 0.0, 0.0, list.get_width(), 2, 32, 2);
    graphics.blit(&Identifier::with_default_namespace(footer), list.get_x(), list.get_bottom(), 0.0, 0.0, list.get_width(), 2, 32, 2);
}

/// `protected void extractListBackground(GuiGraphicsExtractor graphics)`
pub fn extract_list_background<L: SelectionListClass + ?Sized>(this: &L, graphics: &Graphics<'_>) {
    let list = this.list();
    let background = if in_world() { "textures/gui/inworld_menu_list_background.png" } else { "textures/gui/menu_list_background.png" };
    graphics.blit(
        &Identifier::with_default_namespace(background),
        list.get_x(),
        list.get_y(),
        list.get_right() as f32,
        (list.get_bottom() + list.scroll_amount() as i32) as f32,
        list.get_width(),
        list.get_height(),
        32,
        32,
    );
}

/// `AbstractScrollArea.extractScrollbar(GuiGraphicsExtractor graphics, int mouseX, int mouseY)` - the default settings
/// have no disabled sprite, so an unscrollable list draws no bar
fn extract_scrollbar<L: SelectionListClass + ?Sized>(this: &L, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32) {
    let list = this.list();
    let content_height = list.content_height();
    let scroll = list.scroll.borrow();
    if !scroll.scrollable(content_height) {
        return;
    }
    let scrollbar_x = scroll_bar_x(this);
    let scroller_height = scroll.scroller_height(content_height);
    let scroller_y = scroll.scroll_bar_y(content_height);
    graphics.blit_sprite(
        &Identifier::with_default_namespace(abstract_scroll_area::SCROLLER_BACKGROUND_SPRITE),
        scrollbar_x,
        list.get_y(),
        scroll.scrollbar_width(),
        list.get_height(),
    );
    graphics.blit_sprite(
        &Identifier::with_default_namespace(abstract_scroll_area::SCROLLER_SPRITE),
        scrollbar_x,
        scroller_y,
        scroll.scrollbar_width(),
        scroller_height,
    );
    if scroll.is_over_scrollbar(scrollbar_x, mouse_x as f64, mouse_y as f64) {
        graphics.request_cursor(if scroll.is_scrolling() { compat::CursorKind::ResizeNs } else { compat::CursorKind::PointingHand });
    }
}

/// `AbstractWidget.isMouseOver` - `isActive() && areCoordinatesInRectangle(x, y)`
pub fn is_mouse_over<L: SelectionListClass + ?Sized>(this: &L, mouse_x: f64, mouse_y: f64) -> bool {
    this.list().fields.is_mouse_over(mouse_x, mouse_y)
}

/// `ContainerEventHandler.isFocused()` - `getFocused() != null`
pub fn is_focused<L: SelectionListClass + ?Sized>(this: &L) -> bool {
    this.list().get_focused().is_some()
}

/// `AbstractContainerWidget.setFocused(boolean)` - `ContainerEventHandler`'s: `if (!focused) setFocused(null)`
pub fn set_focused<L: SelectionListClass + ?Sized>(this: &L, focused: bool) {
    if !focused {
        set_focused_child(this, None);
    }
}

/// `AbstractContainerWidget.mouseClicked` - `updateScrolling(event)`, then `ContainerEventHandler.mouseClicked`
/// (the entry under the cursor, its `mouseClicked`, the focus and the drag)
pub fn mouse_clicked<L: SelectionListClass + ?Sized>(this: &L, event: &MouseButtonEvent, double_click: bool) -> bool {
    let list = this.list();
    let scrolling = list.update_scrolling(scroll_bar_x(this), event);
    let clicked = match list.get_entry_at_position(event.x(), event.y()) {
        None => false,
        Some(widget) => {
            if widget.mouse_clicked(event, double_click) && widget.should_take_focus_after_interaction() {
                set_focused_child(this, Some(widget));
                if event.button() == 1 {
                    list.set_dragging(true);
                }
            }
            true
        }
    };
    clicked || scrolling
}

/// `AbstractContainerWidget.mouseReleased` - `super.mouseReleased` (the scroll bar lets go) then
/// `ContainerEventHandler.mouseReleased`
pub fn mouse_released<L: SelectionListClass + ?Sized>(this: &L, event: &MouseButtonEvent) -> bool {
    let list = this.list();
    if event.button() == 1 {
        list.scroll.borrow_mut().on_release();
    }
    if event.button() == 1 && list.is_dragging() {
        list.set_dragging(false);
        if let Some(focused) = list.get_focused() {
            return focused.mouse_released(event);
        }
    }
    false
}

/// `AbstractContainerWidget.mouseDragged` - `super.mouseDragged` (`AbstractScrollArea`'s: the scroll bar) then
/// `ContainerEventHandler.mouseDragged`
pub fn mouse_dragged<L: SelectionListClass + ?Sized>(this: &L, event: &MouseButtonEvent, dx: f64, dy: f64) -> bool {
    let list = this.list();
    let content_height = list.content_height();
    let dragged_bar = list.scroll.borrow_mut().mouse_dragged(content_height, event.y(), dy);
    if dragged_bar {
        reposition_entries(this);
    }
    match list.get_focused() {
        Some(focused) if list.is_dragging() && event.button() == 1 => focused.mouse_dragged(event, dx, dy),
        _ => false,
    }
}

/// `AbstractScrollArea.mouseScrolled` - `setScrollAmount(scrollAmount() - scrollY * scrollRate())`
pub fn mouse_scrolled<L: SelectionListClass + ?Sized>(this: &L, scroll_y: f64) -> bool {
    let list = this.list();
    if !list.fields.visible.get() {
        return false;
    }
    let target = list.scroll_amount() - scroll_y * list.scroll.borrow().scroll_rate();
    set_scroll_amount(this, target);
    true
}

/// `ContainerEventHandler.keyPressed` - the focused entry's
pub fn key_pressed<L: SelectionListClass + ?Sized>(this: &L, event: &KeyEvent) -> bool {
    this.list().get_focused().is_some_and(|focused| focused.key_pressed(event))
}

/// `ContainerEventHandler.charTyped` - the focused entry's
pub fn char_typed<L: SelectionListClass + ?Sized>(this: &L, event: &CharacterEvent) -> bool {
    this.list().get_focused().is_some_and(|focused| focused.char_typed(event))
}

/// `@Nullable protected E nextEntry(ScreenDirection dir, Predicate<E> canSelect, @Nullable E startEntry)`
pub fn next_entry<L: SelectionListClass + ?Sized>(
    this: &L,
    dir: ScreenDirection,
    can_select: &dyn Fn(&Arc<L::E>) -> bool,
    start_entry: Option<&Arc<L::E>>,
) -> Option<Arc<L::E>> {
    let list = this.list();
    let delta: i32 = match dir {
        ScreenDirection::Right | ScreenDirection::Left => 0,
        ScreenDirection::Up => -1,
        ScreenDirection::Down => 1,
    };
    let count = list.child_count() as i32;
    if count != 0 && delta != 0 {
        let index = match start_entry {
            None => {
                if delta > 0 {
                    0
                } else {
                    count - 1
                }
            }
            Some(start) => list.index_of(start).map_or(-1, |i| i as i32) + delta,
        };

        let mut i = index;
        while i >= 0 && i < count {
            if let Some(selected) = list.child(i as usize)
                && can_select(&selected)
            {
                return Some(selected);
            }
            i += delta;
        }
    }
    None
}

/// `ContainerObjectSelectionList.narrationPriority()` - focused, else `AbstractSelectionList`'s (hovered or none)
pub fn narration_priority<L: SelectionListClass + ?Sized>(this: &L) -> NarrationPriority {
    if is_focused(this) {
        NarrationPriority::Focused
    } else if this.list().get_hovered().is_some() {
        NarrationPriority::Hovered
    } else {
        NarrationPriority::None
    }
}

/// `public static <T extends YACLSelectionList<?>> WidgetAndType<T> asWidget(T list)` - `WidgetAndType.ofWidget(list)`
pub fn as_widget<T: GuiEventListener + 'static>(list: Arc<T>) -> Arc<dyn crate::dev::isxander::yacl3::gui::widget_and_type::WidgetAndType<T>> {
    crate::dev::isxander::yacl3::gui::widget_and_type::of_widget(list)
}
