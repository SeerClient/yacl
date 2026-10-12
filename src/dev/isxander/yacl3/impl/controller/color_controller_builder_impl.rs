//! Ported from dev.isxander.yacl3.impl.controller.ColorControllerBuilderImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::option::Option as YOption;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::color_controller_builder::ColorControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::color_controller::ColorController;
use mcrust::java::awt::color::Color;

/// java: public class ColorControllerBuilderImpl extends AbstractControllerBuilderImpl<Color> implements ColorControllerBuilder
pub struct ColorControllerBuilderImpl {
    /// java: protected final Option<T> option; (AbstractControllerBuilderImpl)
    pub option: Arc<dyn YOption<Color>>,
    /// java: private boolean allowAlpha = false;
    pub allow_alpha: bool,
}

impl ColorControllerBuilderImpl {
    /// java: public ColorControllerBuilderImpl(Option<Color> option)
    pub fn new(option: Arc<dyn YOption<Color>>) -> Self {
        Self { option, allow_alpha: false }
    }
}

impl ColorControllerBuilder for ColorControllerBuilderImpl {
    /// java: @Override public ColorControllerBuilder allowAlpha(boolean allowAlpha)
    fn allow_alpha(mut self, allow_alpha: bool) -> Self {
        self.allow_alpha = allow_alpha;
        self
    }
}

impl ControllerBuilder<Color> for ColorControllerBuilderImpl {
    /// java: @Override public Controller<Color> build()
    fn build(&self) -> Arc<dyn Controller<Color>> {
        ColorController::new_1(self.option.clone(), self.allow_alpha)
    }
}
