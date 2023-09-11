use bitvec::prelude::*;

use crate::{common::split_u16_into_two_u8s, GameBoy};

struct Interrupts {
    vblank: bool,
    lcd: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl From<u8> for Interrupts {
    fn from(value: u8) -> Self {
        let bits = value.view_bits::<Msb0>();

        Self {
            vblank: bits[0],
            lcd: bits[1],
            timer: bits[2],
            serial: bits[3],
            joypad: bits[4],
        }
    }
}

impl From<Interrupts> for u8 {
    fn from(value: Interrupts) -> Self {
        let mut result: u8 = 0;

        result |= value.vblank as u8;
        result |= (value.lcd as u8) << 1;
        result |= (value.timer as u8) << 2;
        result |= (value.serial as u8) << 3;
        result |= (value.joypad as u8) << 4;

        result
    }
}

impl GameBoy {
    fn interrupt(&mut self, pc_location: u16) {
        let (p, c) = split_u16_into_two_u8s(self.registers.pc);

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.bus[self.registers.sp] = p;
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.bus[self.registers.sp] = c;

        self.registers.pc = pc_location;
    }

    pub(crate) fn execute_interrupts(&mut self) {
        if !self.flags.ime {
            return;
        }

        let is_enabled: Interrupts = self.bus[0xFFFF].into();
        let mut value: Interrupts = self.bus[0xFF0F].into();

        // TODO: Make code DRYer
        if is_enabled.vblank && value.vblank {
            self.interrupt(0x40);

            value.vblank = false;
            self.bus[0xFF0F] = value.into();
        }
    }
}
