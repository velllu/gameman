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

    /// This is increased after each cycle, it's used for the timers
    pub(crate) div_register: u16,

    /// This is increased based on the TAC (timer control) register
    pub(crate) tima_register: u16,
}

impl Cpu {
    pub(crate) fn new() -> Self {
        Self {
            ime: false,
            halt: false,
            div_register: 0,
            tima_register: 0,
        }
    }

    /// Increase the div register
    pub(crate) fn update_div_register(&mut self, bus: &mut Bus, cycles_num: u8) {
        self.div_register = self.div_register.wrapping_add(cycles_num as u16);

        // While the div register is 16 bit, we only actually give the bus the higher 8
        // bits, this means that it will increment after 256 cycles
        let higher_byte = (self.div_register >> 8) as u8;
        bus.write(DIV, higher_byte);
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

        self.tima_register += cycles_num as u16;

        // This is the threshold at which we increment the timer counter
        let increment_every = match (timer_control.get_bit(1), timer_control.get_bit(0)) {
            (false, false) => 256,
            (false, true) => 4,
            (true, false) => 16,
            (true, true) => 64,
        };

        if self.tima_register > increment_every {
            self.tima_register -= increment_every;

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
