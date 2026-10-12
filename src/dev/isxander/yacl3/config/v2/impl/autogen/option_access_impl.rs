//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.OptionAccessImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;

/// java: public class OptionAccessImpl implements OptionAccess
#[derive(Clone)]
pub struct OptionAccessImpl {
    /// java: private final Map<String, Option<?>> storage = new HashMap<>();
    pub storage: std::collections::HashMap<String, compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>>,
    /// java: private final Map<String, Consumer<Option<?>>> scheduledOperations = new HashMap<>();
    pub scheduled_operations: std::collections::HashMap<String, Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>)>>,
}

impl OptionAccessImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            storage: std::collections::HashMap::new(),
            scheduled_operations: std::collections::HashMap::new(),
        });
        __this
    }

    /// java: public void putOption(String fieldName, Option<?> option)
    pub fn put_option(__this: &compat::Shared<Self>, mut field_name: String, mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) {
        let __h0 = field_name.clone();
        let __h1 = option.clone();
        __this.lock().borrow_mut().storage.insert(__h0, __h1);
        let mut consumer: Option<Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>)>> = __this.lock().borrow_mut().scheduled_operations.remove(&field_name);
        if let Some(consumer) = &consumer {
            (consumer)(option.clone());
        }
    }

    /// java: public void checkBadOperations()
    pub fn check_bad_operations(__this: &compat::Shared<Self>) {
        if !(__this.lock().borrow().scheduled_operations.is_empty()) {
            logger().warn(&format!("There are scheduled operations on the `OptionAccess` that tried to reference fields that do not exist. The following have been referenced that do not exist: {}", (__this.lock().borrow().scheduled_operations.keys().cloned().collect::<std::collections::HashSet<_>>()).iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", ")));
        }
    }

}

impl OptionAccess for OptionAccessImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @Nullable Option<?> getOption(String fieldName) (dispatch: the body for this class)
    fn get_option(&self) -> fn(&compat::Shared<dyn OptionAccess>, &str) -> Option<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>> {
        OptionAccessImpl::get_option_dyn
    }

    /// java: @Override public void scheduleOptionOperation(String fieldName, Consumer<Option<?>> optionConsumer) (dispatch: the body for this class)
    fn schedule_option_operation(&self) -> fn(&compat::Shared<dyn OptionAccess>, &str, Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>)>) {
        OptionAccessImpl::schedule_option_operation_dyn
    }

}

impl OptionAccessImpl {
    /// java: @Override public @Nullable Option<?> getOption(String fieldName)
    pub fn get_option_impl(__this: &compat::Shared<Self>, field_name: &str) -> Option<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>> {
        return __this.lock().borrow().storage.get(field_name).cloned();
    }

    /// OptionAccessImpl.getOption as the dispatch of OptionAccess.getOption
    pub fn get_option_dyn(__this: &compat::Shared<dyn OptionAccess>, __p0: &str) -> Option<compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>> {
        Self::get_option_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public void scheduleOptionOperation(String fieldName, Consumer<Option<?>> optionConsumer)
    pub fn schedule_option_operation_impl(__this: &compat::Shared<Self>, field_name: &str, option_consumer: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>)>) {
        if __this.lock().borrow().storage.contains_key(field_name) {
            (option_consumer)(__this.lock().borrow().storage.get(field_name).cloned());
        } else {
            // TODO(fabric2seer): Consumer.andThen
            //   java: scheduledOperations.merge(fieldName, optionConsumer, Consumer::andThen);
        }
    }

    /// OptionAccessImpl.scheduleOptionOperation as the dispatch of OptionAccess.scheduleOptionOperation
    pub fn schedule_option_operation_dyn(__this: &compat::Shared<dyn OptionAccess>, __p0: &str, __p1: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>)>) {
        Self::schedule_option_operation_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}
