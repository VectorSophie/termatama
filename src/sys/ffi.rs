use std::ffi::c_void;

pub type BoolT = u8;
pub type U4T = u8;
pub type U5T = u8;
pub type U8T = u8;
pub type U12T = u16;
pub type U13T = u16;
pub type U32T = u32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Interrupt {
    pub factor_flag_reg: U4T,
    pub mask_reg: U4T,
    pub triggered: BoolT,
    pub vector: U8T,
}

#[repr(C)]
#[derive(Debug)]
pub struct State {
    pub pc: *mut U13T,
    pub x: *mut U12T,
    pub y: *mut U12T,
    pub a: *mut U4T,
    pub b: *mut U4T,
    pub np: *mut U5T,
    pub sp: *mut U8T,
    pub flags: *mut U4T,
    pub tick_counter: *mut U32T,
    pub clk_timer_2hz_timestamp: *mut U32T,
    pub clk_timer_4hz_timestamp: *mut U32T,
    pub clk_timer_8hz_timestamp: *mut U32T,
    pub clk_timer_16hz_timestamp: *mut U32T,
    pub clk_timer_32hz_timestamp: *mut U32T,
    pub clk_timer_64hz_timestamp: *mut U32T,
    pub clk_timer_128hz_timestamp: *mut U32T,
    pub clk_timer_256hz_timestamp: *mut U32T,
    pub prog_timer_timestamp: *mut U32T,
    pub prog_timer_enabled: *mut BoolT,
    pub prog_timer_data: *mut U8T,
    pub prog_timer_rld: *mut U8T,
    pub call_depth: *mut U32T,
    pub interrupts: *mut Interrupt,
    pub cpu_halted: *mut BoolT,
    pub memory: *mut U8T,
}

#[derive(Debug, Clone, Copy)]
pub struct CpuStateView {
    pub pc: U13T,
    pub x: U12T,
    pub y: U12T,
    pub a: U4T,
    pub b: U4T,
    pub np: U5T,
    pub sp: U8T,
    pub tick_counter: U32T,
}

impl CpuStateView {
    pub fn from_raw(ptr: *const State) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }
        let state = unsafe { &*ptr };
        Some(Self {
            pc: unsafe { *state.pc },
            x: unsafe { *state.x },
            y: unsafe { *state.y },
            a: unsafe { *state.a },
            b: unsafe { *state.b },
            np: unsafe { *state.np },
            sp: unsafe { *state.sp },
            tick_counter: unsafe { *state.tick_counter },
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ExecMode {
    Pause = 0,
    Run = 1,
    Step = 2,
    Next = 3,
    ToCall = 4,
    ToRet = 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Button {
    Left = 0,
    Middle = 1,
    Right = 2,
    Tap = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ButtonState {
    Released = 0,
    Pressed = 1,
}

unsafe extern "C" {
    pub fn tamalib_init(program: *const U12T, breakpoints: *mut c_void, freq: U32T) -> BoolT;
    pub fn tamalib_release();
    pub fn tamalib_set_exec_mode(mode: ExecMode);
    pub fn tamalib_step();
    pub fn cpu_get_state() -> *const State;
    pub fn hw_set_button(btn: Button, state: ButtonState);
    pub fn tamars_register_hal();
}
