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
        if gameboy.is_cb {
            println!("CB opcode:")
        }

        println!("{:?}", gameboy.registers);

        gameboy.step();
    }
}
