//! Ported from dev.isxander.yacl3.api.Binding.
//!
//! A Java `Binding<T>` reference is `Arc<dyn Binding<T>>`: every YACL object is shared by reference and keeps its
//! mutable state in cells, so a method takes `&self` (Java's `this`) and can be re-entered by the listeners it
//! fires (PATTERNS_Y2.md P1).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::r#impl::generic_binding_impl::GenericBindingImpl;
use mcrust::net::minecraft::client::option_instance::OptionInstance;
use mcrust::net::minecraft::client::options::OptionsData;

/// java: public interface Binding<T>
pub trait Binding<T: Clone + 'static>: compat::DynAny {
    /// java: void setValue(T value);
    fn set_value(&self, value: T);

    /// java: T getValue();
    fn get_value(&self) -> T;

    /// java: T defaultValue();
    fn default_value(&self) -> T;
}

/// java: default <U> Binding<U> xmap(Function<T, U> to, Function<U, T> from)
/// `this` is the binding's own reference (a default method's receiver handed to the lambdas it builds).
pub fn xmap<T: Clone + 'static, U: Clone + 'static>(
    this: &Arc<dyn Binding<T>>,
    to: impl Fn(T) -> U + 'static,
    from: impl Fn(U) -> T + 'static,
) -> Arc<dyn Binding<U>> {
    let to = Rc::new(to);
    let def = to(this.default_value());
    let getter_this = this.clone();
    let getter_to = to.clone();
    let setter_this = this.clone();
    generic(def, move || getter_to(getter_this.get_value()), move |v: U| setter_this.set_value(from(v)))
}

/// java: static <T> Binding<T> generic(T def, Supplier<T> getter, Consumer<T> setter)
pub fn generic<T: Clone + 'static>(def: T, getter: impl Fn() -> T + 'static, setter: impl Fn(T) + 'static) -> Arc<dyn Binding<T>> {
    // Validate.notNull(def / getter / setter): a Rust value cannot be null.
    Arc::new(GenericBindingImpl::new(def, Arc::new(getter), Arc::new(setter)))
}

/// java: static <T> Binding<T> minecraft(OptionInstance<T> minecraftOption)
///
/// The port's options live in `Minecraft.options` (`OptionsData`, behind its `RefCell`), so the option is named by
/// the accessor that reaches it (`|o| &mut o.fov`) and read / written through the game's `Minecraft` at each call,
/// as Java's `minecraftOption::get` / `::set` reach the one live object.
/// `((OptionInstanceAccessor) minecraftOption).getInitialValue()` is `OptionInstance.initial_value()`.
pub fn minecraft<T: Clone + PartialEq + std::fmt::Debug + Send + Sync + 'static>(
    minecraft_option: fn(&mut OptionsData) -> &mut OptionInstance<T>,
) -> Arc<dyn Binding<T>> {
    let initial = {
        let mut options = compat::minecraft().options.borrow_mut();
        minecraft_option(&mut options).initial_value().clone()
    };
    generic(
        initial,
        move || minecraft_option(&mut compat::minecraft().options.borrow_mut()).get(),
        move |value: T| minecraft_option(&mut compat::minecraft().options.borrow_mut()).set(value),
    )
}

/// java: static <T> Binding<T> immutable(T value)
pub fn immutable<T: Clone + 'static>(value: T) -> Arc<dyn Binding<T>> {
    let held = value.clone();
    generic(value, move || held.clone(), |_changed: T| {})
}
