//! Ported from dev.isxander.yacl3.gui.render.YACLGuiElementRenderState.
//!
//! `interface YACLGuiElementRenderState extends GuiElementRenderState`: its defaults are this trait's; the port's
//! `GuiElementRenderState` (and its `ScreenArea` supertrait) is implemented for each implementor by
//! `impl_gui_element_render_state!`, since a blanket impl of a foreign trait over a local one is not allowed.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::render::base_render_state::BaseRenderState;
use crate::dev::isxander::yacl3::gui::render::gui_render_state_sink;
use mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline;
use mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
use mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup;
use mcrust::net::minecraft::client::renderer::state::gui::gui_element_render_state::GuiElementRenderState;
use std::sync::Arc;

/// java: public interface YACLGuiElementRenderState extends GuiElementRenderState
pub trait YACLGuiElementRenderState: 'static {
    /// java: BaseRenderState baseState();
    fn base_state(&self) -> &BaseRenderState;

    /// java: @Override default void buildVertices(VertexConsumer vertexConsumer)
    fn build_vertices(&self, vertex_consumer: &mut dyn VertexConsumer) {
        self.build_vertices_2(vertex_consumer, 0.0);
    }

    /// java: void buildVertices(VertexConsumer vertexConsumer, float z);
    fn build_vertices_2(&self, vertex_consumer: &mut dyn VertexConsumer, z: f32);

    /// java: @Override default @NotNull RenderPipeline pipeline()
    fn pipeline(&self) -> Arc<RenderPipeline> {
        self.base_state().pipeline()
    }

    /// java: @Override default @NotNull TextureSetup textureSetup()
    fn texture_setup(&self) -> &TextureSetup {
        self.base_state().texture_setup()
    }

    /// java: @Override default @Nullable ScreenRectangle scissorArea()
    fn scissor_area(&self) -> Option<ScreenRectangle> {
        self.base_state().scissor_area()
    }

    /// java: @Override default @Nullable ScreenRectangle bounds()
    fn bounds(&self) -> Option<ScreenRectangle> {
        self.base_state().bounds()
    }

    /// java: default VertexConsumer add2DVertex(VertexConsumer vertexConsumer, float x, float y, float z)
    ///
    /// Answers the consumer for the chained `.setColor(..)`, as Java's does.
    fn add_2d_vertex<'v>(
        &self,
        vertex_consumer: &'v mut dyn VertexConsumer,
        x: f32,
        y: f32,
        _z: f32,
    ) -> &'v mut dyn VertexConsumer {
        vertex_consumer.add_vertex_with_2d_pose(&self.base_state().pose(), x, y);
        vertex_consumer
    }

    /// java: default void submit(GuiGraphicsExtractor graphics)
    fn submit(self, graphics: &GuiGraphicsExtractor)
    where
        Self: Sized + GuiElementRenderState,
    {
        gui_render_state_sink::submit(graphics, Box::new(self));
    }
}

/// `implements YACLGuiElementRenderState` also implements the port's `GuiElementRenderState` with the interface's
/// default bodies (`ScreenArea.bounds`, `buildVertices`, `pipeline`, `textureSetup`, `scissorArea`).
#[macro_export]
macro_rules! impl_gui_element_render_state {
    ($ty:ty) => {
        impl mcrust::net::minecraft::client::renderer::state::gui::screen_area::ScreenArea for $ty {
            fn bounds(&self) -> Option<mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle> {
                <$ty as $crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState>::bounds(self)
            }
        }
        impl mcrust::net::minecraft::client::renderer::state::gui::gui_element_render_state::GuiElementRenderState for $ty {
            fn build_vertices(&self, vertex_consumer: &mut dyn mcrust::com::mojang::blaze3d::vertex::vertex_consumer::VertexConsumer) {
                <$ty as $crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState>::build_vertices(
                    self,
                    vertex_consumer,
                )
            }
            fn pipeline(&self) -> std::sync::Arc<mcrust::com::mojang::blaze3d::pipeline::render_pipeline::RenderPipeline> {
                <$ty as $crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState>::pipeline(self)
            }
            fn texture_setup(&self) -> &mcrust::net::minecraft::client::gui::render::texture_setup::TextureSetup {
                <$ty as $crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState>::texture_setup(
                    self,
                )
            }
            fn scissor_area(&self) -> Option<mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle> {
                <$ty as $crate::dev::isxander::yacl3::gui::render::yacl_gui_element_render_state::YACLGuiElementRenderState>::scissor_area(
                    self,
                )
            }
        }
    };
}
