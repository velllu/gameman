pub(crate) mod bus {
    pub(crate) const EOM_SIZE: usize = 160;
    pub(crate) const HIGH_RAM_SIZE: usize = 127;
    pub(crate) const IO_SIZE: usize = 128;
    pub(crate) const IO_START: usize = 0xFF00;
    pub(crate) const ROM_SIZE: usize = 32768;
    pub(crate) const VIDEO_RAM_SIZE: usize = 8192;
    pub(crate) const WORK_RAM_SIZE: usize = 8192;
}

pub(crate) mod gpu {
    pub(crate) const LY: u16 = 0xFF44;
}

pub(crate) mod display {
    pub(crate) const DISPLAY_SIZE_X: usize = 160;
    pub(crate) const DISPLAY_SIZE_Y: usize = 144;
}
