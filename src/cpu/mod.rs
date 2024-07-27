mod instructions;
mod interrupts;
mod opcodes;

pub struct Cpu {
    // TODO: Remove this, to make the code better. Check `interrupts.rs` for more
    // information on why this is needed
    pub(crate) previous_lcd: Option<bool>,

    /// IME, standing for "Interrupt Master Enable" is basically a switch on whether
    /// interrupts should be handled or not
    pub ime: bool,
}

impl Cpu {
    pub(crate) fn new() -> Self {
        Self {
            previous_lcd: None,
            ime: false,
        }
    }
}
