#![feature(proc_macro_hygiene)]

use smash::app::lua_bind;
use rand::Rng;
pub mod ext;
mod singletons;

static mut SHOULD_END_RESULT_SCREEN: bool = false;

// Skip results screen with start button
#[skyline::hook(offset = 0x3664CE0)]
unsafe fn process_inputs_handheld(controller: &mut ext::Controller) {
    let entry_count = lua_bind::FighterManager::entry_count(singletons::FighterManager());
    if lua_bind::FighterManager::is_result_mode(singletons::FighterManager()) && entry_count > 0 {
        if ninput::any::is_press(ninput::Buttons::PLUS) {
            SHOULD_END_RESULT_SCREEN = true;
        }
        if ninput::any::is_press(ninput::Buttons::B) {
            SHOULD_END_RESULT_SCREEN = false;
        }
        if SHOULD_END_RESULT_SCREEN {
            let mut rng = rand::thread_rng();
            // Need to space apart A-presses so it does not seem like we are holding the button.
            let n: u32 = rng.gen_range(0..3);
            if n == 1 {
                controller.current_buttons.set_a(true);
                controller.just_down.set_a(true);
            }
        }
    }
    if entry_count == 0 {
        SHOULD_END_RESULT_SCREEN = false;
    }
    call_original!(controller);
}

#[skyline::main(name = "results-screen-skip")]
pub fn main() {
    singletons::init();
    skyline::install_hooks!(
        process_inputs_handheld
    );
}
