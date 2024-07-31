use crate::common::{merge_two_u8s_into_u16, split_u16_into_two_u8s};

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

/// A read only register
pub(crate) trait ReadRegister<T: Copy> {
    fn get(&self) -> T;
}

/// A read and write register
pub(crate) trait ReadWriteRegister<T> {
    fn get(&self) -> T;
    fn set(self, data_type: T);
}

impl ReadRegister<u8> for u8 {
    fn get(&self) -> u8 {
        *self
    }
}

impl ReadWriteRegister<u8> for &mut u8 {
    fn get(&self) -> u8 {
        (**self).get()
    }

    fn set(self, number: u8) {
        *self = number;
    }
}

impl ReadRegister<u16> for (u8, u8) {
    fn get(&self) -> u16 {
        merge_two_u8s_into_u16(self.1, self.0)
    }
}

impl ReadWriteRegister<u16> for (&mut u8, &mut u8) {
    fn get(&self) -> u16 {
        (*self.1, *self.0).get()
    }

    fn set(self, number: u16) {
        let (high, low) = split_u16_into_two_u8s(number);

        *self.0 = high;
        *self.1 = low;
    }
}

impl ReadRegister<u16> for u16 {
    fn get(&self) -> u16 {
        *self
    }
}

impl ReadWriteRegister<u16> for &mut u16 {
    fn get(&self) -> u16 {
        (**self).get()
    }

    fn set(self, number: u16) {
        *self = number;
    }
}
