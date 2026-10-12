//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.OptionAccessImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::LOGGER;

/// java: public class OptionAccessImpl implements OptionAccess
///
/// The maps are in cells: an operation runs while the access is in use (it sets an option's availability, whose
/// listeners schedule again), so every borrow ends before a consumer is called.
pub struct OptionAccessImpl {
    /// java: private final Map<String, Option<?>> storage = new HashMap<>();
    pub storage: std::cell::RefCell<std::collections::HashMap<String, Arc<dyn AnyOption>>>,
    /// java: private final Map<String, Consumer<Option<?>>> scheduledOperations = new HashMap<>();
    pub scheduled_operations: std::cell::RefCell<std::collections::HashMap<String, Arc<dyn Fn(Arc<dyn AnyOption>)>>>,
}

impl Default for OptionAccessImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl OptionAccessImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {
            storage: std::cell::RefCell::new(std::collections::HashMap::new()),
            scheduled_operations: std::cell::RefCell::new(std::collections::HashMap::new()),
        }
    }

    /// java: public void putOption(String fieldName, Option<?> option)
    pub fn put_option(&self, field_name: &str, option: Arc<dyn AnyOption>) {
        self.storage.borrow_mut().insert(field_name.to_string(), option.clone());

        let consumer = self.scheduled_operations.borrow_mut().remove(field_name);
        if let Some(consumer) = consumer {
            consumer(option);
        }
    }

    /// java: public void checkBadOperations()
    pub fn check_bad_operations(&self) {
        let scheduled_operations = self.scheduled_operations.borrow();
        if !scheduled_operations.is_empty() {
            LOGGER.warn(&format!(
                "There are scheduled operations on the `OptionAccess` that tried to reference fields that do not exist. The following have been referenced that do not exist: {}",
                scheduled_operations.keys().cloned().collect::<Vec<String>>().join(", ")
            ));
        }
    }
}

impl OptionAccess for OptionAccessImpl {
    /// java: @Override public @Nullable Option<?> getOption(String fieldName)
    fn get_option(&self, field_name: &str) -> Option<Arc<dyn AnyOption>> {
        self.storage.borrow().get(field_name).cloned()
    }

    /// java: @Override public void scheduleOptionOperation(String fieldName, Consumer<Option<?>> optionConsumer)
    fn schedule_option_operation(&self, field_name: &str, option_consumer: Arc<dyn Fn(Arc<dyn AnyOption>)>) {
        let existing = self.storage.borrow().get(field_name).cloned();
        if let Some(option) = existing {
            option_consumer(option);
        } else {
            // java: scheduledOperations.merge(fieldName, optionConsumer, Consumer::andThen);
            let mut scheduled_operations = self.scheduled_operations.borrow_mut();
            let merged: Arc<dyn Fn(Arc<dyn AnyOption>)> = match scheduled_operations.remove(field_name) {
                Some(prev) => Arc::new(move |option: Arc<dyn AnyOption>| {
                    prev(option.clone());
                    option_consumer(option);
                }),
                None => option_consumer,
            };
            scheduled_operations.insert(field_name.to_string(), merged);
        }
    }
}
