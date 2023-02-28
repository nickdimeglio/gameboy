#![allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::gameboy::{GameBoy};

    #[test]
    fn register_access() {
        let mut gb: GameBoy = GameBoy::new();

        // AF Register
        assert_eq!(0x0000, gb.cpu.get_A());
        gb.cpu.set_A(0xD1);
        gb.cpu.set_F(0x99);
        assert_eq!(0xD1, gb.cpu.get_A());
        assert_eq!(0x99, gb.cpu.get_F());

        // BC Register
        assert_eq!(0x0000, gb.cpu.get_B());
        assert_eq!(0x0000, gb.cpu.get_C());
        gb.cpu.set_B(0x99);
        gb.cpu.set_C(0x88);
        assert_eq!(0x99, gb.cpu.get_B());
        assert_eq!(0x88, gb.cpu.get_C());

        // DE Register
        assert_eq!(0x0000, gb.cpu.get_D());
        assert_eq!(0x0000, gb.cpu.get_E());
        gb.cpu.set_D(0xAA);
        gb.cpu.set_E(0xBB);
        assert_eq!(0xAA, gb.cpu.get_D());
        assert_eq!(0xBB, gb.cpu.get_E());

        // HL Register
        assert_eq!(0x0000, gb.cpu.get_H());
        assert_eq!(0x0000, gb.cpu.get_L());
        gb.cpu.set_H(0xAB);
        gb.cpu.set_L(0xCD);
        assert_eq!(0xAB, gb.cpu.get_H());
        assert_eq!(0xCD, gb.cpu.get_L());
    }

    #[test]
    fn no_op() {
        let mut gb: GameBoy = GameBoy::new();
        assert_eq!("NOP", gb.cpu.execute(0x00, &mut gb.memory));
    }

    /*  EIGHT BIT LOADS INTO B
     *
     *
    */

    #[test]
    fn load_B_B() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, B
        gb.cpu.set_B(0xFF);
        assert_eq!(0xFF, gb.cpu.get_B());
        gb.cpu.execute(0x40, &mut gb.memory);               
        assert_eq!(0xFF, gb.cpu.get_B());
    }

    #[test]
    fn load_B_C() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, C
        gb.cpu.set_C(0xAB); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_B());
        gb.cpu.execute(0x41, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_B());
    }


    #[test]
    fn load_B_D() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, D
        gb.cpu.set_D(0xBA);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_B());
        gb.cpu.execute(0x42, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_B());
    }

    #[test]
    fn load_B_E() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, E
        gb.cpu.set_E(0xAB);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_B());
        gb.cpu.execute(0x43, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_B());
    }

    #[test]
    fn load_B_H() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, H
        gb.cpu.set_H(0xBA);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_B());
        gb.cpu.execute(0x44, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_B());
    }

    #[test]
    fn load_B_L() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, L
        gb.cpu.set_L(0xAB);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_B());
        gb.cpu.execute(0x45, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_B());

    }

    // TODO:    LD B, HL

    #[test]
    fn load_B_A() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, A
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_B());
        gb.cpu.execute(0x47, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_B());
    }


    /*  EIGHT BIT LOADS INTO C
     *
     *
    */

    #[test]
    fn load_C_B() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, B
        gb.cpu.set_B(0xFF);
        assert_ne!(gb.cpu.get_B(), gb.cpu.get_C());
        gb.cpu.execute(0x48, &mut gb.memory);               
        assert_eq!(gb.cpu.get_B(), gb.cpu.get_C());
    }

    #[test]
    fn load_C_C() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, C
        assert_ne!(0xAB, gb.cpu.get_C());
        gb.cpu.set_C(0xAB); 
        gb.cpu.execute(0x49, &mut gb.memory);
        assert_eq!(0xAB, gb.cpu.get_C());
    }


    #[test]
    fn load_C_D() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, D
        gb.cpu.set_D(0xBA);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_C());
        gb.cpu.execute(0x4A, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_C());
    }

    #[test]
    fn load_C_E() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, E
        gb.cpu.set_E(0xAB);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_C());
        gb.cpu.execute(0x4B, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_C());
    }

    #[test]
    fn load_C_H() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, H
        gb.cpu.set_H(0xBA);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_C());
        gb.cpu.execute(0x4C, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_C());
    }

    #[test]
    fn load_C_L() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, L
        gb.cpu.set_L(0xAB);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_C());
        gb.cpu.execute(0x4D, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_C());

    }

    // TODO LD C, HL

    #[test]
    fn load_C_A() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, A
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_C());
        gb.cpu.execute(0x4F, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_C());
    }


    /*  EIGHT BIT LOADS INTO D
     *
     *
    */

    #[test]
    fn load_D_B() {
        let mut gb: GameBoy = GameBoy::new();

        // LD D, B
        gb.cpu.set_B(0xFF);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_B());
        gb.cpu.execute(0x50, &mut gb.memory);               
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_B());
    }

    #[test]
    fn load_D_C() {
        let mut gb: GameBoy = GameBoy::new();

        // LD D, C
        gb.cpu.set_C(0xAB); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_D());
        gb.cpu.execute(0x51, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_D());
    }


    #[test]
    fn load_D_D() {
        let mut gb: GameBoy = GameBoy::new();

        // LD D, D
        assert_ne!(gb.cpu.get_D(), 0xBA);
        gb.cpu.set_D(0xBA);
        gb.cpu.execute(0x52, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), 0xBA);
    }

    #[test]
    fn load_D_E() {
        let mut gb: GameBoy = GameBoy::new();

        // LD D, E
        gb.cpu.set_E(0xAB);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_D());
        gb.cpu.execute(0x53, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_D());
    }

    #[test]
    fn load_D_H() {
        let mut gb: GameBoy = GameBoy::new();

        // LD D, H
        gb.cpu.set_H(0xBA);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_D());
        gb.cpu.execute(0x54, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_D());
    }

    #[test]
    fn load_D_L() {
        let mut gb: GameBoy = GameBoy::new();

        // LD D, L
        gb.cpu.set_L(0xAB);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_D());
        gb.cpu.execute(0x55, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_D());

    }

    // TODO LD D, HL

    #[test]
    fn load_D_A() {
        let mut gb: GameBoy = GameBoy::new();

        // LD D, A
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_D());
        gb.cpu.execute(0x57, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_D());
    }


    /*  EIGHT BIT LOADS INTO E
     *
     *
    */

    #[test]
    fn load_E_B() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, B
        gb.cpu.set_B(0xFF);
        assert_ne!(gb.cpu.get_B(), gb.cpu.get_E());
        gb.cpu.execute(0x58, &mut gb.memory);               
        assert_eq!(gb.cpu.get_B(), gb.cpu.get_E());
    }

    #[test]
    fn load_E_C() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, C
        gb.cpu.set_C(0x31); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_E());
        gb.cpu.execute(0x59, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_E());
    }


    #[test]
    fn load_E_D() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, D
        gb.cpu.set_D(0xBA);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_E());
        gb.cpu.execute(0x5A, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_E());
    }

    #[test]
    fn load_E_E() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, E
        assert_ne!(gb.cpu.get_E(), 0xAB);
        gb.cpu.set_E(0xAB);
        gb.cpu.execute(0x5B, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), 0xAB);
    }

    #[test]
    fn load_E_H() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, H
        gb.cpu.set_H(0xBA);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_E());
        gb.cpu.execute(0x5C, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_E());
    }

    #[test]
    fn load_E_L() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, L
        gb.cpu.set_L(0xAB);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_E());
        gb.cpu.execute(0x5D, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_E());

    }

    // TODO LD E, HL

    #[test]
    fn load_E_A() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, A
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_E());
        gb.cpu.execute(0x5F, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_E());
    }


    /*  EIGHT BIT LOADS INTO H
     *
     *
    */

    #[test]
    fn load_H_B() {
        let mut gb: GameBoy = GameBoy::new();

        // LD H, B
        gb.cpu.set_B(0x10);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_B());
        gb.cpu.execute(0x60, &mut gb.memory);               
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_B());
    }

    #[test]
    fn load_H_C() {
        let mut gb: GameBoy = GameBoy::new();

        // LD H, C
        gb.cpu.set_C(0x11); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_H());
        gb.cpu.execute(0x61, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_H());
    }


    #[test]
    fn load_H_D() {
        let mut gb: GameBoy = GameBoy::new();

        // LD H, D
        gb.cpu.set_D(0x12);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_H());
        gb.cpu.execute(0x62, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_H());
    }

    #[test]
    fn load_H_E() {
        let mut gb: GameBoy = GameBoy::new();

        // LD H, E
        gb.cpu.set_E(0x13);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_H());
        gb.cpu.execute(0x63, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_H());
    }

    #[test]
    fn load_H_H() {
        let mut gb: GameBoy = GameBoy::new();

        // LD H, H
        assert_ne!(gb.cpu.get_H(), 0xAA);
        gb.cpu.set_H(0xAA);
        gb.cpu.execute(0x64, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), 0xAA);
    }

    #[test]
    fn load_H_L() {
        let mut gb: GameBoy = GameBoy::new();

        // LD H, L
        gb.cpu.set_L(0x15);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_H());
        gb.cpu.execute(0x65, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_H());

    }

    // TODO: LD H, HL

    #[test]
    fn load_H_A() {
        let mut gb: GameBoy = GameBoy::new();

        // LD H, A
        gb.cpu.set_A(0x16);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_H());
        gb.cpu.execute(0x67, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_H());
    }


    /*  EIGHT BIT LOADS INTO L
     *
     *
    */

    #[test]
    fn load_L_B() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, B
        gb.cpu.set_B(0x21);
        assert_ne!(gb.cpu.get_B(), gb.cpu.get_L());
        gb.cpu.execute(0x68, &mut gb.memory);               
        assert_eq!(gb.cpu.get_B(), gb.cpu.get_L());
    }

    #[test]
    fn load_L_C() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, C
        gb.cpu.set_C(0xAB); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_L());
        gb.cpu.execute(0x69, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_L());
    }


    #[test]
    fn load_L_D() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, D
        gb.cpu.set_D(0xBA);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_L());
        gb.cpu.execute(0x6A, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_L());
    }

    #[test]
    fn load_L_E() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, E
        gb.cpu.set_E(0xAB);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_L());
        gb.cpu.execute(0x6B, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_L());
    }

    #[test]
    fn load_L_H() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, H
        gb.cpu.set_H(0x21);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_L());
        gb.cpu.execute(0x6C, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_L());
    }

    #[test]
    fn load_L_L() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, L
        assert_ne!(gb.cpu.get_L(), 0xAB);
        gb.cpu.set_L(0xAB);
        gb.cpu.execute(0x6D, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), 0xAB);

    }

    // TODO:    LD L, HL

    #[test]
    fn load_L_A() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, A
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_L());
        gb.cpu.execute(0x6F, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_L());
    }

    /*  EIGHT BIT LOADS INTO (HL)
     *
     *
     *
    */

    // TODO: LD (HL), B
    // TODO: LD (HL), C
    // TODO: LD (HL), D
    // TODO: LD (HL), E
    // TODO: LD (HL), H
    // TODO: LD (HL), L
    // TODO: HALT
    // TODO: LD (HL), A
    
    
    /*  EIGHT BIT LOADS INTO A
     *
     *
    */

    fn load_A_B() {
        let mut gb: GameBoy = GameBoy::new();

        // LD A, B
        gb.cpu.set_B(0xFF);
        assert_ne!(gb.cpu.get_B(), gb.cpu.get_A());
        gb.cpu.execute(0x78, &mut gb.memory);
        assert_eq!(gb.cpu.get_B(), gb.cpu.get_A());
    }

    fn load_A_C() {
        let mut gb: GameBoy = GameBoy::new();

        // LD A, C
        gb.cpu.set_C(0xFF);
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_A());
        gb.cpu.execute(0x79, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_A());
    }

    fn load_A_D() {
        let mut gb: GameBoy = GameBoy::new();

        // LD A, D
        gb.cpu.set_D(0xFF);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_A());
        gb.cpu.execute(0x7A, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_A());
    }

    fn load_A_E() {
        let mut gb: GameBoy = GameBoy::new();

        // LD A, E
        gb.cpu.set_B(0xFF);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_A());
        gb.cpu.execute(0x7B, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_A());
    }

    fn load_A_H() {
        let mut gb: GameBoy = GameBoy::new();

        // LD A, H
        gb.cpu.set_H(0xFF);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_A());
        gb.cpu.execute(0x7C, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_A());
    }

    fn load_A_L() {
        let mut gb: GameBoy = GameBoy::new();

        // LD A, L
        gb.cpu.set_L(0xFF);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_A());
        gb.cpu.execute(0x7D, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_A());
    }

    // TODO: LD A, (HL)

    fn load_A_A() {
        let mut gb: GameBoy = GameBoy::new();

        // LD A, A
        assert_ne!(gb.cpu.get_A(), 0x99);
        gb.cpu.set_A(0x99);
        gb.cpu.execute(0x7F, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), 0x99);
    }

}
