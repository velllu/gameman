use emulator::GameBoy;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("You need to specify the rom file");
        std::process::exit(1);
    }

    let rom_path = args.last().unwrap();

    let mut gameboy = GameBoy::new(&rom_path).unwrap();

    loop {
        print!("{:?}", gameboy.registers);

        gameboy.step();

        // if gameboy.current_opcode.unwrap() == 0x00 || gameboy.current_opcode.unwrap() == 0xC8 {
        //     let _ = io::stdin().read_line(&mut String::new());
        // }

        println!("OPCODE: {:x}", gameboy.current_opcode.unwrap());
        println!("");
    }
}
