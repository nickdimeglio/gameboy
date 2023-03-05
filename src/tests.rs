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

    #[test]
    fn memory_access() {
        let mut gb: GameBoy = GameBoy::new();
        assert_eq!(0x0, gb.memory.read(0xF0F0));
        assert_eq!(0x0, gb.memory.read(0xF1F2));
        gb.memory.write(0xF0F0, 0xA);
        gb.memory.write(0xF1F2, 0xB);
        assert_eq!(0xA, gb.memory.read(0xF0F0));
        assert_eq!(0xB, gb.memory.read(0xF1F2));

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

    #[test]
    fn load_B_HL() {
        let mut gb: GameBoy = GameBoy::new();

        // LD B, mem[HL]
        assert_ne!(gb.cpu.get_B(), 0xD);
        gb.memory.write(0xABAB, 0xD);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.execute(0x46, &mut gb.memory);
        assert_eq!(gb.cpu.get_B(), 0xD);
    }

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

    #[test]
    fn load_C_HL() {
        let mut gb: GameBoy = GameBoy::new();

        // LD C, mem[HL]
        assert_ne!(gb.cpu.get_C(), 0xF);
        gb.memory.write(0xABAB, 0xF);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.execute(0x4E, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), 0xF);
    }


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

    #[test]
    fn load_D_HL() {
        let mut gb: GameBoy = GameBoy::new();

        // LD D, mem[HL]
        assert_ne!(gb.cpu.get_D(), 0x2);
        gb.memory.write(0xABAB, 0x2);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.execute(0x56, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), 0x2);
    }

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

    #[test]
    fn load_E_HL() {
        let mut gb: GameBoy = GameBoy::new();

        // LD E, mem[HL]
        assert_ne!(gb.cpu.get_E(), 0x3);
        gb.memory.write(0xABAB, 0x3);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.execute(0x5E, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), 0x3);
    }

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

    #[test]
    fn load_H_HL() {
        let mut gb: GameBoy = GameBoy::new();

        // LD H, mem[HL]
        assert_ne!(gb.cpu.get_H(), 0x9);
        gb.memory.write(0xABA9, 0x9);
        gb.cpu.set_HL(0xABA9);
        gb.cpu.execute(0x66, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), 0x9);
    }

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

    #[test]
    fn load_L_HL() {
        let mut gb: GameBoy = GameBoy::new();

        // LD L, mem[HL]
        assert_ne!(gb.cpu.get_L(), 0xD);
        gb.memory.write(0xABA9, 0xD);
        gb.cpu.set_HL(0xABA9);
        gb.cpu.execute(0x6E, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), 0xD);
    }

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

    #[test]
    fn load_HL_B() {
        let mut gb: GameBoy = GameBoy::new();

        // LD HL, B
        gb.memory.write(0xABAB, 0x9);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.set_B(0xF);
        assert_ne!(gb.memory.read(0xABAB), 0xF);
        gb.cpu.execute(0x70, &mut gb.memory);
        assert_eq!(gb.memory.read(0xABAB), 0xF);
    }

    #[test]
    fn load_HL_C() {
        let mut gb: GameBoy = GameBoy::new();

        // LD HL, C
        gb.memory.write(0x1010, 0xB);
        gb.cpu.set_HL(0x1010);
        gb.cpu.set_C(0xA);
        assert_ne!(gb.memory.read(0x1010), 0xA);
        gb.cpu.execute(0x71, &mut gb.memory);
        assert_eq!(gb.memory.read(0x1010), 0xA);
    }

    #[test]
    fn load_HL_D() {
        let mut gb: GameBoy = GameBoy::new();

        // LD HL, D
        gb.memory.write(0x1212, 0xC);
        gb.cpu.set_HL(0x1212);
        gb.cpu.set_D(0xB);
        assert_ne!(gb.memory.read(0x1212), 0xB);
        gb.cpu.execute(0x72, &mut gb.memory);
        assert_eq!(gb.memory.read(0x1212), 0xB);
    }
   
    #[test]
    fn load_HL_E() {
        let mut gb: GameBoy = GameBoy::new();

        // LD HL, E
        gb.memory.write(0x1505, 0xD);
        gb.cpu.set_HL(0x1505);
        gb.cpu.set_E(0xF);
        assert_ne!(gb.memory.read(0x1505), 0xF);
        gb.cpu.execute(0x73, &mut gb.memory);
        assert_eq!(gb.memory.read(0x1505), 0xF);
    }

    #[test]
    fn load_HL_H() {
        let mut gb: GameBoy = GameBoy::new();

        // LD HL, H
        gb.memory.write(0xBAAA, 0xAB);
        gb.cpu.set_HL(0xBAAA);
        assert_ne!(gb.memory.read(0xBAAA), 0xBA);
        gb.cpu.execute(0x74, &mut gb.memory);
        assert_eq!(gb.memory.read(0xBAAA), 0xBA);
    }

    #[test]
    fn load_HL_L() {
        let mut gb: GameBoy = GameBoy::new();

        // LD HL, L
        gb.memory.write(0xBAAA, 0xAB);
        gb.cpu.set_HL(0xBAAA);
        assert_ne!(gb.memory.read(0xBAAA), 0xAA);
        gb.cpu.execute(0x75, &mut gb.memory);
        assert_eq!(gb.memory.read(0xBAAA), 0xAA);
    }

    // TODO: HALT
   
    #[test]
    fn load_HL_A() {
        let mut gb: GameBoy = GameBoy::new();

        // LD HL, A
        gb.memory.write(0xBBBB, 0xAB);
        gb.cpu.set_HL(0xBBBB);
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.memory.read(0xBBBB), 0xFF);
        gb.cpu.execute(0x77, &mut gb.memory);
        assert_eq!(gb.memory.read(0xBBBB), 0xFF);
    }
    
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

