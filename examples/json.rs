//! This example is useful to run the
//! [gb json tests](https://github.com/SingleStepTests/sm83). This code is terrible, this
//! is just for testing purposes

use std::{
    fs::{read_dir, read_to_string, File, ReadDir},
    io::Read,
    process::exit,
};

use colored::{ColoredString, Colorize};
use gameman::{
    consts::bus::{IO_SIZE, ROM_SIZE},
    flags::Flags,
    registers::Registers,
    GameBoy,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RegisterSchema {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    ime: u8,
    ram: Vec<(u16, u8)>,
}

#[derive(Serialize, Deserialize)]
struct JsonSchema {
    name: String,
    initial: RegisterSchema,
    #[serde(rename = "final")]
    final_: RegisterSchema,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("You need to specify the JSON test file");
        std::process::exit(1);
    }

    let json_path = args.last().unwrap();
    let json_file = read_json_file(json_path);

    match json_file {
        Test::File(path) => {
            let schemas = string_to_schemas(path);
            run_test(schemas);
        }

        Test::Folder(paths) => {
            let mut paths: Vec<_> = paths.map(|r| r.unwrap()).collect();
            paths.sort_by_key(|dir| dir.path());

            for path in paths {
                let json_string = read_to_string(path.path()).unwrap();
                let schemas = string_to_schemas(json_string);
                run_test(schemas);
            }
        }
    }
}

fn run_test(schemas: Vec<JsonSchema>) {
    for schema in schemas {
        print!("Running test {}...", schema.name.yellow());

        // Creating gameboy
        let mut gameboy = GameBoy::new_from_rom_array([0; ROM_SIZE]);

        // IO must be all zero during tests
        gameboy.bus.io = [0; IO_SIZE];

        // Setting values in ram
        for (address, value) in &schema.initial.ram {
            gameboy.bus[*address] = *value;
        }

        // Setting registers
        gameboy.registers = register_schema_to_registers(&schema.initial);
        gameboy.flags = register_schema_to_flags(&schema.initial);

        // Running the test rom
        gameboy.step();

        let mut were_registers_wrong = false;
        let mut were_flags_wrong = false;

        let final_registers = register_schema_to_registers(&schema.final_);
        let final_flags = register_schema_to_flags(&schema.final_);

        // These are not implemented yet
        gameboy.flags.substraction = false;
        gameboy.flags.half_carry = false;

        if gameboy.registers == final_registers {
            print!("{}", format!(" ✔ Registers are correct ").green());
        } else {
            print!("{}", format!(" ✘ Registers are not correct ").red());
            were_registers_wrong = true;
        }

        if gameboy.flags == final_flags {
            print!("{}", format!("✔ Flags are correct").green());
        } else {
            print!("{}", format!("✘ Flags are not correct").red());
            were_flags_wrong = true;
        }

        println!("");

        if were_flags_wrong || were_registers_wrong {
            println!("\n");
        }

        if were_registers_wrong {
            println!("Your registers:");
            println!(
                "A: {}, B: {}, C: {}, D: {}, E: {}, H: {}, L: {}, SP: {}, PC: {}",
                format!("{:x}", gameboy.registers.a).red(),
                format!("{:x}", gameboy.registers.b).red(),
                format!("{:x}", gameboy.registers.c).red(),
                format!("{:x}", gameboy.registers.d).red(),
                format!("{:x}", gameboy.registers.e).red(),
                format!("{:x}", gameboy.registers.h).red(),
                format!("{:x}", gameboy.registers.l).red(),
                format!("{:x}", gameboy.registers.sp).red(),
                format!("{:x}", gameboy.registers.pc).bold().blue(),
            );

            println!("\nCorrect registers:");
            println!(
                "A: {}, B: {}, C: {}, D: {}, E: {}, H: {}, L: {}, SP: {}, PC: {}",
                format!("{:x}", schema.final_.a).green(),
                format!("{:x}", schema.final_.b).green(),
                format!("{:x}", schema.final_.c).green(),
                format!("{:x}", schema.final_.d).green(),
                format!("{:x}", schema.final_.e).green(),
                format!("{:x}", schema.final_.h).green(),
                format!("{:x}", schema.final_.l).green(),
                format!("{:x}", schema.final_.sp).green(),
                format!("{:x}", schema.final_.pc).bold().blue(),
            );

            println!("");
        }

        if were_flags_wrong {
            println!("Your flags:");
            println!(
                "Zero: {}, Carry: {}",
                bool_to_symbol(gameboy.flags.zero),
                bool_to_symbol(gameboy.flags.carry)
            );

            println!("\nCorrect flags:");
            println!(
                "Zero: {}, Carry: {}",
                bool_to_symbol(final_flags.zero),
                bool_to_symbol(final_flags.carry)
            );

            println!("");
        }

        if were_registers_wrong || were_flags_wrong {
            print_opcodes_from_schema(&schema.initial);
            exit(1);
        }
    }
}

fn print_opcodes_from_schema(schema: &RegisterSchema) {
    for (address, opcode) in &schema.ram {
        println!(
            "Address {}: {}",
            format!("{:x}", address).yellow(),
            format!("{:x}", opcode).green()
        );
    }
}

fn register_schema_to_registers(schema: &RegisterSchema) -> Registers {
    Registers {
        a: schema.a,
        b: schema.b,
        c: schema.c,
        d: schema.d,
        e: schema.e,
        h: schema.h,
        l: schema.l,
        sp: schema.sp,
        pc: schema.pc,
    }
}

fn register_schema_to_flags(schema: &RegisterSchema) -> Flags {
    Flags {
        zero: (schema.f >> 7) != 0,
        carry: ((schema.f & 0b0001_0000) >> 4) != 0,
        half_carry: false,
        substraction: false,
    }
}

fn string_to_schemas(json_string: String) -> Vec<JsonSchema> {
    let schema: Vec<JsonSchema> = serde_json::from_str(&json_string).unwrap();
    schema
}

/// If the Test file given is a file or a folder of tests
enum Test {
    File(String),
    Folder(ReadDir),
}

fn read_json_file(path: &str) -> Test {
    let mut file = File::open(path).unwrap();
    let mut json_string = String::new();

    if file.read_to_string(&mut json_string).is_ok() {
        return Test::File(json_string);
    }

    if let Ok(folder) = read_dir(path) {
        return Test::Folder(folder);
    }

    panic!("Could not read file or folder");
}

fn bool_to_symbol(boolean: bool) -> ColoredString {
    match boolean {
        true => String::from("✔️ ").green(),
        false => String::from("❌").red(),
    }
    .bold()
}
