use std::{error::Error, fmt::Display, fs::File, io::Read};

use crate::{common::merge_two_u8s_into_u16, consts::bus::*, registers::Registers};

#[derive(Debug)]
pub enum BusError {
    CouldNotFindRom,
    CouldNotReadRom,
}

impl Error for BusError {}
impl Display for BusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CouldNotFindRom => write!(f, "could not find rom"),
            Self::CouldNotReadRom => write!(f, "could not read rom"),
        }
    }
}

/// GameBoy uses mapped memory, this means that there are various "regions" of RAM, and we
/// need to calculate in what region to put ram, example:
/// ```no_compile
/// - Video ram, size: 100  
/// - Work ram, size: 100  
/// If we need to set address 150 for example, it will go to the 50th bit of the work ram
/// ```
#[allow(unused)]
pub struct Bus {
    eom: [u8; EOM_SIZE],
    external_ram: [u8; EXTERNAL_RAM_SIZE],
    high_ram: [u8; HIGH_RAM_SIZE],
    ie: u8,
    io: [u8; IO_SIZE],
    rom: [u8; ROM_SIZE],
    video_ram: [u8; VIDEO_RAM_SIZE],
    work_ram: [u8; WORK_RAM_SIZE],

    /// TODO: This is not accurate to the gameboy, when a game writes to a ROM address,
    /// it should send commands to the cartridge, however, as of now, I just have a clone
    /// of the ROM stored here, it acts like normal RAM.
    rom_clone: [u8; ROM_SIZE],

    /// TODO: This should actually not be usuable
    unusable_ram: [u8; UNUSABLE_RAM_SIZE],
}

#[rustfmt::skip]
const fn new_io() -> [u8; IO_SIZE] {
    // IO default values
    // Yes, there's no better way to do this
    let mut io = [0u8; IO_SIZE];
    io[0x00] = 0xCF; io[0x02] = 0x7E; io[0x04] = 0x18; io[0x07] = 0xF8; io[0x0F] = 0xE1;
    io[0x10] = 0x80; io[0x11] = 0xBF; io[0x12] = 0xF3; io[0x13] = 0xFF; io[0x14] = 0xBF;
    io[0x16] = 0x3F; io[0x18] = 0xFF; io[0x19] = 0xBF; io[0x1A] = 0x7F; io[0x1B] = 0xFF;
    io[0x1C] = 0x9F; io[0x1D] = 0xFF; io[0x1E] = 0xBF; io[0x20] = 0xFF; io[0x23] = 0xBF;
    io[0x24] = 0x77; io[0x25] = 0xF3; io[0x26] = 0xF1; io[0x40] = 0x91; io[0x41] = 0x81;
    io[0x44] = 0x00; io[0x46] = 0xFF; io[0x47] = 0xFC; io[0x4D] = 0xFF; io[0x4F] = 0xFF;
    io[0x51] = 0xFF; io[0x52] = 0xFF; io[0x53] = 0xFF; io[0x54] = 0xFF; io[0x55] = 0xFF;
    io[0x56] = 0xFF; io[0x68] = 0xFF; io[0x69] = 0xFF; io[0x6A] = 0xFF; io[0x6B] = 0xFF;
    io[0x70] = 0xFF;

    io
}

impl Bus {
    pub(crate) fn new(rom_path: &str) -> Result<Self, BusError> {
        // ROM loading
        let mut rom = [0u8; ROM_SIZE];

        let mut rom_file = match File::open(rom_path) {
            Ok(rom_file) => rom_file,
            Err(_) => return Err(BusError::CouldNotFindRom),
        };

        match rom_file.read(&mut rom) {
            Ok(_) => {}
            Err(_) => return Err(BusError::CouldNotReadRom),
        }

        // Actual returning
        Ok(Self {
            eom: [0u8; EOM_SIZE],
            external_ram: [0u8; EXTERNAL_RAM_SIZE],
            high_ram: [0u8; HIGH_RAM_SIZE],
            ie: 0u8,
            io: new_io(),
            rom,
            rom_clone: rom,
            unusable_ram: [0u8; UNUSABLE_RAM_SIZE],
            video_ram: [0u8; VIDEO_RAM_SIZE],
            work_ram: [0u8; WORK_RAM_SIZE],
        })
    }

    pub(crate) fn new_from_rom_array(rom: [u8; ROM_SIZE]) -> Self {
        Self {
            eom: [0u8; EOM_SIZE],
            external_ram: [0u8; EXTERNAL_RAM_SIZE],
            high_ram: [0u8; HIGH_RAM_SIZE],
            ie: 0u8,
            io: new_io(),
            rom,
            rom_clone: rom,
            unusable_ram: [0u8; UNUSABLE_RAM_SIZE],
            video_ram: [0u8; VIDEO_RAM_SIZE],
            work_ram: [0u8; WORK_RAM_SIZE],
        }
    }
}

// Reading
impl core::ops::Index<u16> for Bus {
    type Output = u8;
    fn index(&self, address: u16) -> &Self::Output {
        // TODO: Remove this when I add joypad support
        if address == 0xFF00 {
            return &0xEF;
        }

        match address {
            0x0000..=0x7FFF => &self.rom_clone[address as usize],
            0x8000..=0x9FFF => &self.video_ram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => &self.external_ram[(address - 0xA000) as usize],
            0xC000..=0xDFFF => &self.work_ram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => &self.work_ram[(address - 0xE000) as usize],
            0xFE00..=0xFE9F => &self.eom[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => &self.unusable_ram[(address - 0xFEA0) as usize],
            0xFF00..=0xFF7F => &self.io[(address - IO_START as u16) as usize],
            0xFF80..=0xFFFE => &self.high_ram[(address - 0xFF80) as usize],
            0xFFFF => &self.ie,
        }
    }
}

// Writing
impl core::ops::IndexMut<u16> for Bus {
    fn index_mut(&mut self, address: u16) -> &mut Self::Output {
        match address {
            0x0000..=0x7FFF => &mut self.rom_clone[address as usize],
            0x8000..=0x9FFF => &mut self.video_ram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => &mut self.external_ram[(address - 0xA000) as usize],
            0xC000..=0xDFFF => &mut self.work_ram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => &mut self.work_ram[(address - 0xE000) as usize],
            0xFE00..=0xFE9F => &mut self.eom[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => &mut self.unusable_ram[(address - 0xFEA0) as usize],
            0xFF00..=0xFF7F => &mut self.io[(address - IO_START as u16) as usize],
            0xFF80..=0xFFFE => &mut self.high_ram[(address - 0xFF80) as usize],
            0xFFFF => &mut self.ie,
        }
    }
}

impl Bus {
    /// Returns the byte X times after the `PC` register
    pub(crate) fn next(&self, offset: u16, registers: &Registers) -> u8 {
        self[registers.pc.wrapping_add(offset)]
    }

    /// Returns the byte after the `PC` register
    pub(crate) fn next_one(&self, registers: &Registers) -> u8 {
        self.next(1, registers)
    }

    /// Returns the next two bytes from the `PC` register in little endian format
    pub(crate) fn next_two(&self, registers: &Registers) -> u16 {
        merge_two_u8s_into_u16(self.next(2, registers), self.next(1, registers))
    }
}
