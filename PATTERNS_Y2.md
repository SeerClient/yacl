# PATTERNS_Y2 - YACL API / impl / GUI, batch Y2

The shapes every YACL object has, so a converted mod (Dandelion, Skyblocker, ...) compiles against them, and what
fabric2seer must generate for a mod's own YACL classes.

## P1 - `Arc<dyn Trait>`, `&self` methods, state in cells

YACL is callback-heavy and re-entrant: `StringControllerElement.write` -> `option.requestSet` -> the option's listeners
-> a listener writes the widget it came from, or the screen (`YACLScreen.onOptionChanged` -> `CategoryTab.updateButtons`).
So every YACL object (options, groups, categories, controllers, widgets, the screen, the list and its entries) is an
`Arc<..>` with `&self` methods and its mutable fields in `Cell` / `RefCell`, borrowed for one statement - never across
a call that may come back. Not `compat::Shared` (whose `&mut self` methods panic when re-entered).

* Listener / option values: `Option<T>` (typed) and `AnyOption` (Java's `Option<?>`: name, description, tooltip,
  available, flags, changed, apply/forget/reset, `provide_controller_widget`, `add_any_listener`,
  `add_any_event_listener`). `Arc<dyn Option<T>>` upcasts to `Arc<dyn AnyOption>`. `AnyListOption: AnyOption +
  OptionGroup`; `ListOption<T>: AnyListOption + Option<Vec<T>>`. Both `AnyOption` and `OptionGroup` declare
  `name()` / `description()` / `tooltip()`: on a list option call them as `AnyOption::name(&*x)`.
* A Java wildcard (`IStringController<?>`, `ICyclingController<?>`, `ISliderController<?>`,
  `AbstractDropdownController<?>`) is a small struct of closures over the members that do not mention `T`
  (`StringControl`, `CyclingControl`, `SliderControl`, `DropdownControl`), built generically where `T` is known.

## P2 - builders by value

`fn x(self, ..) -> Self` and `fn build(self)`, for options, groups, categories, `YetAnotherConfigLib`, descriptions
and every `*ControllerBuilder`. `ControllerBuilder<T>::build(&self) -> Arc<dyn Controller<T>>` (also for
`Arc<dyn ControllerBuilder<T>>` / `Box<..>`, and `controller_builder::of(closure)`); `Option.Builder::controller` takes
`impl Fn(Arc<dyn Option<T>>) -> B` with `B: ControllerBuilder<T>`. `ValueFormattableController<T>::format_value(self,
Arc<dyn ValueFormatter<T>>) -> Self`; any `Fn(T) -> MutableComponent` is a `ValueFormatter<T>`.

## P3 - a Java class hierarchy: one trait per class level, base bodies as free functions, blanket impls downwards

A Java class with overridable methods whose BASE bodies call other overridable methods (`ControllerWidget`,
`StringControllerElement`, `AbstractDropdownControllerElement`, `GroupSeparatorEntry`, `YACLSelectionList`,
`NumberFieldController`, ...) is:

1. a fields struct (`ControllerWidgetBase`, `StringControllerElement`, `AbstractDropdownControllerElement<U>`,
   `GroupSeparatorFields`, `YACLSelectionList<E>`) the subclass embeds;
2. a "class trait" declaring EVERY overridable method of the class and of its superclasses, each defaulting to the
   Java body of THIS class (`ControllerWidget`, `StringControllerElementClass`, `AbstractDropdownControllerElementClass`,
   `GroupSeparatorEntryClass`, `SelectionListClass`) - so a base body's `this.getValueText()` reaches the most-derived
   override;
3. the base bodies as free functions `fn x<W: Class + ?Sized>(this: &W, ..)` - an override calls them for
   `super.x(..)`. A `super` call into the class ABOVE goes through `Base(self)` (`string_controller_element::Base`),
   which is the class seen as its superclass with the virtual methods still forwarding to the subclass;
4. blanket impls downwards: `impl<W: StringControllerElementClass> ControllerWidget for W`, `impl<W: ControllerWidget>
   compat::GuiEventListener for W` and `impl<W: ControllerWidget> AbstractWidget for W`. A leaf class implements only
   its own class trait. When a middle class's overrides would need a second blanket level
   (`AbstractDropdownControllerElement` under `StringControllerElement`), the leaf implements the upper class trait
   and pastes the middle class's overrides with a macro that forwards to the middle class's free functions
   (`abstract_dropdown_controller_element_overrides!`).

A class that `extends` a GAME class the screen does not draw itself (`Button.Plain`, `EditBox`, `TabNavigationBar`,
`AbstractSelectionList`, `Screen`) builds on compat's stand-ins: `compat::Button` (+ `ButtonLook`, the game's look),
`compat::EditBoxWidget` (the game's `EditBox` state and draw), `compat::TabManager` / `TabNavigationBar` /
`MenuTabButton`, `yacl_selection_list::YACLSelectionList` (over the game's `ScrollArea`), `compat::WidgetFields`,
`compat::ContainerEventHandler` (`new_plain()` for a class whose `setFocused(child)` is a field write;
`mouse_clicked_with` for one that overrides `setFocused(child)`).

The YACL screen draws and dispatches all of that itself: `YACLScreen` (an `Arc`) is reached through a thin
`ModScreen` adapter (`YACLScreenAdapter`, `PopupControllerScreenAdapter`) that turns the game's calls into Java's
(`MouseButtonEvent`, `CharacterEvent`, a `compat::Graphics` per frame) and puts one `compat::CursorSink` in the game's
widget tree so `graphics.requestCursor(..)` reaches the window. The frame ends with `Graphics::extract_deferred_tooltip`.

## P4 - name the trait for overlapping method names; no GUI class traits in the prelude

`ControllerWidget`, `StringControllerElementClass`, the YACL `AbstractWidget`, `compat::GuiEventListener` and the list
entry traits declare methods of the same names (Java's overrides of one method). A module that has two of them in
scope calls `ControllerWidget::is_mouse_over(self, ..)` (UFCS). The prelude imports none of them (trait objects'
methods resolve without an import). A trait method that would collide with `Ord::min` / `max` on a generic receiver
is called as `<C as ISliderController<T>>::min(..)`.

## P5 - Java's `this` handed out: `Arc::new_cyclic` + a `Weak` field; back references are `Weak`

A constructor that hands `this` to a child (a button's lambda, a popup's `entryWidget`, a dropdown's element, a list
entry's `OptionListWidget.this`) builds the object with `Arc::new_cyclic` and keeps `this: Weak<Self>`. Every
reference from a child to its owner is a `Weak` (see GAPS_Y2 "not gaps").

## P6 - small rules

* A Java `static final` that is a component / identifier is a `fn` (`ActionController::default_text()`,
  `color_picker_widget::transparent_sprite()`); a `static final` of a `Sync` value is a `LazyLock`
  (`GuiUtils::BUTTON_SPRITES`, `NUMBER_FORMAT`); a render-thread value is a `thread_local!` named `*_CELL` with an
  accessor `fn` of the Java name (`OptionFlag::GAME_RESTART()`, `OptionDescription::EMPTY()`).
* A value type a Java interface names in its own supertrait (`ButtonOption extends Option<BiConsumer<YACLScreen,
  ButtonOption>>`) is a newtype (`ButtonAction`), not an alias (a cycle otherwise).
* Identity: `Arc::ptr_eq` / `std::ptr::eq(Arc::as_ptr(..) as *const (), ..)` for Java `==` between objects (tabs,
  entries, listeners); `compat::JavaRef<T>` for an object held as a VALUE (`Item`).
* Screens: `compat::ScreenRef` names which screen (`Null`, `Parent`, `Current`, a new `Mod` screen with or without
  the current one kept below it). `YetAnotherConfigLib::generate_screen(Some(ScreenRef::Current))` opens the YACL
  screen over the one that is up and returns to it on close.
* Text positions in YACL's own text field count `char`s (Java's `String` indices; YACL fields hold no surrogate
  pairs); `compat::str_len` / `substring2` / `insert_str` are the helpers.
* No per-frame allocation in a draw / dispatch walk: lists are walked by index with a borrow per step
  (`YACLSelectionList::child(i)`), so an entry's handler may still add or remove entries.
