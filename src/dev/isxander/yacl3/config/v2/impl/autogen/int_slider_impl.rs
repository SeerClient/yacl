//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.IntSliderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::integer_slider_controller_builder;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::int_slider::IntSlider;
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

/// java: public class IntSliderImpl extends SimpleOptionFactory<IntSlider, Integer>
#[derive(Clone, Default)]
pub struct IntSliderImpl {}

impl IntSliderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleOptionFactory<IntSlider, i32> for IntSliderImpl {
    /// java: @Override protected ControllerBuilder<Integer> createController(IntSlider annotation, ConfigField<Integer> field, OptionAccess storage, Option<Integer> option)
    fn create_controller(
        &self,
        annotation: &IntSlider,
        field: &Arc<dyn ConfigField<i32>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn YaclOption<i32>>,
    ) -> ControllerBuilderRef<i32> {
        let (this, annotation_c, field_c) = (self.clone(), annotation.clone(), field.clone());
        ControllerBuilderRef::formattable(
            integer_slider_controller_builder::create(option.clone())
                .format_value(Arc::new(move |v: i32| -> MutableComponent {
                    let mut key: String = this.get_translation_key(&field_c, Some(format!("fmt.{}", string_value_of(&v)).as_str()));
                    if language::get_instance().has(&key) {
                        return component::translatable(&key);
                    }
                    key = this.get_translation_key(&field_c, Some("fmt"));
                    if language::get_instance().has(&key) {
                        return component::translatable_args(&key, vec![FormattedText::of(string_value_of(&v))]);
                    }
                    component::literal(&compat::string_format(&annotation_c.format(), &[v.into()]))
                }))
                .range(annotation.min(), annotation.max())
            .step(annotation.step()),
        )
    }
}

impl OptionFactory<IntSlider, i32> for IntSliderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess) (SimpleOptionFactory)
    fn create_option(&self, annotation: &IntSlider, field: &Arc<dyn ConfigField<i32>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<i32>> {
        simple_option_factory::create_option(self, annotation, field, option_access)
    }
}
