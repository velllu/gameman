// Warning: this code is very bad
use std::panic::{catch_unwind, set_hook, AssertUnwindSafe};

use colored::Colorize;
use gameman::GameBoy;

fn main() {
    let mut gb = GameBoy::new_from_rom_array(vec![]);

    let mut working_opcodes = 0;
    let mut total_opcodes = 0;
    let mut working_opcodes_cb = 0;
    let mut total_opcodes_cb = 0;

    // Disable panic messages
    set_hook(Box::new(|_| {}));

    // Non cb opcodes
    for opcode in 0x00..=0xFF {
        // Skip non valid opcodes
        if [
            0xCB, 0xD3, 0xDB, 0xDD, 0xE3, 0xE4, 0xEB, 0xEC, 0xED, 0xF4, 0xFC, 0xFD,
        ]
        .contains(&opcode)
        {
            continue;
        }

        let result = catch_unwind(AssertUnwindSafe(|| {
            gb.cpu
                .interpret_opcode(opcode, &mut gb.flags, &mut gb.registers, &mut gb.bus);
        }));

        if result.is_ok() {
            working_opcodes += 1;
        }

        total_opcodes += 1;
    }

    // cb opcodes
    for opcode in 0x00..=0xFF {
        let result = catch_unwind(AssertUnwindSafe(|| {
            gb.cpu
                .interpret_cb_opcode(opcode, &mut gb.flags, &mut gb.registers, &mut gb.bus);
        }));

        if result.is_ok() {
            working_opcodes_cb += 1;
        }

        total_opcodes_cb += 1;
    }

    let percentage = (working_opcodes * 100) / total_opcodes;
    let percentage_string = format!("{}%", percentage).green();

    let percentage_cb = (working_opcodes_cb * 100) / total_opcodes_cb;
    let percentage_string_cb = format!("{}%", percentage_cb).green();

    let percentage_all =
        ((working_opcodes_cb + working_opcodes) * 100) / (total_opcodes_cb + total_opcodes);
    let percentage_string_all = format!("{}%", percentage_all).green();

    println!("Percentage of implemented opcodes: {}", percentage_string);
    println!(
        "Percentage of implemented CB opcodes: {}",
        percentage_string_cb
    );
    println!("Percentage of non-CB/CB opcodes: {}", percentage_string_all);
}
