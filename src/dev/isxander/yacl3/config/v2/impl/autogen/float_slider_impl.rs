//! Ported from dev.isxander.yacl3.config.v2.impl.autogen.FloatSliderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::binding::Binding;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::float_slider_controller_builder;
use crate::dev::isxander::yacl3::api::option;
use crate::dev::isxander::yacl3::api::option_description::Builder;
use crate::dev::isxander::yacl3::config::v2::api::autogen::float_slider::FloatSlider;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_access::OptionAccess;
use crate::dev::isxander::yacl3::config::v2::api::autogen::option_factory::OptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactory;
use crate::dev::isxander::yacl3::config::v2::api::autogen::simple_option_factory::SimpleOptionFactoryBase;
use crate::dev::isxander::yacl3::config::v2::api::config_field::ConfigField;
use crate::dev::isxander::yacl3::config::v2::api::read_only_field_access::ReadOnlyFieldAccess;
use crate::dev::isxander::yacl3::config::v2::r#impl::autogen::auto_gen_utils::AutoGenUtils;
use crate::dev::isxander::yacl3::config::v2::r#impl::field_backed_binding::FieldBackedBinding;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::network::chat::component;
use crate::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController as _;
use crate::dev::isxander::yacl3::api::option::Builder as _;

/// java: public class FloatSliderImpl extends SimpleOptionFactory<FloatSlider, Float>
#[derive(Clone)]
pub struct FloatSliderImpl {
    /// java: extends SimpleOptionFactory
    pub base: SimpleOptionFactoryBase<FloatSlider, f32>,
}

impl std::ops::Deref for FloatSliderImpl {
    type Target = SimpleOptionFactoryBase<FloatSlider, f32>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for FloatSliderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl FloatSliderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            base: SimpleOptionFactoryBase::<FloatSlider, f32>::new(),
        };
        this
    }

}

impl SimpleOptionFactory<crate::dev::isxander::yacl3::config::v2::api::autogen::float_slider::FloatSlider, f32> for FloatSliderImpl {
    fn simple_option_factory_base(&self) -> &SimpleOptionFactoryBase<crate::dev::isxander::yacl3::config::v2::api::autogen::float_slider::FloatSlider, f32> {
        &self.base
    }
    fn simple_option_factory_base_mut(&mut self) -> &mut SimpleOptionFactoryBase<crate::dev::isxander::yacl3::config::v2::api::autogen::float_slider::FloatSlider, f32> {
        &mut self.base
    }

    /// java: @Override protected ControllerBuilder<Float> createController(FloatSlider annotation, ConfigField<Float> field, OptionAccess storage, Option<Float> option)
    fn create_controller(&self, annotation: &FloatSlider, field: &compat::Shared<dyn ConfigField<f32>>, storage: &compat::Shared<dyn OptionAccess>, option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<f32>>) -> compat::Shared<dyn ControllerBuilder<f32>> {
        // TODO(fabric2seer): `.range(..)` on a B
        //   java: return FloatSliderControllerBuilder.create(option)
        //   java:                 .formatValue(v -> {
        //   java:                     String key = null;
        //   java:                     if (v == annotation.min())
        //   java:                         key = getTranslationKey(field, "fmt.min");
        //   java:                     else if (v == annotation.max())
        //   java:                         key = getTranslationKey(field, "fmt.max");
        //   java:                     if (key != null && Language.getInstance().has(key))
        //   java:                         return Component.translatable(key);
        //   java:                     key = getTranslationKey(field, "fmt");
        //   java:                     if (Language.getInstance().has(key))
        //   java:                         return Component.translatable(key, v);
        //   java:                     return Component.translatable(String.format(annotation.format(), v));
        //   java:                 })
        //   java:                 .range(annotation.min(), annotation.max())
        //   java:                 .step(annotation.step());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl OptionFactory<compat::Object, compat::Object> for FloatSliderImpl {
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
