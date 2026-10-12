//! Ported from dev.isxander.yacl3.api.ListOptionEntry by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::list_option::ListOption;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::api::option::Option as _;

/// java: public interface ListOptionEntry<T> extends Option<T>
pub trait ListOptionEntry<T: Clone + 'static>: crate::dev::isxander::yacl3::api::option::Option {
    /// java: ListOption<T> parentGroup();
    fn parent_group(&self) -> compat::Shared<dyn ListOption<T>>;

    /// java: @Override default @NotNull ImmutableSet<OptionFlag> flags()
    fn flags(&self) -> std::collections::HashSet<compat::Shared<dyn OptionFlag>> {
        return self.parent_group().lock().borrow().flags();
    }

    /// java: @Override default boolean available()
    fn available(&self) -> bool {
        return self.parent_group().lock().borrow().available();
    }

}
