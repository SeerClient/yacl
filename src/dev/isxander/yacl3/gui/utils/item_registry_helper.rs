//! Ported from dev.isxander.yacl3.gui.utils.ItemRegistryHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::net::minecraft::core::registry::Registry;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::world::item::items;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public final class ItemRegistryHelper
#[derive(Clone)]
pub struct ItemRegistryHelper {}

impl ItemRegistryHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean isRegisteredItem(String identifier)
    pub fn is_registered_item(identifier: &str) -> bool {
        // java: try {..} catch (IdentifierException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut item_identifier: Identifier = YACLPlatform::parse_rl(&identifier.to_lowercase());
            return mcrust::net::minecraft::core::registries::built_in_registries::item().contains_key(&item_identifier);
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                return false;
            }
        }
    }

    /// java: public static Item getItemFromName(String identifier, Item defaultItem)
    pub fn get_item_from_name(identifier: &str, default_item: &Arc<Item>) -> Arc<Item> {
        // java: try {..} catch (IdentifierException ignored) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            let mut item_identifier: Identifier = YACLPlatform::parse_rl(&identifier.to_lowercase());
            if mcrust::net::minecraft::core::registries::built_in_registries::item().contains_key(&item_identifier) {
                return ItemRegistryHelper::get_from_registry(mcrust::net::minecraft::core::registries::built_in_registries::item(), &item_identifier);
            }
        }
        // TODO(fabric2seer): the `catch (IdentifierException ignored)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java:         }
        return default_item.clone();
    }

    /// java: public static Item getItemFromName(String identifier)
    pub fn get_item_from_name_1(identifier: &str) -> Arc<Item> {
        return ItemRegistryHelper::get_item_from_name(identifier, &items::items().get("air"));
    }

    /// java: public static Stream<Identifier> getMatchingItemIdentifiers(String value)
    pub fn get_matching_item_identifiers(value: &str) -> Box<dyn Iterator<Item = Identifier> + '_> {
        let mut sep: i32 = compat::index_of(&value, mcrust::net::minecraft::resources::identifier::NAMESPACE_SEPARATOR);
        let mut filter_predicate: Arc<dyn Fn(Identifier) -> bool>;
        if sep == -1 {
            filter_predicate = Arc::new(move |identifier| {
                if identifier.path().contains(value) {
                    return true;
                }
                let mut item: Arc<Item> = ItemRegistryHelper::get_from_registry(mcrust::net::minecraft::core::registries::built_in_registries::item(), &identifier);
                return compat::component_of(item.get_name(&item.get_default_instance())).get_string().to_lowercase().contains(&value.to_lowercase());
            });
        } else {
            let mut namespace: String = compat::substring2(&value, 0, sep);
            let mut path: String = compat::substring(&value, sep.wrapping_add(1));
            filter_predicate = Arc::new(move |identifier| (identifier.namespace() == namespace && identifier.path().starts_with(&path)));
        }
        // TODO(fabric2seer): not a lambda
        //   java: return BuiltInRegistries.ITEM.keySet().stream()
        //   java:                 .filter(filterPredicate)
        //   java:                 /*
        //   java:                  Sort items as follows based on the given "value" string's path:
        //   java:                  - if both items' paths begin with the entered string, sort the identifiers (including namespace)
        //   java:                  - otherwise, if either of the items' path begins with the entered string, sort it to the left
        //   java:                  - else neither path matches: sort by identifiers again
        //   java:
        //   java:                  This allows the user to enter "diamond_ore" and match "minecraft:diamond_ore" before
        //   java:                  "minecraft:deepslate_diamond_ore", even though the second is lexicographically smaller
        //   java:                  */
        //   java:                 .sorted((id1, id2) -> {
        //   java:                     String path = (sep == -1 ? value : value.substring(sep + 1)).toLowerCase();
        //   java:                     boolean id1StartsWith = id1.getPath().toLowerCase().startsWith(path);
        //   java:                     boolean id2StartsWith = id2.getPath().toLowerCase().startsWith(path);
        //   java:                     if (id1StartsWith) {
        //   java:                         if (id2StartsWith) {
        //   java:                             return id1.compareTo(id2);
        //   java:                         }
        //   java:                         return -1;
        //   java:                     }
        //   java:                     if (id2StartsWith) {
        //   java:                         return 1;
        //   java:                     }
        //   java:                     return id1.compareTo(id2);
        //   java:                 });
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static <T> T getFromRegistry(Registry<T> registry, Identifier identifier)
    pub fn get_from_registry<T: 'static>(registry: &Arc<dyn Registry<T>>, identifier: &Identifier) -> T {
        return registry.get_value_by_identifier(identifier).unwrap();
    }

}
