use crate::{
    common::{merge_two_u8s_into_u16, split_u16_into_two_u8s, Bit},
    flags::Flags,
};

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
pub(crate) trait ReadRegister<T> {
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

impl ReadRegister<u8> for Flags {
    fn get(&self) -> u8 {
        let mut new_byte = 0;

        new_byte |= (self.zero as u8) << 7;
        new_byte |= (self.substraction as u8) << 6;
        new_byte |= (self.half_carry as u8) << 5;
        new_byte |= (self.carry as u8) << 4;

        new_byte
    }
}

impl ReadWriteRegister<u8> for &mut Flags {
    fn get(&self) -> u8 {
        (**self).get()
    }

    fn set(self, number: u8) {
        self.zero = number.get_bit(7);
        self.substraction = number.get_bit(6);
        self.half_carry = number.get_bit(5);
        self.carry = number.get_bit(4);
    }
}

impl<R: ReadRegister<u8>, R2: ReadRegister<u8>> ReadRegister<u16> for (R, R2) {
    fn get(&self) -> u16 {
        merge_two_u8s_into_u16(self.0.get(), self.1.get())
    }
}

impl<R: ReadWriteRegister<u8>, R2: ReadWriteRegister<u8>> ReadWriteRegister<u16> for (R, R2) {
    fn get(&self) -> u16 {
        merge_two_u8s_into_u16(self.0.get(), self.1.get())
    }

    fn set(self, number: u16) {
        let (high, low) = split_u16_into_two_u8s(number);

        self.0.set(high);
        self.1.set(low);
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
