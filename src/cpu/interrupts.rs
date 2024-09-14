use crate::{
    bus::Bus,
    common::{split_u16_into_two_u8s, Bit},
    consts::{
        cpu::{IE, IF},
        gpu::{LY, LYC, STAT},
    },
    gpu::Gpu,
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
}

impl Cpu {
    /// Each interrupt has a condition on wheter it gets activated or not, when its bit
    /// in IE is activated and the condition applies, we start Interrupt Handling
    pub(crate) fn execute_interrupts(
        &mut self,
        gpu: &Gpu,
        registers: &mut Registers,
        bus: &mut Bus,
    ) {
        let interrupt_enable = bus[IE];

        if interrupt_enable.get_bit(0) && gpu.has_just_entered_vblank {
            self.handle_interrupt(Interrupt::VBlank, registers, bus);
            return;
        }

        if interrupt_enable.get_bit(1) {
            let stat = bus[STAT];

            if stat.get_bit(3) && gpu.has_just_entered_hblank {
                self.handle_interrupt(Interrupt::Stat, registers, bus);
                return;
            }

            if stat.get_bit(4) && gpu.has_just_entered_vblank {
                self.handle_interrupt(Interrupt::Stat, registers, bus);
                return;
            }

            if stat.get_bit(5) && gpu.has_just_entered_oam_scan {
                self.handle_interrupt(Interrupt::Stat, registers, bus);
                return;
            }

            if stat.get_bit(6) && bus[LY] == bus[LYC] {
                self.handle_interrupt(Interrupt::Stat, registers, bus);
                return;
            }

            return;
        }
    }

    /// We only dispatch an interrupt if IME is true, but regardless of that we reset the
    /// interrupt bit in IF, this is not used by the emulator but by the program itself
    fn handle_interrupt(&mut self, interrupt: Interrupt, registers: &mut Registers, bus: &mut Bus) {
        let mut input_flags = bus[IF];
        input_flags.set_bit(interrupt as u8, false);
        bus[IF] = input_flags;

        if self.ime {
            self.dispatch_interrupt(interrupt, registers, bus);

            // We disable IME so interrupts are not called immediately, interrupts
            // typically end with a `RETI` instruction that turns this back on
            self.ime = false;
        }
    }

    /// We `CALL` the arbitrary address specified by the interrupt
    fn dispatch_interrupt(&self, interrupt: Interrupt, registers: &mut Registers, bus: &mut Bus) {
        let return_address = match interrupt {
            Interrupt::VBlank => 0x40,
            Interrupt::Stat => 0x48,
        };

        // This is like the call instruction but we don't subtract three
        let (p, c) = split_u16_into_two_u8s(registers.pc);
        registers.sp = registers.sp.wrapping_sub(1);
        bus[registers.sp] = p;
        registers.sp = registers.sp.wrapping_sub(1);
        bus[registers.sp] = c;
        registers.pc = return_address;
    }
}
