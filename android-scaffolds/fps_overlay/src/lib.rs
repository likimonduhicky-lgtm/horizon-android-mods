//! Minimal Android port of fps_overlay using the public horizon-native-abi.
//! This is a starting point only. Real FPS counting needs a proper frame callback
//! that the current ABI may or may not expose depending on the Android port version.

use horizon_native_abi::{Host, NativeMod, State, export_mod};

struct FpsOverlay {
    // In a real port you would track frame times here.
    // The basic ABI only gives you `hud` and `tick`.
}

impl NativeMod for FpsOverlay {
    fn new(host: &mut Host) -> Self {
        host.log("fps_overlay (Android scaffold) loaded");
        FpsOverlay {}
    }

    fn hud(&mut self, host: &mut Host, game: &State) {
        if game.in_world {
            // Placeholder – real FPS needs timing data the basic ABI may not provide yet.
            host.hud_line("FPS: --", 0x55FF55);
        }
    }
}

export_mod!(FpsOverlay);
