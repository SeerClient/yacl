//! Ported from dev.isxander.yacl3.config.v2.impl.ReflectionFieldAccess by fabric2seer.
//!
//! YACL's config v2 reflects over the USER's config class (`Class.getDeclaredFields()`, `Field.get/set`,
//! `Field.getAnnotation`, `Field.getGenericType()`, `Class.getEnumConstants()`). Rust has no reflection, so this file
//! also holds the stand-ins for the parts of `java.lang.reflect` YACL reads, which fabric2seer GENERATES per config
//! class (see PATTERNS_Y1.md "R1"):
//!
//! * [`ReflectedClass`] - `Class<T>` of a config class: its declared fields and its class annotations.
//! * [`Field`] (built from a generated [`DeclaredField`]) - `java.lang.reflect.Field`.
//! * [`FieldType`] - `Field.getGenericType()`: its name and the Gson form of a value of it.
//! * [`TypeClass`] - `Field.getType()`: the field's class (an enum's constants, `NameableEnum` / `CyclableEnum`).
//! * [`ClassRef`] - a `Class<? extends I>` element of an annotation (`@CustomFormat`, `@CustomImage.factory`,
//!   `@ListGroup.valueFactory/controllerFactory`): the class's name and its public no-args constructor.
//! * [`EnumValue`] - `Enum<?>`: an enum constant whose class YACL does not know.
//!
//! A config object is referenced as `compat::Shared<C>` (a Java reference to a mutable object of the mod); where
//! Java holds it as `Object` (`ReflectionFieldAccess.instance`) it is a `compat::Object` holding that `Shared<C>`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::any::{Any, TypeId};

// ---------------------------------------------------------------------------------------------------------------
// java.lang.Class of a config class
// ---------------------------------------------------------------------------------------------------------------

/// `Class<T>` of a config class that YACL reflects over (`ConfigClassHandler.createBuilder(MyConfig.class)`,
/// `GsonConfigInstance`). fabric2seer implements it for every class a `X.class` literal hands to YACL.
pub trait ReflectedClass: Sized + Send + 'static {
    /// `MyConfig.class`
    fn class_of() -> compat::ClassOf<Self>;

    /// `getDeclaredFields()`: the class's own fields in declaration order (static fields excluded - Java's
    /// `getDeclaredFields` lists them, but every YACL reader filters on annotations a static field cannot carry
    /// usefully, and `Field.get(instance)` of a static field ignores the instance).
    fn declared_fields() -> Vec<Field>;

    /// `getAnnotations()` of the class itself (only `@SerialEntry` is read, `ConfigClassHandlerImpl.discoverFields`).
    fn class_annotations() -> Vec<compat::Object> {
        Vec::new()
    }
}

/// `Class.getAnnotation(A.class)` of a reflected class.
pub fn class_annotation<A: Clone + 'static>(annotations: &[compat::Object]) -> Option<A> {
    annotations.iter().find_map(|a| (**a).downcast_ref::<A>().cloned())
}

// ---------------------------------------------------------------------------------------------------------------
// java.lang.reflect.Field
// ---------------------------------------------------------------------------------------------------------------

/// The operations `java.lang.reflect.Field` answers, on a config object held as `compat::Object`
/// (`Arc<compat::Shared<C>>`). Implemented by [`DeclaredField`]; a `dyn` so a `Field` is one type for every class.
pub trait FieldOps: Send + Sync {
    /// `getName()` - the JAVA name (translation keys, serial names and `@MasterTickBox` read it).
    fn name(&self) -> &'static str;
    /// `getGenericType().getTypeName()`
    fn generic_type_name(&self) -> &'static str;
    /// `getType()`
    fn type_class(&self) -> TypeClass;
    /// `getAnnotations()`: every RUNTIME annotation on the field, each the annotation's struct (`IntSlider`, ..).
    fn annotations(&self) -> Vec<compat::Object>;
    /// `get(instance)`
    fn get(&self, instance: &compat::Object) -> compat::Object;
    /// `set(instance, value)`
    fn set(&self, instance: &compat::Object, value: compat::Object);
    /// `gson.toJsonTree(value, getGenericType())`
    fn to_json_tree(&self, value: &compat::Object) -> compat::GsonValue;
    /// `gson.fromJson(json, getGenericType())`: `None` where Gson would throw (or answer `null` for a type that
    /// cannot hold it - a primitive)
    fn from_json_tree(&self, json: &serde_json::Value) -> Option<compat::Object>;
    /// `@ListGroup` on a `List<X>` field: `ListGroupImpl<X>.createOption` instantiated for X (Rust monomorphises
    /// what Java erases - see `list_group_impl::create_erased`). `None` for any other field.
    fn list_group(&self) -> Option<ListGroupFactoryFn> {
        None
    }
}

/// `ListGroupImpl<X>.createOption` for one element type X, over the erased field (see `FieldOps::list_group`).
pub type ListGroupFactoryFn = fn(
    &compat::Object,
    &Arc<dyn ConfigField<compat::Object>>,
    &Arc<dyn OptionAccess>,
) -> Arc<dyn crate::dev::isxander::yacl3::api::option::AnyOption>;

/// One field of a config class `C` whose Rust type is `F`, as fabric2seer generates it (`Field::of(DeclaredField {
/// .. })` in `ReflectedClass::declared_fields`).
pub struct DeclaredField<C, F> {
    /// the Java field name
    pub name: &'static str,
    /// `getGenericType().getTypeName()`, e.g. `java.util.List<java.lang.String>`
    pub generic_type: &'static str,
    /// `getType()`
    pub type_class: TypeClass,
    /// the field's RUNTIME annotations (a non-capturing closure building the annotation structs)
    pub annotations: fn() -> Vec<compat::Object>,
    pub get: fn(&C) -> F,
    pub set: fn(&mut C, F),
    /// the Gson form of a value of the field's type (the generated `__gson_write` of that type)
    pub to_gson: fn(&F) -> compat::GsonValue,
    /// Gson's read of a value of the field's type: `None` where Gson would reject it
    pub read_gson: fn(&serde_json::Value) -> Option<F>,
    /// `Some(list_group_impl::create_erased::<X>)` for a `List<X>` field annotated `@ListGroup`
    pub list_group: Option<ListGroupFactoryFn>,
}

impl<C, F> DeclaredField<C, F> {
    fn instance<'a>(&self, instance: &'a compat::Object) -> &'a compat::Shared<C>
    where
        C: Send + 'static,
    {
        match (**instance).downcast_ref::<compat::Shared<C>>() {
            Some(handle) => handle,
            None => panic!("IllegalArgumentException: Can not get field {} on an object of another class", self.name),
        }
    }
}

impl<C: Send + 'static, F: Clone + Send + Sync + 'static> FieldOps for DeclaredField<C, F> {
    fn name(&self) -> &'static str {
        self.name
    }

    fn generic_type_name(&self) -> &'static str {
        self.generic_type
    }

    fn type_class(&self) -> TypeClass {
        self.type_class
    }

    fn annotations(&self) -> Vec<compat::Object> {
        (self.annotations)()
    }

    fn get(&self, instance: &compat::Object) -> compat::Object {
        let value: F = compat::read(self.instance(instance), |c| (self.get)(c));
        Arc::new(value)
    }

    fn set(&self, instance: &compat::Object, value: compat::Object) {
        let value: F = match (*value).downcast_ref::<F>() {
            Some(v) => v.clone(),
            None => panic!("IllegalArgumentException: Can not set {} field {} to a value of another type", self.generic_type, self.name),
        };
        compat::write(self.instance(instance), |c| (self.set)(c, value));
    }

    fn to_json_tree(&self, value: &compat::Object) -> compat::GsonValue {
        match (**value).downcast_ref::<F>() {
            Some(v) => (self.to_gson)(v),
            None => panic!("ClassCastException: a value of another type for the field {}", self.name),
        }
    }

    fn from_json_tree(&self, json: &serde_json::Value) -> Option<compat::Object> {
        (self.read_gson)(json).map(|v| Arc::new(v) as compat::Object)
    }

    fn list_group(&self) -> Option<ListGroupFactoryFn> {
        self.list_group
    }
}

/// java: `java.lang.reflect.Field` of a config class.
#[derive(Clone)]
pub struct Field {
    ops: Arc<dyn FieldOps>,
}

impl Field {
    /// The field a generated [`DeclaredField`] describes.
    pub fn of(field: impl FieldOps + 'static) -> Field {
        Field { ops: Arc::new(field) }
    }

    /// java: String getName()
    pub fn get_name(&self) -> String {
        self.ops.name().to_string()
    }

    /// java: Type getGenericType()
    pub fn get_generic_type(&self) -> FieldType {
        FieldType { ops: self.ops.clone() }
    }

    /// java: Class<?> getType()
    pub fn get_type(&self) -> TypeClass {
        self.ops.type_class()
    }

    /// java: <A extends Annotation> A getAnnotation(Class<A> annotationClass) - `None` is Java's `null`
    pub fn get_annotation(&self, annotation_class: &TypeId) -> Option<compat::Object> {
        self.ops.annotations().into_iter().find(|a| Any::type_id(&**a) == *annotation_class)
    }

    /// java: Annotation[] getAnnotations()
    pub fn get_annotations(&self) -> Vec<compat::Object> {
        self.ops.annotations()
    }

    /// java: boolean isAnnotationPresent(Class<? extends Annotation> annotationClass)
    pub fn is_annotation_present(&self, annotation_class: &TypeId) -> bool {
        self.get_annotation(annotation_class).is_some()
    }

    /// java: void setAccessible(boolean flag) - the generated accessors are not access-checked
    pub fn set_accessible(&self, _flag: bool) {}

    /// java: Object get(Object obj)
    pub fn get(&self, obj: &compat::Object) -> compat::Object {
        self.ops.get(obj)
    }

    /// java: void set(Object obj, Object value)
    pub fn set(&self, obj: &compat::Object, value: compat::Object) {
        self.ops.set(obj, value)
    }

    /// the `@ListGroup` instantiation of this field (see [`FieldOps::list_group`])
    pub fn list_group(&self) -> Option<ListGroupFactoryFn> {
        self.ops.list_group()
    }
}

impl std::fmt::Display for Field {
    /// java: Field.toString() - `<type> <declaring class>.<name>`; the declaring class is not recorded, the
    /// type and name are what the log line (`OptionFactoryRegistry.createOption`) needs
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.ops.generic_type_name(), self.ops.name())
    }
}

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

/// java: `java.lang.reflect.Type` - a field's generic type. Gson (de)serialises a value "by type": the per-field Gson
/// form fabric2seer generated for that type.
#[derive(Clone)]
pub struct FieldType {
    ops: Arc<dyn FieldOps>,
}

impl FieldType {
    /// java: String getTypeName()
    pub fn get_type_name(&self) -> String {
        self.ops.generic_type_name().to_string()
    }

    /// java: gson.toJsonTree(value, type)
    pub fn to_json_tree(&self, value: &compat::Object) -> compat::GsonValue {
        self.ops.to_json_tree(value)
    }

    /// java: gson.fromJson(json, type)
    pub fn from_json_tree(&self, json: &serde_json::Value) -> Option<compat::Object> {
        self.ops.from_json_tree(json)
    }
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.ops.generic_type_name())
    }
}

// ---------------------------------------------------------------------------------------------------------------
// java.lang.Class of a field's type
// ---------------------------------------------------------------------------------------------------------------

/// java: `Class<T>` of a field's type (`Field.getType()`, `ReadOnlyFieldAccess.typeClass()`): what YACL asks of it -
/// its names, and for an enum its constants (`getEnumConstants`), each constant's `name()` and whether the enum
/// implements `NameableEnum` / `EnumCycler.CyclableEnum` (`isAssignableFrom` / `instanceof`).
#[derive(Clone, Copy)]
pub struct TypeClass {
    /// `getName()`
    pub name: &'static str,
    /// `getSimpleName()`
    pub simple_name: &'static str,
    /// `getEnumConstants()`: `None` for a class that is not an enum (Java's `null`)
    pub enum_constants: Option<fn() -> Vec<compat::Object>>,
    /// `Enum.name()` of a constant of this enum
    pub enum_name: Option<fn(&compat::Object) -> String>,
    /// `((NameableEnum) v).getDisplayName()`: `Some` when the enum implements `NameableEnum`
    pub nameable: Option<fn(&compat::Object) -> MutableComponent>,
    /// `((CyclableEnum<?>) v).allowedValues()`: `Some` when the enum implements `EnumCycler.CyclableEnum`
    pub cyclable: Option<fn(&compat::Object) -> Vec<compat::Object>>,
}

impl TypeClass {
    /// a class that is not an enum
    pub const fn plain(name: &'static str, simple_name: &'static str) -> TypeClass {
        TypeClass { name, simple_name, enum_constants: None, enum_name: None, nameable: None, cyclable: None }
    }

    /// the class of a field whose type is the enum `E` (`compat::JavaEnum`, which a converted enum implements)
    pub fn of_enum<E: compat::JavaEnum + Send + Sync>(name: &'static str, simple_name: &'static str) -> TypeClass {
        TypeClass {
            name,
            simple_name,
            enum_constants: Some(|| E::enum_constants().into_iter().map(|e| Arc::new(e) as compat::Object).collect()),
            enum_name: Some(|o| enum_constant::<E>(o).name()),
            nameable: Some(|o| match enum_constant::<E>(o).nameable_display_name() {
                Some(display_name) => display_name,
                // a constant of an enum that does not implement NameableEnum: `isAssignableFrom` is per class, so a
                // converted enum answers for all of its constants alike - see `of_plain_enum`
                None => mcrust::net::minecraft::network::chat::component::literal(&enum_constant::<E>(o).name()),
            }),
            cyclable: None,
        }
    }

    /// [`TypeClass::of_enum`] of an enum that does NOT implement `NameableEnum`
    pub fn of_plain_enum<E: compat::JavaEnum + Send + Sync>(name: &'static str, simple_name: &'static str) -> TypeClass {
        TypeClass { nameable: None, ..TypeClass::of_enum::<E>(name, simple_name) }
    }

    /// [`TypeClass::of_enum`] of an enum that implements `EnumCycler.CyclableEnum`
    pub fn of_cyclable_enum<E>(name: &'static str, simple_name: &'static str, nameable: bool) -> TypeClass
    where
        E: compat::JavaEnum + Send + Sync + crate::dev::isxander::yacl3::config::v2::api::autogen::enum_cycler::CyclableEnum<E>,
    {
        let class = if nameable { TypeClass::of_enum::<E>(name, simple_name) } else { TypeClass::of_plain_enum::<E>(name, simple_name) };
        TypeClass {
            cyclable: Some(|o| {
                use crate::dev::isxander::yacl3::config::v2::api::autogen::enum_cycler::CyclableEnum as _;
                enum_constant::<E>(o).allowed_values().into_iter().map(|e| Arc::new(e) as compat::Object).collect()
            }),
            ..class
        }
    }

    /// java: String getName()
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    /// java: String getSimpleName()
    pub fn get_simple_name(&self) -> String {
        self.simple_name.to_string()
    }

    /// java: boolean isEnum()
    pub fn is_enum(&self) -> bool {
        self.enum_constants.is_some()
    }

    /// java: T[] getEnumConstants() - `None` is Java's `null` (not an enum)
    pub fn get_enum_constants(&self) -> Option<Vec<compat::Object>> {
        self.enum_constants.map(|f| f())
    }

    /// java: NameableEnum.class.isAssignableFrom(this)
    pub fn is_nameable_enum(&self) -> bool {
        self.nameable.is_some()
    }
}

/// `(E) o` of an enum constant held as `Object`
fn enum_constant<E: compat::JavaEnum + Send + Sync>(o: &compat::Object) -> &E {
    match (**o).downcast_ref::<E>() {
        Some(e) => e,
        None => panic!("ClassCastException: not a constant of {}", std::any::type_name::<E>()),
    }
}

impl std::fmt::Display for TypeClass {
    /// java: Class.toString()
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {}", self.name)
    }
}

impl std::fmt::Debug for TypeClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl PartialEq for TypeClass {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

/// java: `Enum<?>` - a constant of an enum YACL does not know (`EnumCyclerImpl` is a
/// `SimpleOptionFactory<EnumCycler, Enum<?>>`): the constant itself plus what `Enum` answers of it. Two are equal
/// when they are the same constant (Java compares enum constants by identity).
#[derive(Clone)]
pub struct EnumValue {
    /// the constant (the generated Rust enum value)
    pub value: compat::Object,
    /// its class
    pub class: TypeClass,
    /// java: Enum.ordinal()
    pub ordinal: i32,
    /// java: Enum.name()
    pub name: String,
}

impl EnumValue {
    /// the `Enum<?>` view of a constant of `class`
    pub fn of(value: compat::Object, class: TypeClass) -> EnumValue {
        let name = match class.enum_name {
            Some(name_of) => name_of(&value),
            None => panic!("ClassCastException: class {} cannot be cast to class java.lang.Enum", class.name),
        };
        let ordinal = class
            .get_enum_constants()
            .unwrap_or_default()
            .iter()
            .position(|c| class.enum_name.map(|n| n(c)).as_deref() == Some(name.as_str()))
            .map_or(-1, |i| i as i32);
        EnumValue { value, class, ordinal, name }
    }

    /// java: Enum.name()
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: Enum.ordinal()
    pub fn ordinal(&self) -> i32 {
        self.ordinal
    }
}

impl PartialEq for EnumValue {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class && self.ordinal == other.ordinal
    }
}

impl std::fmt::Debug for EnumValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl std::fmt::Display for EnumValue {
    /// java: Enum.toString() - its name
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

// ---------------------------------------------------------------------------------------------------------------
// a Class element of an annotation
// ---------------------------------------------------------------------------------------------------------------

/// java: a `Class<? extends I>` element of an annotation. `new_instance` is `getConstructor().newInstance()`: the new
/// object as the interface the element is bounded by - a `compat::Object` holding a
/// `compat::RenderThread<Arc<dyn I<X>>>` (X the interface's type argument the class implements; `RenderThread`
/// because an `Object` is `Send + Sync` and the mod's objects are not); `None` when the class has no public no-args
/// constructor (`NoSuchMethodException`).
#[derive(Clone, Copy)]
pub struct ClassRef {
    /// `getName()`
    pub name: &'static str,
    pub new_instance: Option<fn() -> compat::Object>,
}

impl ClassRef {
    /// java: String getName()
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}

impl PartialEq for ClassRef {
    /// java: Class.equals - one class per name
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::fmt::Debug for ClassRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {}", self.name)
    }
}

impl std::fmt::Display for ClassRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {}", self.name)
    }
}

// ---------------------------------------------------------------------------------------------------------------
// Java's generic casts of an Object
// ---------------------------------------------------------------------------------------------------------------

/// `(T) obj`: Java's unchecked generic cast of an `Object`, done where Rust needs the static type. `T` = `Object`
/// is the object itself; `T` = [`EnumValue`] views an enum constant of `class`. A value of another type is the
/// `ClassCastException` Java throws where the value is used.
pub fn object_to<T: Clone + 'static>(o: compat::Object, class: TypeClass) -> T {
    if let Some(same) = (&o as &dyn Any).downcast_ref::<T>() {
        return same.clone();
    }
    if let Some(v) = (*o).downcast_ref::<T>() {
        return v.clone();
    }
    if TypeId::of::<T>() == TypeId::of::<EnumValue>() {
        let e = EnumValue::of(o, class);
        return (&e as &dyn Any).downcast_ref::<T>().unwrap().clone();
    }
    panic!("ClassCastException: a value of class {} cannot be cast to {}", class.name, std::any::type_name::<T>());
}

/// `(Object) value`: a value as Java's `Object` (an `Object` stays itself; an [`EnumValue`] is its constant).
pub fn object_from<T: Clone + Send + Sync + 'static>(value: T) -> compat::Object {
    if let Some(o) = (&value as &dyn Any).downcast_ref::<compat::Object>() {
        return o.clone();
    }
    if let Some(e) = (&value as &dyn Any).downcast_ref::<EnumValue>() {
        return e.value.clone();
    }
    Arc::new(value)
}

/// `Optional<A>` of an annotation looked up by class (`ReadOnlyFieldAccess.getAnnotation(A.class)`).
pub fn annotation_as<A: Clone + 'static>(annotation: Option<compat::Object>) -> Option<A> {
    annotation.and_then(|a| (*a).downcast_ref::<A>().cloned())
}

// ---------------------------------------------------------------------------------------------------------------
// the record
// ---------------------------------------------------------------------------------------------------------------

/// java: public record ReflectionFieldAccess<T>(Field field, Object instance) implements FieldAccess<T>
pub struct ReflectionFieldAccess<T> {
    /// java: Field field
    pub field: Field,
    /// java: Object instance - the config object (`Arc<compat::Shared<C>>`)
    pub instance: compat::Object,
    _type: std::marker::PhantomData<fn() -> T>,
}

impl<T> Clone for ReflectionFieldAccess<T> {
    fn clone(&self) -> Self {
        ReflectionFieldAccess { field: self.field.clone(), instance: self.instance.clone(), _type: std::marker::PhantomData }
    }
}

impl<T: Clone + 'static> ReflectionFieldAccess<T> {
    /// java: the canonical record constructor
    pub fn new(field: Field, instance: compat::Object) -> Self {
        Self { field, instance, _type: std::marker::PhantomData }
    }

    /// java: the record accessor `field()`
    pub fn field(&self) -> Field {
        self.field.clone()
    }

    /// java: the record accessor `instance()`
    pub fn instance(&self) -> compat::Object {
        self.instance.clone()
    }
}

impl<T: Clone + Send + Sync + 'static> FieldAccess<T> for ReflectionFieldAccess<T> {
    /// java: @Override public void set(T value)
    fn set(&self, value: T) {
        // java: try { field.set(instance, value); } catch (IllegalAccessException e) - the generated accessors are
        // never access-checked, so the catch cannot fire
        self.field.set(&self.instance, object_from(value));
    }
}

impl<T: Clone + Send + Sync + 'static> ReadOnlyFieldAccess<T> for ReflectionFieldAccess<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public T get()
    fn get(&self) -> T {
        // java: try { return (T) field.get(instance); } catch (IllegalAccessException e) - never access-checked
        object_to::<T>(self.field.get(&self.instance), self.field.get_type())
    }

    /// java: @Override public String name()
    fn name(&self) -> String {
        self.field.get_name()
    }

    /// java: @Override public Type type()
    fn r#type(&self) -> FieldType {
        self.field.get_generic_type()
    }

    /// java: @Override public Class<T> typeClass()
    fn type_class(&self) -> TypeClass {
        self.field.get_type()
    }

    /// java: @Override public <A extends Annotation> Optional<A> getAnnotation(Class<A> annotationClass)
    fn get_annotation(&self, annotation_class: &TypeId) -> Option<compat::Object> {
        self.field.get_annotation(annotation_class)
    }
}
