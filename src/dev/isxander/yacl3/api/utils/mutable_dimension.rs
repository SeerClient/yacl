//! Ported from dev.isxander.yacl3.api.utils.MutableDimension.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;

/// java: public interface MutableDimension<T extends Number> extends Dimension<T>
///
/// The setters change this object and answer it (Java's `return this`), so `dim.expand(..).move(..)` chains.
pub trait MutableDimension<T: Copy + 'static>: Dimension<T> {
    /// java: MutableDimension<T> setX(T x);
    fn set_x(&self, x: T) -> &dyn MutableDimension<T>;

    /// java: MutableDimension<T> setY(T y);
    fn set_y(&self, y: T) -> &dyn MutableDimension<T>;

    /// java: MutableDimension<T> setWidth(T width);
    fn set_width(&self, width: T) -> &dyn MutableDimension<T>;

    /// java: MutableDimension<T> setHeight(T height);
    fn set_height(&self, height: T) -> &dyn MutableDimension<T>;

    /// java: MutableDimension<T> move(T x, T y);
    fn r#move(&self, x: T, y: T) -> &dyn MutableDimension<T>;

    /// java: MutableDimension<T> expand(T width, T height);
    fn expand(&self, width: T, height: T) -> &dyn MutableDimension<T>;
}
