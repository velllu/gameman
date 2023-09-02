use crate::{consts::bus::ROM_SIZE, GameBoy};

/// Creates a `GameBoy` struct, with just one opcode as the rom, for testing porpuses
fn create_gb_from_opcode(opcode: u8) -> GameBoy {
    let mut rom = [0u8; ROM_SIZE];
    rom[0x0100] = opcode;

    GameBoy::new_from_rom_array(rom)
}

fn create_gb_from_opcode_with_immediate_data(
    opcode: u8,
    immediate1: u8,
    immediate2: u8,
) -> GameBoy {
    let mut rom = [0u8; ROM_SIZE];
    rom[0x0100] = opcode;
    rom[0x0101] = immediate1;
    rom[0x0102] = immediate2;

    GameBoy::new_from_rom_array(rom)
}

mod cpu {
    use super::{create_gb_from_opcode, create_gb_from_opcode_with_immediate_data};

    #[test]
    fn increment() {
        let mut gb = create_gb_from_opcode(0x04);
        gb.step();

        assert_eq!(0x01, gb.registers.b);
        assert_eq!(false, gb.flags.zero);
    }

    #[test]
    fn decrement() {
        let mut gb = create_gb_from_opcode(0x25);
        gb.step();

        assert_eq!(0x00, gb.registers.h);
        assert_eq!(true, gb.flags.zero);
    }

    #[test]
    fn load_r_into_r() {
        let mut gb = create_gb_from_opcode(0x4C);
        gb.step();

        assert_eq!(0x01, gb.registers.c);
    }

    #[test]
    fn load_ii_into_rr() {
        let mut gb = create_gb_from_opcode_with_immediate_data(0x31, 0x42, 0x69);
        gb.step();

        assert_eq!(0x6942, gb.registers.sp);
    }

    #[test]
    fn xor_r() {
        let mut gb = create_gb_from_opcode(0xAC);
        gb.step();

        assert_eq!(0, gb.registers.h);
    }

    // Jumps vary so much, so it's better to have more tests
    mod jumps {
        #[test]
        fn c3() {
            let mut gb = super::create_gb_from_opcode_with_immediate_data(0xC3, 0x42, 0x69);
            gb.step();

            assert_eq!(0x6942, gb.registers.pc);
        }

        #[test]
        fn ca() {
            let mut gb = super::create_gb_from_opcode_with_immediate_data(0xCA, 0x42, 0x69);
            gb.step();

            // this should not jump at all
            assert_eq!(0x0103, gb.registers.pc);
        }
    }
}
