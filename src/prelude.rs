//! Everything the generated modules use.
#![allow(unused_imports)]
pub use std::sync::{Arc, LazyLock, Mutex};
pub use std::rc::Rc;
pub use mcrust::mcrust::modding::hook::{self, At, CallbackInfo, InjectOptions, InjectorKind, Original, Shift};
pub use mcrust::mcrust::modding::events::{client_command_registration_callback, client_play_connection_events, client_tick_events, hud_render_callback, item_tooltip_callback, key_binding_helper, screen_events, screen_keyboard_events, screen_mouse_events, world_render_events};
pub use mcrust::mcrust::modding::script_host::{self, ScriptGraphics};
pub use mcrust::mcrust::modding::attach::{AttachmentTarget, AttachmentType};
pub use mcrust::mcrust::modding::networking;
pub use mcrust::mcrust::modding::log::Logger;
pub use mcrust::net::minecraft::client::key_mapping::{self, Category, KeyMapping};
pub use mcrust::net::minecraft::client::minecraft::Minecraft;
pub use mcrust::net::minecraft::client::gui::screens::screen::Screen;
pub use mcrust::net::minecraft::client::input::key_event::KeyEvent;
pub use mcrust::net::minecraft::client::multiplayer::client_packet_listener::ClientPacketListener;
pub use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
pub use mcrust::net::minecraft::world::item::item::TooltipContext;
pub use mcrust::net::minecraft::world::item::item_stack::ItemStack;
pub use mcrust::net::minecraft::world::item::tooltip_flag::TooltipFlag;
pub use crate::compat;
// the mod's traits (their methods resolve on any type that implements them)
pub use crate::dev::isxander::yacl3::api::binding::Binding as _;
pub use crate::dev::isxander::yacl3::api::button_option::ButtonOption as _;
pub use crate::dev::isxander::yacl3::api::button_option::Builder as _;
pub use crate::dev::isxander::yacl3::api::config_category::ConfigCategory as _;
pub use crate::dev::isxander::yacl3::api::config_category::Builder as _;
pub use crate::dev::isxander::yacl3::api::controller::Controller as _;
pub use crate::dev::isxander::yacl3::api::custom_tab_provider::CustomTabProvider as _;
pub use crate::dev::isxander::yacl3::api::label_option::LabelOption as _;
pub use crate::dev::isxander::yacl3::api::label_option::Builder as _;
pub use crate::dev::isxander::yacl3::api::list_option::ListOption as _;
pub use crate::dev::isxander::yacl3::api::list_option::Builder as _;
pub use crate::dev::isxander::yacl3::api::list_option_entry::ListOptionEntry as _;
pub use crate::dev::isxander::yacl3::api::nameable_enum::NameableEnum as _;
pub use crate::dev::isxander::yacl3::api::option::Option as _;
pub use crate::dev::isxander::yacl3::api::option::Builder as _;
pub use crate::dev::isxander::yacl3::api::option_addable::OptionAddable as _;
pub use crate::dev::isxander::yacl3::api::option_description::OptionDescription as _;
pub use crate::dev::isxander::yacl3::api::option_description::Builder as _;
pub use crate::dev::isxander::yacl3::api::option_event_listener::OptionEventListener as _;
pub use crate::dev::isxander::yacl3::api::option_flag::OptionFlag as _;
pub use crate::dev::isxander::yacl3::api::option_group::OptionGroup as _;
pub use crate::dev::isxander::yacl3::api::option_group::Builder as _;
pub use crate::dev::isxander::yacl3::api::placeholder_category::PlaceholderCategory as _;
pub use crate::dev::isxander::yacl3::api::placeholder_category::Builder as _;
pub use crate::dev::isxander::yacl3::api::state_manager::StateManager as _;
pub use crate::dev::isxander::yacl3::api::state_manager::StateListener as _;
pub use crate::dev::isxander::yacl3::api::yet_another_config_lib::YetAnotherConfigLib as _;
pub use crate::dev::isxander::yacl3::api::yet_another_config_lib::Builder as _;
pub use crate::dev::isxander::yacl3::api::yet_another_config_lib::ConfigBackedBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::boolean_controller_builder::BooleanControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::color_controller_builder::ColorControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::cycling_list_controller_builder::CyclingListControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::double_field_controller_builder::DoubleFieldControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::double_slider_controller_builder::DoubleSliderControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::dropdown_string_controller_builder::DropdownStringControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::enum_controller_builder::EnumControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::enum_dropdown_controller_builder::EnumDropdownControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::float_field_controller_builder::FloatFieldControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::float_slider_controller_builder::FloatSliderControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::integer_field_controller_builder::IntegerFieldControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::integer_slider_controller_builder::IntegerSliderControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::item_controller_builder::ItemControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::long_field_controller_builder::LongFieldControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::long_slider_controller_builder::LongSliderControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::string_controller_builder::StringControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::tick_box_controller_builder::TickBoxControllerBuilder as _;
pub use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController as _;
pub use crate::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter as _;
pub use crate::dev::isxander::yacl3::api::utils::dimension::Dimension as _;
pub use crate::dev::isxander::yacl3::api::utils::mutable_dimension::MutableDimension as _;
pub use crate::dev::isxander::yacl3::config::config_instance::ConfigInstance as _;
pub use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::ConfigClassHandler as _;
pub use crate::dev::isxander::yacl3::config::v2::api::config_class_handler::Builder as _;
pub use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField as _;
pub use crate::dev::isxander::yacl3::config::v2::api::config_serializer::ConfigSerializer as _;
pub use crate::dev::isxander::yacl3::config::v2::api::field_access::FieldAccess as _;
pub use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess as _;
pub use crate::dev::isxander::yacl3::config::v2::api::serial_field::SerialField as _;
pub use crate::dev::isxander::yacl3::config::v2::api::autogen::auto_gen_field::AutoGenField as _;
pub use crate::dev::isxander::yacl3::config::v2::api::autogen::custom_image::CustomImageFactory as _;
pub use crate::dev::isxander::yacl3::config::v2::api::autogen::enum_cycler::CyclableEnum as _;
pub use crate::dev::isxander::yacl3::config::v2::api::autogen::list_group::ControllerFactory as _;
pub use crate::dev::isxander::yacl3::config::v2::api::autogen::list_group::ValueFactory as _;
pub use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess as _;
pub use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory as _;
pub use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory as _;
pub use crate::dev::isxander::yacl3::config::v2::api::serializer::gson_config_serializer_builder::GsonConfigSerializerBuilder as _;
pub use crate::dev::isxander::yacl3::config::v3::abstract_config_entry::AbstractConfigEntry as _;
pub use crate::dev::isxander::yacl3::config::v3::abstract_readonly_config_entry::AbstractReadonlyConfigEntry as _;
pub use crate::dev::isxander::yacl3::config::v3::codec_config::CodecConfig as _;
pub use crate::dev::isxander::yacl3::config::v3::config_entry::ConfigEntry as _;
pub use crate::dev::isxander::yacl3::config::v3::entry_addable::EntryAddable as _;
pub use crate::dev::isxander::yacl3::config::v3::json_file_codec_config::JsonFileCodecConfig as _;
pub use crate::dev::isxander::yacl3::config::v3::readonly_config_entry::ReadonlyConfigEntry as _;
pub use crate::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget as _;
pub use crate::dev::isxander::yacl3::gui::option_list_widget::Entry as _;
pub use crate::dev::isxander::yacl3::gui::option_list_widget::GroupSeparatorEntry as _;
pub use crate::dev::isxander::yacl3::gui::tooltip_button_widget::TooltipButtonWidget as _;
pub use crate::dev::isxander::yacl3::gui::widget_and_type::WidgetAndType as _;
pub use crate::dev::isxander::yacl3::gui::yacl_selection_list::YACLSelectionList as _;
pub use crate::dev::isxander::yacl3::gui::yacl_selection_list::Entry as _;
pub use crate::dev::isxander::yacl3::gui::controllers::controller_popup_widget::ControllerPopupWidget as _;
pub use crate::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget as _;
pub use crate::dev::isxander::yacl3::gui::controllers::cycling::cycling_list_controller::CyclingListController as _;
pub use crate::dev::isxander::yacl3::gui::controllers::cycling::i_cycling_controller::ICyclingController as _;
pub use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller::AbstractDropdownController as _;
pub use crate::dev::isxander::yacl3::gui::controllers::dropdown::abstract_dropdown_controller_element::AbstractDropdownControllerElement as _;
pub use crate::dev::isxander::yacl3::gui::controllers::slider::i_slider_controller::ISliderController as _;
pub use crate::dev::isxander::yacl3::gui::controllers::string::i_string_controller::IStringController as _;
pub use crate::dev::isxander::yacl3::gui::controllers::string::string_controller_element::StringControllerElement as _;
pub use crate::dev::isxander::yacl3::gui::controllers::string::number::number_field_controller::NumberFieldController as _;
pub use crate::dev::isxander::yacl3::gui::image::image_renderer::ImageRenderer as _;
pub use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::ImageRendererFactory as _;
pub use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::ImageSupplier as _;
pub use crate::dev::isxander::yacl3::gui::image::image_renderer_factory::OnThread as _;
pub use crate::dev::isxander::yacl3::gui::image::r#impl::animated_dynamic_texture_image::AnimFrameProvider as _;
pub use crate::dev::isxander::yacl3::gui::image::r#impl::dynamic_texture_image::DynamicTextureImage as _;
pub use crate::dev::isxander::yacl3::gui::render::gui_render_state_sink::GuiRenderStateSink as _;
pub use crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState as _;
pub use crate::dev::isxander::yacl3::gui::tab::tab_ext::TabExt as _;
pub use crate::dev::isxander::yacl3::r#impl::option_impl::OptionImpl as _;
pub use crate::dev::isxander::yacl3::r#impl::provides_binding_for_deprecation::ProvidesBindingForDeprecation as _;
pub use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl as _;
pub use crate::dev::isxander::yacl3::r#impl::controller::string_controller_builder_impl::StringControllerBuilderImpl as _;

pub const MOD_ID: &str = "yet_another_config_lib_v3";

pub fn logger() -> Logger {
    Logger::new(MOD_ID)
}

/// Logs a failed hook registration once (Mixin would refuse to apply).
pub fn report<T>(r: Result<T, mcrust::mcrust::modding::hook::HookError>) {
    if let Err(e) = r {
        logger().warn(&format!("hook not registered: {e:?}"));
    }
}
