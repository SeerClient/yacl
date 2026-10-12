//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.BooleanImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::boolean_controller_builder;
use crate::dev::isxander::yacl3::api::option::Option as YaclOption;
use crate::dev::isxander::yacl3::config::v2::api::autogen::boolean::Boolean;
use crate::dev::isxander::yacl3::config::v2::api::autogen::boolean::Formatter;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::ControllerBuilderRef;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class BooleanImpl extends SimpleOptionFactory<Boolean, java.lang.Boolean>
#[derive(Clone, Default)]
pub struct BooleanImpl {}

impl BooleanImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl SimpleOptionFactory<Boolean, bool> for BooleanImpl {
    /// java: @Override protected ControllerBuilder<java.lang.Boolean> createController(Boolean annotation, ConfigField<java.lang.Boolean> field, OptionAccess storage, Option<java.lang.Boolean> option)
    fn create_controller(
        &self,
        annotation: &Boolean,
        field: &Arc<dyn ConfigField<bool>>,
        storage: &Arc<dyn OptionAccess>,
        option: &Arc<dyn YaclOption<bool>>,
    ) -> ControllerBuilderRef<bool> {
        let mut builder = boolean_controller_builder::create(option.clone()).coloured(annotation.colored());

        builder = match annotation.formatter() {
            Formatter::OnOff => builder.on_off_formatter(),
            Formatter::YesNo => builder.yes_no_formatter(),
            Formatter::TrueFalse => builder.true_false_formatter(),
            Formatter::Custom => {
                let (this, field_c) = (self.clone(), field.clone());
                builder.format_value(Arc::new(move |v: bool| -> MutableComponent {
                    component::translatable(&this.get_translation_key(&field_c, Some(format!("fmt.{}", v).as_str())))
                }))
            }
        };

        ControllerBuilderRef::formattable(builder)
    }
}

impl OptionFactory<Boolean, bool> for BooleanImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess) (SimpleOptionFactory)
    fn create_option(&self, annotation: &Boolean, field: &Arc<dyn ConfigField<bool>>, option_access: &Arc<dyn OptionAccess>) -> Arc<dyn YaclOption<bool>> {
        simple_option_factory::create_option(self, annotation, field, option_access)
    }
}
