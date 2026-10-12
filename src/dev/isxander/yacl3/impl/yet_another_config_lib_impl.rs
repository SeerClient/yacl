//! Ported from dev.isxander.yacl3.impl.YetAnotherConfigLibImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::config_category::ConfigCategory;
use crate::dev::isxander::yacl3::api::yet_another_config_lib::Builder;
use crate::dev::isxander::yacl3::api::yet_another_config_lib::YetAnotherConfigLib;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: @ApiStatus.Internal public final class YetAnotherConfigLibImpl implements YetAnotherConfigLib
#[derive(Clone)]
pub struct YetAnotherConfigLibImpl {
    /// java: private final Component title;
    pub title: MutableComponent,
    /// java: private final ImmutableList<ConfigCategory> categories;
    pub categories: Vec<compat::Shared<dyn ConfigCategory>>,
    /// java: private final Runnable saveFunction;
    pub save_function: Arc<dyn Fn()>,
    /// java: private final Consumer<YACLScreen> initConsumer;
    pub init_consumer: Arc<dyn Fn(compat::Shared<YACLScreen>)>,
    /// java: private boolean generated = false;
    pub generated: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl YetAnotherConfigLibImpl {
    /// java: public YetAnotherConfigLibImpl(Component title, ImmutableList<ConfigCategory> categories, Runnable saveFunction, Consumer<YACLScreen> initConsumer)
    pub fn new(mut title: MutableComponent, mut categories: Vec<compat::Shared<dyn ConfigCategory>>, save_function: Arc<dyn Fn()>, init_consumer: Arc<dyn Fn(compat::Shared<YACLScreen>)>) -> compat::Shared<Self> {
        let __init_title = title.clone();
        let __init_categories = categories.clone();
        let __init_save_function = save_function.clone();
        let __init_init_consumer = init_consumer.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            title: __init_title,
            categories: __init_categories,
            save_function: __init_save_function,
            init_consumer: __init_init_consumer,
            generated: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl YetAnotherConfigLib for YetAnotherConfigLibImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Screen generateScreen(Screen parent) (dispatch: the body for this class)
    fn generate_screen(&self) -> fn(&compat::Shared<dyn YetAnotherConfigLib>, Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        YetAnotherConfigLibImpl::generate_screen_dyn
    }

    /// java: @Override public Component title() (dispatch: the body for this class)
    fn title(&self) -> fn(&compat::Shared<dyn YetAnotherConfigLib>) -> MutableComponent {
        YetAnotherConfigLibImpl::title_dyn
    }

    /// java: @Override public ImmutableList<ConfigCategory> categories() (dispatch: the body for this class)
    fn categories(&self) -> fn(&compat::Shared<dyn YetAnotherConfigLib>) -> Vec<compat::Shared<dyn ConfigCategory>> {
        YetAnotherConfigLibImpl::categories_dyn
    }

    /// java: @Override public Runnable saveFunction() (dispatch: the body for this class)
    fn save_function(&self) -> fn(&compat::Shared<dyn YetAnotherConfigLib>) -> Arc<dyn Fn()> {
        YetAnotherConfigLibImpl::save_function_dyn
    }

    /// java: @Override public Consumer<YACLScreen> initConsumer() (dispatch: the body for this class)
    fn init_consumer(&self) -> fn(&compat::Shared<dyn YetAnotherConfigLib>) -> Arc<dyn Fn(compat::Shared<YACLScreen>)> {
        YetAnotherConfigLibImpl::init_consumer_dyn
    }

}

impl YetAnotherConfigLibImpl {
    /// java: @Override public Screen generateScreen(Screen parent)
    pub fn generate_screen_impl(__this: &compat::Shared<Self>, mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        if compat::read(__this, |__t| __t.generated) {
            panic!("UnsupportedOperationException: {}", "To prevent memory leaks, you should only generate a Screen once per instance. Please re-build the instance to generate another GUI.");
        }
        logger().info("Generating YACL screen");
        { let __v = true; compat::write(__this, |__t| __t.generated = __v); }
        return YACLScreen::new((__this.clone() as compat::Shared<dyn YetAnotherConfigLib>), parent.clone().unwrap());
    }

    /// YetAnotherConfigLibImpl.generateScreen as the dispatch of YetAnotherConfigLib.generateScreen
    pub fn generate_screen_dyn(__this: &compat::Shared<dyn YetAnotherConfigLib>, __p0: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        Self::generate_screen_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Component title()
    pub fn title_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return compat::read(__this, |__t| __t.title.clone());
    }

    /// YetAnotherConfigLibImpl.title as the dispatch of YetAnotherConfigLib.title
    pub fn title_dyn(__this: &compat::Shared<dyn YetAnotherConfigLib>) -> MutableComponent {
        Self::title_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public ImmutableList<ConfigCategory> categories()
    pub fn categories_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<dyn ConfigCategory>> {
        return compat::read(__this, |__t| __t.categories.clone());
    }

    /// YetAnotherConfigLibImpl.categories as the dispatch of YetAnotherConfigLib.categories
    pub fn categories_dyn(__this: &compat::Shared<dyn YetAnotherConfigLib>) -> Vec<compat::Shared<dyn ConfigCategory>> {
        Self::categories_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Runnable saveFunction()
    pub fn save_function_impl(__this: &compat::Shared<Self>) -> Arc<dyn Fn()> {
        return compat::read(__this, |__t| __t.save_function.clone());
    }

    /// YetAnotherConfigLibImpl.saveFunction as the dispatch of YetAnotherConfigLib.saveFunction
    pub fn save_function_dyn(__this: &compat::Shared<dyn YetAnotherConfigLib>) -> Arc<dyn Fn()> {
        Self::save_function_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Consumer<YACLScreen> initConsumer()
    pub fn init_consumer_impl(__this: &compat::Shared<Self>) -> Arc<dyn Fn(compat::Shared<YACLScreen>)> {
        return compat::read(__this, |__t| __t.init_consumer.clone());
    }

    /// YetAnotherConfigLibImpl.initConsumer as the dispatch of YetAnotherConfigLib.initConsumer
    pub fn init_consumer_dyn(__this: &compat::Shared<dyn YetAnotherConfigLib>) -> Arc<dyn Fn(compat::Shared<YACLScreen>)> {
        Self::init_consumer_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: @ApiStatus.Internal public static final class BuilderImpl implements Builder
#[derive(Clone)]
pub struct BuilderImpl {
    /// java: private Component title;
    pub title: Option<MutableComponent>,
    /// java: private final List<ConfigCategory> categories = new ArrayList<>();
    pub categories: Vec<compat::Shared<dyn ConfigCategory>>,
    /// java: private Runnable saveFunction = () -> {};
    pub save_function: Arc<dyn Fn()>,
    /// java: private Consumer<YACLScreen> initConsumer = screen -> {};
    pub init_consumer: Arc<dyn Fn(compat::Shared<YACLScreen>)>,
}

impl BuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            title: None,
            categories: Vec::new(),
            save_function: Arc::new(move || {

        }),
            init_consumer: Arc::new(move |screen| {

        }),
        });
        __this
    }

}

impl Builder for BuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder title(@NotNull Component title) (dispatch: the body for this class)
    fn title(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        BuilderImpl::title_dyn
    }

    /// java: @Override public Builder category(@NotNull ConfigCategory category) (dispatch: the body for this class)
    fn category(&self) -> fn(&compat::Shared<dyn Builder>, compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn Builder> {
        BuilderImpl::category_dyn
    }

    /// java: @Override public Builder categories(@NotNull Collection<? extends @NotNull ConfigCategory> categories) (dispatch: the body for this class)
    fn categories(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<compat::Shared<dyn ConfigCategory>>) -> compat::Shared<dyn Builder> {
        BuilderImpl::categories_dyn
    }

    /// java: @Override public Builder save(@NotNull Runnable saveFunction) (dispatch: the body for this class)
    fn save(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn()>) -> compat::Shared<dyn Builder> {
        BuilderImpl::save_dyn
    }

    /// java: @Override public Builder screenInit(@NotNull Consumer<YACLScreen> initConsumer) (dispatch: the body for this class)
    fn screen_init(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn(compat::Shared<YACLScreen>)>) -> compat::Shared<dyn Builder> {
        BuilderImpl::screen_init_dyn
    }

    /// java: @Override public YetAnotherConfigLib build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn YetAnotherConfigLib> {
        BuilderImpl::build_dyn
    }

}

impl BuilderImpl {
    /// java: @Override public Builder title(@NotNull Component title)
    pub fn title_impl(__this: &compat::Shared<Self>, mut title: MutableComponent) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(title, "`title` cannot be null");
        { let __v = Some(title.clone()); compat::write(__this, |__t| __t.title = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.title as the dispatch of Builder.title
    pub fn title_dyn(__this: &compat::Shared<dyn Builder>, __p0: MutableComponent) -> compat::Shared<dyn Builder> {
        Self::title_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder category(@NotNull ConfigCategory category)
    pub fn category_impl(__this: &compat::Shared<Self>, mut category: compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(category, "`category` cannot be null");
        let __h0 = category.clone();
        __this.lock().borrow_mut().categories.push(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.category as the dispatch of Builder.category
    pub fn category_dyn(__this: &compat::Shared<dyn Builder>, __p0: compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn Builder> {
        Self::category_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder categories(@NotNull Collection<? extends @NotNull ConfigCategory> categories)
    pub fn categories_impl(__this: &compat::Shared<Self>, categories: &Vec<compat::Shared<dyn ConfigCategory>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(categories, "`categories` cannot be null");
        let __h0 = categories.iter().cloned();
        __this.lock().borrow_mut().categories.extend(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.categories as the dispatch of Builder.categories
    pub fn categories_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<compat::Shared<dyn ConfigCategory>>) -> compat::Shared<dyn Builder> {
        Self::categories_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder save(@NotNull Runnable saveFunction)
    pub fn save_impl(__this: &compat::Shared<Self>, save_function: Arc<dyn Fn()>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(saveFunction, "`saveFunction` cannot be null");
        { let __v = save_function.clone(); compat::write(__this, |__t| __t.save_function = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.save as the dispatch of Builder.save
    pub fn save_dyn(__this: &compat::Shared<dyn Builder>, __p0: Arc<dyn Fn()>) -> compat::Shared<dyn Builder> {
        Self::save_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder screenInit(@NotNull Consumer<YACLScreen> initConsumer)
    pub fn screen_init_impl(__this: &compat::Shared<Self>, init_consumer: Arc<dyn Fn(compat::Shared<YACLScreen>)>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(initConsumer, "`initConsumer` cannot be null");
        { let __v = init_consumer.clone(); compat::write(__this, |__t| __t.init_consumer = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BuilderImpl.screenInit as the dispatch of Builder.screenInit
    pub fn screen_init_dyn(__this: &compat::Shared<dyn Builder>, __p0: Arc<dyn Fn(compat::Shared<YACLScreen>)>) -> compat::Shared<dyn Builder> {
        Self::screen_init_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public YetAnotherConfigLib build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn YetAnotherConfigLib> {
        // TODO(fabric2seer): static call Validate.notNull(..)
        //   java: Validate.notNull(title, "`title must not be null to build `YetAnotherConfigLib`");
        // TODO(fabric2seer): static call Validate.notEmpty(..)
        //   java: Validate.notEmpty(categories, "`categories` must not be empty to build `YetAnotherConfigLib`");
        // TODO(fabric2seer): static call Validate.isTrue(..)
        //   java: Validate.isTrue(!categories.stream().allMatch(category -> category instanceof PlaceholderCategory), "At least one regular category is required to build `YetAnotherConfigLib`");
        return (YetAnotherConfigLibImpl::new(compat::read(__this, |__t| __t.title.clone()).unwrap(), __this.lock().borrow().categories.iter().cloned().collect::<Vec<_>>(), compat::read(__this, |__t| __t.save_function.clone()), compat::read(__this, |__t| __t.init_consumer.clone())) as compat::Shared<dyn YetAnotherConfigLib>);
    }

    /// BuilderImpl.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn YetAnotherConfigLib> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
