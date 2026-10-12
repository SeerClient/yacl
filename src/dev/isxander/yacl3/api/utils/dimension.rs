//! Ported from dev.isxander.yacl3.api.utils.Dimension by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::mutable_dimension::MutableDimension;
use crate::dev::isxander::yacl3::r#impl::utils::dimension_integer_impl::DimensionIntegerImpl;

/// java: public interface Dimension<T extends Number>
pub trait Dimension<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
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
    fn is_point_inside(&self, x: &T, y: &T) -> bool;

    /// java: MutableDimension<T> clone();
    fn clone(&self) -> compat::Shared<dyn MutableDimension<T>>;

    /// java: Dimension<T> withX(T x);
    fn with_x(&self, x: &T) -> compat::Shared<dyn Dimension<T>>;

    /// java: Dimension<T> withY(T y);
    fn with_y(&self, y: &T) -> compat::Shared<dyn Dimension<T>>;

    /// java: Dimension<T> withWidth(T width);
    fn with_width(&self, width: &T) -> compat::Shared<dyn Dimension<T>>;

    /// java: Dimension<T> withHeight(T height);
    fn with_height(&self, height: &T) -> compat::Shared<dyn Dimension<T>>;

    /// java: Dimension<T> moved(T x, T y);
    fn moved(&self, x: &T, y: &T) -> compat::Shared<dyn Dimension<T>>;

    /// java: Dimension<T> expanded(T width, T height);
    fn expanded(&self, width: &T, height: &T) -> compat::Shared<dyn Dimension<T>>;

}

/// java: static MutableDimension<Integer> ofInt(int x, int y, int width, int height)
pub fn of_int(mut x: i32, mut y: i32, mut width: i32, mut height: i32) -> compat::Shared<dyn MutableDimension<i32>> {
    return (compat::shared(DimensionIntegerImpl::new(x, y, width, height)) as compat::Shared<dyn MutableDimension<compat::Object>>);
}
