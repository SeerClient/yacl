//! Ported from dev.isxander.yacl3.api.controller.ColorControllerBuilder.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::r#impl::controller::color_controller_builder_impl::ColorControllerBuilderImpl;
use mcrust::java::awt::color::Color;

/// java: public interface ColorControllerBuilder extends ControllerBuilder<Color>
pub trait ColorControllerBuilder: ControllerBuilder<Color> + Sized {
    /// java: ColorControllerBuilder allowAlpha(boolean allowAlpha);
    fn allow_alpha(self, allow_alpha: bool) -> Self;
}

/// java: static ColorControllerBuilder create(Option<Color> option)
pub fn create(option: Arc<dyn YOption<Color>>) -> ColorControllerBuilderImpl {
    ColorControllerBuilderImpl::new(option)
}
