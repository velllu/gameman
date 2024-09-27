use std::{error::Error, fmt::Display, fs::File, io::Read};

use mbc1::Mbc1;
use mbc_no::NoMbc;

use crate::{
    common::merge_two_u8s_into_u16,
    consts::{bus::*, cpu::DIV},
    registers::Registers,
};

mod mbc1;
mod mbc_no;

pub trait Mbc: Send {
    fn new(rom: Vec<u8>) -> Self
    where
        Self: Sized;

    /// Get byte in rom from address `0x0000` to `0x3FFF`
    fn get_rom_section_0(&self, address: u16) -> u8;

    /// Get byte in rom from address `0x4000` to `0x7FFF`
    fn get_rom_section_1(&self, address: u16) -> u8;

    /// Get byte in ram from address `0xA000` to `0xBFFF`
    fn get_external_ram(&self, address: u16) -> u8;

    /// Like the `get_external_ram` function but it sets a value in ram
    fn set_external_ram(&mut self, address: u16, value: u8);

    /// This is triggered when trying to write in ram, sometimes this is used to signal
    /// some kind of event in cartridges
    fn signal_rom_write(&mut self, address: u16, value: u8);

    /// Writes to rom without signaling anything, this is the only way to write to rom and
    /// it's used in examples
    fn direct_rom_write(&mut self, address: u16, value: u8);
}

pub struct Bus {
    pub mbc: Box<dyn Mbc>,
    pub video_ram: [u8; VIDEO_RAM_SIZE],
    pub work_ram: [u8; WORK_RAM_SIZE],
    pub eom: [u8; EOM_SIZE],
    pub unusable_ram: [u8; UNUSABLE_RAM_SIZE], // TODO: This should not be usuable
    pub io: [u8; IO_SIZE],
    pub high_ram: [u8; HIGH_RAM_SIZE],
    pub ie: u8,

    /// Gets true when the user writes to OAM DMA register
    pub needs_to_dispatch_oam_dma: bool,

    /// Gets true when the emulator writes to DIV, this means that we must reset the div
    /// register internal cycle counter
    pub(crate) needs_to_reset_div_register: bool,
}

impl Bus {
    pub(crate) fn new(rom_path: &str) -> Result<Self, BusError> {
        // ROM loading
        let mut rom: Vec<u8> = Vec::new();

        let mut rom_file = match File::open(rom_path) {
            Ok(rom_file) => rom_file,
            Err(_) => return Err(BusError::CouldNotFindRom),
        };

        match rom_file.read_to_end(&mut rom) {
            Ok(_) => {}
            Err(_) => return Err(BusError::CouldNotReadRom),
        }

        // Actual returning
        Ok(Self {
            mbc: new_mbc(rom),
            video_ram: [0u8; VIDEO_RAM_SIZE],
            work_ram: [0u8; WORK_RAM_SIZE],
            eom: [0u8; EOM_SIZE],
            high_ram: [0u8; HIGH_RAM_SIZE],
            ie: 0u8,
            io: new_io(),
            unusable_ram: [0u8; UNUSABLE_RAM_SIZE],
            needs_to_dispatch_oam_dma: false,
            needs_to_reset_div_register: false,
        })
    }

    pub(crate) fn new_from_rom_array(rom: Vec<u8>) -> Self {
        Self {
            mbc: new_mbc(rom),
            video_ram: [0u8; VIDEO_RAM_SIZE],
            work_ram: [0u8; WORK_RAM_SIZE],
            eom: [0u8; EOM_SIZE],
            high_ram: [0u8; HIGH_RAM_SIZE],
            ie: 0u8,
            io: new_io(),
            unusable_ram: [0u8; UNUSABLE_RAM_SIZE],
            needs_to_dispatch_oam_dma: false,
            needs_to_reset_div_register: false,
        }
    }
}

// Reading
impl Bus {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.mbc.get_rom_section_0(address),
            0x4000..=0x7FFF => self.mbc.get_rom_section_1(address),
            0x8000..=0x9FFF => self.video_ram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => self.mbc.get_external_ram(address - 0xA000),
            0xC000..=0xDFFF => self.work_ram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.work_ram[(address - 0xE000) as usize],
            0xFE00..=0xFE9F => self.eom[(address - 0xFE00) as usize],
            0xFEA0..=0xFEFF => self.unusable_ram[(address - 0xFEA0) as usize],
            0xFF00..=0xFF7F => self.io[(address - IO_START as u16) as usize],
            0xFF80..=0xFFFE => self.high_ram[(address - 0xFF80) as usize],
            0xFFFF => self.ie,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            DMA => {
                self.needs_to_dispatch_oam_dma = true;
                self.io[0x46] = value;
            }

            DIV => {
                self.needs_to_reset_div_register = true;
                self.io[0x04] = 0;
            }

            0x0000..=0x7FFF => self.mbc.signal_rom_write(address, value),
            0x8000..=0x9FFF => self.video_ram[(address - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.mbc.set_external_ram(address - 0xA000, value),
            0xC000..=0xDFFF => self.work_ram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.work_ram[(address - 0xE000) as usize] = value,
            0xFE00..=0xFE9F => self.eom[(address - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => self.unusable_ram[(address - 0xFEA0) as usize] = value,
            0xFF00..=0xFF7F => self.io[(address - IO_START as u16) as usize] = value,
            0xFF80..=0xFFFE => self.high_ram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.ie = value,
        };
    }
}

impl Bus {
    /// Returns the byte X times after the `PC` register
    pub(crate) fn next(&self, offset: u16, registers: &Registers) -> u8 {
        self.read(registers.pc.wrapping_add(offset))
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

impl Bus {
    pub(crate) fn dispatch_oam_transfer(&mut self) {
        let oam_dma_start = (self.read(DMA) as u16) << 8;
        let oam_dma_end = oam_dma_start | 0x9F;
        let difference = oam_dma_end - oam_dma_start;

        for i in 0..difference {
            self.write(0xFE00 + i, self.read(oam_dma_start + i));
        }
    }
}

/// Creates a specific MBC cartridge based on the header data from the rom
pub(crate) fn new_mbc(rom: Vec<u8>) -> Box<dyn Mbc> {
    let mbc_type = *rom.get(0x147).unwrap_or(&0) as usize;

    match mbc_type {
        0 => Box::new(NoMbc::new(rom)),
        1 | 2 | 3 => Box::new(Mbc1::new(rom)),

        _ => Box::new(NoMbc::new(rom)),
    }
}

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

pub(crate) fn vector_to_array<const SIZE: usize>(vec: Vec<u8>) -> [u8; SIZE] {
    let mut array = [0; SIZE];

    for (i, &val) in vec.iter().enumerate() {
        array[i] = val;
    }

    array
}
