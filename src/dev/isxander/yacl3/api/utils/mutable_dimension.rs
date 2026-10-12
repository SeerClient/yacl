//! Ported from dev.isxander.yacl3.api.utils.MutableDimension by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;

/// java: public interface MutableDimension<T extends Number> extends Dimension<T>
pub trait MutableDimension<T: Clone + 'static>: Dimension {
    /// java: MutableDimension<T> setX(T x);
    fn set_x(&mut self, x: &T) -> compat::Shared<dyn MutableDimension<T>>;

    /// java: MutableDimension<T> setY(T y);
    fn set_y(&mut self, y: &T) -> compat::Shared<dyn MutableDimension<T>>;

    /// java: MutableDimension<T> setWidth(T width);
    fn set_width(&mut self, width: &T) -> compat::Shared<dyn MutableDimension<T>>;

    /// java: MutableDimension<T> setHeight(T height);
    fn set_height(&mut self, height: &T) -> compat::Shared<dyn MutableDimension<T>>;

    /// java: MutableDimension<T> move(T x, T y);
    fn r#move(&mut self, x: &T, y: &T) -> compat::Shared<dyn MutableDimension<T>>;

    /// java: MutableDimension<T> expand(T width, T height);
    fn expand(&mut self, width: &T, height: &T) -> compat::Shared<dyn MutableDimension<T>>;

}
