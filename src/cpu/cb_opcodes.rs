use crate::GameBoy;

use super::{Bytes, Cycles};

impl GameBoy {
    pub(crate) fn interpret_cb_opcode(&mut self, opcode: u8) -> (Bytes, Cycles) {
        match opcode {
            _ => panic!("Opcode 0xcb{:x} not implemented yet", opcode),
        }
    }
}
