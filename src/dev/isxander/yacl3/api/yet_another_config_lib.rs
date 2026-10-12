//! Ported from dev.isxander.yacl3.api.YetAnotherConfigLib.

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
    /// java: Component title();
    fn title(&self) -> MutableComponent;

    /// java: ImmutableList<ConfigCategory> categories();
    fn categories(&self) -> Vec<Arc<dyn ConfigCategory>>;

    /// java: Runnable saveFunction();
    fn save_function(&self) -> Arc<dyn Fn()>;

    /// java: Consumer<YACLScreen> initConsumer();
    fn init_consumer(&self) -> Arc<dyn Fn(&Arc<YACLScreen>)>;

    /// java: Screen generateScreen(@Nullable Screen parent);
    ///
    /// The screen is opened with `ScreenRef::set()` (Java's `minecraft.setScreen(..)`). `parent` is
    /// `Some(ScreenRef::Current)` for the screen that is up (the YACL screen returns to it on close), `None` for none.
    fn generate_screen(self: Arc<Self>, parent: Option<compat::ScreenRef>) -> compat::ScreenRef;
}

/// java: static Builder createBuilder()
pub fn create_builder() -> BuilderImpl {
    BuilderImpl::new()
}

/// java: static <T> YetAnotherConfigLib create(ConfigClassHandler<T> configHandler, ConfigBackedBuilder<T> builder)
pub fn create<T: 'static>(
    config_handler: Arc<dyn ConfigClassHandler<T>>,
    builder: &dyn ConfigBackedBuilder<compat::Shared<T>>,
) -> Arc<dyn YetAnotherConfigLib> {
    let handler = config_handler.clone();
    builder.build(config_handler.defaults(), config_handler.instance(), create_builder().save(move || handler.save())).build()
}

/// java: @Deprecated static <T> YetAnotherConfigLib create(ConfigInstance<T> configInstance, ConfigBackedBuilder<T> builder)
pub fn create_config_instance_config_backed_builder<T: 'static>(
    config_instance: Arc<dyn ConfigInstance<T>>,
    builder: &dyn ConfigBackedBuilder<compat::Shared<T>>,
) -> Arc<dyn YetAnotherConfigLib> {
    let instance = config_instance.clone();
    builder.build(config_instance.get_defaults(), config_instance.get_config(), create_builder().save(move || instance.save())).build()
}

/// java: interface Builder (setters by value, Java's `return this`)
pub trait Builder: Sized {
    /// java: Builder title(@NotNull Component title);
    fn title(self, title: MutableComponent) -> Self;

    /// java: Builder category(@NotNull ConfigCategory category);
    fn category(self, category: Arc<dyn ConfigCategory>) -> Self;

    /// java: default Builder category(@NotNull Supplier<@NotNull ConfigCategory> categorySupplier)
    fn category_supplier(self, category_supplier: impl FnOnce() -> Arc<dyn ConfigCategory>) -> Self {
        self.category(category_supplier())
    }

    /// java: default Builder categoryIf(boolean condition, @NotNull ConfigCategory category)
    fn category_if(self, condition: bool, category: Arc<dyn ConfigCategory>) -> Self {
        if condition { self.category(category) } else { self }
    }

    /// java: default Builder categoryIf(boolean condition, @NotNull Supplier<@NotNull ConfigCategory> categorySupplier)
    fn category_if_boolean_supplier(self, condition: bool, category_supplier: impl FnOnce() -> Arc<dyn ConfigCategory>) -> Self {
        if condition { self.category_supplier(category_supplier) } else { self }
    }

    /// java: Builder categories(@NotNull Collection<? extends @NotNull ConfigCategory> categories);
    fn categories(self, categories: Vec<Arc<dyn ConfigCategory>>) -> Self;

    /// java: default Builder categories(@NotNull Supplier<@NotNull Collection<? extends @NotNull ConfigCategory>> categoriesSupplier)
    fn categories_supplier(self, categories_supplier: impl FnOnce() -> Vec<Arc<dyn ConfigCategory>>) -> Self {
        self.categories(categories_supplier())
    }

    /// java: default Builder categoriesIf(boolean condition, @NotNull Collection<? extends @NotNull ConfigCategory> categories)
    fn categories_if(self, condition: bool, categories: Vec<Arc<dyn ConfigCategory>>) -> Self {
        if condition { self.categories(categories) } else { self }
    }

    /// java: default Builder categoriesIf(boolean condition, @NotNull Supplier<@NotNull Collection<..>> categoriesSupplier)
    fn categories_if_boolean_supplier(self, condition: bool, categories_supplier: impl FnOnce() -> Vec<Arc<dyn ConfigCategory>>) -> Self {
        if condition { self.categories_supplier(categories_supplier) } else { self }
    }

    /// java: Builder save(@NotNull Runnable saveFunction);
    fn save(self, save_function: impl Fn() + 'static) -> Self;

    /// java: Builder screenInit(@NotNull Consumer<YACLScreen> initConsumer);
    fn screen_init(self, init_consumer: impl Fn(&Arc<YACLScreen>) + 'static) -> Self;

    /// java: YetAnotherConfigLib build();
    fn build(self) -> Arc<dyn YetAnotherConfigLib>;
}

/// java: @FunctionalInterface interface ConfigBackedBuilder<T> (any `Fn(T, T, Builder) -> Builder` is one)
pub trait ConfigBackedBuilder<T> {
    /// java: YetAnotherConfigLib.Builder build(T defaults, T config, YetAnotherConfigLib.Builder builder);
    fn build(&self, defaults: T, config: T, builder: BuilderImpl) -> BuilderImpl;
}

impl<T, F: Fn(T, T, BuilderImpl) -> BuilderImpl> ConfigBackedBuilder<T> for F {
    fn build(&self, defaults: T, config: T, builder: BuilderImpl) -> BuilderImpl {
        self(defaults, config, builder)
    }
}
