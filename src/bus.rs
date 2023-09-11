use std::{fs::File, io::Read};

use crate::{consts::bus::*, errors::EmuError};

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
    high_ram: [u8; HIGH_RAM_SIZE],
    ie: u8,
    io: [u8; IO_SIZE],
    rom: [u8; ROM_SIZE],
    video_ram: [u8; VIDEO_RAM_SIZE],
    work_ram: [u8; WORK_RAM_SIZE],
}

#[rustfmt::skip]
fn new_io() -> [u8; IO_SIZE] {
    // IO default values
    // Yes, there's no better way to do this
    let mut io = [0u8; IO_SIZE];
    io[0x00] = 0xCF; io[0x02] = 0x7E; io[0x04] = 0x18; io[0x07] = 0xF8; io[0x0F] = 0xE1;
    io[0x10] = 0x80; io[0x11] = 0xBF; io[0x12] = 0xF3; io[0x13] = 0xFF; io[0x14] = 0xBF;
    io[0x16] = 0x3F; io[0x18] = 0xFF; io[0x19] = 0xBF; io[0x1A] = 0x7F; io[0x1B] = 0xFF;
    io[0x1C] = 0x9F; io[0x1D] = 0xFF; io[0x1E] = 0xBF; io[0x20] = 0xFF; io[0x23] = 0xBF;
    io[0x24] = 0x77; io[0x25] = 0xF3; io[0x26] = 0xF1; io[0x40] = 0x91; io[0x41] = 0x81;
    io[0x44] = 0x91; io[0x46] = 0xFF; io[0x47] = 0xFC; io[0x4D] = 0xFF; io[0x4F] = 0xFF;
    io[0x51] = 0xFF; io[0x52] = 0xFF; io[0x53] = 0xFF; io[0x54] = 0xFF; io[0x55] = 0xFF;
    io[0x56] = 0xFF; io[0x68] = 0xFF; io[0x69] = 0xFF; io[0x6A] = 0xFF; io[0x6B] = 0xFF;
    io[0x70] = 0xFF;

    io
}

impl Bus {
    pub(crate) fn new(rom_path: &str) -> Result<Self, EmuError> {
        // ROM loading
        let mut rom = [0u8; ROM_SIZE];

        let mut rom_file = match File::open(rom_path) {
            Ok(rom_file) => rom_file,
            Err(_) => return Err(EmuError::CouldNotFindRom),
        };

        match rom_file.read(&mut rom) {
            Ok(_) => {}
            Err(_) => return Err(EmuError::CouldNotReadRom),
        }

        // Actual returning
        Ok(Self {
            eom: [0u8; EOM_SIZE],
            high_ram: [0u8; HIGH_RAM_SIZE],
            ie: 0u8,
            io: new_io(),
            rom,
            video_ram: [0u8; VIDEO_RAM_SIZE],
            work_ram: [0u8; WORK_RAM_SIZE],
        })
    }

    pub(crate) fn new_from_rom_array(rom: [u8; ROM_SIZE]) -> Self {
        Self {
            eom: [0u8; EOM_SIZE],
            high_ram: [0u8; HIGH_RAM_SIZE],
            ie: 0u8,
            io: new_io(),
            rom,
            video_ram: [0u8; VIDEO_RAM_SIZE],
            work_ram: [0u8; WORK_RAM_SIZE],
        }
    }
}

// Reading
impl core::ops::Index<u16> for Bus {
    type Output = u8;
    fn index(&self, address: u16) -> &Self::Output {
        match address {
            0x8000..=0x9FFF => &self.video_ram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => todo!(), // external ram
            0xC000..=0xDFFF => &self.work_ram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => todo!(), // mirror ram
            0xFE00..=0xFE9F => &self.eom[(address - 0xFE00) as usize],
            0xFF00..=0xFF7F => &self.io[(address - IO_START as u16) as usize],
            0xFF80..=0xFFFE => &self.high_ram[(address - 0xFF80) as usize],
            0xFFFF => &self.ie,
            _ => todo!(),
        }
    }
}

// Writing
impl core::ops::IndexMut<u16> for Bus {
    fn index_mut(&mut self, address: u16) -> &mut Self::Output {
        match address {
            0x8000..=0x9FFF => &mut self.video_ram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => todo!(), // external ram
            0xC000..=0xDFFF => &mut self.work_ram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => todo!(), // mirror ram
            0xFE00..=0xFE9F => &mut self.eom[(address - 0xFE00) as usize],
            0xFF00..=0xFF7F => &mut self.io[(address - IO_START as u16) as usize],
            0xFF80..=0xFFFE => &mut self.high_ram[(address - 0xFF80) as usize],
            0xFFFF => &mut self.ie,
            _ => todo!(),
        }
    }
}

impl Bus {
    /// Returns the opcode at specified address
    pub fn read_from_rom(&self, pc: u16) -> u8 {
        self.rom[pc as usize]
    }
}
