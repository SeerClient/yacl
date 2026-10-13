//! Ported from dev.isxander.yacl3.platform.YACLConfig.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::{self, ConfigClassHandler};
use crate::dev::isxander::yacl3::config::v2::api::config_serializer::ConfigSerializer;
use crate::dev::isxander::yacl3::config::v2::api::serial_entry::SerialEntry;
use crate::dev::isxander::yacl3::config::v2::api::serializer::gson_config_serializer_builder;
use crate::dev::isxander::yacl3::config::v2::r#impl::reflection_field_access::{DeclaredField, Field, ReflectedClass, TypeClass};
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;

/// java: public static final ConfigClassHandler<YACLConfig> HANDLER = ConfigClassHandler.createBuilder(YACLConfig.class)
///     .id(YACLPlatform.rl("config"))
///     .serializer(config -> GsonConfigSerializerBuilder.create(config)
///         .setPath(YACLPlatform.getConfigDir().resolve("yacl.json5"))
///         .setJson5(true)
///         .build())
///     .build();
pub static HANDLER: compat::MainThread<Arc<dyn ConfigClassHandler<YACLConfig>>> = compat::MainThread::new(|| {
    config_class_handler::create_builder(&YACLConfig::class_of())
        .id(YACLPlatform::rl("config"))
        .serializer(Arc::new(|config: Arc<dyn ConfigClassHandler<YACLConfig>>| -> Arc<dyn ConfigSerializer<YACLConfig>> {
            gson_config_serializer_builder::create(&config)
                .set_path(YACLPlatform::get_config_dir().join("yacl.json5"))
                .set_json5(true)
                .build()
        }))
        .build()
});

/// java: public class YACLConfig
#[derive(Clone)]
pub struct YACLConfig {
    /// java: @SerialEntry(comment = "Show the flashing colour picker hint (auto disables after first use)")
    /// public boolean showColorPickerIndicator = true;
    pub show_color_picker_indicator: bool,
    /// java: @SerialEntry(comment = "Load .webp and .gif during Minecraft resource reload instead of on-demand (can decrease startup time)")
    /// public boolean preloadComplexImageFormats = false;
    pub preload_complex_image_formats: bool,
}

impl YACLConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { show_color_picker_indicator: true, preload_complex_image_formats: false }
    }

    fn to_gson(&self) -> compat::GsonValue {
        compat::GsonValue::Obj(vec![
            ("showColorPickerIndicator".to_string(), compat::GsonValue::Bool(self.show_color_picker_indicator)),
            ("preloadComplexImageFormats".to_string(), compat::GsonValue::Bool(self.preload_complex_image_formats)),
        ])
    }

    fn read_gson(json: &serde_json::Value) -> Option<YACLConfig> {
        let object = json.as_object()?;
        let mut config = YACLConfig::new();
        if let Some(v) = object.get("showColorPickerIndicator").and_then(compat::gson_bool) {
            config.show_color_picker_indicator = v;
        }
        if let Some(v) = object.get("preloadComplexImageFormats").and_then(compat::gson_bool) {
            config.preload_complex_image_formats = v;
        }
        Some(config)
    }
}

impl Default for YACLConfig {
    fn default() -> Self {
        Self::new()
    }
}

fn serial_entry(comment: &str) -> compat::Object {
    let mut entry = SerialEntry::of();
    entry.comment = comment.to_string();
    Arc::new(entry)
}

/// `YACLConfig.class` as YACL reflects over it (`getDeclaredFields()` with their `@SerialEntry`s).
impl ReflectedClass for YACLConfig {
    fn class_of() -> compat::ClassOf<YACLConfig> {
        compat::ClassOf {
            name: "dev.isxander.yacl3.platform.YACLConfig",
            new: YACLConfig::new,
            to_gson: YACLConfig::to_gson,
            read_gson: YACLConfig::read_gson,
            patch: |config: &mut YACLConfig, path: &[&str], value: &compat::PatchValue| match (path, value) {
                (["showColorPickerIndicator"], compat::PatchValue::Bool(b)) => {
                    config.show_color_picker_indicator = *b;
                    true
                }
                (["preloadComplexImageFormats"], compat::PatchValue::Bool(b)) => {
                    config.preload_complex_image_formats = *b;
                    true
                }
                _ => false,
            },
            is_enum: false,
            enum_constants: Vec::new,
        }
    }

    fn declared_fields() -> Vec<Field> {
        vec![
            Field::of(DeclaredField::<YACLConfig, bool> {
                name: "showColorPickerIndicator",
                generic_type: "boolean",
                type_class: TypeClass::plain("boolean", "boolean"),
                annotations: || vec![serial_entry("Show the flashing colour picker hint (auto disables after first use)")],
                get: |c| c.show_color_picker_indicator,
                set: |c, v| c.show_color_picker_indicator = v,
                to_gson: |v| compat::GsonValue::Bool(*v),
                read_gson: compat::gson_bool,
                list_group: None,
            }),
            Field::of(DeclaredField::<YACLConfig, bool> {
                name: "preloadComplexImageFormats",
                generic_type: "boolean",
                type_class: TypeClass::plain("boolean", "boolean"),
                annotations: || {
                    vec![serial_entry(
                        "Load .webp and .gif during Minecraft resource reload instead of on-demand (can decrease startup time)",
                    )]
                },
                get: |c| c.preload_complex_image_formats,
                set: |c, v| c.preload_complex_image_formats = v,
                to_gson: |v| compat::GsonValue::Bool(*v),
                read_gson: compat::gson_bool,
                list_group: None,
            }),
        ]
    }
}
