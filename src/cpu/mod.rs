use crate::{bus::Bus, consts::cpu::DIV};

mod interrupts;
mod opcodes;
mod opcodes_cb;

/// The number of bytes to skip after interpreting the instruction, if the instruction is
/// 2 bytes long we will need to skip 2 bytes
pub type Bytes = u8;

/// The amounts of cycles and instruction takes
pub type Cycles = u8;

pub struct Cpu {
    /// IME, standing for "Interrupt Master Enable" is basically a switch on whether
    /// interrupts should be handled or not
    pub ime: bool,

    /// Wheter or not the CPU is halted
    pub halt: bool,

    /// This is increased after each cycle, it's used for the timers
    pub(crate) div_register: u16,
}

impl Cpu {
    pub(crate) fn new() -> Self {
        Self {
            ime: false,
            halt: false,
            div_register: 0,
        }
    }

    pub(crate) fn update_div_register(&mut self, bus: &mut Bus, cycles_num: u8) {
        self.div_register = self.div_register.wrapping_add(cycles_num as u16);

        // While the div register is 16 bit, we only actually give the bus the higher 8
        // bits, this means that it will increment after 256 cycles
        let higher_byte = (self.div_register >> 8) as u8;
        bus[DIV] = higher_byte;
    }
}
