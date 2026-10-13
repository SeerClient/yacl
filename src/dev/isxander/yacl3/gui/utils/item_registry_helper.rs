//! Ported from dev.isxander.yacl3.gui.utils.ItemRegistryHelper.
//!
//! An `Item` is answered as `compat::JavaRef<Item>`: the registry's own `Arc<Item>`, compared by identity (Java's
//! `Item.equals` is `Object.equals`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::com::mojang::serialization::cast;
use mcrust::net::minecraft::core::registries::built_in_registries;
use mcrust::net::minecraft::core::registry::Registry as _;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::resources::identifier::{self, Identifier};
use mcrust::net::minecraft::world::item::item::{ErasedItem, Item};
use mcrust::net::minecraft::world::item::items;

/// java: public final class ItemRegistryHelper
pub struct ItemRegistryHelper;

impl ItemRegistryHelper {
    /// java: public static boolean isRegisteredItem(String identifier)
    ///
    /// `Identifier.parse` throwing `IdentifierException` is `try_parse` answering `None` (the catch returns false).
    pub fn is_registered_item(identifier: &str) -> bool {
        match Identifier::try_parse(&identifier.to_lowercase()) {
            Some(item_identifier) => built_in_registries::item().contains_identifier(&item_identifier),
            None => false,
        }
    }

    /// java: public static Item getItemFromName(String identifier, Item defaultItem)
    pub fn get_item_from_name(identifier: &str, default_item: Option<&compat::JavaRef<Item>>) -> Option<compat::JavaRef<Item>> {
        // try { .. } catch (IdentifierException ignored) {}
        if let Some(item_identifier) = Identifier::try_parse(&identifier.to_lowercase())
            && built_in_registries::item().contains_identifier(&item_identifier)
        {
            return ItemRegistryHelper::get_from_registry(&item_identifier);
        }
        default_item.cloned()
    }

    /// java: public static Item getItemFromName(String identifier) - `getItemFromName(identifier, Items.AIR)`
    pub fn get_item_from_name_1(identifier: &str) -> compat::JavaRef<Item> {
        let air = compat::JavaRef::new(items::air());
        ItemRegistryHelper::get_item_from_name(identifier, Some(&air)).unwrap_or(air)
    }

    /// java: public static Stream<Identifier> getMatchingItemIdentifiers(String value)
    pub fn get_matching_item_identifiers(value: &str) -> Vec<Identifier> {
        let sep = compat::index_of(value, identifier::NAMESPACE_SEPARATOR);
        let filter_predicate: Box<dyn Fn(&Identifier) -> bool + '_> = if sep == -1 {
            Box::new(move |identifier: &Identifier| {
                if identifier.path().contains(value) {
                    return true;
                }
                match ItemRegistryHelper::get_from_registry(identifier) {
                    Some(item) => {
                        let name = item.get_name(&item.0.get_default_instance());
                        compat::component_of(name).get_string().to_lowercase().contains(&value.to_lowercase())
                    }
                    None => false,
                }
            })
        } else {
            let namespace = compat::substring2(value, 0, sep);
            let path = compat::substring(value, sep + 1);
            Box::new(move |identifier: &Identifier| identifier.namespace() == namespace && identifier.path().starts_with(&path))
        };
        let mut identifiers: Vec<Identifier> =
            built_in_registries::item().key_set().into_iter().filter(|identifier| filter_predicate(identifier)).collect();
        /*
         Sort items as follows based on the given "value" string's path:
         - if both items' paths begin with the entered string, sort the identifiers (including namespace)
         - otherwise, if either of the items' path begins with the entered string, sort it to the left
         - else neither path matches: sort by identifiers again

         This allows the user to enter "diamond_ore" and match "minecraft:diamond_ore" before
         "minecraft:deepslate_diamond_ore", even though the second is lexicographically smaller
         */
        let path = (if sep == -1 { value.to_string() } else { compat::substring(value, sep + 1) }).to_lowercase();
        identifiers.sort_by(|id1, id2| {
            let id1_starts_with = id1.path().to_lowercase().starts_with(&path);
            let id2_starts_with = id2.path().to_lowercase().starts_with(&path);
            if id1_starts_with {
                if id2_starts_with {
                    return id1.cmp(id2);
                }
                return std::cmp::Ordering::Less;
            }
            if id2_starts_with {
                return std::cmp::Ordering::Greater;
            }
            id1.cmp(id2)
        });
        identifiers
    }

    /// java: public static <T> T getFromRegistry(Registry<T> registry, Identifier identifier)
    ///
    /// `registry.getValue(identifier)` (nullable) on `BuiltInRegistries.ITEM`, the only registry YACL passes.
    pub fn get_from_registry(identifier: &Identifier) -> Option<compat::JavaRef<Item>> {
        let value = built_in_registries::item().get_value_by_identifier(identifier)?;
        cast::<ErasedItem>(&value).map(|ErasedItem(item)| compat::JavaRef::new(item.clone()))
    }
}
