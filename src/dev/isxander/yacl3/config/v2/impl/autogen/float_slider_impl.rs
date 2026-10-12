//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.FloatSliderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::float_slider_controller_builder;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::float_slider::FloatSlider;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::ControllerBuilderRef;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::string_value_of;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::locale::language::LanguageObj as _;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class FloatSliderImpl extends SimpleOptionFactory<FloatSlider, Float>
#[derive(Clone, Default)]
pub struct FloatSliderImpl {}

impl FloatSliderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleOptionFactory<FloatSlider, f32> for FloatSliderImpl {
    /// java: @Override protected ControllerBuilder<Float> createController(FloatSlider annotation, ConfigField<Float> field, OptionAccess storage, Option<Float> option)
    fn create_controller(
        &self,
        annotation: &FloatSlider,
        field: &Arc<dyn ConfigField<f32>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn YaclOption<f32>>,
    ) -> ControllerBuilderRef<f32> {
        let (this, annotation_c, field_c) = (self.clone(), annotation.clone(), field.clone());
        ControllerBuilderRef::formattable(
            float_slider_controller_builder::create(option.clone())
                .format_value(Arc::new(move |v: f32| -> MutableComponent {
                    let mut key: Option<String> = None;
                    if v == annotation_c.min() {
                        key = Some(this.get_translation_key(&field_c, Some("fmt.min")));
                    } else if v == annotation_c.max() {
                        key = Some(this.get_translation_key(&field_c, Some("fmt.max")));
                    }
                    if let Some(key) = &key {
                        if language::get_instance().has(key) {
                            return component::translatable(key);
                        }
                    }
                    let key = this.get_translation_key(&field_c, Some("fmt"));
                    if language::get_instance().has(&key) {
                        return component::translatable_args(&key, vec![FormattedText::of(string_value_of(&v))]);
                    }
                    // java: Component.translatable(String.format(annotation.format(), v)) - translatable, as written
                    component::translatable(&compat::string_format(&annotation_c.format(), &[v.into()]))
                }))
                .range(annotation.min(), annotation.max())
                .step(annotation.step()),
        )
    }
}

impl OptionFactory<FloatSlider, f32> for FloatSliderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess) (SimpleOptionFactory)
    fn create_option(&self, annotation: &FloatSlider, field: &Arc<dyn ConfigField<f32>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<f32>> {
        simple_option_factory::create_option(self, annotation, field, option_access)
    }
}
