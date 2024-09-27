use crate::{
    bus::Bus,
    common::Bit,
    consts::cpu::{DIV, IF, TAC, TIMA, TMA},
};

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

    /// A cycle counter for keeping track of when to increment the DIV register
    pub(crate) div_cycle_counter: u8,

    /// A cycle counter for keeping track of when to increment the TIMA register
    pub(crate) tima_cycle_counter: u16,
}

impl Cpu {
    pub(crate) fn new() -> Self {
        Self {
            ime: false,
            halt: false,
            div_cycle_counter: 1,
            tima_cycle_counter: 0,
        }
    }

    /// Increment the div register every 64 cycles
    pub(crate) fn update_div_register(&mut self, bus: &mut Bus, cycles_num: u8) {
        // When we write to the div register we also reset the cycle counter
        if bus.needs_to_reset_div_register {
            // I don't know why this must be reset to 1, but it works!
            self.div_cycle_counter = 1;
            bus.needs_to_reset_div_register = false;
        }

        self.div_cycle_counter += cycles_num;

        if self.div_cycle_counter >= 64 {
            self.div_cycle_counter -= 64;

            // Increment the DIV register, by using IO directly, otherwise writing to DIV
            // will trigger a DIV reset
            bus.io[0x04] = bus.read(DIV).wrapping_add(1);
        }
    }

    /// Increse the tima register based on the contents of the timer control register
    pub(crate) fn update_tima_register(&mut self, bus: &mut Bus, cycles_num: u8) {
        let timer_counter = bus.read(TIMA);
        let timer_control = bus.read(TAC);
        let timer_module = bus.read(TMA);

        // The second byte of timer control indicates wheter or not we are counting
        if !timer_control.get_bit(2) {
            return;
        }

        self.tima_cycle_counter += cycles_num as u16;

        // This is the threshold at which we increment the timer counter
        let increment_every = match (timer_control.get_bit(1), timer_control.get_bit(0)) {
            (false, false) => 256,
            (false, true) => 4,
            (true, false) => 16,
            (true, true) => 64,
        };

        if self.tima_cycle_counter > increment_every {
            self.tima_cycle_counter -= increment_every;

            // When the timer counter overflows we have to run a timer interrupt
            let (result, has_overflown) = timer_counter.overflowing_add(1);

            if has_overflown {
                // When overflowing, we both trigger a timer interrupt and we reset the
                // value of the timer counter to that of the timer module
                bus.write(IF, bus.read(IF) | 0b00000100); // Enabling timer interrupt
                bus.write(TIMA, timer_module);
            } else {
                // When not overflowing, we just increment the timer
                bus.write(TAC, result);
            }
        }
    }
}
