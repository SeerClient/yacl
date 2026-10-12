# GAPS_Y1 - YACL config (v2 / v3 / legacy), batch Y1

Each row: the Java member, what the port / crate has, what is needed, sites. "converter" = fabric2seer must generate
it; "compat" = `src/compat.rs` (batch Y2 / orchestrator); "Y2" = another file of this crate.

| # | Java | Port / crate now | Needed | Sites |
|---|---|---|---|---|
| G1 | `GsonBuilder.setFieldNamingPolicy(LOWER_CASE_WITH_UNDERSCORES)` (YACL's default for `GsonConfigSerializer` and `GsonConfigInstance`) on NESTED objects | `compat::FieldNamingPolicy` has only `Identity`; the generated per-type Gson forms write Java field names | the generated `to_gson` / `read_gson` of a mod class take the Gson's naming policy (or a second, snake_case form) - compat + converter | `gson_config_serializer.rs` (`Builder::new`), `gson_config_instance.rs` (top level IS applied by YACL's own field walk; nested is not) |
| G2 | `registerTypeHierarchyAdapter(Component/Style/Color/Item, ..)` | `compat::GsonBuilder::register_type_hierarchy_adapter()` is a no-op; the adapters' bodies are ported (`StyleTypeAdapter`, `ColorTypeAdapter`, `ItemTypeAdapter`, `CodecSerializerAdapter`) | the generated Gson form of a `Component` / `Style` / `Color` / `Item` field must be these bodies (Color = `getRGB()` int, Item = registry key string, Style/Component = their codecs) - converter | 2 builders |
| G3 | Gson's reflective adapter: skips `transient` fields, includes superclass fields | `ReflectedClass::declared_fields` carries no modifiers and only the class's own fields (right for v2, which uses `getDeclaredFields`) | `GsonConfigInstance` (legacy) walks only declared fields; a transient `@ConfigEntry` field or a superclass's would differ - converter (modifiers / superclass list) | `gson_config_instance.rs` |
| G4 | `@CustomImage.factory().getTypeParameters()[0].getName()` (error message) | `ClassRef` records no type parameters; the message names the class | nothing functional | `simple_option_factory.rs` |
| G5 | GC of a handler | handler -> serializer -> handler (`ConfigSerializerBase.config`) is an `Arc` cycle; a handler is never freed | acceptable (handlers are statics); a `Weak` would change `ConfigSerializer.config`'s type | `config_serializer.rs` |
| G6 | `YACLPlatform.getEnvironment()` | Y2's `yacl_platform.rs` body is still `todo!()` | Y2: `Env::Client` on the client - until then `ConfigClassHandlerImpl::new` panics | `config_class_handler_impl.rs` |
| G7 | `Item` option values | Y2's item controller takes `compat::JavaRef<Item>` (not yet in compat); `ItemRegistryHelper` still answers `Arc<Item>` | compat `JavaRef<Item>` must be `Clone + PartialEq + Send + Sync`; the Gson `ItemTypeAdapter` uses `Arc<Item>` until `ItemRegistryHelper` moves to `JavaRef` - Y2 | `item_field_impl.rs`, `option_factory_registry.rs`, the two `ItemTypeAdapter`s |
| G8 | `JsonParser.parseString` (lenient Gson) in `JsonFileCodecConfig.loadFromFile` / `GsonConfigInstance.load` | parsed by the quilt-JSON5 reader ported in `gson_config_serializer.rs` (comments, unquoted names, single quotes, trailing commas) | Gson's LENIENT also takes unquoted string VALUES and `=`/`=>`/`;` separators - not read | `parse_lenient` |
| G9 | `ConfigClassHandler.id()` answering `null` | `id()` panics (`NullPointerException`) where Java returns null | nothing for YACL (every reader dereferences it); a user calling `id()` on an id-less handler gets the NPE at the call | `config_class_handler.rs` |
| G10 | `gson.fromJson(json, Config.class)` answering `null` for the text `null` (legacy `GsonConfigInstance.load` -> `setConfig(null)`) | a `compat::Shared<T>` cannot be null: the JSON `null` document is an exception (`Expected BEGIN_OBJECT`) | none practical | `gson_config_instance.rs` |
| G11 | a JSON `null` read into an OBJECT-typed field (Gson sets the field to null) | a non-`Option` Rust field cannot hold null: the field keeps its default (`GsonConfigSerializer` logs "Leaving as default", the legacy instance skips it) | a nullable field is `Option<X>` in the generated class and its `read_gson` answers `Some(None)` - converter | `gson_config_serializer.rs`, `gson_config_instance.rs` |
| G12 | `CodecConfig implements Codec<S>` as a value of the config itself | the port's `Codec` is `Send + Sync`; a config holds the mod's objects: `codec_config::codec(&this)` wraps the config in `compat::RenderThread` (render-thread only, checked) | none (a config is only encoded on the render thread) | `codec_config.rs` |
| G13 | `JsonFileCodecConfig`'s constructor calling the virtual `createGson()` | `gson` is a `OnceCell` filled on first read through the override (`JsonFileCodecConfig::gson`) | none (same object, same override; built later than Java) | `json_file_codec_config.rs` |

## Cross-batch API this batch relies on (Y2 / compat), as it stood when written

* `api::option`: `AnyOption`, `Option<T>`, `create_builder::<T>()` (by-value `Builder`), `Event`, `OptionEventListener`
  closures `Fn(&dyn Option<T>, Event)`.
* `api::controller::*_controller_builder::create(Arc<dyn Option<X>>)` (by value), `ValueFormattableController<T>`
  (`format_value(self, Arc<dyn ValueFormatter<T>>) -> Self`), `SliderControllerBuilder::range/step`,
  `NumberFieldControllerBuilder::range`, `BooleanControllerBuilder::coloured/*_formatter`, `ColorControllerBuilder::allow_alpha`,
  `DropdownStringControllerBuilder::values_string/allow_empty_value/allow_any_value`,
  `CyclingListControllerBuilder::values_iterable`, `ControllerBuilder<T>` for `Arc<dyn ControllerBuilder<T>>`.
* `api::config_category` / `api::option_group` / `api::option_addable` / `api::yet_another_config_lib` by-value builders
  (`r#impl::config_category_impl::BuilderImpl`, `r#impl::option_group_impl::BuilderImpl`), `option_description::of(Vec)`,
  `option_description::create_builder()` (`r#impl::option_description_impl::BuilderImpl`), `ImageFuture`.
* `api::list_option::create_builder::<T>()`, `api::label_option::create(MutableComponent)`, `api::binding::generic`.
* `compat::LocalFuture`, `compat::RenderThread`, `compat::JavaEnum`, `compat::JavaRef` (G7),
  `r#impl::utils::yacl_constants::LOGGER`, `YACLPlatform::{get_environment, is_development_env, rl_2}`.
