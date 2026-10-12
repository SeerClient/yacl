//! Ported from dev.isxander.yacl3.gui.RequireRestartScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::dev::isxander::yacl3::gui::utils::gui_utils::GuiUtils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::screens::confirm_screen::ConfirmScreen;
use mcrust::net::minecraft::client::minecraft;
use mcrust::net::minecraft::network::chat::component;

/// java: public class RequireRestartScreen extends ConfirmScreen
pub struct RequireRestartScreen {
    /// java: extends net.minecraft.client.gui.screens.ConfirmScreen
    pub base: ConfirmScreen,
}

impl std::ops::Deref for RequireRestartScreen {
    type Target = ConfirmScreen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for RequireRestartScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl RequireRestartScreen {
    /// java: public RequireRestartScreen(Screen parent)
    pub fn new(mut parent: mcrust::net::minecraft::client::gui::screens::screen::Screen) -> Self {
        // TODO(fabric2seer): `new ConfirmScreen(..)`: the port has no `ConfirmScreen::new`
        //   java: super(option -> {
        //   java:             if (option) Minecraft.getInstance().stop();
        //   java:             else GuiUtils.setScreen(parent);
        //   java:         },
        //   java:                 Component.translatable("yacl.restart.title").withStyle(ChatFormatting.RED, ChatFormatting.BOLD),
        //   java:                 Component.translatable("yacl.restart.message"),
        //   java:                 Component.translatable("yacl.restart.yes"),
        //   java:                 Component.translatable("yacl.restart.no")
        //   java:         );
        let mut this: Self = todo!("constructor");
        this
    }

}
