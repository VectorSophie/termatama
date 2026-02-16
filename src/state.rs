use super::sys::ffi::{BoolT, Interrupt, U12T, U13T, U32T, U4T, U5T, U8T};
use serde::{Deserialize, Serialize};

pub const MEM_BUFFER_SIZE: usize = 464;
pub const INT_SLOT_NUM: usize = 6;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Snapshot {
    pub pc: U13T,
    pub x: U12T,
    pub y: U12T,
    pub a: U4T,
    pub b: U4T,
    pub np: U5T,
    pub sp: U8T,
    pub flags: U4T,
    pub tick_counter: U32T,
    pub clk_timer_2hz_timestamp: U32T,
    pub clk_timer_4hz_timestamp: U32T,
    pub clk_timer_8hz_timestamp: U32T,
    pub clk_timer_16hz_timestamp: U32T,
    pub clk_timer_32hz_timestamp: U32T,
    pub clk_timer_64hz_timestamp: U32T,
    pub clk_timer_128hz_timestamp: U32T,
    pub clk_timer_256hz_timestamp: U32T,
    pub prog_timer_timestamp: U32T,
    pub prog_timer_enabled: BoolT,
    pub prog_timer_data: U8T,
    pub prog_timer_rld: U8T,
    pub call_depth: U32T,
    pub interrupts: Vec<SnapshotInterrupt>,
    pub cpu_halted: BoolT,
    pub memory: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SnapshotInterrupt {
    pub factor_flag_reg: U4T,
    pub mask_reg: U4T,
    pub triggered: BoolT,
    pub vector: U8T,
}

impl From<Interrupt> for SnapshotInterrupt {
    fn from(i: Interrupt) -> Self {
        Self {
            factor_flag_reg: i.factor_flag_reg,
            mask_reg: i.mask_reg,
            triggered: i.triggered,
            vector: i.vector,
        }
    }
}

impl From<SnapshotInterrupt> for Interrupt {
    fn from(i: SnapshotInterrupt) -> Self {
        Self {
            factor_flag_reg: i.factor_flag_reg,
            mask_reg: i.mask_reg,
            triggered: i.triggered,
            vector: i.vector,
        }
    }
}
