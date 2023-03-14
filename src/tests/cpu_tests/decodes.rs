#[cfg(test)]
mod tests {
    use std::fs::read;
    use crate::gameboy::GameBoy;

    fn setup() -> (GameBoy, Vec<u8>) {
        let mut gb = GameBoy::new();
        let rom = read("./src/tests/test_bytes.gbc").expect("Invalid test ROM...");
        (gb, rom)
    }

    #[test]
    fn no_op() {
        let (mut gb, rom) = setup();
        assert_eq!("NOP", gb.cpu.execute(0x00, &rom, &mut gb.memory));
    }


    #[test]
    fn loads() {
        let (mut gb,  rom) = setup();

        for op in 0x40..0x7F {
            let result = gb.cpu.execute(op, &rom, &mut gb.memory);               
            if op != 0x76 {
                assert_eq!(result, "LD X, Y");
            }
        }
    }

    #[test]
    fn loads_from_a() {
        let (mut gb,  rom) = setup();
        gb.cpu.set_PC(0x2);
        assert_eq!("LD X, Y", gb.cpu.execute(0x02, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x12, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x22, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x32, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0xE0, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0xE2, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0xEA, &rom, &mut gb.memory));
    }

    #[test]
    fn loads_from_indirect() {
        let (mut gb, rom) = setup();
        gb.cpu.set_PC(0x2);
        assert_eq!("LD X, Y", gb.cpu.execute(0x06, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x16, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x26, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x36, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x0A, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x1A, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x2A, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x3A, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x0E, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x1E, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x2E, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0x3E, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0xF0, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0xF2, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0xFA, &rom, &mut gb.memory));

    }

    #[test]
    fn loads_16() {
        let (mut gb, rom) = setup();
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0x01, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0x08, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0x11, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0x21, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0x31, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0xC1, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0xD1, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0xE1, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0xF1, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0xC5, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0xD5, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0xE5, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0xF5, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0xF8, &rom, &mut gb.memory));
        assert_eq!("LD X, Y (16)", gb.cpu.execute(0xF9, &rom, &mut gb.memory));
    }
}

