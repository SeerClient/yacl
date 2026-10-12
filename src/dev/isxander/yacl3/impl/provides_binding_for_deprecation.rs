//! Ported from dev.isxander.yacl3.impl.ProvidesBindingForDeprecation.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;

/// java: public interface ProvidesBindingForDeprecation<T>
///
/// `stateManager instanceof ProvidesBindingForDeprecation` (OptionImpl.binding, ListOptionImpl.binding) is
/// `StateManager::get_binding_for_deprecation`, which the two state managers that implement this answer.
pub trait ProvidesBindingForDeprecation<T: Clone + 'static> {
    /// java: Binding<T> getBinding();
    fn get_binding(&self) -> Arc<dyn Binding<T>>;
}
