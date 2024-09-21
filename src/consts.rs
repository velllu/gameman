pub mod bus {
    pub const EOM_SIZE: usize = 160;
    pub const HIGH_RAM_SIZE: usize = 127;
    pub const IO_SIZE: usize = 128;
    pub const IO_START: usize = 0xFF00;
    pub const VIDEO_RAM_SIZE: usize = 8192;
    pub const WORK_RAM_SIZE: usize = 8192;
    pub const UNUSABLE_RAM_SIZE: usize = 96;
    pub const DMA: u16 = 0xFF46;
}

pub mod gpu {
    pub const LCDC: u16 = 0xFF40;
    pub const STAT: u16 = 0xFF41;
    pub const SCY: u16 = 0xFF42;
    pub const SCX: u16 = 0xFF43;
    pub const LY: u16 = 0xFF44;
    pub const LYC: u16 = 0xFF45;
    pub const BGP: u16 = 0xFF47;
    pub const OBP0: u16 = 0xFF48;
    pub const OBP1: u16 = 0xFF49;
    pub const WY: u16 = 0xFF4A;
    pub const WX: u16 = 0xFF4B;
}

pub mod cpu {
    pub const IE: u16 = 0xFFFF;
    pub const IF: u16 = 0xFF0F;
    pub const DIV: u16 = 0xFF04;
}

pub mod display {
    pub const DISPLAY_SIZE_X: usize = 160;
    pub const DISPLAY_SIZE_Y: usize = 144;
}

pub mod joypad {
    pub const JOYP: u16 = 0xFF00;
}
