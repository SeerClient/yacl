//! Ported from dev.isxander.yacl3.impl.controller.AbstractControllerBuilderImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;

/// java: public abstract class AbstractControllerBuilderImpl<T> implements ControllerBuilder<T>
#[derive(Clone)]
pub struct AbstractControllerBuilderImplBase<T> {
    /// java: protected final Option<T> option;
    pub option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>,
}

impl<T: Clone + 'static> AbstractControllerBuilderImplBase<T> {
    /// java: protected AbstractControllerBuilderImpl(Option<T> option)
    pub fn new(mut option: compat::Shared<dyn crate::dev::isxander::yacl3::api::option::Option<T>>) -> Self {
        let __init_option = option.clone();
        let mut this = Self {
            option: __init_option,
        };
        this
    }

}

/// java: the virtual methods of AbstractControllerBuilderImpl (implemented by AbstractControllerBuilderImplBase and every subclass)
pub trait AbstractControllerBuilderImpl<T: Clone + 'static>: ControllerBuilder {
    fn abstract_controller_builder_impl_base(&self) -> &AbstractControllerBuilderImplBase<T>;
    fn abstract_controller_builder_impl_base_mut(&mut self) -> &mut AbstractControllerBuilderImplBase<T>;

}
