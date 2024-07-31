mod instructions;
mod interrupts;
mod opcodes;
mod opcodes_cb;

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
