use std::sync::{Mutex, OnceLock};
use std::time::Instant;

use super::ffi::{BoolT, U32T, U8T};

struct HalState {
    start: Instant,
    lcd: [[bool; 32]; 16],
    icons: [bool; 8],
    frequency_dhz: u32,
    playing: bool,
}

impl Default for HalState {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            lcd: [[false; 32]; 16],
            icons: [false; 8],
            frequency_dhz: 0,
            playing: false,
        }
    }
}

static HAL_STATE: OnceLock<Mutex<HalState>> = OnceLock::new();

fn state() -> &'static Mutex<HalState> {
    HAL_STATE.get_or_init(|| Mutex::new(HalState::default()))
}

pub fn get_lcd_matrix() -> [[bool; 32]; 16] {
    state().lock().expect("hal lock").lcd
}

pub fn install_hal() {
    let _ = state();
    unsafe {
        super::ffi::tamars_register_hal();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hal_halt() {}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hal_is_log_enabled(_level: i32) -> BoolT {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hal_sleep_until(_ts: U32T) {}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hal_get_timestamp() -> U32T {
    let elapsed = state()
        .lock()
        .expect("hal lock")
        .start
        .elapsed()
        .as_micros();
    (elapsed.min(u32::MAX as u128)) as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hal_update_screen() {}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hal_set_lcd_matrix(x: U8T, y: U8T, val: BoolT) {
    if (x as usize) < 32 && (y as usize) < 16 {
        let mut guard = state().lock().expect("hal lock");
        guard.lcd[y as usize][x as usize] = val != 0;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hal_set_lcd_icon(icon: U8T, val: BoolT) {
    if (icon as usize) < 8 {
        let mut guard = state().lock().expect("hal lock");
        guard.icons[icon as usize] = val != 0;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hal_set_frequency(freq: U32T) {
    let mut guard = state().lock().expect("hal lock");
    guard.frequency_dhz = freq;
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hal_play_frequency(enabled: BoolT) {
    let mut guard = state().lock().expect("hal lock");
    guard.playing = enabled != 0;
}

#[unsafe(no_mangle)]
pub extern "C" fn rs_hal_handler() -> i32 {
    0
}
