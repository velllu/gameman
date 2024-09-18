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
        let mut joyp = bus.read(JOYP) & 0b11110000;

        // The higher nibble instead, is used to select which "group" of buttons to listen
        let are_buttons_enabled = joyp.get_bit(5);
        let is_dpad_enabled = joyp.get_bit(4);

        // Bit 5 tells the GameBoy to listen to the buttons
        if are_buttons_enabled && !is_dpad_enabled {
            joyp |= self.is_right_pressed as u8;
            joyp |= (self.is_left_pressed as u8) << 1;
            joyp |= (self.is_up_pressed as u8) << 2;
            joyp |= (self.is_down_pressed as u8) << 3;
        }

        // Bit 4 tells the GameBoy to listen to the directional pad
        if is_dpad_enabled && !are_buttons_enabled {
            joyp |= self.is_a_pressed as u8;
            joyp |= (self.is_b_pressed as u8) << 1;
            joyp |= (self.is_select_pressed as u8) << 2;
            joyp |= (self.is_start_pressed as u8) << 3;
        }

        // For some reason in the gameboy 0 means pressed and 1 means not pressed, so we
        // need to invert the lower nibble
        joyp ^ 0b00001111
    }
}
