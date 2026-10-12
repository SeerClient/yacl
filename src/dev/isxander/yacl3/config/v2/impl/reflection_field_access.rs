//! Ported from dev.isxander.yacl3.config.v2.impl.ReflectionFieldAccess by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;

/// java: public record ReflectionFieldAccess<T>(Field field, Object instance) implements FieldAccess<T>
pub struct ReflectionFieldAccess<T> {
    /// java: Field field
    pub field: crate::unresolved::Field,
    /// java: Object instance
    pub instance: compat::Object,
}

impl<T: Clone + 'static> ReflectionFieldAccess<T> {
    /// java: the canonical record constructor
    pub fn new(mut field: crate::unresolved::Field, mut instance: compat::Object) -> Self {
        Self { field, instance }
    }

    /// java: the record accessor `field()`
    pub fn field(&self) -> crate::unresolved::Field {
        self.field.clone()
    }

    /// java: the record accessor `instance()`
    pub fn instance(&self) -> compat::Object {
        self.instance.clone()
    }

}

impl<T: Clone + 'static> FieldAccess<T> for ReflectionFieldAccess<T> {
    /// java: @Override public void set(T value)
    fn set(&mut self, mut value: T) {
        // java: try {..} catch (IllegalAccessException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `.set(..)` on a Field
            //   java: field.set(instance, value);
        }));
        if let Err(e) = __result {
            panic!("YACLAutoGenException: {}", format!("Failed to set field '{}'", self.name()).to_string());
        }
    }

}

impl<T: Clone + 'static> ReadOnlyFieldAccess<compat::Object> for ReflectionFieldAccess<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public T get()
    fn get(&self) -> T {
        // java: try {..} catch (IllegalAccessException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): `.get(..)` on a Field
            //   java: return (T) field.get(instance);
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                panic!("YACLAutoGenException: {}", format!("Failed to access field '{}'", self.name()).to_string());
            }
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public String name()
    fn name(&self) -> String {
        // TODO(fabric2seer): `.getName(..)` on a Field
        //   java: return field.getName();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Type type()
    fn r#type(&self) -> crate::unresolved::Type {
        // TODO(fabric2seer): `.getGenericType(..)` on a Field
        //   java: return field.getGenericType();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Class<T> typeClass()
    fn type_class(&self) -> compat::ClassOf<T> {
        // TODO(fabric2seer): `.getType(..)` on a Field
        //   java: return (Class<T>) field.getType();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public <A extends Annotation> Optional<A> getAnnotation(Class<A> annotationClass)
    fn get_annotation<A: 'static>(&self, annotation_class: &compat::ClassOf<A>) -> Option<A> where Self: Sized {
        // TODO(fabric2seer): `.getAnnotation(..)` on a Field
        //   java: return Optional.ofNullable(field.getAnnotation(annotationClass));
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
