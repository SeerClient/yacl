//! Ported from dev.isxander.yacl3.gui.SearchFieldWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::net::minecraft::client::gui::components::edit_box;
use mcrust::net::minecraft::client::gui::components::edit_box::EditBox;
use mcrust::net::minecraft::client::gui::components::edit_box::Responder;
use mcrust::net::minecraft::client::gui::components::edit_box::ResponderLambda;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class SearchFieldWidget extends EditBox
pub struct SearchFieldWidget {
    /// java: extends net.minecraft.client.gui.components.EditBox
    pub base: EditBox,
    /// java: private Component emptyText;
    pub empty_text: Option<MutableComponent>,
    /// java: private final YACLScreen yaclScreen;
    pub yacl_screen: Option<compat::Shared<YACLScreen>>,
    /// java: private final Font font;
    pub font: Option<Font>,
    /// java: private final Consumer<String> updateConsumer;
    pub update_consumer: Option<Arc<dyn Fn(String)>>,
    /// java: private boolean isEmpty = true;
    pub is_empty: bool,
    /// java: private boolean doNotUpdate = false;
    pub do_not_update: bool,
}

impl std::ops::Deref for SearchFieldWidget {
    type Target = EditBox;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SearchFieldWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SearchFieldWidget {
    /// java: public SearchFieldWidget(YACLScreen yaclScreen, Font font, int x, int y, int width, int height, Component text, Component emptyText, Consumer<String> updateConsumer)
    pub fn new(mut yacl_screen: compat::Shared<YACLScreen>, mut font: Font, mut x: i32, mut y: i32, mut width: i32, mut height: i32, mut text: MutableComponent, mut empty_text: MutableComponent, update_consumer: Arc<dyn Fn(String)>) -> Self {
        // TODO(fabric2seer): `new` takes 5 argument(s) in the port (`splitter: &StringSplitter<W>, x: i32, y: i32, width: i32, height: i32`), Java passes 6
        //   java: super(font, x, y, width, height, text);
        let mut this: Self = todo!("constructor");
        this.set_responder(Responder::Lambda(ResponderLambda::new(move |__a0| { this.update(__a0); })));
        this.yacl_screen = Some(yacl_screen.clone());
        this.font = Some(compat::font());
        this.empty_text = Some(empty_text.clone());
        this.update_consumer = Some(update_consumer.clone());
        this
    }

    /// java: @Override public void extractWidgetRenderState(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_widget_render_state(&self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        // TODO(fabric2seer): `extract_widget_render_state` takes 7 argument(s) in the port (`gl: &crate::lwjgl::opengl::Gl, gfx: &crate::net::minecraft::client::minecraft::Minecraft, w: &super::abstract_widget::AbstractWidget, e: &EditBox, millis: i64, color: i32, apply_format: Option<&dyn Fn(&str, i32) -> Option<crate::net::minecr ..`), Java passes 4
        // rust: edit_box::extract_widget_render_state(graphics, &mouse_x, &mouse_y, &a);
        // TODO(fabric2seer): `EditBox.isVisible` has no Rust method in the port (looked for `is_visible` in EditBox and its supertypes)
        if self.is_visible() && self.is_empty() {
            gui_graphics_extractor::draw_formatted_text_with_drop_shadow(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(self.empty_text.clone().unwrap()), self.get_x().wrapping_add(4), self.get_y().wrapping_add((self.height.wrapping_sub(8)).wrapping_div(2)), 0x707070, true);
        }
    }

    /// java: public void setValueDoNotUpdate(String value)
    pub fn set_value_do_not_update(&mut self, value: &str) {
        self.do_not_update = true;
        // TODO(fabric2seer): `set_value` takes 2 argument(s) in the port (`splitter: &StringSplitter<W>, value: &str`), Java passes 1
        // rust: self.set_value(value);
        self.do_not_update = false;
    }

    /// java: private void update(String query)
    pub fn update(&mut self, query: &str) {
        let mut was_empty: bool = self.is_empty;
        self.is_empty = query.is_empty();
        if self.is_empty && was_empty {
            return;
        }
        if self.do_not_update {
            return;
        }
        (self.update_consumer.as_ref().unwrap())(query.to_string());
    }

    /// java: public String getQuery()
    pub fn get_query(&self) -> String {
        return self.get_value().to_lowercase();
    }

    /// java: public boolean isEmpty()
    pub fn is_empty(&self) -> bool {
        return self.is_empty;
    }

    /// java: public Component getEmptyText()
    pub fn get_empty_text(&self) -> MutableComponent {
        return self.empty_text.clone().unwrap();
    }

    /// java: public void setEmptyText(Component emptyText)
    pub fn set_empty_text(&mut self, mut empty_text: MutableComponent) {
        self.empty_text = Some(empty_text.clone());
    }

}

// NOTE(fabric2seer): extractWidgetRenderState override(s) of EditBox - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
