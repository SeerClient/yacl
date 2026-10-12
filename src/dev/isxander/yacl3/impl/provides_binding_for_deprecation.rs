//! Ported from dev.isxander.yacl3.impl.ProvidesBindingForDeprecation by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;

/// java: public interface ProvidesBindingForDeprecation<T>
pub trait ProvidesBindingForDeprecation<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Binding<T> getBinding();
    fn get_binding(&self) -> compat::Shared<dyn Binding<T>>;

}
