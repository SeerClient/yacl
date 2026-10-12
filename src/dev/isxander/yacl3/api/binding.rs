//! Ported from dev.isxander.yacl3.api.Binding by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::r#impl::generic_binding_impl::GenericBindingImpl;
use mcrust::net::minecraft::client::option_instance::OptionInstance;

/// java: public interface Binding<T>
pub trait Binding<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void setValue(T value);
    fn set_value(&mut self, value: T);

    /// java: T getValue();
    fn get_value(&self) -> T;

    /// java: T defaultValue();
    fn default_value(&self) -> T;

    /// java: default <U> Binding<U> xmap(Function<T, U> to, Function<U, T> from)
    fn xmap<U: Clone + 'static>(&self, to: Arc<dyn Fn(T) -> U>, from: Arc<dyn Fn(U) -> T>) -> compat::Shared<dyn Binding<U>> where Self: Sized {
        return generic(&(to)(self.default_value()), Arc::new(move || (to)(self.get_value())), Arc::new(move |v| { self.set_value((from)(v.clone())); }));
    }

}

/// java: static <T> Binding<T> generic(T def, Supplier<T> getter, Consumer<T> setter)
pub fn generic<T: Clone + 'static>(def: &T, getter: Arc<dyn Fn() -> T>, setter: Arc<dyn Fn(T)>) -> compat::Shared<dyn Binding<T>> {
    // TODO(fabric2seer): static call Validate.notNull(..)
    //   java: Validate.notNull(def, "`def` must not be null");
    // TODO(fabric2seer): static call Validate.notNull(..)
    //   java: Validate.notNull(getter, "`getter` must not be null");
    // TODO(fabric2seer): static call Validate.notNull(..)
    //   java: Validate.notNull(setter, "`setter` must not be null");
    return (compat::shared(GenericBindingImpl::new(def.clone(), getter.clone(), setter.clone())) as compat::Shared<dyn Binding<compat::Object>>);
}

/// java: static <T> Binding<T> minecraft(OptionInstance<T> minecraftOption)
pub fn minecraft<T: Clone + 'static>(minecraft_option: &OptionInstance<T>) -> compat::Shared<dyn Binding<T>> {
    // TODO(fabric2seer): static call Validate.notNull(..)
    //   java: Validate.notNull(minecraftOption, "`minecraftOption` must not be null");
    return (compat::shared(GenericBindingImpl::new(minecraft_option.initial_value, Arc::new(move || minecraft_option.get()), Arc::new(move |__a0| { minecraft_option.set(__a0.clone()); }))) as compat::Shared<dyn Binding<compat::Object>>);
}

/// java: static <T> Binding<T> immutable(T value)
pub fn immutable<T: Clone + 'static>(value: &T) -> compat::Shared<dyn Binding<T>> {
    // TODO(fabric2seer): static call Validate.notNull(..)
    //   java: Validate.notNull(value, "`value` must not be null");
    return (compat::shared(GenericBindingImpl::new(value.clone(), Arc::new(move || value.clone()), Arc::new(move |changed| {

        }))) as compat::Shared<dyn Binding<compat::Object>>);
}
