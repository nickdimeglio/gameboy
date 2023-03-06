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
        assert_eq!("LD X, Y", gb.cpu.execute(0xE2, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0xE1, &rom, &mut gb.memory));
        assert_eq!("LD X, Y", gb.cpu.execute(0xEA, &rom, &mut gb.memory));
    }
}

