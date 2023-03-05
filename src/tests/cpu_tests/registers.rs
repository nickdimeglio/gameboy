#[cfg(test)]
mod tests {
    use crate::gameboy::GameBoy;

    #[test]
    fn register_access() {
        let mut gb: GameBoy = GameBoy::new();

        // AF Register
        assert_eq!(0x0000, gb.cpu.get_A());
        gb.cpu.set_A(0xD1);
        gb.cpu.set_F(0x99);
        assert_eq!(0xD1, gb.cpu.get_A());
        assert_eq!(0x99, gb.cpu.get_F());
        gb.cpu.set_AF(0x1D1E);
        assert_eq!(0x1D1E, gb.cpu.get_AF());

        // BC Register
        assert_eq!(0x0000, gb.cpu.get_B());
        assert_eq!(0x0000, gb.cpu.get_C());
        gb.cpu.set_B(0x99);
        gb.cpu.set_C(0x88);
        assert_eq!(0x99, gb.cpu.get_B());
        assert_eq!(0x88, gb.cpu.get_C());
        gb.cpu.set_BC(0xAABB);
        assert_eq!(0xAABB, gb.cpu.get_BC());

        // DE Register
        assert_eq!(0x0000, gb.cpu.get_D());
        assert_eq!(0x0000, gb.cpu.get_E());
        gb.cpu.set_D(0xAA);
        gb.cpu.set_E(0xBB);
        assert_eq!(0xAA, gb.cpu.get_D());
        assert_eq!(0xBB, gb.cpu.get_E());
        gb.cpu.set_DE(0xFFFF);
        assert_eq!(0xFFFF, gb.cpu.get_DE());

        // HL Register
        assert_eq!(0x0000, gb.cpu.get_H());
        assert_eq!(0x0000, gb.cpu.get_L());
        gb.cpu.set_H(0xAB);
        gb.cpu.set_L(0xCD);
        assert_eq!(0xAB, gb.cpu.get_H());
        assert_eq!(0xCD, gb.cpu.get_L());
        gb.cpu.set_HL(0x9898);
        assert_eq!(0x9898, gb.cpu.get_HL());
    }
}
