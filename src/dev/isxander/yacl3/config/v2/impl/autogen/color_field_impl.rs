//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.ColorFieldImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::color_controller_builder;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::color_field::ColorField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::ControllerBuilderRef;
use mcrust::java::awt::color::Color;

/// java: public class ColorFieldImpl extends SimpleOptionFactory<ColorField, Color>
#[derive(Clone, Default)]
pub struct ColorFieldImpl {}

impl ColorFieldImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleOptionFactory<ColorField, Color> for ColorFieldImpl {
    /// java: @Override protected ControllerBuilder<Color> createController(ColorField annotation, ConfigField<Color> field, OptionAccess storage, Option<Color> option)
    fn create_controller(
        &self,
        annotation: &ColorField,
        field: &Arc<dyn ConfigField<Color>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn YaclOption<Color>>,
    ) -> ControllerBuilderRef<Color> {
        ControllerBuilderRef::plain(color_controller_builder::create(option.clone()).allow_alpha(annotation.allow_alpha()))
    }
}

impl OptionFactory<ColorField, Color> for ColorFieldImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess) (SimpleOptionFactory)
    fn create_option(&self, annotation: &ColorField, field: &Arc<dyn ConfigField<Color>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<Color>> {
        simple_option_factory::create_option(self, annotation, field, option_access)
    }
}
