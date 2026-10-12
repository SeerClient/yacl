//! Ported from dev.isxander.yacl3.impl.controller.ItemControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::item_controller_builder::ItemControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::item_controller::ItemController;
use mcrust::net::minecraft::world::item::item::Item;

/// java: public class ItemControllerBuilderImpl extends AbstractControllerBuilderImpl<Item> implements ItemControllerBuilder
pub struct ItemControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<compat::JavaRef<Item>>>,
}

impl ItemControllerBuilderImpl {
    /// java: public ItemControllerBuilderImpl(Option<Item> option)
    pub fn new(option: Arc<dyn YOption<compat::JavaRef<Item>>>) -> Self {
        Self { option }
    }
}

impl ControllerBuilder<compat::JavaRef<Item>> for ItemControllerBuilderImpl {
    /// java: @Override public Controller<Item> build()
    fn build(&self) -> Arc<dyn Controller<compat::JavaRef<Item>>> {
        ItemController::new(self.option.clone())
    }
}

impl ItemControllerBuilder for ItemControllerBuilderImpl {}
