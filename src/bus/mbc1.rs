use crate::common::Bit;

use super::{
    calculate_ram_address, calculate_rom_address, get_ram_size, get_rom_size, Mbc, ROM_BANK_SIZE,
};

enum BankingMode {
    Simple,
    Advanced,
}

pub(crate) struct Mbc1 {
    rom: Vec<u8>,
    external_ram: Vec<u8>,
    is_ram_enabled: bool,
    rom_bank_number: usize,
    ram_bank_number: usize,
    banking_mode: BankingMode,
    rom_size: usize,
    ram_size: usize,
}

impl Mbc for Mbc1 {
    fn new(mut rom: Vec<u8>) -> Self {
        let rom_size = get_rom_size(&rom);
        let ram_size = get_ram_size(&rom);

        // Resizing given rom
        rom.resize(rom_size, 0);

        // Creating the new ram
        let ram: Vec<u8> = vec![0; ram_size];

        Self {
            rom,
            external_ram: ram,
            is_ram_enabled: false,
            rom_bank_number: 0,
            ram_bank_number: 0,
            banking_mode: BankingMode::Simple,
            rom_size,
            ram_size,
        }
    }

    /// Section 0 just uses the first bank in simple banking mode, in advanced banking, it
    /// uses the ram bank number to calculate the new address
    fn get_rom_section_0(&self, address: u16) -> u8 {
        match self.banking_mode {
            BankingMode::Simple => self.rom[address as usize],
            BankingMode::Advanced => {
                let bank = self.ram_bank_number << 5;
                let new_address = calculate_rom_address(self.rom_size, address, bank);

                self.rom[new_address]
            }
        }
    }

    /// Section 1 rom uses both the rom and ram bank number to calculate the new address
    fn get_rom_section_1(&self, address: u16) -> u8 {
        let mut bank = (self.ram_bank_number << 5) | self.rom_bank_number;

        // For a bug these are not accessible
        if bank == 0x20 || bank == 0x40 || bank == 0x60 {
            bank += 1;
        }

        let new_address =
            calculate_rom_address(self.rom_size, address - ROM_BANK_SIZE as u16, bank);

        self.rom[new_address]
    }

    fn get_external_ram(&self, address: u16) -> u8 {
        if !self.is_ram_enabled || self.ram_size == 0 {
            return 0xFF;
        }

        let new_address = self.calculate_ram_address(address);
        self.external_ram[new_address]
    }

    fn set_external_ram(&mut self, address: u16, value: u8) {
        if !self.is_ram_enabled || self.ram_size == 0 {
            return;
        }

        let new_address = self.calculate_ram_address(address);
        self.external_ram[new_address] = value;
    }

    fn direct_rom_write(&mut self, address: u16, value: u8) {
        self.rom[address as usize] = value;
    }

    fn signal_rom_write(&mut self, address: u16, value: u8) {
        // You enable or disable ram by writing to 0000-1FFF, it turns on if the game
        // writes "A", and any other number will turn it off for some reason
        if address <= 0x1FFF {
            self.is_ram_enabled = value & 0x0F == 0xA;
            return;
        }

        // Address 2000-3FFF is the five bit rom bank number register
        if (0x2000..=0x3FFF).contains(&address) {
            self.rom_bank_number = value as usize & 0b00011111;

            // The rom bank cannot be 0
            if self.rom_bank_number == 0 {
                self.rom_bank_number = 1;
            }

            return;
        }

        // Address 4000-5FFF is the two bit ram bank number register
        if (0x4000..=0x5FFF).contains(&address) {
            self.ram_bank_number = value as usize & 0b00000011;
            return;
        }

        // Address 6000-7FFF is the one bit banking mode register
        if (0x6000..=0x7FFF).contains(&address) {
            self.banking_mode = match value.get_bit(0) {
                false => BankingMode::Simple,
                true => BankingMode::Advanced,
            };
        }
    }
}

impl Mbc1 {
    /// Calculates the new ram address based on the address the game tells us and the ram
    /// banking mode
    fn calculate_ram_address(&self, address: u16) -> usize {
        match self.banking_mode {
            // If it's simple we just use bank 0
            BankingMode::Simple => address as usize % self.ram_size,

            // Otherwise we use the ram bank number
            BankingMode::Advanced => {
                calculate_ram_address(self.ram_size, address, self.ram_bank_number)
            }
        }
    }
}
