use emulator::GameBoy;

fn main() {
    let mut gameboy =
        GameBoy::new("/home/vellu/Projects/gameboy-emu/emulator/roms/Tetris.gb").unwrap();

    loop {
        if gameboy.is_cb {
            println!("CB opcode:")
        }

        println!("{:?}", gameboy.registers);

        gameboy.step();
    }
}
