//! Ported from dev.isxander.yacl3.platform.YACLConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler;
use crate::dev::isxander::yacl3::config::v2::api::serializer::gson_config_serializer_builder;
use crate::dev::isxander::yacl3::platform::yacl_platform::YACLPlatform;
use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::Builder as _;
use crate::dev::isxander::yacl3::config::v2::api::serializer::gson_config_serializer_builder::GsonConfigSerializerBuilder as _;

/// java: public static final ConfigClassHandler<YACLConfig> HANDLER = ConfigClassHandler.createBuilder(YACLConfig.class) .id(YACLPlatform.rl("config")) .serializer(config -> GsonConfigSerializerBuilder.create(
pub static HANDLER: compat::MainThread<compat::Shared<dyn ConfigClassHandler<YACLConfig>>> = compat::MainThread::new(|| ({ let (__a0,) = (Arc::new(move |config| ({ let (__a0,) = (YACLPlatform::get_config_dir().join("yacl.json5"),); gson_config_serializer_builder::create(&config).lock().borrow_mut().set_path(__a0) }.lock().borrow_mut()).set_json5(true).lock().borrow().build()),); ({ let (__a0,) = (YACLPlatform::rl("config"),); (config_class_handler::create_builder(&std::any::TypeId::of::<YACLConfig>()).lock().borrow_mut()).id(__a0) }.lock().borrow_mut()).serializer(__a0) }.lock().borrow()).build());

/// java: public class YACLConfig
#[derive(Clone)]
pub struct YACLConfig {
    /// java: @SerialEntry(comment = "Show the flashing colour picker hint (auto disables after first use)") public boolean showColorPickerIndicator = true;
    pub show_color_picker_indicator: bool,
    /// java: @SerialEntry(comment = "Load .webp and .gif during Minecraft resource reload instead of on-demand (can decrease startup time)") public boolean preloadComplexIma
    pub preload_complex_image_formats: bool,
}

impl YACLConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            show_color_picker_indicator: true,
            preload_complex_image_formats: false,
        };
        this
    }

}
