//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.ItemFieldImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::item_controller_builder;
use crate::dev::isxander::yacl3::api::option;
use crate::dev::isxander::yacl3::api::option_description::Builder;
use crate::dev::isxander::yacl3::config::v2::api::autogen::item_field::ItemField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactoryBase;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::AutoGenUtils;
use crate::dev::isxander::yacl3::config::v2::r#impl::field_backed_binding::FieldBackedBinding;
use mcrust::net::minecraft::world::item::item::Item;
use crate::dev::isxander::yacl3::api::option::Builder as _;

/// java: public class ItemFieldImpl extends SimpleOptionFactory<ItemField, Item>
#[derive(Clone)]
pub struct ItemFieldImpl {
    /// java: extends SimpleOptionFactory
    pub base: SimpleOptionFactoryBase<ItemField, Arc<Item>>,
}

impl std::ops::Deref for ItemFieldImpl {
    type Target = SimpleOptionFactoryBase<ItemField, Arc<Item>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ItemFieldImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ItemFieldImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: SimpleOptionFactoryBase::<ItemField, Arc<Item>>::new(),
        };
        this
    }

}

impl SimpleOptionFactory<crate::dev::isxander::yacl3::config::v2::api::autogen::item_field::ItemField, Arc<mcrust::net::minecraft::world::item::item::Item>> for ItemFieldImpl {
    fn simple_option_factory_base(&self) -> &SimpleOptionFactoryBase<crate::dev::isxander::yacl3::config::v2::api::autogen::item_field::ItemField, Arc<mcrust::net::minecraft::world::item::item::Item>> {
        &self.base
    }
    fn simple_option_factory_base_mut(&mut self) -> &mut SimpleOptionFactoryBase<crate::dev::isxander::yacl3::config::v2::api::autogen::item_field::ItemField, Arc<mcrust::net::minecraft::world::item::item::Item>> {
        &mut self.base
    }

    /// java: @Override protected ControllerBuilder<Item> createController(ItemField annotation, ConfigField<Item> field, OptionAccess storage, Option<Item> option)
    fn create_controller(&self, annotation: &ItemField, field: &compat::Shared<dyn ConfigField<Arc<Item>>>, storage: &compat::Shared<dyn OptionAccess>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Arc<Item>>>) -> compat::Shared<dyn ControllerBuilder<Arc<Item>>> {
        return (item_controller_builder::create(option) as compat::Shared<dyn ControllerBuilder<compat::Object>>);
    }

}

impl OptionFactory<compat::Object, compat::Object> for ItemFieldImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess)
    fn create_option(&self, annotation: &A, field: &compat::Shared<dyn ConfigField<T>>, option_access: &compat::Shared<dyn OptionAccess>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>> {
        // TODO(fabric2seer): `.pendingValue(..)` on a value of unknown type
        //   java: Option<T> option = Option.<T>createBuilder()
        //   java:                 .name(this.name(annotation, field, optionAccess))
        //   java:                 .description(v -> this.description(v, annotation, field, optionAccess).build())
        //   java:                 .binding(new FieldBackedBinding<>(field.access(), field.defaultAccess()))
        //   java:                 .controller(opt -> {
        //   java:                     ControllerBuilder<T> builder = this.createController(annotation, field, optionAccess, opt);
        //   java:
        //   java:                     AutoGenUtils.addCustomFormatterToController(builder, field.access());
        //   java:
        //   java:                     return builder;
        //   java:                 })
        //   java:                 .available(this.available(annotation, field, optionAccess))
        //   java:                 .flags(this.flags(annotation, field, optionAccess))
        //   java:                 .addListener((opt, event) -> this.listener(annotation, field, optionAccess, opt, opt.pendingValue()))
        //   java:                 .build();
        let mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>> = todo!("fabric2seer: the initialiser of `option` (TODO above)");
        self.post_init(annotation, field, option_access, &option);
        return option;
    }

}
