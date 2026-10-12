//! Ported from dev.isxander.yacl3.api.controller.ItemControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::item_controller_builder_impl::ItemControllerBuilderImpl;
use mcrust::net::minecraft::world::item::item::Item;

/// java: public interface ItemControllerBuilder extends ControllerBuilder<Item>
pub trait ItemControllerBuilder: ControllerBuilder + compat::DynAny {
}

/// java: static ItemControllerBuilder create(Option<Item> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Arc<Item>>>) -> compat::Shared<dyn ItemControllerBuilder> {
    return (ItemControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn ItemControllerBuilder>);
}
