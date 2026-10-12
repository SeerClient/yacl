//! Ported from dev.isxander.yacl3.gui.utils.KeyUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::platform::input_constants;

/// java: public final class KeyUtils
#[derive(Clone)]
pub struct KeyUtils {}

impl KeyUtils {
    /// java: private KeyUtils()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean hasShiftDown()
    pub fn has_shift_down() -> bool {
        let window = compat::minecraft().get_window();
        return (input_constants::is_key_down(window.clone(), mcrust::com::mojang::blaze3d::platform::input_constants::KEY_LSHIFT) || input_constants::is_key_down(window.clone(), mcrust::com::mojang::blaze3d::platform::input_constants::KEY_RSHIFT));
    }

    /// java: public static boolean hasControlDown()
    pub fn has_control_down() -> bool {
        let window = compat::minecraft().get_window();
        return (input_constants::is_key_down(window.clone(), mcrust::com::mojang::blaze3d::platform::input_constants::KEY_LCONTROL) || input_constants::is_key_down(window.clone(), mcrust::com::mojang::blaze3d::platform::input_constants::KEY_RCONTROL));
    }

}
