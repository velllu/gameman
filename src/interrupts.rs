use crate::{common::Bit, GameBoy};

struct Interrupts {
    vblank: bool,
    lcd: bool,
    timer: bool,
    serial: bool,
    joypad: bool,
}

impl From<u8> for Interrupts {
    fn from(value: u8) -> Self {
        Self {
            vblank: value.get_bit(0),
            lcd: value.get_bit(1),
            timer: value.get_bit(2),
            serial: value.get_bit(3),
            joypad: value.get_bit(4),
        }
    }
}

impl From<Interrupts> for u8 {
    fn from(value: Interrupts) -> Self {
        let mut result: u8 = 0;

        result.set_bit(0, value.vblank);
        result.set_bit(1, value.lcd);
        result.set_bit(2, value.timer);
        result.set_bit(3, value.serial);
        result.set_bit(4, value.joypad);

        result
    }
}

impl GameBoy {
    pub(crate) fn execute_interrupts(&mut self) {
        if !self.flags.ime {
            return;
        }

        let is_enabled: Interrupts = self.bus[0xFFFF].into();
        let mut value: Interrupts = self.bus[0xFF0F].into();

        // TODO: Make code DRYer
        if is_enabled.vblank && value.vblank {
            self.call(0x40);

            value.vblank = false;
            self.bus[0xFF0F] = value.into();
        }
    }
}
