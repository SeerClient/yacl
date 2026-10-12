//! Ported from dev.isxander.yacl3.config.v2.api.autogen.OptionAccess by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface OptionAccess
pub trait OptionAccess: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: @Nullable Option<?> getOption(String fieldName); (dispatch: the body for this class)
    fn get_option(&self) -> fn(&compat::Shared<dyn OptionAccess>, &str) -> Option<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>;

    /// java: void scheduleOptionOperation(String fieldName, Consumer<Option<?>> optionConsumer); (dispatch: the body for this class)
    fn schedule_option_operation(&self) -> fn(&compat::Shared<dyn OptionAccess>, &str, Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>)>);

}
