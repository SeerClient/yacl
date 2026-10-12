//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.ColorFieldImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::color_controller_builder;
use crate::dev::isxander::yacl3::api::controller::color_controller_builder::ColorControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::option;
use crate::dev::isxander::yacl3::api::option_description::Builder;
use crate::dev::isxander::yacl3::config::v2::api::autogen::color_field::ColorField;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactoryBase;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::AutoGenUtils;
use crate::dev::isxander::yacl3::config::v2::r#impl::field_backed_binding::FieldBackedBinding;
use mcrust::java::awt::color::Color;
use crate::dev::isxander::yacl3::api::option::Builder as _;

/// java: public class ColorFieldImpl extends SimpleOptionFactory<ColorField, Color>
#[derive(Clone)]
pub struct ColorFieldImpl {
    /// java: extends SimpleOptionFactory
    pub base: SimpleOptionFactoryBase<ColorField, Color>,
}

impl std::ops::Deref for ColorFieldImpl {
    type Target = SimpleOptionFactoryBase<ColorField, Color>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColorFieldImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColorFieldImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: SimpleOptionFactoryBase::<ColorField, Color>::new(),
        };
        this
    }

}

impl SimpleOptionFactory<crate::dev::isxander::yacl3::config::v2::api::autogen::color_field::ColorField, mcrust::java::awt::color::Color> for ColorFieldImpl {
    fn simple_option_factory_base(&self) -> &SimpleOptionFactoryBase<crate::dev::isxander::yacl3::config::v2::api::autogen::color_field::ColorField, mcrust::java::awt::color::Color> {
        &self.base
    }
    fn simple_option_factory_base_mut(&mut self) -> &mut SimpleOptionFactoryBase<crate::dev::isxander::yacl3::config::v2::api::autogen::color_field::ColorField, mcrust::java::awt::color::Color> {
        &mut self.base
    }

    /// java: @Override protected ControllerBuilder<Color> createController(ColorField annotation, ConfigField<Color> field, OptionAccess storage, Option<Color> option)
    fn create_controller(&self, annotation: &ColorField, field: &compat::Shared<dyn ConfigField<Color>>, storage: &compat::Shared<dyn OptionAccess>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Color>>) -> compat::Shared<dyn ControllerBuilder<Color>> {
        return (({ let __r = &color_controller_builder::create(option); let __f = compat::dispatch(__r, |__t| __t.allow_alpha()); __f(__r, annotation.allow_alpha()) }) as compat::Shared<dyn ControllerBuilder<compat::Object>>);
    }

}

impl OptionFactory<compat::Object, compat::Object> for ColorFieldImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<T> createOption(A annotation, ConfigField<T> field, OptionAccess optionAccess)
    fn create_option(&self, annotation: &A, field: &compat::Shared<dyn ConfigField<T>>, option_access: &compat::Shared<dyn OptionAccess>) -> compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>> {
        // TODO(fabric2seer): `.pendingValue(..)` on a value of unknown type
        //   java: Option<T> option = Option.<T>createBuilder()
        //   java:                 .name(this.name(annotation, field, optionAccess))
        //   java:                 .description(v -> this.description(v, annotation, field, optionAccess).build())
        //   java:                 .binding(new FieldBackedBinding<>(field.access(), field.defaultAccess()))
        //   java:                 .controller(opt -> {
        //   java:                     ControllerBuilder<T> builder = this.createController(annotation, field, optionAccess, opt);
        //   java:
        //   java:                     AutoGenUtils.addCustomFormatterToController(builder, field.access());
        //   java:
        //   java:                     return builder;
        //   java:                 })
        //   java:                 .available(this.available(annotation, field, optionAccess))
        //   java:                 .flags(this.flags(annotation, field, optionAccess))
        //   java:                 .addListener((opt, event) -> this.listener(annotation, field, optionAccess, opt, opt.pendingValue()))
        //   java:                 .build();
        let mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>> = todo!("fabric2seer: the initialiser of `option` (TODO above)");
        self.post_init(annotation, field, option_access, &option);
        return option;
    }

}
