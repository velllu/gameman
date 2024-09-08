use crate::{bus::Bus, common::Bit, consts::joypad::JOYP};

pub struct Joypad {
    pub is_a_pressed: bool,
    pub is_b_pressed: bool,
    pub is_select_pressed: bool,
    pub is_start_pressed: bool,
    pub is_right_pressed: bool,
    pub is_left_pressed: bool,
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
}

impl Joypad {
    pub(crate) fn new() -> Self {
        Self {
            is_a_pressed: false,
            is_b_pressed: false,
            is_select_pressed: false,
            is_start_pressed: false,
            is_right_pressed: false,
            is_left_pressed: false,
            is_up_pressed: false,
            is_down_pressed: false,
        }
    }
}

impl Joypad {
    pub(crate) fn to_byte(&self, bus: &Bus) -> u8 {
        // The lower nibble is read only and contains button data
        let mut joyp = bus[JOYP] & 0b11110000;

        // If bit 4 is enabled then we are listening to the directional pad buttons
        if joyp.get_bit(4) {
            joyp |= self.is_right_pressed as u8;
            joyp |= (self.is_left_pressed as u8) << 1;
            joyp |= (self.is_up_pressed as u8) << 2;
            joyp |= (self.is_down_pressed as u8) << 3;
        }

        // If bit 5 is enabled then we are listening to the buttons
        if joyp.get_bit(5) {
            joyp |= self.is_a_pressed as u8;
            joyp |= (self.is_b_pressed as u8) << 1;
            joyp |= (self.is_select_pressed as u8) << 2;
            joyp |= (self.is_start_pressed as u8) << 3;
        }

        // For some reason in the gameboy 0 means pressed and 1 means not pressed, so we
        // need to invert the nibble
        joyp ^ 0b00001111
    }
}
