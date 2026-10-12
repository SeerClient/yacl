//! Ported from dev.isxander.yacl3.impl.utils.DimensionIntegerImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::api::utils::dimension::Dimension;
use crate::dev::isxander::yacl3::api::utils::mutable_dimension::MutableDimension;
use std::cell::Cell;

/// java: public class DimensionIntegerImpl implements MutableDimension<Integer>
pub struct DimensionIntegerImpl {
    /// java: private int x, y;
    pub x: Cell<i32>,
    pub y: Cell<i32>,
    /// java: private int width, height;
    pub width: Cell<i32>,
    pub height: Cell<i32>,
}

impl DimensionIntegerImpl {
    /// java: public DimensionIntegerImpl(int x, int y, int width, int height)
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x: Cell::new(x), y: Cell::new(y), width: Cell::new(width), height: Cell::new(height) }
    }
}

impl Dimension<i32> for DimensionIntegerImpl {
    /// java: @Override public Integer x()
    fn x(&self) -> i32 {
        self.x.get()
    }

    /// java: @Override public Integer y()
    fn y(&self) -> i32 {
        self.y.get()
    }

    /// java: @Override public Integer width()
    fn width(&self) -> i32 {
        self.width.get()
    }

    /// java: @Override public Integer height()
    fn height(&self) -> i32 {
        self.height.get()
    }

    /// java: @Override public Integer xLimit()
    fn x_limit(&self) -> i32 {
        self.x.get().wrapping_add(self.width.get())
    }

    /// java: @Override public Integer yLimit()
    fn y_limit(&self) -> i32 {
        self.y.get().wrapping_add(self.height.get())
    }

    /// java: @Override public Integer centerX()
    fn center_x(&self) -> i32 {
        self.x.get().wrapping_add(self.width.get() / 2)
    }

    /// java: @Override public Integer centerY()
    fn center_y(&self) -> i32 {
        self.y.get().wrapping_add(self.height.get() / 2)
    }

    /// java: @Override public boolean isPointInside(Integer x, Integer y)
    fn is_point_inside(&self, x: i32, y: i32) -> bool {
        x >= self.x() && x <= self.x_limit() && y >= self.y() && y <= self.y_limit()
    }

    /// java: @Override public MutableDimension<Integer> clone()
    fn clone(&self) -> Arc<dyn MutableDimension<i32>> {
        Arc::new(DimensionIntegerImpl::new(self.x(), self.y(), self.width(), self.height()))
    }

    /// java: @Override public Dimension<Integer> withX(Integer x)
    fn with_x(&self, x: i32) -> Arc<dyn Dimension<i32>> {
        Arc::new(DimensionIntegerImpl::new(x, self.y(), self.width(), self.height()))
    }

    /// java: @Override public Dimension<Integer> withY(Integer y)
    fn with_y(&self, y: i32) -> Arc<dyn Dimension<i32>> {
        Arc::new(DimensionIntegerImpl::new(self.x(), y, self.width(), self.height()))
    }

    /// java: @Override public Dimension<Integer> withWidth(Integer width)
    fn with_width(&self, width: i32) -> Arc<dyn Dimension<i32>> {
        Arc::new(DimensionIntegerImpl::new(self.x(), self.y(), width, self.height()))
    }

    /// java: @Override public Dimension<Integer> withHeight(Integer height)
    fn with_height(&self, height: i32) -> Arc<dyn Dimension<i32>> {
        Arc::new(DimensionIntegerImpl::new(self.x(), self.y(), self.width(), height))
    }

    /// java: @Override public Dimension<Integer> moved(Integer x, Integer y)
    fn moved(&self, x: i32, y: i32) -> Arc<dyn Dimension<i32>> {
        Arc::new(DimensionIntegerImpl::new(self.x().wrapping_add(x), self.y().wrapping_add(y), self.width(), self.height()))
    }

    /// java: @Override public Dimension<Integer> expanded(Integer width, Integer height)
    fn expanded(&self, width: i32, height: i32) -> Arc<dyn Dimension<i32>> {
        Arc::new(DimensionIntegerImpl::new(self.x(), self.y(), self.width().wrapping_add(width), self.height().wrapping_add(height)))
    }
}

impl MutableDimension<i32> for DimensionIntegerImpl {
    /// java: @Override public MutableDimension<Integer> setX(Integer x)
    fn set_x(&self, x: i32) -> &dyn MutableDimension<i32> {
        self.x.set(x);
        self
    }

    /// java: @Override public MutableDimension<Integer> setY(Integer y)
    fn set_y(&self, y: i32) -> &dyn MutableDimension<i32> {
        self.y.set(y);
        self
    }

    /// java: @Override public MutableDimension<Integer> setWidth(Integer width)
    fn set_width(&self, width: i32) -> &dyn MutableDimension<i32> {
        self.width.set(width);
        self
    }

    /// java: @Override public MutableDimension<Integer> setHeight(Integer height)
    fn set_height(&self, height: i32) -> &dyn MutableDimension<i32> {
        self.height.set(height);
        self
    }

    /// java: @Override public MutableDimension<Integer> move(Integer x, Integer y)
    fn r#move(&self, x: i32, y: i32) -> &dyn MutableDimension<i32> {
        self.x.set(self.x.get().wrapping_add(x));
        self.y.set(self.y.get().wrapping_add(y));
        self
    }

    /// java: @Override public MutableDimension<Integer> expand(Integer width, Integer height)
    fn expand(&self, width: i32, height: i32) -> &dyn MutableDimension<i32> {
        self.width.set(self.width.get().wrapping_add(width));
        self.height.set(self.height.get().wrapping_add(height));
        self
    }
}
