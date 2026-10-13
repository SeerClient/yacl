//! Ported from dev.isxander.yacl3.gui.image.ImageRenderer.

#[allow(unused_imports)]
use crate::prelude::*;
use compat::Graphics;

/// java: public interface ImageRenderer
///
/// Held as `Arc<dyn ImageRenderer>` by the image cache and the option descriptions; its state is in cells.
pub trait ImageRenderer {
    /// java: int render(GuiGraphicsExtractor graphics, int x, int y, int renderWidth, float tickDelta);
    fn render(&self, graphics: &Graphics<'_>, x: i32, y: i32, render_width: i32, tick_delta: f32) -> i32;

    /// java: void close();
    fn close(&self);

    /// java: default void tick()
    fn tick(&self) {}
}
