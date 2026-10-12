//! Ported from dev.isxander.yacl3.config.v2.api.autogen.SimpleOptionFactory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option;
use crate::dev::isxander::yacl3::api::option::Builder as _;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::api::option_description;
use crate::dev::isxander::yacl3::api::option_description::Builder as _;
use crate::dev::isxander::yacl3::api::option_event_listener::Event;
use crate::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_description::CustomDescription;
use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_image::CustomImage;
use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_image::CustomImageFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_name::CustomName;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::AutoGenUtils;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::ControllerBuilderRef;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::empty_custom_image_factory;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::yacl_auto_gen_exception::YACLAutoGenException;
use crate::dev::isxander::yacl3::config::v2::r#impl::field_backed_binding::FieldBackedBinding;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::annotation_as;
use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use crate::dev::isxander::yacl3::r#impl::option_description_impl::BuilderImpl as OptionDescriptionBuilder;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::locale::language::LanguageObj as _;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public abstract class SimpleOptionFactory<A extends Annotation, T> implements OptionFactory<A, T>
///
/// The class has no fields; its methods are this trait's (each subclass implements `create_controller` and
/// overrides the others it overrides). A subclass's `impl OptionFactory` is [`create_option`] (Java's inherited
/// `createOption`).
///
/// `T: PartialEq` is `Option.createBuilder()`'s bound; `Send + Sync` is a field value's (it crosses as `Object`).
pub trait SimpleOptionFactory<A: Clone + 'static, T: Clone + PartialEq + Send + Sync + 'static>: OptionFactory<A, T> {
    /// java: protected abstract ControllerBuilder<T> createController(A annotation, ConfigField<T> field, OptionAccess storage, Option<T> option);
    ///
    /// The builder is answered as what its class is ([`ControllerBuilderRef::formattable`] /
    /// [`ControllerBuilderRef::plain`]): `AutoGenUtils` asks it `instanceof ValueFormattableController`.
    fn create_controller(
        &self,
        annotation: &A,
        field: &Arc<dyn ConfigField<T>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn YaclOption<T>>,
    ) -> ControllerBuilderRef<T>;

    /// java: protected MutableComponent name(A annotation, ConfigField<T> field, OptionAccess storage)
    fn name(&self, annotation: &A, field: &Arc<dyn ConfigField<T>>, storage: &Arc<dyn OptionAccess>) -> MutableComponent {
        let custom_name: Option<CustomName> = annotation_as(field.access().get_annotation(&std::any::TypeId::of::<CustomName>()));
        component::translatable(&custom_name.map(|c| c.value()).unwrap_or(self.get_translation_key(field, None)))
    }

    /// java: protected OptionDescription.Builder description(T value, A annotation, ConfigField<T> field, OptionAccess storage)
    fn description(&self, value: T, annotation: &A, field: &Arc<dyn ConfigField<T>>, storage: &Arc<dyn OptionAccess>) -> OptionDescriptionBuilder {
        let mut builder = option_description::create_builder();

        let mut key: String = self.get_translation_key(field, Some("desc"));
        if language::get_instance().has(&key) {
            builder = builder.text(vec![component::translatable(&key)]);
        } else {
            key.push('.');
            let mut i: i32 = 1;
            while language::get_instance().has(&format!("{}{}", key, i)) {
                builder = builder.text(vec![component::translatable(&format!("{}{}", key, i))]);
                i += 1;
            }
        }

        let access = field.access();
        let custom_description: Option<CustomDescription> = annotation_as(access.get_annotation(&std::any::TypeId::of::<CustomDescription>()));
        if let Some(custom_description) = custom_description {
            for line in custom_description.value() {
                builder = builder.text(vec![component::translatable(&line)]);
            }
        }

        let image_override_opt: Option<CustomImage> = annotation_as(access.get_annotation(&std::any::TypeId::of::<CustomImage>()));
        if let Some(image_override) = image_override_opt {
            if image_override.factory() != empty_custom_image_factory::CLASS {
                let factory_class = image_override.factory();
                let field_name = access.name();
                // java: (CustomImage.CustomImageFactory<T>) AutoGenUtils.constructNoArgsClass(..)
                let instance = AutoGenUtils::construct_no_args_class(
                    &factory_class,
                    || format!("'{}': The factory class on @OverrideImage has no no-args constructor.", field_name),
                    || format!("'{}': Failed to instantiate factory class {}.", field_name, factory_class.get_name()),
                );
                // java: catch (ClassCastException e)
                let image_factory: Arc<dyn CustomImageFactory<T>> =
                    match (*instance).downcast_ref::<compat::RenderThread<Arc<dyn CustomImageFactory<T>>>>() {
                        Some(image_factory) => image_factory.get().clone(),
                        None => YACLAutoGenException::new(format!(
                            "'{}': The factory class on @OverrideImage is of incorrect type. Expected {}, got {}.",
                            access.name(),
                            access.r#type().get_type_name(),
                            // java: imageOverride.factory().getTypeParameters()[0].getName() - a ClassRef records no
                            // type parameters: the class itself
                            factory_class.get_name()
                        ))
                        .throw(),
                    };

                // java: builder.customImage(imageFactory.createImage(value, field, storage).thenApply(Optional::of));
                builder = builder.custom_image(image_factory.create_image(value, field, storage).then_apply(|image: Arc<dyn ImageRenderer>| Some(image)));
            } else if !image_override.value().is_empty() {
                let path: String = image_override.value();
                let image_location: Identifier = YACLPlatform::rl_2(field.parent().id().namespace(), &path);
                let extension: String = compat::substring(&path, compat::last_index_of(&path, '.').wrapping_add(1));
                match extension.as_str() {
                    "png" | "jpg" | "jpeg" => builder = builder.image(image_location, image_override.width(), image_override.height()),
                    "webp" => builder = builder.webp_image(image_location),
                    "gif" => builder = builder.gif_image(image_location),
                    _ => YACLAutoGenException::new(format!(
                        "'{}': Invalid image extension '{}' on @OverrideImage. Expected: ('png','jpg','webp','gif')",
                        access.name(),
                        extension
                    ))
                    .throw(),
                }
            } else {
                YACLAutoGenException::new(format!("'{}': @OverrideImage has no value or factory class.", access.name())).throw();
            }
        } else {
            let mut image_path: String = format!("textures/yacl3/{}/{}.webp", field.parent().id().path(), access.name());
            image_path = compat::regex::replace_all(&image_path.to_lowercase(), "[^a-z0-9/._:-]", "_");
            let image_location: Identifier = YACLPlatform::rl_2(field.parent().id().namespace(), &image_path);
            if compat::minecraft().get_resource_manager().get_resource(&image_location).is_some() {
                builder = builder.webp_image(image_location);
            }
        }

        builder
    }

    /// java: protected boolean available(A annotation, ConfigField<T> field, OptionAccess storage)
    fn available(&self, annotation: &A, field: &Arc<dyn ConfigField<T>>, storage: &Arc<dyn OptionAccess>) -> bool {
        true
    }

    /// java: protected Set<OptionFlag> flags(A annotation, ConfigField<T> field, OptionAccess storage)
    fn flags(&self, annotation: &A, field: &Arc<dyn ConfigField<T>>, storage: &Arc<dyn OptionAccess>) -> Vec<Arc<dyn OptionFlag>> {
        // java: Set.of()
        Vec::new()
    }

    /// java: protected void listener(A annotation, ConfigField<T> field, OptionAccess storage, Option<T> option, T value)
    fn listener(&self, annotation: &A, field: &Arc<dyn ConfigField<T>>, storage: &Arc<dyn OptionAccess>, option: &dyn YaclOption<T>, value: T) {}

    /// java: protected void postInit(A annotation, ConfigField<T> field, OptionAccess storage, Option<T> option)
    fn post_init(&self, annotation: &A, field: &Arc<dyn ConfigField<T>>, storage: &Arc<dyn OptionAccess>, option: &Arc<dyn YaclOption<T>>) {}

    /// java: protected String getTranslationKey(ConfigField<T> field, @Nullable String suffix)
    fn get_translation_key(&self, field: &Arc<dyn ConfigField<T>>, suffix: Option<&str>) -> String {
        let mut key: String = format!("yacl3.config.{}.{}", field.parent().id(), field.access().name());
        if let Some(suffix) = suffix {
            key.push_str(&format!(".{}", suffix));
        }
        key
    }
}

/// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess)
///
/// SimpleOptionFactory's `createOption`, which every subclass inherits (`impl OptionFactory for X` calls it with
/// `self`). Its lambdas keep `this`: the factory, which has no state, is captured by value.
pub fn create_option<A, T, F>(this: &F, annotation: &A, field: &Arc<dyn ConfigField<T>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<T>>
where
    A: Clone + 'static,
    T: Clone + PartialEq + Send + Sync + 'static,
    F: SimpleOptionFactory<A, T> + Clone + 'static,
{
    let option: Arc<dyn YaclOption<T>> = option::create_builder::<T>()
        .name(this.name(annotation, field, option_access))
        .description_function({
            let (this, annotation, field, option_access) = (this.clone(), annotation.clone(), field.clone(), option_access.clone());
            move |v: T| this.description(v, &annotation, &field, &option_access).build()
        })
        .binding(Arc::new(FieldBackedBinding::new(field.access(), field.default_access())))
        .controller({
            let (this, annotation, field, option_access) = (this.clone(), annotation.clone(), field.clone(), option_access.clone());
            move |opt: Arc<dyn YaclOption<T>>| {
                let builder = this.create_controller(&annotation, &field, &option_access, &opt);

                let field_access = field.access();
                AutoGenUtils::add_custom_formatter_to_controller(builder, &(field_access as Arc<dyn crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess<T>>))
            }
        })
        .available(this.available(annotation, field, option_access))
        .flags(this.flags(annotation, field, option_access))
        .add_listener({
            let (this, annotation, field, option_access) = (this.clone(), annotation.clone(), field.clone(), option_access.clone());
            Arc::new(move |opt: &dyn YaclOption<T>, event: Event| this.listener(&annotation, &field, &option_access, opt, opt.pending_value()))
        })
        .build();

    this.post_init(annotation, field, option_access, &option);

    option
}
