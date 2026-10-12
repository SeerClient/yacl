//! Ported from dev.isxander.yacl3.impl.controller.ColorControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::Controller;
use crate::dev::isxander::yacl3::api::controller::color_controller_builder::ColorControllerBuilder;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::dev::isxander::yacl3::gui::controllers::color_controller::ColorController;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImpl;
use crate::dev::isxander::yacl3::r#impl::controller::abstract_controller_builder_impl::AbstractControllerBuilderImplBase;
use mcrust::java::awt::color::Color;

/// java: public class ColorControllerBuilderImpl extends AbstractControllerBuilderImpl<Color> implements ColorControllerBuilder
#[derive(Clone)]
pub struct ColorControllerBuilderImpl {
    /// java: extends AbstractControllerBuilderImpl
    pub base: AbstractControllerBuilderImplBase<Color>,
    /// java: private boolean allowAlpha = false;
    pub allow_alpha: bool,
}

impl std::ops::Deref for ColorControllerBuilderImpl {
    type Target = AbstractControllerBuilderImplBase<Color>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColorControllerBuilderImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColorControllerBuilderImpl {
    /// java: public ColorControllerBuilderImpl(Option<Color> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<Color>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: AbstractControllerBuilderImplBase::<Color>::new(option.clone()),
            allow_alpha: false,
        });
        __this
    }

}

impl AbstractControllerBuilderImpl<mcrust::java::awt::color::Color> for ColorControllerBuilderImpl {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<mcrust::java::awt::color::Color> {
        &self.base
    }
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<mcrust::java::awt::color::Color> {
        &mut self.base
    }

}

impl ControllerBuilder<compat::Object> for ColorControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Controller<Color> build()
    fn build(&mut self) -> compat::Shared<dyn Controller<Color>> {
        return (compat::shared(ColorController::new_canonical(self.option.clone(), self.allow_alpha)) as compat::Shared<dyn Controller<compat::Object>>);
    }

}

impl ColorControllerBuilder for ColorControllerBuilderImpl {
    /// java: @Override public ColorControllerBuilder allowAlpha(boolean allowAlpha) (dispatch: the body for this class)
    fn allow_alpha(&self) -> fn(&compat::Shared<dyn ColorControllerBuilder>, bool) -> compat::Shared<dyn ColorControllerBuilder> {
        ColorControllerBuilderImpl::allow_alpha_dyn
    }

}

impl ColorControllerBuilderImpl {
    /// java: @Override public ColorControllerBuilder allowAlpha(boolean allowAlpha)
    pub fn allow_alpha_impl(__this: &compat::Shared<Self>, mut allow_alpha: bool) -> compat::Shared<dyn ColorControllerBuilder> {
        { let __v = allow_alpha; compat::write(__this, |__t| __t.allow_alpha = __v); }
        return (__this.clone() as compat::Shared<dyn ColorControllerBuilder>);
    }

    /// ColorControllerBuilderImpl.allowAlpha as the dispatch of ColorControllerBuilder.allowAlpha
    pub fn allow_alpha_dyn(__this: &compat::Shared<dyn ColorControllerBuilder>, __p0: bool) -> compat::Shared<dyn ColorControllerBuilder> {
        Self::allow_alpha_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
