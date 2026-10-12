//! Ported from dev.isxander.yacl3.config.v2.api.autogen.SimpleOptionFactory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::option_description;
use crate::dev::isxander::yacl3::api::option_description::Builder;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_description::CustomDescription;
use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_image::CustomImage;
use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_image::CustomImageFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_name::CustomName;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::AutoGenUtils;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::empty_custom_image_factory::EmptyCustomImageFactory;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler as _;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess as _;

/// java: public abstract class SimpleOptionFactory<A extends Annotation, T> implements OptionFactory<A, T>
#[derive(Clone)]
pub struct SimpleOptionFactoryBase<A, T> {}

impl<A: Clone + 'static, T: Clone + 'static> SimpleOptionFactoryBase<A, T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}

/// java: the virtual methods of SimpleOptionFactory (implemented by SimpleOptionFactoryBase and every subclass)
pub trait SimpleOptionFactory<A: Clone + 'static, T: Clone + 'static>: OptionFactory {
    fn simple_option_factory_base(&self) -> &SimpleOptionFactoryBase<A, T>;
    fn simple_option_factory_base_mut(&mut self) -> &mut SimpleOptionFactoryBase<A, T>;

    /// java: protected abstract ControllerBuilder<T> createController(A annotation, ConfigField<T> field, OptionAccess storage, Option<T> option);
    fn create_controller(&self, annotation: &A, field: &compat::Shared<dyn ConfigField<T>>, storage: &compat::Shared<dyn OptionAccess>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> compat::Shared<dyn ControllerBuilder<T>>;

    /// java: protected MutableComponent name(A annotation, ConfigField<T> field, OptionAccess storage)
    fn name(&self, annotation: &A, field: &compat::Shared<dyn ConfigField<T>>, storage: &compat::Shared<dyn OptionAccess>) -> MutableComponent {
        let mut custom_name: Option<CustomName> = { let (__a0,) = (&std::any::TypeId::of::<CustomName>(),); field.lock().borrow().access().lock().borrow().get_annotation(__a0) };
        return component::translatable(&custom_name.clone().map(|__a0| __a0.value()).unwrap_or(self.get_translation_key(field, None)));
    }

    /// java: protected OptionDescription.Builder description(T value, A annotation, ConfigField<T> field, OptionAccess storage)
    fn description(&self, value: &T, annotation: &A, field: &compat::Shared<dyn ConfigField<T>>, storage: &compat::Shared<dyn OptionAccess>) -> compat::Shared<dyn Builder> {
        let mut builder: compat::Shared<dyn Builder> = option_description::create_builder();
        let mut key: String = self.get_translation_key(field, Some("desc".to_string()));
        if language::get_instance().has(&key) {
            ({ let __f = compat::dispatch(&builder, |__t| __t.text_collection()); __f(&builder, &component::translatable(&key)) });
        } else {
            key.push_str(".");
            let mut i: i32 = 1;
            while language::get_instance().has(&format!("{}{}", key, i)) {
                ({ let __f = compat::dispatch(&builder, |__t| __t.text_collection()); __f(&builder, &component::translatable(&format!("{}{}", key, i))) });
                i += 1;
            }
        }
        if let Some(custom_description) = { let (__a0,) = (&std::any::TypeId::of::<CustomDescription>(),); field.lock().borrow().access().lock().borrow().get_annotation(__a0) } {
            // TODO(fabric2seer): `.value(..)` on a A
            //   java: for (String line : customDescription.value()) {
            //   java:                 builder.text(Component.translatable(line));
            //   java:             }
        }
        let mut image_override_opt: Option<CustomImage> = { let (__a0,) = (&std::any::TypeId::of::<CustomImage>(),); field.lock().borrow().access().lock().borrow().get_annotation(__a0) };
        if image_override_opt.is_some() {
            let mut image_override: CustomImage = image_override_opt.clone().unwrap();
            if !(image_override.factory() == std::any::TypeId::of::<EmptyCustomImageFactory>()) {
                let mut image_factory: compat::Shared<dyn CustomImageFactory<T>>;
                // java: try {..} catch (ClassCastException e) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    image_factory = AutoGenUtils::construct_no_args_class(&image_override.factory(), Arc::new(move || format!("'{}': The factory class on @OverrideImage has no no-args constructor.", field.lock().borrow().access().lock().borrow().name())), Arc::new(move || format!("'{}': Failed to instantiate factory class {}.", field.lock().borrow().access().lock().borrow().name(), image_override.factory().get_name())));
                }));
                if let Err(e) = __result {
                    // TODO(fabric2seer): `.getTypeName(..)` on a Type
                    //   java: throw new YACLAutoGenException("'%s': The factory class on @OverrideImage is of incorrect type. Expected %s, got %s.".formatted(field.access().name(), field.access().type().getTypeName(), imageOverride.factory().getTypeParameters()[0].getName()));
                }
                ({ let __f = compat::dispatch(&builder, |__t| __t.custom_image()); __f(&builder, { let (__q0,) = (|__a0| Some(__a0.clone()),); (image_factory.lock().borrow().create_image(value, field, storage)).then_apply(__q0) }) });
            } else if !(image_override.value().is_empty()) {
                let mut path: String = image_override.value();
                let mut image_location: Identifier = YACLPlatform::rl_2(field.lock().borrow().parent().lock().borrow().id().namespace(), &path);
                let mut extension: String = compat::substring(&path, (compat::last_index_of(&path, '.')).wrapping_add(1));
                match extension.as_str() {
                    "png" | "jpg" | "jpeg" => {
                        ({ let __f = compat::dispatch(&builder, |__t| __t.image()); __f(&builder, &image_location, image_override.width(), image_override.height()) });
                    }
                    "webp" => {
                        ({ let __f = compat::dispatch(&builder, |__t| __t.webp_image()); __f(&builder, &image_location) });
                    }
                    "gif" => {
                        ({ let __f = compat::dispatch(&builder, |__t| __t.gif_image()); __f(&builder, &image_location) });
                    }
                    _ => {
                        panic!("YACLAutoGenException: {}", format!("'{}': Invalid image extension '{}' on @OverrideImage. Expected: ('png','jpg','webp','gif')", field.lock().borrow().access().lock().borrow().name(), extension).to_string());
                    }
                }
            } else {
                panic!("YACLAutoGenException: {}", format!("'{}': @OverrideImage has no value or factory class.", field.lock().borrow().access().lock().borrow().name()).to_string());
            }
        } else {
            let mut image_path: String = format!("textures/yacl3/{}/{}.webp", field.lock().borrow().parent().lock().borrow().id().path(), field.lock().borrow().access().lock().borrow().name());
            image_path = compat::regex::replace_all(&image_path.to_lowercase(), "[^a-z0-9/._:-]", "_");
            let mut image_location: Identifier = YACLPlatform::rl_2(field.lock().borrow().parent().lock().borrow().id().namespace(), &image_path);
            if compat::minecraft().get_resource_manager().get_resource(&image_location).is_some() {
                ({ let __f = compat::dispatch(&builder, |__t| __t.webp_image()); __f(&builder, &image_location) });
            }
        }
        return builder;
    }

    /// java: protected boolean available(A annotation, ConfigField<T> field, OptionAccess storage)
    fn available(&self, annotation: &A, field: &compat::Shared<dyn ConfigField<T>>, storage: &compat::Shared<dyn OptionAccess>) -> bool {
        return true;
    }

    /// java: protected Set<OptionFlag> flags(A annotation, ConfigField<T> field, OptionAccess storage)
    fn flags(&self, annotation: &A, field: &compat::Shared<dyn ConfigField<T>>, storage: &compat::Shared<dyn OptionAccess>) -> std::collections::HashSet<compat::Shared<dyn OptionFlag>> {
        return std::collections::HashSet::from([]);
    }

    /// java: protected void listener(A annotation, ConfigField<T> field, OptionAccess storage, Option<T> option, T value)
    fn listener(&self, annotation: &A, field: &compat::Shared<dyn ConfigField<T>>, storage: &compat::Shared<dyn OptionAccess>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>, value: &T) {
    }

    /// java: protected void postInit(A annotation, ConfigField<T> field, OptionAccess storage, Option<T> option)
    fn post_init(&self, annotation: &A, field: &compat::Shared<dyn ConfigField<T>>, storage: &compat::Shared<dyn OptionAccess>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) {
    }

    /// java: protected String getTranslationKey(ConfigField<T> field, @Nullable String suffix)
    fn get_translation_key(&self, field: &compat::Shared<dyn ConfigField<T>>, mut suffix: Option<String>) -> String {
        // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // rust: let mut key: String = format!("yacl3.config.{}.{}", field.lock().borrow().parent().lock().borrow().id().fmt(), field.lock().borrow().access().lock().borrow().name());
        let mut key: String = todo!("fabric2seer: the initialiser of `key` (TODO above)");
        if let Some(suffix) = &suffix {
            key.push_str(&format!(".{}", suffix).to_string());
        }
        return key;
    }

}

// SimpleOptionFactory.createOption implements OptionFactory.createOption: it is emitted in each concrete subclass's `impl OptionFactory` (Rust has no inherited impls)
