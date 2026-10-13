//! The types the converter (fabric2seer) could not place, and where each is now. Nothing is left unresolved; the module
//! stays so `lib.rs`'s layout is the converter's.
//!
//! - `java.lang.Class`, `Constructor`, `Field`, `Type`, `Annotation`: the config reflection is
//!   `config::v2::impl::reflection_field_access` (`ReflectedClass`, `DeclaredField`, `compat::ClassOf`).
//! - `java.lang.Enum`: `compat::JavaEnum`.
//! - `Option`, `OptionAddable`, `OptionEventListener`, `OptionFlag`, `OptionGroup`, `OptionFactory`,
//!   `ReadonlyConfigEntry`, `CategoryAndGroups`, `Component`, `Screen`, `NarrationPriority`, `ContainerEventHandler`,
//!   `TabNavigationBarAccessor`: misattributed packages of the mod's own / the port's types (`api::option::Option`,
//!   `compat::ContainerEventHandler`, `mixins::tab_navigation_bar_accessor`, ...).
//! - `java.awt.Color` (`com.google.gson.Color`): `mcrust::java::awt::color::Color`.
//! - `Executor`, `ExecutorService`, `PreparationBarrier`, `SharedState`, `Collector`, `Characteristics`: the image
//!   thread and reload listener (`gui::image::image_renderer_manager`, `gui::image::yacl_image_reload_listener`).
//! - `BufferedImage`, `Graphics2D`, `ImageReader`, `IIOMetadata`, `IIOMetadataNode`, `Rectangle`, `InputStream`: GIF /
//!   WebP decoding - GAPS_Y2.md G10 (`gui::image::impl::animated_dynamic_texture_image`).
//! - `JsonReader`, `JsonWriter`, `GsonReader`, `GsonWriter`, `FieldAttributes`, `JsonSerializationContext`,
//!   `JsonDeserializationContext`, `StringWriter`, `RuntimeException`: the config serializers (`compat`'s Gson /
//!   JSON5 support, Y1).
//! - `T`: a type parameter.
