//! Ported from dev.isxander.yacl3.impl.YetAnotherConfigLibImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::api::yet_another_config_lib::{Builder, YetAnotherConfigLib};
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use crate::dev::isxander::yacl3::r#impl::utils::yacl_constants::YACLConstants;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::cell::Cell;

/// java: @ApiStatus.Internal public final class YetAnotherConfigLibImpl implements YetAnotherConfigLib
pub struct YetAnotherConfigLibImpl {
    /// java: private final Component title;
    pub title: MutableComponent,
    /// java: private final ImmutableList<ConfigCategory> categories;
    pub categories: Vec<Arc<dyn ConfigCategory>>,
    /// java: private final Runnable saveFunction;
    pub save_function: Arc<dyn Fn()>,
    /// java: private final Consumer<YACLScreen> initConsumer;
    pub init_consumer: Arc<dyn Fn(&Arc<YACLScreen>)>,
    /// java: private boolean generated = false;
    pub generated: Cell<bool>,
}

impl YetAnotherConfigLibImpl {
    /// java: public YetAnotherConfigLibImpl(Component title, ImmutableList<ConfigCategory> categories, Runnable saveFunction, Consumer<YACLScreen> initConsumer)
    pub fn new(
        title: MutableComponent,
        categories: Vec<Arc<dyn ConfigCategory>>,
        save_function: Arc<dyn Fn()>,
        init_consumer: Arc<dyn Fn(&Arc<YACLScreen>)>,
    ) -> Self {
        Self { title, categories, save_function, init_consumer, generated: Cell::new(false) }
    }
}

impl YetAnotherConfigLib for YetAnotherConfigLibImpl {
    /// java: @Override public Screen generateScreen(Screen parent)
    fn generate_screen(self: Arc<Self>, parent: Option<compat::ScreenRef>) -> compat::ScreenRef {
        if self.generated.get() {
            panic!(
                "UnsupportedOperationException: To prevent memory leaks, you should only generate a Screen once per instance. Please re-build the instance to generate another GUI."
            );
        }
        YACLConstants::logger().info("Generating YACL screen");
        self.generated.set(true);
        let config: Arc<dyn YetAnotherConfigLib> = self;
        YACLScreen::new_screen(config, parent)
    }

    /// java: @Override public Component title()
    fn title(&self) -> MutableComponent {
        self.title.clone()
    }

    /// java: @Override public ImmutableList<ConfigCategory> categories()
    fn categories(&self) -> Vec<Arc<dyn ConfigCategory>> {
        self.categories.clone()
    }

    /// java: @Override public Runnable saveFunction()
    fn save_function(&self) -> Arc<dyn Fn()> {
        self.save_function.clone()
    }

    /// java: @Override public Consumer<YACLScreen> initConsumer()
    fn init_consumer(&self) -> Arc<dyn Fn(&Arc<YACLScreen>)> {
        self.init_consumer.clone()
    }
}

/// java: @ApiStatus.Internal public static final class BuilderImpl implements Builder
pub struct BuilderImpl {
    /// java: private Component title;
    pub title: Option<MutableComponent>,
    /// java: private final List<ConfigCategory> categories = new ArrayList<>();
    pub categories: Vec<Arc<dyn ConfigCategory>>,
    /// java: private Runnable saveFunction = () -> {};
    pub save_function: Arc<dyn Fn()>,
    /// java: private Consumer<YACLScreen> initConsumer = screen -> {};
    pub init_consumer: Arc<dyn Fn(&Arc<YACLScreen>)>,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {
            title: None,
            categories: Vec::new(),
            save_function: Arc::new(|| {}),
            init_consumer: Arc::new(|_screen: &Arc<YACLScreen>| {}),
        }
    }
}

impl Default for BuilderImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder for BuilderImpl {
    /// java: @Override public Builder title(@NotNull Component title)
    fn title(mut self, title: MutableComponent) -> Self {
        self.title = Some(title);
        self
    }

    /// java: @Override public Builder category(@NotNull ConfigCategory category)
    fn category(mut self, category: Arc<dyn ConfigCategory>) -> Self {
        self.categories.push(category);
        self
    }

    /// java: @Override public Builder categories(@NotNull Collection<? extends @NotNull ConfigCategory> categories)
    fn categories(mut self, categories: Vec<Arc<dyn ConfigCategory>>) -> Self {
        self.categories.extend(categories);
        self
    }

    /// java: @Override public Builder save(@NotNull Runnable saveFunction)
    fn save(mut self, save_function: impl Fn() + 'static) -> Self {
        self.save_function = Arc::new(save_function);
        self
    }

    /// java: @Override public Builder screenInit(@NotNull Consumer<YACLScreen> initConsumer)
    fn screen_init(mut self, init_consumer: impl Fn(&Arc<YACLScreen>) + 'static) -> Self {
        self.init_consumer = Arc::new(init_consumer);
        self
    }

    /// java: @Override public YetAnotherConfigLib build()
    fn build(self) -> Arc<dyn YetAnotherConfigLib> {
        let title = self.title.expect("`title must not be null to build `YetAnotherConfigLib`");
        assert!(!self.categories.is_empty(), "`categories` must not be empty to build `YetAnotherConfigLib`");
        assert!(
            !self.categories.iter().all(|category| category.as_placeholder_category().is_some()),
            "At least one regular category is required to build `YetAnotherConfigLib`"
        );
        Arc::new(YetAnotherConfigLibImpl::new(title, self.categories, self.save_function, self.init_consumer))
    }
}
