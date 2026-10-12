//! Ported from dev.isxander.yacl3.api.controller.ColorControllerBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::color_controller_builder_impl::ColorControllerBuilderImpl;
use mcrust::java::awt::color::Color;

/// java: public interface ColorControllerBuilder extends ControllerBuilder<Color>
pub trait ColorControllerBuilder: ControllerBuilder + compat::DynAny {
    /// java: ColorControllerBuilder allowAlpha(boolean allowAlpha); (dispatch: the body for this class)
    fn allow_alpha(&self) -> fn(&compat::Shared<dyn ColorControllerBuilder>, bool) -> compat::Shared<dyn ColorControllerBuilder>;

}

/// java: static ColorControllerBuilder create(Option<Color> option)
pub fn create(option: &compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Color>>) -> compat::Shared<dyn ColorControllerBuilder> {
    return (ColorControllerBuilderImpl::new(option.clone()) as compat::Shared<dyn ColorControllerBuilder>);
}
