//! Ported from dev.isxander.yacl3.impl.utils.DimensionIntegerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::api::utils::mutable_dimension::MutableDimension;

/// java: public class DimensionIntegerImpl implements MutableDimension<Integer>
#[derive(Clone)]
pub struct DimensionIntegerImpl {
    /// java: private int x, y;
    pub x: i32,
    /// java: private int x, y;
    pub y: i32,
    /// java: private int width, height;
    pub width: i32,
    /// java: private int width, height;
    pub height: i32,
}

impl DimensionIntegerImpl {
    /// java: public DimensionIntegerImpl(int x, int y, int width, int height)
    pub fn new(mut x: i32, mut y: i32, mut width: i32, mut height: i32) -> Self {
        let __init_x = x;
        let __init_y = y;
        let __init_width = width;
        let __init_height = height;
        let mut this = Self {
            x: __init_x,
            y: __init_y,
            width: __init_width,
            height: __init_height,
        };
        this
    }

}

impl MutableDimension<i32> for DimensionIntegerImpl {
    /// java: @Override public MutableDimension<Integer> setX(Integer x)
    fn set_x(&mut self, x: &i32) -> compat::Shared<dyn MutableDimension<i32>> {
        self.x = *x;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public MutableDimension<Integer> setY(Integer y)
    fn set_y(&mut self, y: &i32) -> compat::Shared<dyn MutableDimension<i32>> {
        self.y = *y;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public MutableDimension<Integer> setWidth(Integer width)
    fn set_width(&mut self, width: &i32) -> compat::Shared<dyn MutableDimension<i32>> {
        self.width = *width;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public MutableDimension<Integer> setHeight(Integer height)
    fn set_height(&mut self, height: &i32) -> compat::Shared<dyn MutableDimension<i32>> {
        self.height = *height;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public MutableDimension<Integer> move(Integer x, Integer y)
    fn r#move(&mut self, x: &i32, y: &i32) -> compat::Shared<dyn MutableDimension<i32>> {
        self.x = self.x.wrapping_add(*x);
        self.y = self.y.wrapping_add(*y);
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public MutableDimension<Integer> expand(Integer width, Integer height)
    fn expand(&mut self, width: &i32, height: &i32) -> compat::Shared<dyn MutableDimension<i32>> {
        self.width = self.width.wrapping_add(*width);
        self.height = self.height.wrapping_add(*height);
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl Dimension<compat::Object> for DimensionIntegerImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Integer x()
    fn x(&self) -> i32 {
        return self.x;
    }

    /// java: @Override public Integer y()
    fn y(&self) -> i32 {
        return self.y;
    }

    /// java: @Override public Integer width()
    fn width(&self) -> i32 {
        return self.width;
    }

    /// java: @Override public Integer height()
    fn height(&self) -> i32 {
        return self.height;
    }

    /// java: @Override public Integer xLimit()
    fn x_limit(&self) -> i32 {
        return self.x.wrapping_add(self.width);
    }

    /// java: @Override public Integer yLimit()
    fn y_limit(&self) -> i32 {
        return self.y.wrapping_add(self.height);
    }

    /// java: @Override public Integer centerX()
    fn center_x(&self) -> i32 {
        return self.x.wrapping_add(self.width.wrapping_div(2));
    }

    /// java: @Override public Integer centerY()
    fn center_y(&self) -> i32 {
        return self.y.wrapping_add(self.height.wrapping_div(2));
    }

    /// java: @Override public boolean isPointInside(Integer x, Integer y)
    fn is_point_inside(&self, x: &i32, y: &i32) -> bool {
        return (*x >= self.x() && *x <= self.x_limit() && *y >= self.y() && *y <= self.y_limit());
    }

    /// java: @Override public MutableDimension<Integer> clone()
    fn clone(&self) -> compat::Shared<dyn MutableDimension<i32>> {
        return (compat::shared(DimensionIntegerImpl::new(self.x, self.y, self.width, self.height)) as compat::Shared<dyn MutableDimension<compat::Object>>);
    }

    /// java: @Override public Dimension<Integer> withX(Integer x)
    fn with_x(&self, x: &i32) -> compat::Shared<dyn Dimension<i32>> {
        return (self.clone().lock().borrow_mut().set_x(x) as compat::Shared<dyn Dimension<compat::Object>>);
    }

    /// java: @Override public Dimension<Integer> withY(Integer y)
    fn with_y(&self, y: &i32) -> compat::Shared<dyn Dimension<i32>> {
        return (self.clone().lock().borrow_mut().set_y(y) as compat::Shared<dyn Dimension<compat::Object>>);
    }

    /// java: @Override public Dimension<Integer> withWidth(Integer width)
    fn with_width(&self, width: &i32) -> compat::Shared<dyn Dimension<i32>> {
        return (self.clone().lock().borrow_mut().set_width(width) as compat::Shared<dyn Dimension<compat::Object>>);
    }

    /// java: @Override public Dimension<Integer> withHeight(Integer height)
    fn with_height(&self, height: &i32) -> compat::Shared<dyn Dimension<i32>> {
        return (self.clone().lock().borrow_mut().set_height(height) as compat::Shared<dyn Dimension<compat::Object>>);
    }

    /// java: @Override public Dimension<Integer> moved(Integer x, Integer y)
    fn moved(&self, x: &i32, y: &i32) -> compat::Shared<dyn Dimension<i32>> {
        return (self.clone().lock().borrow_mut().r#move(x, y) as compat::Shared<dyn Dimension<compat::Object>>);
    }

    /// java: @Override public Dimension<Integer> expanded(Integer width, Integer height)
    fn expanded(&self, width: &i32, height: &i32) -> compat::Shared<dyn Dimension<i32>> {
        return (self.clone().lock().borrow_mut().expand(width, height) as compat::Shared<dyn Dimension<compat::Object>>);
    }

}
