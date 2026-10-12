//! Ported from dev.isxander.yacl3.api.utils.OptionUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::api::list_option::ListOption;
use crate::dev::isxander::yacl3::api::option_group::OptionGroup;
use crate::dev::isxander::yacl3::api::yet_another_config_lib::YetAnotherConfigLib;

/// java: public class OptionUtils
#[derive(Clone)]
pub struct OptionUtils {}

impl OptionUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static Stream<Option<?>> getFlatOptions(YetAnotherConfigLib yacl)
    pub fn get_flat_options(yacl: &compat::Shared<dyn YetAnotherConfigLib>) -> Box<dyn Iterator<Item = compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>> + '_> {
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: return yacl.categories().stream()
        //   java:                 .flatMap(category -> category.groups().stream())
        //   java:                 .flatMap(group -> group instanceof ListOption<?> list
        //   java:                         ? Stream.of(list)
        //   java:                         : group.options().stream());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static void consumeOptions(YetAnotherConfigLib yacl, Function<Option<?>, Boolean> consumer)
    pub fn consume_options(yacl: &compat::Shared<dyn YetAnotherConfigLib>, consumer: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>) -> bool>) {
        for category in ({ let __f = compat::dispatch(yacl, |__t| __t.categories()); __f(yacl) }) {
            for group in ({ let __f = compat::dispatch(&category, |__t| __t.groups()); __f(&category) }) {
                if let Some(list) = group.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn ListOption<compat::Object>>>() {
                    if (consumer)(list.clone()) {
                        return;
                    }
                } else {
                    for option in ({ let __f = compat::dispatch(&group, |__t| __t.options()); __f(&group) }) {
                        if (consumer)(option.clone()) {
                            return;
                        }
                    }
                }
            }
        }
    }

    /// java: public static void forEachOptions(YetAnotherConfigLib yacl, Consumer<Option<?>> consumer)
    pub fn for_each_options(yacl: &compat::Shared<dyn YetAnotherConfigLib>, consumer: Arc<dyn Fn(compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<compat::Object>>)>) {
        OptionUtils::consume_options(yacl, Arc::new(move |opt| {
            (consumer)(opt.clone());
            return false;
        }));
    }

}
