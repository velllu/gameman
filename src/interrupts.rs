use crate::{
    common::Bit,
    consts::gpu::{IE, IF, LY, LYC, STAT},
    GameBoy,
};

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

impl GameBoy {
    pub(crate) fn execute_interrupts(&mut self) {
        if !self.flags.ime {
            return;
        }

        let stat = self.bus[STAT];
        if stat.get_bit(6) && self.bus[LYC] == self.bus[LY] {
            self.execute_interrupt(Interrupt::Stat);
        }
    }

    fn execute_interrupt(&mut self, interrupt: Interrupt) {
        // We don't want to call the same interrupt twice
        if let Some(previous_interrupt) = &self.previous_interrupt {
            if *previous_interrupt == interrupt {
                return;
            }
        }

        // The bit corresponding to the correct interrupt, both in Interrupt Enable, and
        // Interrupt Flag bytes
        let if_bit: u8 = match interrupt {
            Interrupt::Stat => 1,
        };

        // We can only execute an interrupt if it's turned on in the Interrupt Enable and
        // Interrupt Flag bytes
        if self.bus[IE].get_bit(if_bit) && self.bus[IF].get_bit(if_bit) {
            return;
        }

        let return_address: u16 = match interrupt {
            Interrupt::Stat => 0x48,
        };

        self.call(return_address);

        let mut input_flags = self.bus[IF];
        input_flags.set_bit(if_bit, false);
        self.bus[IF] = input_flags;

        self.previous_interrupt = Some(interrupt);
    }
}
