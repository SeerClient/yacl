//! Ported from dev.isxander.yacl3.impl.controller.ItemControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::item_controller_builder::ItemControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::dropdown::item_controller::ItemController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;
use mcrust::net::minecraft::world::item::item::Item;

/// java: public class ItemControllerBuilderImpl extends AbstractControllerBuilderImpl<Item> implements ItemControllerBuilder
#[derive(Clone)]
pub struct ItemControllerBuilderImpl {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<Arc<Item>>,
}

impl std::ops::Deref for ItemControllerBuilderImpl {
    type Target = AbstractControllerBuilderImplBase<Arc<Item>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemControllerBuilderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemControllerBuilderImpl {
    /// java: public ItemControllerBuilderImpl(Option<Item> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Arc<Item>>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractControllerBuilderImplBase::<Arc<Item>>::new(option.clone()),
        });
        __this
    }

}

impl AbstractControllerBuilderImpl<Arc<mcrust::net::minecraft::world::item::item::Item>> for ItemControllerBuilderImpl {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<Arc<mcrust::net::minecraft::world::item::item::Item>> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<Arc<mcrust::net::minecraft::world::item::item::Item>> {
        &mut self.base
    }

}

impl ControllerBuilder<compat::Object> for ItemControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<Item> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<Arc<Item>>> {
        return (ItemController::new(self.option.clone()) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl ItemControllerBuilder for ItemControllerBuilderImpl {
}
