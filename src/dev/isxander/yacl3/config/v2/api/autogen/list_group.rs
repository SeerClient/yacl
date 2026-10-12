//! Ported from dev.isxander.yacl3.config.v2.api.autogen.ListGroup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ClassRef;

/// java: @Retention(RetentionPolicy.RUNTIME) public @interface ListGroup
///
/// An option factory: a List option with a custom controller.
///
/// An annotation instance is a value: one field per element, the declared default filled in where the use site
/// leaves the element out. `Field.getAnnotation(X.class)` answers it (`reflection_field_access::Field`).
#[derive(Clone, Debug, PartialEq)]
pub struct ListGroup {
    /// java: Class<? extends ValueFactory<?>> valueFactory();
    /// `new_instance` yields a `compat::RenderThread<Arc<dyn ValueFactory<X>>>`.
    pub value_factory: ClassRef,
    /// java: Class<? extends ControllerFactory<?>> controllerFactory();
    /// `new_instance` yields a `compat::RenderThread<Arc<dyn ControllerFactory<X>>>`.
    pub controller_factory: ClassRef,
    /// java: int maxEntries() default 0;
    pub max_entries: i32,
    /// java: int minEntries() default 0;
    pub min_entries: i32,
    /// java: boolean addEntriesToBottom() default false;
    pub add_entries_to_bottom: bool,
}

impl ListGroup {
    /// The annotation as written with only its required elements: every other element takes its declared default.
    pub fn of(value_factory: ClassRef, controller_factory: ClassRef) -> ListGroup {
        ListGroup { value_factory, controller_factory, max_entries: 0, min_entries: 0, add_entries_to_bottom: false }
    }

    /// java: Class<? extends ValueFactory<?>> valueFactory();
    pub fn value_factory(&self) -> ClassRef {
        self.value_factory
    }

    /// java: Class<? extends ControllerFactory<?>> controllerFactory();
    pub fn controller_factory(&self) -> ClassRef {
        self.controller_factory
    }

    /// java: int maxEntries() default 0;
    pub fn max_entries(&self) -> i32 {
        self.max_entries
    }

    /// java: int minEntries() default 0;
    pub fn min_entries(&self) -> i32 {
        self.min_entries
    }

    /// java: boolean addEntriesToBottom() default false;
    pub fn add_entries_to_bottom(&self) -> bool {
        self.add_entries_to_bottom
    }
}

/// java: interface ValueFactory<T>
pub trait ValueFactory<T: Clone + 'static> {
    /// java: T provideNewValue();
    fn provide_new_value(&self) -> T;
}

/// java: interface ControllerFactory<T>
pub trait ControllerFactory<T: Clone + 'static> {
    /// java: ControllerBuilder<T> createController(ListGroup annotation, ConfigField<List<T>> field, OptionAccess storage, Option<T> option);
    fn create_controller(
        &self,
        annotation: &ListGroup,
        field: &Arc<dyn ConfigField<Vec<T>>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn crate::dev::isxander::yacl3::api::option::Option<T>>,
    ) -> Arc<dyn ControllerBuilder<T>>;
}
