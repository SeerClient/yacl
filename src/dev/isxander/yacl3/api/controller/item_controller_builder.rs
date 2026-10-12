//! Ported from dev.isxander.yacl3.api.controller.ItemControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::item_controller_builder_impl::ItemControllerBuilderImpl;
use mcrust::net::minecraft::world::item::item::Item;

/// java: public interface ItemControllerBuilder extends ControllerBuilder<Item>
/// (an `Item` value is `compat::JavaRef<Item>`: the registered object, compared by identity as Java's `equals` does)
pub trait ItemControllerBuilder: ControllerBuilder<compat::JavaRef<Item>> {}

/// java: static ItemControllerBuilder create(Option<Item> option)
pub fn create(option: Arc<dyn YOption<compat::JavaRef<Item>>>) -> ItemControllerBuilderImpl {
    ItemControllerBuilderImpl::new(option)
}
