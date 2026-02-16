pub(crate) mod ffi;
mod hal;

use crate::state::{Snapshot, SnapshotInterrupt, INT_SLOT_NUM, MEM_BUFFER_SIZE};
use std::fmt::{Display, Formatter};

pub use ffi::{Button, CpuStateView};

#[derive(Debug)]
pub enum EngineError {
    InitFailed,
    Rom(crate::rom::RomDecodeError),
}

impl Display for EngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitFailed => write!(f, "tamalib_init returned failure"),
            Self::Rom(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for EngineError {}

pub struct TamaEngine {
    rom_words: Vec<u16>,
    initialized: bool,
}

impl TamaEngine {
    pub fn save_snapshot(&self) -> Snapshot {
        unsafe {
            let state_ptr = ffi::cpu_get_state();
            let state = &*state_ptr;

            let mut interrupts = Vec::with_capacity(INT_SLOT_NUM);
            for i in 0..INT_SLOT_NUM {
                let intr = *state.interrupts.add(i);
                interrupts.push(SnapshotInterrupt::from(intr));
            }

            let mut memory = vec![0u8; MEM_BUFFER_SIZE];
            std::ptr::copy_nonoverlapping(state.memory, memory.as_mut_ptr(), MEM_BUFFER_SIZE);

            Snapshot {
                pc: *state.pc,
                x: *state.x,
                y: *state.y,
                a: *state.a,
                b: *state.b,
                np: *state.np,
                sp: *state.sp,
                flags: *state.flags,
                tick_counter: *state.tick_counter,
                clk_timer_2hz_timestamp: *state.clk_timer_2hz_timestamp,
                clk_timer_4hz_timestamp: *state.clk_timer_4hz_timestamp,
                clk_timer_8hz_timestamp: *state.clk_timer_8hz_timestamp,
                clk_timer_16hz_timestamp: *state.clk_timer_16hz_timestamp,
                clk_timer_32hz_timestamp: *state.clk_timer_32hz_timestamp,
                clk_timer_64hz_timestamp: *state.clk_timer_64hz_timestamp,
                clk_timer_128hz_timestamp: *state.clk_timer_128hz_timestamp,
                clk_timer_256hz_timestamp: *state.clk_timer_256hz_timestamp,
                prog_timer_timestamp: *state.prog_timer_timestamp,
                prog_timer_enabled: *state.prog_timer_enabled,
                prog_timer_data: *state.prog_timer_data,
                prog_timer_rld: *state.prog_timer_rld,
                call_depth: *state.call_depth,
                interrupts,
                cpu_halted: *state.cpu_halted,
                memory,
            }
        }
    }

    pub fn load_snapshot(&mut self, snap: &Snapshot) {
        unsafe {
            let state_ptr = ffi::cpu_get_state();
            let state = &*state_ptr;

            *state.pc = snap.pc;
            *state.x = snap.x;
            *state.y = snap.y;
            *state.a = snap.a;
            *state.b = snap.b;
            *state.np = snap.np;
            *state.sp = snap.sp;
            *state.flags = snap.flags;
            *state.tick_counter = snap.tick_counter;
            *state.clk_timer_2hz_timestamp = snap.clk_timer_2hz_timestamp;
            *state.clk_timer_4hz_timestamp = snap.clk_timer_4hz_timestamp;
            *state.clk_timer_8hz_timestamp = snap.clk_timer_8hz_timestamp;
            *state.clk_timer_16hz_timestamp = snap.clk_timer_16hz_timestamp;
            *state.clk_timer_32hz_timestamp = snap.clk_timer_32hz_timestamp;
            *state.clk_timer_64hz_timestamp = snap.clk_timer_64hz_timestamp;
            *state.clk_timer_128hz_timestamp = snap.clk_timer_128hz_timestamp;
            *state.clk_timer_256hz_timestamp = snap.clk_timer_256hz_timestamp;
            *state.prog_timer_timestamp = snap.prog_timer_timestamp;
            *state.prog_timer_enabled = snap.prog_timer_enabled;
            *state.prog_timer_data = snap.prog_timer_data;
            *state.prog_timer_rld = snap.prog_timer_rld;
            *state.call_depth = snap.call_depth;
            *state.cpu_halted = snap.cpu_halted;

            for i in 0..INT_SLOT_NUM.min(snap.interrupts.len()) {
                *state.interrupts.add(i) = ffi::Interrupt::from(snap.interrupts[i]);
            }

            let copy_len = MEM_BUFFER_SIZE.min(snap.memory.len());
            std::ptr::copy_nonoverlapping(snap.memory.as_ptr(), state.memory, copy_len);
        }
    }

    pub fn new(rom_words: Vec<u16>) -> Result<Self, EngineError> {
        hal::install_hal();

        let init_result =
            unsafe { ffi::tamalib_init(rom_words.as_ptr(), std::ptr::null_mut(), 1_000_000) };

        if init_result != 0 {
            return Err(EngineError::InitFailed);
        }

        unsafe {
            ffi::tamalib_set_exec_mode(ffi::ExecMode::Run);
        }

        Ok(Self {
            rom_words,
            initialized: true,
        })
    }

    pub fn tick(&mut self) {
        if !self.initialized {
            return;
        }
        unsafe {
            ffi::tamalib_step();
        }
    }

    pub fn tick_many(&mut self, steps: usize) {
        for _ in 0..steps {
            self.tick();
        }
    }

    pub fn set_button(&mut self, button: Button, pressed: bool) {
        let state = if pressed {
            ffi::ButtonState::Pressed
        } else {
            ffi::ButtonState::Released
        };
        unsafe {
            ffi::hw_set_button(button, state);
        }
    }

    pub fn state(&self) -> Option<CpuStateView> {
        let ptr = unsafe { ffi::cpu_get_state() };
        CpuStateView::from_raw(ptr)
    }

    pub fn get_lcd(&self) -> [[bool; 32]; 16] {
        hal::get_lcd_matrix()
    }

    pub fn rom_len_words(&self) -> usize {
        self.rom_words.len()
    }
}

impl Drop for TamaEngine {
    fn drop(&mut self) {
        if self.initialized {
            unsafe { ffi::tamalib_release() };
            self.initialized = false;
        }
    }
}
