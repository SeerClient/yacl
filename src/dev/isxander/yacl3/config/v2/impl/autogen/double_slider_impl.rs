//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.DoubleSliderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::double_slider_controller_builder;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::double_slider::DoubleSlider;
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

/// java: public class DoubleSliderImpl extends SimpleOptionFactory<DoubleSlider, Double>
#[derive(Clone, Default)]
pub struct DoubleSliderImpl {}

impl DoubleSliderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleOptionFactory<DoubleSlider, f64> for DoubleSliderImpl {
    /// java: @Override protected ControllerBuilder<Double> createController(DoubleSlider annotation, ConfigField<Double> field, OptionAccess storage, Option<Double> option)
    fn create_controller(
        &self,
        annotation: &DoubleSlider,
        field: &Arc<dyn ConfigField<f64>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn YaclOption<f64>>,
    ) -> ControllerBuilderRef<f64> {
        let (this, annotation_c, field_c) = (self.clone(), annotation.clone(), field.clone());
        ControllerBuilderRef::formattable(
            double_slider_controller_builder::create(option.clone())
                .format_value(Arc::new(move |v: f64| -> MutableComponent {
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

impl OptionFactory<DoubleSlider, f64> for DoubleSliderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess) (SimpleOptionFactory)
    fn create_option(&self, annotation: &DoubleSlider, field: &Arc<dyn ConfigField<f64>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<f64>> {
        simple_option_factory::create_option(self, annotation, field, option_access)
    }
}
