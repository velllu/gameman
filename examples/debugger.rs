use colored::{ColoredString, Colorize};
use std::{
    fmt::{Binary, LowerHex},
    io::{self, stdin, stdout, Write},
    process::exit,
};

use gameman::{
    consts::gpu::{LCDC, LY, STAT},
    GameBoy,
};

fn ask_input(text: &str) -> String {
    let mut input = String::new();

    print!("{}", text);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Could not read string");

    input
}

// Output functions
fn hex_to_string<T: LowerHex>(hex: T) -> ColoredString {
    format!("{:x}", hex).bold().green()
}

fn bin_to_string<T: Binary>(bin: T) -> ColoredString {
    format!("{:b}", bin).bold().green()
}

fn bool_to_symbol(boolean: bool) -> ColoredString {
    match boolean {
        true => String::from("✔️ ").green(),
        false => String::from("❌").red(),
    }
    .bold()
}

fn pretty_print_gameboy(gameboy: &GameBoy) -> Result<(), io::Error> {
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    writeln!(lock, "")?;

    writeln!(lock, "{}", "Registers".bold().red())?;
    writeln!(
        lock,
        "  A: {}, B: {}, C: {}, D: {}, E: {}, H: {}, L: {}, SP: {}, PC: {}",
        hex_to_string(gameboy.registers.a),
        hex_to_string(gameboy.registers.b),
        hex_to_string(gameboy.registers.c),
        hex_to_string(gameboy.registers.d),
        hex_to_string(gameboy.registers.e),
        hex_to_string(gameboy.registers.h),
        hex_to_string(gameboy.registers.l),
        hex_to_string(gameboy.registers.sp),
        format!("{:x}", gameboy.registers.pc).bold().blue(),
    )?;

    writeln!(lock, "{}", "Flags".bold().red())?;
    writeln!(
        lock,
        "  Zero: {}, Carry: {}, Subtraction: {}, Half Carry: {}, IME: {}",
        bool_to_symbol(gameboy.flags.zero),
        bool_to_symbol(gameboy.flags.carry),
        bool_to_symbol(gameboy.flags.subtraction),
        bool_to_symbol(gameboy.flags.half_carry),
        bool_to_symbol(gameboy.cpu.ime),
    )?;

    writeln!(lock, "{}", "Others".bold().red())?;

    writeln!(
        lock,
        "  First byte of immediate data: {}",
        hex_to_string(gameboy.bus.read(gameboy.registers.pc + 1))
    )?;

    writeln!(
        lock,
        "  Second byte of immediate data: {}",
        hex_to_string(gameboy.bus.read(gameboy.registers.pc + 2))
    )?;

    writeln!(
        lock,
        "  Current opcode: {}",
        hex_to_string(gameboy.bus.read(gameboy.registers.pc))
    )?;

    writeln!(lock, "{}", "Special addresses".bold().red())?;

    writeln!(
        lock,
        "  LY: {} ({})",
        hex_to_string(gameboy.bus.read(LY)),
        bin_to_string(gameboy.bus.read(LY))
    )?;

    writeln!(
        lock,
        "  LCDC: {} ({})",
        hex_to_string(gameboy.bus.read(LCDC)),
        bin_to_string(gameboy.bus.read(LCDC))
    )?;

    writeln!(
        lock,
        "  STAT: {} ({})",
        hex_to_string(gameboy.bus.read(STAT)),
        bin_to_string(gameboy.bus.read(STAT))
    )?;

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("You need to specify the rom file");
        std::process::exit(1);
    }

    let rom_path = args.last().unwrap();
    let mut gameboy = GameBoy::new(&rom_path).unwrap();

    // Input
    #[rustfmt::skip]
    println!("Choose one of the options");
    println!("{}. Run until PC is a certain value", "1".red().bold());
    println!(
        "{}. Run until a specific opcode is executed",
        "2".red().bold()
    );
    println!("Input anything else to just run");

    let input = ask_input("Enter setting: ");
    let mut additional_input: u16 = 0;

    let input = input.as_str().trim();

    // Some options will require additional input
    match input {
        "1" | "2" => {
            additional_input =
                // We need to convert the input to hexadecimal
                u16::from_str_radix(ask_input("Enter additional input: ").trim(), 16).unwrap()
        }

        _ => {}
    }

    // Actually running the emulator
    loop {
        let _ = pretty_print_gameboy(&gameboy);

        // Stopping the emulator
        match input {
            "1" => {
                if gameboy.registers.pc == additional_input {
                    exit(0);
                }
            }

            "2" => {
                if gameboy.bus.read(gameboy.registers.pc) == additional_input as u8 {
                    exit(0);
                }
            }

            _ => {}
        }

        gameboy.step();
    }
}
