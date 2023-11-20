//! There are two opcode tables, one of them is prefixed with `0xCB` and one isn't.  
//! For example, `0xCBF7` is a "cb opcode" (this is how I call it), while something like
//! `0xC3` is just an opcode. You can find normal opcodes at `opcodes.rs` and cb opcodes
//! at `cb_opcodes.rs`

// My preferred opcode reference is: https://meganesu.github.io/generate-gb-opcodes/

// You might be wondering why I did not use rust enums to represent all opcodes,
// I originally did that, and it transforms into spaghetti code really quick, and this is
// far more readable in my opinion, both to rust users, and to anyone that doesn't know
// anything about rust

// NAMING CONVENTIONS:
// r -> one byte register
// ra -> register a in particular
// rr -> two byte register
// ii -> the two bytes of immediate data
// i -> the first byte of immediate data
// ram -> a byte from ram

/// The number of bytes an opcode needs, examples:
/// - NOP - 1 byte, since it just takes the "NOP" byte, so every opcode has *at least* 1
/// byte
/// - LD BC, d16 - 2 bytes, since it also requires the byte after the opcode
type Bytes = u8;

/// The amount of "steps" the gameboy needs to execute a specific instruction
type Cycles = u8;

mod cb_opcodes;
mod opcodes;
