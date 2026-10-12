# PATTERNS_Y1 - YACL config (v2 / v3 / legacy), batch Y1

Every file under `src/dev/isxander/yacl3/config/` was rewritten by hand against the Java (9406d5c, 26.2-fabric) and
Y2's new API shape (`Arc<dyn Trait>`, `&self` methods with state in cells, builders by value). These are the shapes
used, so a converted mod using YACL config compiles against them, and what fabric2seer must generate.

## R1 - Java reflection over a USER config class (converter requirement)

YACL v2 (`ConfigClassHandler`, `@SerialEntry`, `@AutoGen`, `GsonConfigSerializer`) and the legacy
`GsonConfigInstance` reflect over the mod's config class. Rust has no reflection; the stand-ins live in
`config/v2/impl/reflection_field_access.rs` and fabric2seer must GENERATE, for every class a `X.class` literal hands
to `ConfigClassHandler.createBuilder` / `GsonConfigInstance`:

```rust
impl yacl::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::ReflectedClass for MyConfig {
    fn class_of() -> compat::ClassOf<MyConfig> { /* the X.class literal it already generates */ }
    fn declared_fields() -> Vec<Field> {            // getDeclaredFields(), declaration order, no statics
        vec![
            Field::of(DeclaredField::<MyConfig, i32> {
                name: "maxCount",                      // the JAVA field name (keys, serial names, @MasterTickBox)
                generic_type: "int",                   // getGenericType().getTypeName()
                type_class: TypeClass::plain("int", "int"),          // getType(); enums: see below
                annotations: || vec![                  // RUNTIME annotations, as YACL's annotation STRUCTS
                    Arc::new(SerialEntry::of()) as compat::Object,
                    Arc::new(AutoGen { category: "general".into(), group: "".into() }),
                    Arc::new(IntSlider::of(0, 10, 1)),
                ],
                get: |c| c.max_count,
                set: |c, v| c.max_count = v,
                to_gson: |v| compat::GsonValue::int(*v as i64),   // the field type's Gson form (as for ClassOf)
                read_gson: |j| compat::gson_i64(j).map(|v| v as i32), // None = Gson would reject / cannot hold null
                list_group: None,
            }),
            // ...
        ]
    }
    fn class_annotations() -> Vec<compat::Object> { vec![] }   // the class's own @SerialEntry, if any
}
```

* An annotation instance is the annotation's struct (`IntSlider { min, max, step, format }`), every element filled
  (declared defaults where the use site leaves them out - each struct has `of(required..)` with the defaults).
  `X.class` of an annotation is `std::any::TypeId::of::<X>()` (the converter already does this).
* A `Class<? extends I<?>>` annotation element (`@CustomFormat.value`, `@CustomImage.factory`,
  `@ListGroup.valueFactory/controllerFactory`) is a `ClassRef { name, new_instance }` whose `new_instance` returns
  `Arc::new(compat::RenderThread::new(Arc::new(TheClass::new()) as Arc<dyn I<X>>)) as compat::Object` (`None` when
  the class has no public no-args constructor). X is the interface's type argument the class implements.
* An enum-typed field: `type_class: TypeClass::of_enum::<E>(fqcn, simple)` (E implements `NameableEnum`),
  `TypeClass::of_plain_enum::<E>(..)` (it does not), `TypeClass::of_cyclable_enum::<E>(.., nameable)` (it implements
  `EnumCycler.CyclableEnum`). E must implement `compat::JavaEnum` (Y2's trait) and be `Send + Sync`.
* A `List<X>` field annotated `@ListGroup`: `list_group: Some(list_group_impl::create_erased::<X>)` (Java erases X;
  Rust must instantiate `ListGroupImpl<X>` for the field).
* The config object is `compat::Shared<MyConfig>` (a Java reference to a mutable mod object); MyConfig must be `Send`
  (it rides in a `compat::Object`). Field types must be `Clone + Send + Sync + 'static`.

## R2 - a config object is `compat::Shared<T>`; `instance()` is the live object

`ConfigClassHandler<T>::instance()/defaults()` and `ConfigInstance<T>::get_config()/get_defaults()` answer
`compat::Shared<T>`, the same object Java's reference is: `compat::write(&HANDLER.instance(), |c| c.x = 5)` is
`HANDLER.instance().x = 5`. (Y2's `YetAnotherConfigLib.create` already hands these to the `ConfigBackedBuilder`.)

## R3 - `&self` everywhere, state in cells (shared with Y2's P1)

Handlers, serializers, field accesses, entries and their builders are `Arc<dyn X>` with `&self` methods; fields Java
assigns later are `RefCell` / `Cell` / `OnceCell`. A handler is re-entered by its serializer while it loads
(`load -> serializer.loadSafely -> config.fields()`), and a builder chain reaches the same object while the previous
call's receiver is borrowed - a `&mut self` method would panic there. User code calls `HANDLER.load()` /
`HANDLER.save()` directly on the `Arc` (no `lock().borrow_mut()`).
`ConfigClassHandler.Builder` and `GsonConfigSerializerBuilder` are `&self` (they are interfaces held as
`Arc<dyn ..>`); the concrete `GsonConfigInstance.Builder` is by value (`self -> Self`), as Y2's builders.

## R4 - Java's `this` handed out: `Arc::new_cyclic` + `self_ref: Weak<Self>`

`return this` from a builder / entry (`modifyGet`), and `this` handed to a constructor argument
(`serializerFactory.apply(this)`, each field's `parent`), use the object's own `Weak` (`Arc::new_cyclic`). A default
interface method that needs `this` in a lambda is a free function taking `this: &Arc<dyn X>`
(`config_entry::as_binding`), as Y2's `binding::xmap`.

## R5 - wildcard views (`<?>`)

* `ConfigField<?>` is `Arc<dyn ConfigField<compat::Object>>`; a factory's `ConfigField<T>` is
  `option_factory_registry::TypedConfigField<T>` over it (Java's unchecked cast); values cross with
  `reflection_field_access::object_to` / `object_from`.
* `Enum<?>` is `reflection_field_access::EnumValue` (`EnumCyclerImpl` is a factory for `EnumValue`).
* `Option<?>` is Y2's `Arc<dyn AnyOption>` (a typed option upcasts).
* `ConfigClassHandler<?>` is `config_class_handler_impl::WildcardClassHandler<T>` (holds the handler weakly).
* `ReadonlyConfigEntry<?>` is `readonly_config_entry::AnyReadonlyConfigEntry` (supertrait, like `AnyOption`).

## R6 - covariant override under the prelude

Java `ConfigEntry` overrides `ReadonlyConfigEntry.modifyGet/onGet` with a covariant return. Both traits are in the
prelude, so a same-named method is ambiguous at every call: the overrides are `modify_get_entry` / `on_get_entry`.
Count: 2 (any other covariant interface override in the crate has the same problem).

## R7 - `instanceof` an interface on an erased builder

`AutoGenUtils.addCustomFormatterToController` asks `controller instanceof ValueFormattableController`.
`SimpleOptionFactory::create_controller` answers `auto_gen_utils::ControllerBuilderRef<T>`:
`ControllerBuilderRef::formattable(builder)` for a builder class implementing `ValueFormattableController`,
`ControllerBuilderRef::plain(builder)` otherwise. A converted user `SimpleOptionFactory` subclass must pick by the
returned builder's class. Count: 18 factories (16 in YACL).

## R8 - inherited abstract-class methods

`SimpleOptionFactory.createOption` is inherited by every subclass: each `impl OptionFactory for X` calls
`simple_option_factory::create_option(self, ..)` (X: `Clone`, the lambdas keep `this` by value - the factories are
stateless). `CodecConfig.register` is `CodecConfigBase::register/register_2`, called from each subclass's
`impl EntryAddable`. Abstract bases are `XBase` structs + `X` traits whose provided methods hold the bodies.

## R9 - Java exceptions

`throw new YACLAutoGenException(msg)` is `YACLAutoGenException::new(msg).throw()` (a panic, `-> !`); a `catch` is
`std::panic::catch_unwind`. Other exceptions are `panic!("ExceptionClass: msg")` as elsewhere in the crate.

## R10 - logging

`YACLConstants.LOGGER` is Y2's `r#impl::utils::yacl_constants::LOGGER` (not the prelude's `logger()`, which is the
mod id's). Count: 7 files.
