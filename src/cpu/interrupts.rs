use crate::{
    bus::Bus,
    common::{split_u16_into_two_u8s, Bit},
    consts::cpu::IF,
    registers::Registers,
};

use super::Cpu;

#[derive(Clone, Copy)]
pub(crate) enum Interrupt {
    /// Triggers when vblank is entered
    VBlank = 0,

    /// Triggers if:
    /// - STAT.3: Just entered PPU mode 0
    /// - STAT.4: Just entered PPU mode 1
    /// - STAT.5: Just entered PPU mode 2
    /// - STAT.6: LYC == LY
    Stat = 1,

    /// Triggers when TIMA register overflows
    Timer = 2,
}

impl Cpu {
    /// We start executing an interrupt when both the interrupt enable and interrupt flag
    /// are enabled
    pub(crate) fn execute_interrupts(&mut self, registers: &mut Registers, bus: &mut Bus) {
        let interrupt_enable = bus.ie;
        let interrupt_flag = bus.read(IF);

        if interrupt_enable.get_bit(0) && interrupt_flag.get_bit(0) {
            self.handle_interrupt(Interrupt::VBlank, registers, bus);
            return;
        }

        if interrupt_enable.get_bit(1) && interrupt_flag.get_bit(1) {
            self.handle_interrupt(Interrupt::Stat, registers, bus);
            return;
        }

        if interrupt_enable.get_bit(2) && interrupt_flag.get_bit(2) {
            self.handle_interrupt(Interrupt::Timer, registers, bus);
            return;
        }
    }

    /// We only dispatch an interrupt if IME is true, but regardless of that we reset the
    /// interrupt bit in IF, this is not used by the emulator but by the program itself
    fn handle_interrupt(&mut self, interrupt: Interrupt, registers: &mut Registers, bus: &mut Bus) {
        let mut input_flags = bus.read(IF);
        input_flags.set_bit(interrupt as u8, false);
        bus.write(IF, input_flags);

        if self.ime {
            self.dispatch_interrupt(interrupt, registers, bus);

            // We disable IME so interrupts are not called immediately, interrupts
            // typically end with a `RETI` instruction that turns this back on
            self.ime = false;

            // And we also un-halt the CPU
            self.halt = false;
        }
    }

    /// We `CALL` the arbitrary address specified by the interrupt
    fn dispatch_interrupt(&self, interrupt: Interrupt, registers: &mut Registers, bus: &mut Bus) {
        let return_address = match interrupt {
            Interrupt::VBlank => 0x40,
            Interrupt::Stat => 0x48,
            Interrupt::Timer => 0x50,
        };

        // This is like the call instruction but we don't subtract three
        let (p, c) = split_u16_into_two_u8s(registers.pc);
        registers.sp = registers.sp.wrapping_sub(1);
        bus.write(registers.sp, p);
        registers.sp = registers.sp.wrapping_sub(1);
        bus.write(registers.sp, c);
        registers.pc = return_address;
    }
}
