/* 
#[cfg(test)]
mod tests {
    use crate::gameboy::{GameBoy};

    #[test]
    fn read_mem() {
        let mut gb = GameBoy::new();

        assert_ne!(gb.memory.read(0xF0F1), 0x88);
        gb.memory.write(0xF0F1, 0x88);
        assert_eq!(gb.memory.read(0xF0F1), 0x88);
    }
}
*/