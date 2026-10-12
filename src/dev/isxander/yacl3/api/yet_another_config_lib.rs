//! Ported from dev.isxander.yacl3.api.YetAnotherConfigLib by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::config::config_instance::ConfigInstance;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::yet_another_config_lib_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface YetAnotherConfigLib
pub trait YetAnotherConfigLib: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Component title(); (dispatch: the body for this class)
    fn title(&self) -> fn(&compat::Shared<dyn YetAnotherConfigLib>) -> MutableComponent;

    /// java: ImmutableList<ConfigCategory> categories(); (dispatch: the body for this class)
    fn categories(&self) -> fn(&compat::Shared<dyn YetAnotherConfigLib>) -> Vec<compat::Shared<dyn ConfigCategory>>;

    /// java: Runnable saveFunction(); (dispatch: the body for this class)
    fn save_function(&self) -> fn(&compat::Shared<dyn YetAnotherConfigLib>) -> Arc<dyn Fn()>;

    /// java: Consumer<YACLScreen> initConsumer(); (dispatch: the body for this class)
    fn init_consumer(&self) -> fn(&compat::Shared<dyn YetAnotherConfigLib>) -> Arc<dyn Fn(compat::Shared<YACLScreen>)>;

    /// java: Screen generateScreen(@Nullable Screen parent); (dispatch: the body for this class)
    fn generate_screen(&self) -> fn(&compat::Shared<dyn YetAnotherConfigLib>, Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen;

}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (BuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: static <T> YetAnotherConfigLib create(ConfigClassHandler<T> configHandler, ConfigBackedBuilder<T> builder)
pub fn create<T: 'static>(config_handler: &compat::Shared<dyn ConfigClassHandler<T>>, builder: &compat::Shared<dyn ConfigBackedBuilder<T>>) -> compat::Shared<dyn YetAnotherConfigLib> {
    return ({ let __r = &{ let (__a0, __a1, __a2,) = (&config_handler.lock().borrow().defaults(), &config_handler.lock().borrow().instance(), &({ let __r = &create_builder(); let __f = compat::dispatch(__r, |__t| __t.save()); __f(__r, Arc::new(move || { config_handler.lock().borrow().save(); })) }),); builder.lock().borrow().build(__a0, __a1, __a2) }; let __f = compat::dispatch(__r, |__t| __t.build()); __f(__r) });
}

/// java: @Deprecated static <T> YetAnotherConfigLib create(ConfigInstance<T> configInstance, ConfigBackedBuilder<T> builder)
pub fn create_config_instance_config_backed_builder<T: 'static>(config_instance: &compat::Shared<dyn ConfigInstance<T>>, builder: &compat::Shared<dyn ConfigBackedBuilder<T>>) -> compat::Shared<dyn YetAnotherConfigLib> {
    return ({ let __r = &{ let (__a0, __a1, __a2,) = (&config_instance.lock().borrow().get_defaults(), &config_instance.lock().borrow().get_config(), &({ let __r = &create_builder(); let __f = compat::dispatch(__r, |__t| __t.save()); __f(__r, Arc::new(move || { config_instance.lock().borrow().save(); })) }),); builder.lock().borrow().build(__a0, __a1, __a2) }; let __f = compat::dispatch(__r, |__t| __t.build()); __f(__r) });
}

/// java: interface Builder
pub trait Builder: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Builder title(@NotNull Component title); (dispatch: the body for this class)
    fn title(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder category(@NotNull ConfigCategory category); (dispatch: the body for this class)
    fn category(&self) -> fn(&compat::Shared<dyn Builder>, compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn Builder>;

    /// java: default Builder category(@NotNull Supplier<@NotNull ConfigCategory> categorySupplier) (dispatch: the body for this class)
    fn category_supplier(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn() -> compat::Shared<dyn ConfigCategory>>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::category_supplier_body
    }

    /// java: default Builder categoryIf(boolean condition, @NotNull ConfigCategory category) (dispatch: the body for this class)
    fn category_if(&self) -> fn(&compat::Shared<dyn Builder>, bool, &compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::category_if_body
    }

    /// java: default Builder categoryIf(boolean condition, @NotNull Supplier<@NotNull ConfigCategory> categorySupplier) (dispatch: the body for this class)
    fn category_if_boolean_supplier(&self) -> fn(&compat::Shared<dyn Builder>, bool, Arc<dyn Fn() -> compat::Shared<dyn ConfigCategory>>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::category_if_boolean_supplier_body
    }

    /// java: Builder categories(@NotNull Collection<? extends @NotNull ConfigCategory> categories); (dispatch: the body for this class)
    fn categories(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<compat::Shared<dyn ConfigCategory>>) -> compat::Shared<dyn Builder>;

    /// java: default Builder categories(@NotNull Supplier<@NotNull Collection<? extends @NotNull ConfigCategory>> categoriesSupplier) (dispatch: the body for this class)
    fn categories_supplier(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn() -> Vec<compat::Shared<dyn ConfigCategory>>>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::categories_supplier_body
    }

    /// java: default Builder categoriesIf(boolean condition, @NotNull Collection<? extends @NotNull ConfigCategory> categories) (dispatch: the body for this class)
    fn categories_if(&self) -> fn(&compat::Shared<dyn Builder>, bool, &Vec<compat::Shared<dyn ConfigCategory>>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::categories_if_body
    }

    /// java: default Builder categoriesIf(boolean condition, @NotNull Supplier<@NotNull Collection<? extends @NotNull ConfigCategory>> categoriesSupplier) (dispatch: the body for this class)
    fn categories_if_boolean_supplier(&self) -> fn(&compat::Shared<dyn Builder>, bool, Arc<dyn Fn() -> Vec<compat::Shared<dyn ConfigCategory>>>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::categories_if_boolean_supplier_body
    }

    /// java: Builder save(@NotNull Runnable saveFunction); (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn()>) -> compat::Shared<dyn Builder>;

    /// java: Builder screenInit(@NotNull Consumer<YACLScreen> initConsumer); (dispatch: the body for this class)
    fn screen_init(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn(compat::Shared<YACLScreen>)>) -> compat::Shared<dyn Builder>;

    /// java: YetAnotherConfigLib build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn YetAnotherConfigLib>;

}

impl dyn Builder {
    /// java: default Builder category(@NotNull Supplier<@NotNull ConfigCategory> categorySupplier)
    pub fn category_supplier_body(__this: &compat::Shared<dyn Builder>, category_supplier: Arc<dyn Fn() -> compat::Shared<dyn ConfigCategory>>) -> compat::Shared<dyn Builder> {
        return ({ let __f = compat::dispatch(__this, |__t| __t.category()); __f(__this, (category_supplier)()) });
    }

    /// java: default Builder categoryIf(boolean condition, @NotNull ConfigCategory category)
    pub fn category_if_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, category: &compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn Builder> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.category()); __f(__this, category.clone()) }) } else { __this.clone() });
    }

    /// java: default Builder categoryIf(boolean condition, @NotNull Supplier<@NotNull ConfigCategory> categorySupplier)
    pub fn category_if_boolean_supplier_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, category_supplier: Arc<dyn Fn() -> compat::Shared<dyn ConfigCategory>>) -> compat::Shared<dyn Builder> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.category_supplier()); __f(__this, category_supplier.clone()) }) } else { __this.clone() });
    }

    /// java: default Builder categories(@NotNull Supplier<@NotNull Collection<? extends @NotNull ConfigCategory>> categoriesSupplier)
    pub fn categories_supplier_body(__this: &compat::Shared<dyn Builder>, categories_supplier: Arc<dyn Fn() -> Vec<compat::Shared<dyn ConfigCategory>>>) -> compat::Shared<dyn Builder> {
        return ({ let __f = compat::dispatch(__this, |__t| __t.categories()); __f(__this, &(categories_supplier)()) });
    }

    /// java: default Builder categoriesIf(boolean condition, @NotNull Collection<? extends @NotNull ConfigCategory> categories)
    pub fn categories_if_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, categories: &Vec<compat::Shared<dyn ConfigCategory>>) -> compat::Shared<dyn Builder> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.categories()); __f(__this, categories) }) } else { __this.clone() });
    }

    /// java: default Builder categoriesIf(boolean condition, @NotNull Supplier<@NotNull Collection<? extends @NotNull ConfigCategory>> categoriesSupplier)
    pub fn categories_if_boolean_supplier_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, categories_supplier: Arc<dyn Fn() -> Vec<compat::Shared<dyn ConfigCategory>>>) -> compat::Shared<dyn Builder> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.categories_supplier()); __f(__this, categories_supplier.clone()) }) } else { __this.clone() });
    }

}

/// java: @FunctionalInterface interface ConfigBackedBuilder<T>
pub trait ConfigBackedBuilder<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: YetAnotherConfigLib.Builder build(T defaults, T config, YetAnotherConfigLib.Builder builder);
    fn build(&self, defaults: &T, config: &T, builder: &compat::Shared<dyn Builder>) -> compat::Shared<dyn Builder>;

}
