//! Ported from dev.isxander.yacl3.config.v2.api.ReadOnlyFieldAccess by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::FieldType;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::TypeClass;

/// java: public interface ReadOnlyFieldAccess<T>
///
/// An abstract interface for accessing properties of an instance of a field.
pub trait ReadOnlyFieldAccess<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: T get();
    fn get(&self) -> T;

    /// java: String name();
    fn name(&self) -> String;

    /// java: Type type();
    fn r#type(&self) -> FieldType;

    /// java: Class<T> typeClass();
    fn type_class(&self) -> TypeClass;

    /// java: <A extends Annotation> Optional<A> getAnnotation(Class<A> annotationClass);
    ///
    /// `A.class` of an annotation is its `TypeId`; the answer is the annotation struct as an `Object` -
    /// `reflection_field_access::annotation_as::<A>(..)` is the typed `Optional<A>`.
    fn get_annotation(&self, annotation_class: &std::any::TypeId) -> Option<compat::Object>;
}
