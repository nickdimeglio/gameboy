#[cfg(test)]
mod tests {
    use crate::gameboy::GameBoy;

    #[test]
    fn no_op() {
        let mut gb: GameBoy = GameBoy::new();
        assert_eq!("NOP", gb.cpu.execute(0x00, &mut gb.memory));
    }


    #[test]
    fn loads() {
        let mut gb: GameBoy = GameBoy::new();

        for op in 0x40..0x7F {
            let result = gb.cpu.execute(op, &mut gb.memory);               
            if op != 0x76 {
                assert_eq!(result, "LD X, Y");
            }
        }

    }
}

