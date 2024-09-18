//! The most simple MBC, it just has rom and external ram, no banks or anything

use super::{vector_to_array, Mbc, EXTERNAL_RAM_SIZE};

const ROM_SIZE: usize = 0x8000;

pub(crate) struct NoMbc {
    rom: [u8; ROM_SIZE],
    external_ram: [u8; EXTERNAL_RAM_SIZE],
}

impl Mbc for NoMbc {
    fn new(rom: Vec<u8>) -> Self {
        Self {
            rom: vector_to_array::<ROM_SIZE>(rom),
            external_ram: [0; EXTERNAL_RAM_SIZE],
        }
    }

    fn get_rom_section_0(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn get_rom_section_1(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn get_external_ram(&self, address: u16) -> u8 {
        self.external_ram[address as usize]
    }

    fn set_external_ram(&mut self, address: u16, value: u8) {
        self.external_ram[address as usize] = value;
    }

    fn direct_rom_write(&mut self, address: u16, value: u8) {
        self.rom[address as usize] = value;
    }

    // We do nothing when writing to ROM
    fn signal_rom_write(&mut self, _address: u16, _value: u8) {}
}
