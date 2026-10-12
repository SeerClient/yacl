//! Ported from dev.isxander.yacl3.config.v2.api.ReadOnlyFieldAccess by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface ReadOnlyFieldAccess<T>
pub trait ReadOnlyFieldAccess<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: T get();
    fn get(&self) -> T;

    /// java: String name();
    fn name(&self) -> String;

    /// java: Type type();
    fn r#type(&self) -> crate::unresolved::Type;

    /// java: Class<T> typeClass();
    fn type_class(&self) -> compat::ClassOf<T>;

    /// java: <A extends Annotation> Optional<A> getAnnotation(Class<A> annotationClass);
    fn get_annotation<A: 'static>(&self, annotation_class: &compat::ClassOf<A>) -> Option<A> where Self: Sized;

}
