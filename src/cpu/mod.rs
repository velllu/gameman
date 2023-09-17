//! There are two opcode tables, one of them is prefixed with `0xCB` and one isn't.  
//! For example, `0xCBF7` is a "cb opcode" (this is how I call it), while something like
//! `0xC3` is just an opcode. You can find normal opcodes at `opcodes.rs` and cb opcodes
//! at `cb_opcodes.rs`

/// The number of bytes an opcode needs, examples:
/// - NOP - 1 byte, since it just takes the "NOP" byte, so every opcode has *at least* 1
/// byte
/// - LD BC, d16 - 2 bytes, since it also requires the byte after the opcode
type Bytes = u8;

/// The amount of "steps" the gameboy needs to execute a specific instruction
type Cycles = u8;

mod cb_opcodes;
mod opcodes;
