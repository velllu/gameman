mod interrupts;
mod opcodes;
mod opcodes_cb;

/// The number of bytes to skip after interpreting the instruction, if the instruction is
/// 2 bytes long we will need to skip 2 bytes
pub type Bytes = u8;

/// The amounts of cycles and instruction takes
pub type Cycles = u8;

pub struct Cpu {
    // TODO: Remove this, to make the code better. Check `interrupts.rs` for more
    // information on why this is needed
    pub(crate) previous_lcd: Option<bool>,

    /// Wheter or not the next instruction is `0xCB` fixed instruction
    pub(crate) is_cb: bool,

    /// IME, standing for "Interrupt Master Enable" is basically a switch on whether
    /// interrupts should be handled or not
    pub ime: bool,
}

impl Cpu {
    pub(crate) fn new() -> Self {
        Self {
            previous_lcd: None,
            is_cb: false,
            ime: false,
        }
    }
}
