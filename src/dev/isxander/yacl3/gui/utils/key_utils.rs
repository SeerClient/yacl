//! Ported from dev.isxander.yacl3.gui.utils.KeyUtils.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::platform::input_constants;

/// java: public final class KeyUtils
pub struct KeyUtils;

impl KeyUtils {
    /// java: public static boolean hasShiftDown()
    pub fn has_shift_down() -> bool {
        // 26.3: `InputConstants.isKeyDown(key)` reads SDL's keyboard state - no window argument
        input_constants::is_key_down(input_constants::KEY_LSHIFT) || input_constants::is_key_down(input_constants::KEY_RSHIFT)
    }

    /// java: public static boolean hasControlDown()
    pub fn has_control_down() -> bool {
        input_constants::is_key_down(input_constants::KEY_LCONTROL) || input_constants::is_key_down(input_constants::KEY_RCONTROL)
    }
}
