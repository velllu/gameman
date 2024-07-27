use crate::{
    bus::Bus,
    common::Bit,
    consts::gpu::{IE, IF, LY, LYC, STAT},
};

use super::Cpu;

#[derive(Debug, PartialEq)]
pub enum Interrupt {
    /// https://gbdev.io/pandocs/STAT.html?highlight=ff41#ff41--stat-lcd-status
    /// This will get triggered if:
    /// - STAT.3: Just entered PPU mode 0 (TODO)
    /// - STAT.4: Just entered PPU mode 1 (TODO)
    /// - STAT.5: Just entered PPU mode 2 (TODO)
    /// - STAT.6: LYC == LY
    Stat,
}

impl Cpu {
    pub(crate) fn execute_interrupts(&mut self, bus: &mut Bus) {
        if !self.ime {
            return;
        }

        let stat = bus[STAT];
        let is_lcd_enabled = stat.get_bit(6) && bus[LYC] == bus[LY];

        // All this thing is done because an interrupt can only be triggered if it was
        // previously off
        if let Some(is_previous_lcd_enabled) = self.previous_lcd {
            if is_lcd_enabled && !is_previous_lcd_enabled {
                self.execute_interrupt(Interrupt::Stat, bus);
            }
        }

        self.previous_lcd = Some(is_lcd_enabled);
    }

    #[allow(unused)]
    fn execute_interrupt(&mut self, interrupt: Interrupt, bus: &mut Bus) {
        // The bit corresponding to the correct interrupt, both in Interrupt Enable, and
        // Interrupt Flag bytes
        let if_bit: u8 = match interrupt {
            Interrupt::Stat => 1,
        };

        // We can only execute an interrupt if it's turned on in the Interrupt Enable and
        // Interrupt Flag bytes
        if bus[IE].get_bit(if_bit) && bus[IF].get_bit(if_bit) {
            return;
        }

        let return_address: u16 = match interrupt {
            Interrupt::Stat => 0x48,
        };

        // TODO: Add this back
        // self.call(return_address, false);

        let mut input_flags = bus[IF];
        input_flags.set_bit(if_bit, false);
        bus[IF] = input_flags;
    }
}
