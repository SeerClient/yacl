//! Ported from dev.isxander.yacl3.gui.YACLSelectionList by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::widget_and_type;
use crate::dev::isxander::yacl3::gui::widget_and_type::WidgetAndType;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;

/// java: public abstract class YACLSelectionList<E extends YACLSelectionList.Entry<E>> extends ContainerObjectSelectionList<E>
pub struct YACLSelectionListBase<E> {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
    /// java: private boolean doneRefresh;
    pub done_refresh: bool,
}

impl<E: 'static> std::ops::Deref for YACLSelectionListBase<E> {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl<E: 'static> std::ops::DerefMut for YACLSelectionListBase<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl<E: Clone + 'static> YACLSelectionListBase<E> {
    /// java: public YACLSelectionList(Minecraft minecraft, int width, int height, int y)
    pub fn new(minecraft: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut width: i32, mut height: i32, mut y: i32) -> Self {
        let mut this = Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, y, 20),
            done_refresh: false,
        };
        this
    }

    /// java: public static <T extends YACLSelectionList<?>> WidgetAndType<T> asWidget(T list)
    pub fn as_widget(list: &compat::Shared<dyn YACLSelectionList<compat::Object>>) -> compat::Shared<dyn WidgetAndType<compat::Shared<dyn YACLSelectionList<compat::Object>>>> {
        return widget_and_type::of_widget(list);
    }

}

/// java: the virtual methods of YACLSelectionList (implemented by YACLSelectionListBase and every subclass)
pub trait YACLSelectionList<E: Clone + 'static> {
    fn yacl_selection_list_base(&self) -> &YACLSelectionListBase<E>;
    fn yacl_selection_list_base_mut(&mut self) -> &mut YACLSelectionListBase<E>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override public void extractWidgetRenderState(@NonNull GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_widget_render_state(&mut self, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        if !(self.yacl_selection_list_base().done_refresh) {
            self.yacl_selection_list_base().reposition_entries();
            self.yacl_selection_list_base_mut().done_refresh = true;
        }
        // TODO(fabric2seer): `ContainerObjectSelectionList.extractWidgetRenderState` has no Rust method in the port (looked for `extract_widget_render_state` in ContainerObjectSelectionList and its supertypes)
        // rust: self.yacl_selection_list_base().base.extract_widget_render_state(graphics, mouse_x, mouse_y, a);
    }

}

/// java: public static abstract class Entry<E extends YACLSelectionList.Entry<E>> extends ContainerObjectSelectionList.Entry<E>
pub struct EntryBase<E> {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl<E: 'static> std::ops::Deref for EntryBase<E> {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<E: 'static> std::ops::DerefMut for EntryBase<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<E: Clone + 'static> EntryBase<E> {
    /// java: public Entry(YACLSelectionList<E> parent)
    pub fn new(mut parent: compat::Shared<dyn YACLSelectionList<E>>) -> Self {
        let mut this = Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
        };
        this
    }

}

/// java: the virtual methods of Entry (implemented by EntryBase and every subclass)
pub trait Entry<E: Clone + 'static> {
    fn entry_base(&self) -> &EntryBase<E>;
    fn entry_base_mut(&mut self) -> &mut EntryBase<E>;
    fn as_any(&self) -> &dyn std::any::Any;

}
