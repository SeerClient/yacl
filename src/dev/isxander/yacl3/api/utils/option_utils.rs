//! Ported from dev.isxander.yacl3.api.utils.OptionUtils.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::AnyOption;
use crate::dev::isxander::yacl3::api::yet_another_config_lib::YetAnotherConfigLib;

/// java: public class OptionUtils
pub struct OptionUtils;

impl OptionUtils {
    /// java: public static Stream<Option<?>> getFlatOptions(YetAnotherConfigLib yacl)
    pub fn get_flat_options(yacl: &dyn YetAnotherConfigLib) -> Vec<Arc<dyn AnyOption>> {
        let mut out: Vec<Arc<dyn AnyOption>> = Vec::new();
        for category in yacl.categories() {
            for group in category.groups() {
                match group.as_list_option() {
                    Some(list) => out.push(list),
                    None => out.extend(group.options()),
                }
            }
        }
        out
    }

    /// java: public static void consumeOptions(YetAnotherConfigLib yacl, Function<Option<?>, Boolean> consumer)
    pub fn consume_options(yacl: &dyn YetAnotherConfigLib, mut consumer: impl FnMut(&Arc<dyn AnyOption>) -> bool) {
        for category in yacl.categories() {
            for group in category.groups() {
                if let Some(list) = group.as_list_option() {
                    let list: Arc<dyn AnyOption> = list;
                    if consumer(&list) {
                        return;
                    }
                } else {
                    for option in group.options() {
                        if consumer(&option) {
                            return;
                        }
                    }
                }
            }
        }
    }

    /// java: public static void forEachOptions(YetAnotherConfigLib yacl, Consumer<Option<?>> consumer)
    pub fn for_each_options(yacl: &dyn YetAnotherConfigLib, mut consumer: impl FnMut(&Arc<dyn AnyOption>)) {
        OptionUtils::consume_options(yacl, |opt| {
            consumer(opt);
            false
        });
    }
}
