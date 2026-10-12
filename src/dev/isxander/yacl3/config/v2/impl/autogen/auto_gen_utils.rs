//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.AutoGenUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_format::CustomFormat;
use crate::dev::isxander::yacl3::config::v2::api::autogen::format_translation::FormatTranslation;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::yacl_auto_gen_exception::YACLAutoGenException;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::annotation_as;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ClassRef;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: @ApiStatus.Internal public final class AutoGenUtils
#[derive(Clone, Default)]
pub struct AutoGenUtils {}

impl AutoGenUtils {
    /// java: public static <T> void addCustomFormatterToController(ControllerBuilder<T> controller, ReadOnlyFieldAccess<T> field)
    ///
    /// The builders take themselves by value, so the (possibly re-formatted) builder is handed back.
    pub fn add_custom_formatter_to_controller<T: Clone + 'static>(
        controller: ControllerBuilderRef<T>,
        field: &Arc<dyn ReadOnlyFieldAccess<T>>,
    ) -> ControllerBuilderRef<T> {
        let formatter: Option<CustomFormat> = annotation_as(field.get_annotation(&std::any::TypeId::of::<CustomFormat>()));
        let translation: Option<FormatTranslation> = annotation_as(field.get_annotation(&std::any::TypeId::of::<FormatTranslation>()));

        if formatter.is_some() && translation.is_some() {
            YACLAutoGenException::new(format!(
                "'{}': Cannot use both @CustomFormatter and @FormatTranslation on the same field.",
                field.name()
            ))
            .throw();
        } else if formatter.is_none() && translation.is_none() {
            return controller;
        }

        // java: if (!(controller instanceof ValueFormattableController<?, ?>))
        let ControllerBuilderRef::Formattable(mut typed_builder) = controller else {
            YACLAutoGenException::new(format!(
                "Attempted to use @CustomFormatter or @FormatTranslation on an option factory for field '{}' that uses a controller that does not support this.",
                field.name()
            ))
            .throw();
        };

        if let Some(formatter_class) = formatter {
            // java: typedBuilder.formatValue((ValueFormatter<T>) formatterClass.value().getConstructor().newInstance());
            let created = formatter_class
                .value()
                .new_instance
                .and_then(|new_instance| std::panic::catch_unwind(new_instance).ok())
                .and_then(|instance| (*instance).downcast_ref::<compat::RenderThread<Arc<dyn ValueFormatter<T>>>>().map(|f| f.get().clone()));
            match created {
                Some(value_formatter) => typed_builder = typed_builder.format_value_boxed(value_formatter),
                None => YACLAutoGenException::new(format!(
                    "'{}': Failed to instantiate formatter class {}.",
                    field.name(),
                    formatter_class.value().get_name()
                ))
                .throw(),
            }
        }

        if let Some(annotation) = translation {
            // java: typedBuilder.formatValue(v -> Component.translatable(annotation.value(), v));
            let key = annotation.value();
            typed_builder = typed_builder.format_value_boxed(Arc::new(move |v: T| -> MutableComponent {
                component::translatable_args(&key, vec![FormattedText::of(string_value_of(&v))])
            }));
        }

        ControllerBuilderRef::Formattable(typed_builder)
    }

    /// java: public static <T> T constructNoArgsClass(Class<T> clazz, Supplier<String> constructorNotFoundConsumer, Supplier<String> constructorFailedConsumer)
    ///
    /// The new object is answered as the `Object` the class's generated constructor hands out (an `Arc<dyn I<X>>`
    /// inside); the caller's cast to `I<T>` is Java's unchecked `(T)`.
    pub fn construct_no_args_class(
        clazz: &ClassRef,
        constructor_not_found_consumer: impl Fn() -> String,
        constructor_failed_consumer: impl Fn() -> String,
    ) -> compat::Object {
        // java: clazz.getConstructor() - NoSuchMethodException
        let Some(new_instance) = clazz.new_instance else {
            YACLAutoGenException::new(constructor_not_found_consumer()).throw();
        };
        // java: .newInstance() - any other Exception
        match std::panic::catch_unwind(new_instance) {
            Ok(instance) => instance,
            Err(_) => YACLAutoGenException::new(constructor_failed_consumer()).throw(),
        }
    }
}

/// java: `String.valueOf(value)` - how `Component.translatable(key, value)` and `"..." + value` print a value of a
/// field type: Java's `toString` of the boxed primitive (`Float.toString` / `Double.toString` differ from Rust's
/// `Display`), of a `String` and of an enum constant (its name).
pub(crate) fn string_value_of<T: 'static>(value: &T) -> String {
    let any = value as &dyn std::any::Any;
    if let Some(v) = any.downcast_ref::<f32>() {
        return mcrust::javacompat::java_float_to_string(*v);
    }
    if let Some(v) = any.downcast_ref::<f64>() {
        return mcrust::javacompat::java_double_to_string(*v);
    }
    if let Some(v) = any.downcast_ref::<i32>() {
        return v.to_string();
    }
    if let Some(v) = any.downcast_ref::<i64>() {
        return v.to_string();
    }
    if let Some(v) = any.downcast_ref::<bool>() {
        return v.to_string();
    }
    if let Some(v) = any.downcast_ref::<String>() {
        return v.clone();
    }
    if let Some(v) = any.downcast_ref::<crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::EnumValue>() {
        return v.name();
    }
    // a type whose Java `toString` the port does not know: the Rust type's name
    std::any::type_name::<T>().to_string()
}

/// java: a `ControllerBuilder<T>` as `SimpleOptionFactory.createController` answers it. `AutoGenUtils` asks it
/// `instanceof ValueFormattableController`; Rust cannot test an erased builder for an interface, so the builder is
/// handed over as what its class is - [`ControllerBuilderRef::formattable`] for a class that implements
/// `ValueFormattableController` (Boolean, slider, field and cycling builders), [`ControllerBuilderRef::plain`]
/// otherwise (tick box, string, dropdown, colour, item).
pub enum ControllerBuilderRef<T: Clone + 'static> {
    Formattable(Box<dyn FormattableBuilder<T>>),
    Plain(Box<dyn ControllerBuilder<T>>),
}

impl<T: Clone + 'static> ControllerBuilderRef<T> {
    /// a builder whose class implements `ValueFormattableController<T, B>`
    pub fn formattable<B: ValueFormattableController<T> + 'static>(builder: B) -> ControllerBuilderRef<T> {
        ControllerBuilderRef::Formattable(Box::new(builder))
    }

    /// a builder whose class does not
    pub fn plain<B: ControllerBuilder<T> + 'static>(builder: B) -> ControllerBuilderRef<T> {
        ControllerBuilderRef::Plain(Box::new(builder))
    }
}

impl<T: Clone + 'static> ControllerBuilder<T> for ControllerBuilderRef<T> {
    fn build(&self) -> Arc<dyn Controller<T>> {
        match self {
            ControllerBuilderRef::Formattable(builder) => builder.build(),
            ControllerBuilderRef::Plain(builder) => builder.build(),
        }
    }
}

/// `ValueFormattableController<T, ?>` as a trait object: its `formatValue` on a boxed builder.
pub trait FormattableBuilder<T: Clone + 'static>: ControllerBuilder<T> {
    /// java: B formatValue(ValueFormatter<T> formatter);
    fn format_value_boxed(self: Box<Self>, formatter: Arc<dyn ValueFormatter<T>>) -> Box<dyn FormattableBuilder<T>>;
}

impl<T: Clone + 'static, B: ValueFormattableController<T> + 'static> FormattableBuilder<T> for B {
    fn format_value_boxed(self: Box<Self>, formatter: Arc<dyn ValueFormatter<T>>) -> Box<dyn FormattableBuilder<T>> {
        Box::new((*self).format_value(formatter))
    }
}
