#[cfg(test)]
mod tests {
    use std::fs::read;
    use crate::gameboy::GameBoy;

    fn setup() -> (GameBoy, Vec<u8>) {
        let mut gb = GameBoy::new();
        let rom = read("./src/tests/test_bytes.gbc").expect("Invalid test ROM...");
        (gb, rom)
    }

    /*  EIGHT BIT LOADS INTO B
     *
     *
    */

    #[test]
    fn load_B_B() {
        let (mut gb, rom) = setup();

        // LD C, B
        gb.cpu.set_B(0xFF);
        assert_eq!(0xFF, gb.cpu.get_B());
        gb.cpu.execute(0x40, &rom, &mut gb.memory);               
        assert_eq!(0xFF, gb.cpu.get_B());
    }

    #[test]
    fn load_B_C() {
        let (mut gb, rom) = setup();

        // LD C, C
        gb.cpu.set_C(0xAB); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_B());
        gb.cpu.execute(0x41, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_B());
    }


    #[test]
    fn load_B_D() {
        let (mut gb, rom) = setup();

        // LD C, D
        gb.cpu.set_D(0xBA);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_B());
        gb.cpu.execute(0x42, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_B());
    }

    #[test]
    fn load_B_E() {
        let (mut gb, rom) = setup();

        // LD C, E
        gb.cpu.set_E(0xAB);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_B());
        gb.cpu.execute(0x43, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_B());
    }

    #[test]
    fn load_B_H() {
        let (mut gb, rom) = setup();

        // LD C, H
        gb.cpu.set_H(0xBA);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_B());
        gb.cpu.execute(0x44, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_B());
    }

    #[test]
    fn load_B_L() {
        let (mut gb, rom) = setup();

        // LD C, L
        gb.cpu.set_L(0xAB);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_B());
        gb.cpu.execute(0x45, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_B());

    }

    #[test]
    fn load_B_HL() {
        let (mut gb, rom) = setup();

        // LD B, mem[HL]
        assert_ne!(gb.cpu.get_B(), 0xD);
        gb.memory.write(0xABAB, 0xD);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.execute(0x46, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_B(), 0xD);
    }

    #[test]
    fn load_B_A() {
        let (mut gb, rom) = setup();

        // LD C, A
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_B());
        gb.cpu.execute(0x47, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_B());
    }


    /*  EIGHT BIT LOADS INTO C
     *
     *
    */

    #[test]
    fn load_C_B() {
        let (mut gb, rom) = setup();

        // LD C, B
        gb.cpu.set_B(0xFF);
        assert_ne!(gb.cpu.get_B(), gb.cpu.get_C());
        gb.cpu.execute(0x48, &rom, &mut gb.memory);               
        assert_eq!(gb.cpu.get_B(), gb.cpu.get_C());
    }

    #[test]
    fn load_C_C() {
        let (mut gb, rom) = setup();

        // LD C, C
        assert_ne!(0xAB, gb.cpu.get_C());
        gb.cpu.set_C(0xAB); 
        gb.cpu.execute(0x49, &rom, &mut gb.memory);
        assert_eq!(0xAB, gb.cpu.get_C());
    }


    #[test]
    fn load_C_D() {
        let (mut gb, rom) = setup();

        // LD C, D
        gb.cpu.set_D(0xBA);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_C());
        gb.cpu.execute(0x4A, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_C());
    }

    #[test]
    fn load_C_E() {
        let (mut gb, rom) = setup();

        // LD C, E
        gb.cpu.set_E(0xAB);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_C());
        gb.cpu.execute(0x4B, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_C());
    }

    #[test]
    fn load_C_H() {
        let (mut gb, rom) = setup();

        // LD C, H
        gb.cpu.set_H(0xBA);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_C());
        gb.cpu.execute(0x4C, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_C());
    }

    #[test]
    fn load_C_L() {
        let (mut gb, rom) = setup();

        // LD C, L
        gb.cpu.set_L(0xAB);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_C());
        gb.cpu.execute(0x4D, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_C());

    }

    #[test]
    fn load_C_HL() {
        let (mut gb, rom) = setup();

        // LD C, mem[HL]
        assert_ne!(gb.cpu.get_C(), 0xF);
        gb.memory.write(0xABAB, 0xF);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.execute(0x4E, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), 0xF);
    }


    #[test]
    fn load_C_A() {
        let (mut gb, rom) = setup();

        // LD C, A
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_C());
        gb.cpu.execute(0x4F, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_C());
    }


    /*  EIGHT BIT LOADS INTO D
     *
     *
    */

    #[test]
    fn load_D_B() {
        let (mut gb, rom) = setup();

        // LD D, B
        gb.cpu.set_B(0xFF);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_B());
        gb.cpu.execute(0x50, &rom, &mut gb.memory);               
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_B());
    }

    #[test]
    fn load_D_C() {
        let (mut gb, rom) = setup();

        // LD D, C
        gb.cpu.set_C(0xAB); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_D());
        gb.cpu.execute(0x51, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_D());
    }


    #[test]
    fn load_D_D() {
        let (mut gb, rom) = setup();

        // LD D, D
        assert_ne!(gb.cpu.get_D(), 0xBA);
        gb.cpu.set_D(0xBA);
        gb.cpu.execute(0x52, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), 0xBA);
    }

    #[test]
    fn load_D_E() {
        let (mut gb, rom) = setup();

        // LD D, E
        gb.cpu.set_E(0xAB);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_D());
        gb.cpu.execute(0x53, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_D());
    }

    #[test]
    fn load_D_H() {
        let (mut gb, rom) = setup();

        // LD D, H
        gb.cpu.set_H(0xBA);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_D());
        gb.cpu.execute(0x54, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_D());
    }

    #[test]
    fn load_D_L() {
        let (mut gb, rom) = setup();

        // LD D, L
        gb.cpu.set_L(0xAB);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_D());
        gb.cpu.execute(0x55, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_D());

    }

    #[test]
    fn load_D_HL() {
        let (mut gb, rom) = setup();

        // LD D, mem[HL]
        assert_ne!(gb.cpu.get_D(), 0x2);
        gb.memory.write(0xABAB, 0x2);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.execute(0x56, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), 0x2);
    }

    #[test]
    fn load_D_A() {
        let (mut gb, rom) = setup();

        // LD D, A
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_D());
        gb.cpu.execute(0x57, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_D());
    }


    /*  EIGHT BIT LOADS INTO E
     *
     *
    */

    #[test]
    fn load_E_B() {
        let (mut gb, rom) = setup();

        // LD E, B
        gb.cpu.set_B(0xFF);
        assert_ne!(gb.cpu.get_B(), gb.cpu.get_E());
        gb.cpu.execute(0x58, &rom, &mut gb.memory);               
        assert_eq!(gb.cpu.get_B(), gb.cpu.get_E());
    }

    #[test]
    fn load_E_C() {
        let (mut gb, rom) = setup();

        // LD E, C
        gb.cpu.set_C(0x31); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_E());
        gb.cpu.execute(0x59, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_E());
    }


    #[test]
    fn load_E_D() {
        let (mut gb, rom) = setup();

        // LD E, D
        gb.cpu.set_D(0xBA);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_E());
        gb.cpu.execute(0x5A, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_E());
    }

    #[test]
    fn load_E_E() {
        let (mut gb, rom) = setup();

        // LD E, E
        assert_ne!(gb.cpu.get_E(), 0xAB);
        gb.cpu.set_E(0xAB);
        gb.cpu.execute(0x5B, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), 0xAB);
    }

    #[test]
    fn load_E_H() {
        let (mut gb, rom) = setup();

        // LD E, H
        gb.cpu.set_H(0xBA);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_E());
        gb.cpu.execute(0x5C, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_E());
    }

    #[test]
    fn load_E_L() {
        let (mut gb, rom) = setup();

        // LD E, L
        gb.cpu.set_L(0xAB);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_E());
        gb.cpu.execute(0x5D, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_E());

    }

    #[test]
    fn load_E_HL() {
        let (mut gb, rom) = setup();

        // LD E, mem[HL]
        assert_ne!(gb.cpu.get_E(), 0x3);
        gb.memory.write(0xABAB, 0x3);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.execute(0x5E, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), 0x3);
    }

    #[test]
    fn load_E_A() {
        let (mut gb, rom) = setup();

        // LD E, A
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_E());
        gb.cpu.execute(0x5F, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_E());
    }


    /*  EIGHT BIT LOADS INTO H
     *
     *
    */

    #[test]
    fn load_H_B() {
        let (mut gb, rom) = setup();

        // LD H, B
        gb.cpu.set_B(0x10);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_B());
        gb.cpu.execute(0x60, &rom, &mut gb.memory);               
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_B());
    }

    #[test]
    fn load_H_C() {
        let (mut gb, rom) = setup();

        // LD H, C
        gb.cpu.set_C(0x11); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_H());
        gb.cpu.execute(0x61, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_H());
    }


    #[test]
    fn load_H_D() {
        let (mut gb, rom) = setup();

        // LD H, D
        gb.cpu.set_D(0x12);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_H());
        gb.cpu.execute(0x62, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_H());
    }

    #[test]
    fn load_H_E() {
        let (mut gb, rom) = setup();

        // LD H, E
        gb.cpu.set_E(0x13);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_H());
        gb.cpu.execute(0x63, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_H());
    }

    #[test]
    fn load_H_H() {
        let (mut gb, rom) = setup();

        // LD H, H
        assert_ne!(gb.cpu.get_H(), 0xAA);
        gb.cpu.set_H(0xAA);
        gb.cpu.execute(0x64, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), 0xAA);
    }

    #[test]
    fn load_H_L() {
        let (mut gb, rom) = setup();

        // LD H, L
        gb.cpu.set_L(0x15);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_H());
        gb.cpu.execute(0x65, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_H());

    }

    #[test]
    fn load_H_HL() {
        let (mut gb, rom) = setup();

        // LD H, mem[HL]
        assert_ne!(gb.cpu.get_H(), 0x9);
        gb.memory.write(0xABA9, 0x9);
        gb.cpu.set_HL(0xABA9);
        gb.cpu.execute(0x66, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), 0x9);
    }

    #[test]
    fn load_H_A() {
        let (mut gb, rom) = setup();

        // LD H, A
        gb.cpu.set_A(0x16);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_H());
        gb.cpu.execute(0x67, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_H());
    }


    /*  EIGHT BIT LOADS INTO L
     *
     *
    */

    #[test]
    fn load_L_B() {
        let (mut gb, rom) = setup();

        // LD E, B
        gb.cpu.set_B(0x21);
        assert_ne!(gb.cpu.get_B(), gb.cpu.get_L());
        gb.cpu.execute(0x68, &rom, &mut gb.memory);               
        assert_eq!(gb.cpu.get_B(), gb.cpu.get_L());
    }

    #[test]
    fn load_L_C() {
        let (mut gb, rom) = setup();

        // LD E, C
        gb.cpu.set_C(0xAB); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_L());
        gb.cpu.execute(0x69, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_L());
    }


    #[test]
    fn load_L_D() {
        let (mut gb, rom) = setup();

        // LD E, D
        gb.cpu.set_D(0xBA);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_L());
        gb.cpu.execute(0x6A, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_L());
    }

    #[test]
    fn load_L_E() {
        let (mut gb, rom) = setup();

        // LD E, E
        gb.cpu.set_E(0xAB);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_L());
        gb.cpu.execute(0x6B, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_L());
    }

    #[test]
    fn load_L_H() {
        let (mut gb, rom) = setup();

        // LD E, H
        gb.cpu.set_H(0x21);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_L());
        gb.cpu.execute(0x6C, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_L());
    }

    #[test]
    fn load_L_L() {
        let (mut gb, rom) = setup();

        // LD E, L
        assert_ne!(gb.cpu.get_L(), 0xAB);
        gb.cpu.set_L(0xAB);
        gb.cpu.execute(0x6D, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), 0xAB);

    }

    #[test]
    fn load_L_HL() {
        let (mut gb, rom) = setup();

        // LD L, mem[HL]
        assert_ne!(gb.cpu.get_L(), 0xD);
        gb.memory.write(0xABA9, 0xD);
        gb.cpu.set_HL(0xABA9);
        gb.cpu.execute(0x6E, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), 0xD);
    }

    #[test]
    fn load_L_A() {
        let (mut gb, rom) = setup();

        // LD E, A
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_L());
        gb.cpu.execute(0x6F, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_L());
    }

    /*  EIGHT BIT LOADS INTO (HL)
     *
     *
     *
    */

    #[test]
    fn load_HL_B() {
        let (mut gb, rom) = setup();

        // LD HL, B
        gb.memory.write(0xABAB, 0x9);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.set_B(0xF);
        assert_ne!(gb.memory.read(0xABAB), 0xF);
        gb.cpu.execute(0x70, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0xABAB), 0xF);
    }

    #[test]
    fn load_HL_C() {
        let (mut gb, rom) = setup();

        // LD HL, C
        gb.memory.write(0x1010, 0xB);
        gb.cpu.set_HL(0x1010);
        gb.cpu.set_C(0xA);
        assert_ne!(gb.memory.read(0x1010), 0xA);
        gb.cpu.execute(0x71, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0x1010), 0xA);
    }

    #[test]
    fn load_HL_D() {
        let (mut gb, rom) = setup();

        // LD HL, D
        gb.memory.write(0x1212, 0xC);
        gb.cpu.set_HL(0x1212);
        gb.cpu.set_D(0xB);
        assert_ne!(gb.memory.read(0x1212), 0xB);
        gb.cpu.execute(0x72, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0x1212), 0xB);
    }
   
    #[test]
    fn load_HL_E() {
        let (mut gb, rom) = setup();

        // LD HL, E
        gb.memory.write(0x1505, 0xD);
        gb.cpu.set_HL(0x1505);
        gb.cpu.set_E(0xF);
        assert_ne!(gb.memory.read(0x1505), 0xF);
        gb.cpu.execute(0x73, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0x1505), 0xF);
    }

    #[test]
    fn load_HL_H() {
        let (mut gb, rom) = setup();

        // LD HL, H
        gb.memory.write(0xBAAA, 0xAB);
        gb.cpu.set_HL(0xBAAA);
        assert_ne!(gb.memory.read(0xBAAA), 0xBA);
        gb.cpu.execute(0x74, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0xBAAA), 0xBA);
    }

    #[test]
    fn load_HL_L() {
        let (mut gb, rom) = setup();

        // LD HL, L
        gb.memory.write(0xBAAA, 0xAB);
        gb.cpu.set_HL(0xBAAA);
        assert_ne!(gb.memory.read(0xBAAA), 0xAA);
        gb.cpu.execute(0x75, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0xBAAA), 0xAA);
    }

    // TODO: HALT
   
    #[test]
    fn load_HL_A() {
        let (mut gb, rom) = setup();

        // LD HL, A
        gb.memory.write(0xBBBB, 0xAB);
        gb.cpu.set_HL(0xBBBB);
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.memory.read(0xBBBB), 0xFF);
        gb.cpu.execute(0x77, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0xBBBB), 0xFF);
    }
    
    /*  EIGHT BIT LOADS INTO A
     *
     *
    */

    #[test]
    fn load_A_B() {
        let (mut gb, rom) = setup();

        // LD A, B
        gb.cpu.set_B(0xFF);
        assert_ne!(gb.cpu.get_B(), gb.cpu.get_A());
        gb.cpu.execute(0x78, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_B(), gb.cpu.get_A());
    }

    #[test]
    fn load_A_C() {
        let (mut gb, rom) = setup();

        // LD A, C
        gb.cpu.set_C(0xFF);
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_A());
        gb.cpu.execute(0x79, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_A());
    }

    #[test]
    fn load_A_D() {
        let (mut gb, rom) = setup();

        // LD A, D
        gb.cpu.set_D(0xFF);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_A());
        gb.cpu.execute(0x7A, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_A());
    }

    #[test]
    fn load_A_E() {
        let (mut gb, rom) = setup();

        // LD A, E
        gb.cpu.set_E(0xFF);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_A());
        gb.cpu.execute(0x7B, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_A());
    }

    #[test]
    fn load_A_H() {
        let (mut gb, rom) = setup();

        // LD A, H
        gb.cpu.set_H(0xFF);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_A());
        gb.cpu.execute(0x7C, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_A());
    }

    #[test]
    fn load_A_L() {
        let (mut gb, rom) = setup();

        // LD A, L
        gb.cpu.set_L(0xFF);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_A());
        gb.cpu.execute(0x7D, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_A());
    }

    #[test]
    fn load_A_A() {
        let (mut gb, rom) = setup();

        // LD A, A
        assert_ne!(gb.cpu.get_A(), 0x99);
        gb.cpu.set_A(0x99);
        gb.cpu.execute(0x7F, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), 0x99);
    }

    /*  8-bit loads from A
     *
     *
    */


    #[test]
    fn load_indirect_BC_A() {
        let (mut gb, rom) = setup();

        // LD (BC), A
        gb.cpu.set_A(0xFF);
        gb.cpu.set_BC(0xABAB);
        assert_ne!(gb.memory.read(0xABAB), gb.cpu.get_A());
        gb.cpu.execute(0x02, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0xABAB), gb.cpu.get_A());
    }

    #[test]
    fn load_indirect_DE_A() {
        let (mut gb, rom) = setup();

        // LD (DE), A
        gb.cpu.set_A(0xCC);
        gb.cpu.set_DE(0x1111);
        assert_ne!(gb.memory.read(0x1111), gb.cpu.get_A());
        gb.cpu.execute(0x12, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0x1111), gb.cpu.get_A());
    }

    #[test]
    fn load_indirect_HL_inc_A() {
        let (mut gb, rom) = setup();

        // LD (HL+), A
        gb.cpu.set_HL(0x9999);
        gb.cpu.set_A(0xDD);
        assert_ne!(gb.memory.read(0x9999), gb.cpu.get_A());
        assert_ne!(gb.cpu.get_HL(), 0x999A);
        gb.cpu.execute(0x22, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0x9999), gb.cpu.get_A());
        assert_eq!(gb.cpu.get_HL(), 0x999A);
    }

    #[test]
    fn load_indirect_HL_dec_A() {
        let (mut gb, rom) = setup();

        // LD (HL-), A
        gb.cpu.set_HL(0x8888);
        gb.cpu.set_A(0xBB);
        assert_ne!(gb.memory.read(0x8888), gb.cpu.get_A());
        assert_ne!(gb.cpu.get_HL(), 0x8887);
        gb.cpu.execute(0x32, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0x8888), gb.cpu.get_A());
        assert_eq!(gb.cpu.get_HL(), 0x8887);
    }

    #[test]
    fn load_high_A8_A() {
        let (mut gb, rom) = setup();

        // LDH (a8), A
        gb.cpu.set_A(0x99);
        assert_ne!(gb.memory.read(0xFF22), gb.cpu.get_A());
        gb.cpu.set_PC(0x0);
        gb.cpu.execute(0xE0, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0xFF22), gb.cpu.get_A());

    }

    #[test]
    fn load_high_indirect_C_A() {
        let (mut gb, rom) = setup();

        // LD (C), A
        gb.cpu.set_A(0xCC);
        gb.cpu.set_C(0xAB);
        assert_ne!(gb.memory.read(0xFFAB), gb.cpu.get_A());
        gb.cpu.execute(0xE2, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0xFFAB), gb.cpu.get_A());

    }

    #[test]
    fn load_indirect_A16_A() {
        let (mut gb, rom) = setup();

        // LD (a16), A
        gb.cpu.set_PC(0x0);
        gb.cpu.set_A(0xDD);
        assert_ne!(gb.memory.read(0x3322), gb.cpu.get_A());
        gb.cpu.execute(0xEA, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0x3322), gb.cpu.get_A());
    }

}

