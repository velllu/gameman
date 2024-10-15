// TODO: Implement timer and day counter

use super::{
    calculate_ram_address, calculate_rom_address, get_ram_size, get_rom_size, Mbc, ROM_BANK_SIZE,
};

pub(crate) struct Mbc3 {
    rom: Vec<u8>,
    external_ram: Vec<u8>,
    are_ram_and_timer_enabled: bool,
    rom_size: usize,
    ram_size: usize,
    ram_bank_number: usize,
    rom_bank_number: usize,
}

impl Mbc for Mbc3 {
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
            are_ram_and_timer_enabled: false,
            rom_bank_number: 0,
            ram_bank_number: 0,
            rom_size,
            ram_size,
        }
    }

    /// Section 0 is just the first bank
    fn get_rom_section_0(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    /// Section 1 is like MBC1's except there's no bug that prohibits access to some banks
    fn get_rom_section_1(&self, address: u16) -> u8 {
        let new_address = calculate_rom_address(
            self.rom_size,
            address - ROM_BANK_SIZE as u16,
            self.rom_bank_number,
        );

        self.rom[new_address]
    }

    fn get_external_ram(&self, address: u16) -> u8 {
        let new_address = calculate_ram_address(self.ram_size, address, self.ram_bank_number);
        self.external_ram[new_address]
    }

    fn set_external_ram(&mut self, address: u16, value: u8) {
        let new_address = calculate_ram_address(self.ram_size, address, self.ram_bank_number);
        self.external_ram[new_address] = value;
    }

    fn direct_rom_write(&mut self, address: u16, value: u8) {
        self.rom[address as usize] = value;
    }

    fn signal_rom_write(&mut self, address: u16, value: u8) {
        // You enable or disable ram and the timer by writing to 0000-1FFF, it turns on if
        // the game writes "A", and any other number will turn it off for some reason
        if address <= 0x1FFF {
            self.are_ram_and_timer_enabled = value & 0x0F == 0xA;
            return;
        }

        // Address 2000-3FFF is the seven bit rom bank number register
        if (0x2000..=0x3FFF).contains(&address) {
            self.rom_bank_number = value as usize & 0b01111111;
            return;
        }

        // Address 4000-5FFF is the two bit ram bank number register
        if (0x4000..=0x5FFF).contains(&address) {
            self.ram_bank_number = value as usize & 0b00000011;
            return;
        }
    }
}
