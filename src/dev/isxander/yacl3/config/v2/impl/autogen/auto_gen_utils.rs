//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.AutoGenUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController;
use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_format::CustomFormat;
use crate::dev::isxander::yacl3::config::v2::api::autogen::format_translation::FormatTranslation;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use mcrust::net::minecraft::network::chat::component;

/// java: @ApiStatus.Internal public final class AutoGenUtils
#[derive(Clone)]
pub struct AutoGenUtils {}

impl AutoGenUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static <T> void addCustomFormatterToController(ControllerBuilder<T> controller, ReadOnlyFieldAccess<T> field)
    pub fn add_custom_formatter_to_controller<T: Clone + 'static>(controller: &compat::Shared<dyn ControllerBuilder<T>>, field: &compat::Shared<dyn ReadOnlyFieldAccess<T>>) {
        let mut formatter: Option<CustomFormat> = { let (__a0,) = (&std::any::TypeId::of::<CustomFormat>(),); field.lock().borrow().get_annotation(__a0) };
        let mut translation: Option<FormatTranslation> = { let (__a0,) = (&std::any::TypeId::of::<FormatTranslation>(),); field.lock().borrow().get_annotation(__a0) };
        if formatter.is_some() && translation.is_some() {
            panic!("YACLAutoGenException: {}", format!("'{}': Cannot use both @CustomFormatter and @FormatTranslation on the same field.", field.lock().borrow().name()).to_string());
        } else if formatter.is_none() && translation.is_none() {
            return;
        }
        if !(controller.lock().borrow().as_any().is::<compat::Shared<dyn ValueFormattableController<compat::Object, compat::Object>>>()) {
            panic!("YACLAutoGenException: {}", format!("Attempted to use @CustomFormatter or @FormatTranslation on an option factory for field '{}' that uses a controller that does not support this.", field.lock().borrow().name()).to_string());
        }
        let mut typed_builder: compat::Shared<dyn ValueFormattableController<T, compat::Object>> = controller.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn ValueFormattableController<T, compat::Object>>>().unwrap().clone();
        if let Some(formatter_class) = formatter.clone() {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.getConstructor(..)` on a Class
                //   java: typedBuilder.formatValue((ValueFormatter<T>) formatterClass.value().getConstructor().newInstance());
            }));
            if let Err(e) = __result {
                panic!("YACLAutoGenException: {}", format!("'{}': Failed to instantiate formatter class {}.", field.lock().borrow().name(), formatter_class.value().get_name()).to_string());
            }
        }
        if let Some(annotation) = translation.clone() {
            { let (__a0,) = (Arc::new(move |v| component::translatable_args(&annotation.value(), vec![v.clone()])),); typed_builder.lock().borrow_mut().format_value(__a0) };
        }
    }

    /// java: public static <T> T constructNoArgsClass(Class<T> clazz, Supplier<String> constructorNotFoundConsumer, Supplier<String> constructorFailedConsumer)
    pub fn construct_no_args_class<T: 'static>(clazz: &compat::ClassOf<T>, constructor_not_found_consumer: Arc<dyn Fn() -> String>, constructor_failed_consumer: Arc<dyn Fn() -> String>) -> T {
        // java: try {..} catch (NoSuchMethodException e) catch (Exception e) - the body runs inline: a Rust body does not throw checked exceptions (a panic in it propagates)
        {
            // TODO(fabric2seer): `.getConstructor(..)` on a Class
            //   java: return clazz.getConstructor().newInstance();
        }
        // TODO(fabric2seer): the `catch (NoSuchMethodException e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java:             throw new YACLAutoGenException(constructorNotFoundConsumer.get(), e);
        //   java:         }
        // TODO(fabric2seer): the `catch (Exception e)` block is not reached in Rust - the try body returns, breaks or continues, so it cannot run inside `catch_unwind`
        //   java: {
        //   java:             throw new YACLAutoGenException(constructorFailedConsumer.get(), e);
        //   java:         }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
