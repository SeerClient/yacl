//! Ported from dev.isxander.yacl3.api.utils.Dimension.
//!
//! A `Dimension<Integer>` reference is `Arc<dyn Dimension<i32>>` (a `MutableDimension` one coerces to it): the object
//! is shared by the widgets that hold it, and a `MutableDimension`'s setters change it for all of them, as in Java.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::mutable_dimension::MutableDimension;
use crate::dev::isxander::yacl3::r#impl::utils::dimension_integer_impl::DimensionIntegerImpl;

/// java: public interface Dimension<T extends Number>
pub trait Dimension<T: Copy + 'static>: compat::DynAny {
    /// java: T x();
    fn x(&self) -> T;

    /// java: T y();
    fn y(&self) -> T;

    /// java: T width();
    fn width(&self) -> T;

    /// java: T height();
    fn height(&self) -> T;

    /// java: T xLimit();
    fn x_limit(&self) -> T;

    /// java: T yLimit();
    fn y_limit(&self) -> T;

    /// java: T centerX();
    fn center_x(&self) -> T;

    /// java: T centerY();
    fn center_y(&self) -> T;

    /// java: boolean isPointInside(T x, T y);
    fn is_point_inside(&self, x: T, y: T) -> bool;

    /// java: MutableDimension<T> clone();
    fn clone(&self) -> Arc<dyn MutableDimension<T>>;

    /// java: Dimension<T> withX(T x);
    fn with_x(&self, x: T) -> Arc<dyn Dimension<T>>;

    /// java: Dimension<T> withY(T y);
    fn with_y(&self, y: T) -> Arc<dyn Dimension<T>>;

    /// java: Dimension<T> withWidth(T width);
    fn with_width(&self, width: T) -> Arc<dyn Dimension<T>>;

    /// java: Dimension<T> withHeight(T height);
    fn with_height(&self, height: T) -> Arc<dyn Dimension<T>>;

    /// java: Dimension<T> moved(T x, T y);
    fn moved(&self, x: T, y: T) -> Arc<dyn Dimension<T>>;

    /// java: Dimension<T> expanded(T width, T height);
    fn expanded(&self, width: T, height: T) -> Arc<dyn Dimension<T>>;
}

/// java: static MutableDimension<Integer> ofInt(int x, int y, int width, int height)
pub fn of_int(x: i32, y: i32, width: i32, height: i32) -> Arc<dyn MutableDimension<i32>> {
    Arc::new(DimensionIntegerImpl::new(x, y, width, height))
}
