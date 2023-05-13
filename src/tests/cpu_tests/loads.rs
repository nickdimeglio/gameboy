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
    fn op_0x01() {
        let (mut gb, rom) = setup();

        // LD BC, d16
        gb.cpu.set_PC(0x0);
        assert_ne!(gb.cpu.get_BC(), 0x3322);
        gb.cpu.execute(0x01, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_BC(), 0x3322);
    }

    #[test]
    fn op_0x06() {
        let (mut gb, rom) = setup();

        // LD B, D8
        gb.cpu.set_PC(0x0); 
        assert_ne!(gb.cpu.get_B(), 0x22);
        gb.cpu.execute(0x06, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_B(), 0x22);
    }

    #[test]
    fn op_0x08() {
        /* LD (a16), SP
           3 byte, 5 cycles
        */
        let (mut gb, rom) = setup();

        gb.cpu.set_PC(0x0); 
        gb.cpu.set_SP(0x9876);
        assert_ne!(gb.memory.read(0x3322), 0x76);
        assert_ne!(gb.memory.read(0x3323), 0x98);
        gb.cpu.execute(0x08, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0x3322), 0x76);
        assert_eq!(gb.memory.read(0x3323), 0x98);
    }

    #[test]
    fn op_0x0A() {
        let (mut gb, rom) = setup();

        // LD A, (BC)
        gb.cpu.set_BC(0xAB12); 
        gb.memory.write(0xAB12, 0xFD);
        assert_ne!(gb.cpu.get_A(), gb.memory.read(gb.cpu.get_BC() as usize));
        gb.cpu.execute(0x0A, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.memory.read(gb.cpu.get_BC() as usize));
    }

    #[test]
    fn op_0x0E() {
        let (mut gb, rom) = setup();

        // LD C, D8
        gb.cpu.set_PC(0x0); 
        assert_ne!(gb.cpu.get_C(), 0x22);
        gb.cpu.execute(0x0E, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), 0x22);
    }

    #[test]
    fn op_0x11() {
        let (mut gb, rom) = setup();

        // LD DE, d16
        gb.cpu.set_PC(0x0);
        assert_ne!(gb.cpu.get_DE(), 0x3322);
        gb.cpu.execute(0x11, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_DE(), 0x3322);
    }

    #[test]
    fn op_0x16() {
        let (mut gb, rom) = setup();

        // LD D D8
        gb.cpu.set_PC(0x0); 
        assert_ne!(gb.cpu.get_D(), 0x22);
        gb.cpu.execute(0x16, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), 0x22);
    }

    #[test]
    fn op_0x1A() {
        let (mut gb, rom) = setup();

        // LD A, (DE)
        gb.cpu.set_DE(0xAB12); 
        gb.memory.write(0xAB12, 0x99);
        assert_ne!(gb.cpu.get_A(), gb.memory.read(gb.cpu.get_DE() as usize));
        gb.cpu.execute(0x1A, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.memory.read(gb.cpu.get_DE() as usize));
    }

    #[test]
    fn op_0x1E() {
        let (mut gb, rom) = setup();

        // LD E, D8
        gb.cpu.set_PC(0x0); 
        assert_ne!(gb.cpu.get_E(), 0x22);
        gb.cpu.execute(0x1E, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), 0x22);
    }

    #[test]
    fn op_0x21() {
        let (mut gb, rom) = setup();

        // LD HL, d16
        gb.cpu.set_PC(0x0);
        assert_ne!(gb.cpu.get_HL(), 0x3322);
        gb.cpu.execute(0x21, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_HL(), 0x3322);
    }

    #[test]
    fn op_0x26() {
        let (mut gb, rom) = setup();

        // LD H D8
        gb.cpu.set_PC(0x0); 
        assert_ne!(gb.cpu.get_H(), 0x22);
        gb.cpu.execute(0x26, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), 0x22);
    }

    #[test]
    fn op_0x2A() {
        let (mut gb, rom) = setup();

        // LD A, (HL+)
        gb.cpu.set_HL(0xAB12); 
        gb.memory.write(0xAB12, 0x76);
        assert_ne!(gb.cpu.get_A(), 0x76);
        assert_ne!(gb.cpu.get_HL(), 0xAB13);
        gb.cpu.execute(0x2A, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), 0x76);
        assert_eq!(gb.cpu.get_HL(), 0xAB13);
    }

    #[test]
    fn op_0x2E() {
        let (mut gb, rom) = setup();

        // LD L, D8
        gb.cpu.set_PC(0x0); 
        assert_ne!(gb.cpu.get_L(), 0x22);
        gb.cpu.execute(0x2E, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), 0x22);
    }

    #[test]
    fn op_0x31() {
        let (mut gb, rom) = setup();

        // LD SP, d16
        gb.cpu.set_PC(0x0);
        assert_ne!(gb.cpu.get_SP(), 0x3322);
        gb.cpu.execute(0x31, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_SP(), 0x3322);
    }

    #[test]
    fn op_0x36() {
        let (mut gb, rom) = setup();

        // LD (HL) D8
        gb.cpu.set_HL(0xAB12); 
        gb.cpu.set_PC(0x0); 
        assert_ne!(gb.memory.read(gb.cpu.get_HL() as usize), 0x22);
        gb.cpu.execute(0x36, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(gb.cpu.get_HL() as usize), 0x22);
    }

    #[test]
    fn op_0x3A() {
        let (mut gb, rom) = setup();

        // LD A, (HL-)
        gb.cpu.set_HL(0xAB10); 
        gb.memory.write(0xAB10, 0x70);
        assert_ne!(gb.cpu.get_A(), 0x70);
        assert_ne!(gb.cpu.get_HL(), 0xAB0F);
        gb.cpu.execute(0x3A, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), 0x70);
        assert_eq!(gb.cpu.get_HL(), 0xAB0F);
    }

    #[test]
    fn op_0x3E() {
        let (mut gb, rom) = setup();

        // LD A, D8
        gb.cpu.set_PC(0x0); 
        assert_ne!(gb.cpu.get_A(), 0x22);
        gb.cpu.execute(0x3E, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), 0x22);
    }

    #[test]
    fn op_0x40() {
        let (mut gb, rom) = setup();

        // LD C, B
        gb.cpu.set_B(0xFF);
        assert_eq!(0xFF, gb.cpu.get_B());
        gb.cpu.execute(0x40, &rom, &mut gb.memory);               
        assert_eq!(0xFF, gb.cpu.get_B());
    }

    #[test]
    fn op_0x41() {
        let (mut gb, rom) = setup();

        // LD C, C
        gb.cpu.set_C(0xAB); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_B());
        gb.cpu.execute(0x41, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_B());
    }


    #[test]
    fn op_0x42() {
        let (mut gb, rom) = setup();

        // LD C, D
        gb.cpu.set_D(0xBA);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_B());
        gb.cpu.execute(0x42, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_B());
    }

    #[test]
    fn op_0x43() {
        let (mut gb, rom) = setup();

        // LD C, E
        gb.cpu.set_E(0xAB);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_B());
        gb.cpu.execute(0x43, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_B());
    }

    #[test]
    fn op_0x44() {
        let (mut gb, rom) = setup();

        // LD C, H
        gb.cpu.set_H(0xBA);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_B());
        gb.cpu.execute(0x44, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_B());
    }

    #[test]
    fn op_0x45() {
        let (mut gb, rom) = setup();

        // LD C, L
        gb.cpu.set_L(0xAB);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_B());
        gb.cpu.execute(0x45, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_B());

    }

    #[test]
    fn op_0x46() {
        let (mut gb, rom) = setup();

        // LD B, mem[HL]
        assert_ne!(gb.cpu.get_B(), 0xD);
        gb.memory.write(0xABAB, 0xD);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.execute(0x46, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_B(), 0xD);
    }

    #[test]
    fn op_0x47() {
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
    fn op_0x48() {
        let (mut gb, rom) = setup();

        // LD C, B
        gb.cpu.set_B(0xFF);
        assert_ne!(gb.cpu.get_B(), gb.cpu.get_C());
        gb.cpu.execute(0x48, &rom, &mut gb.memory);               
        assert_eq!(gb.cpu.get_B(), gb.cpu.get_C());
    }

    #[test]
    fn op_0x49() {
        let (mut gb, rom) = setup();

        // LD C, C
        assert_ne!(0xAB, gb.cpu.get_C());
        gb.cpu.set_C(0xAB); 
        gb.cpu.execute(0x49, &rom, &mut gb.memory);
        assert_eq!(0xAB, gb.cpu.get_C());
    }


    #[test]
    fn op_0x4A() {
        let (mut gb, rom) = setup();

        // LD C, D
        gb.cpu.set_D(0xBA);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_C());
        gb.cpu.execute(0x4A, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_C());
    }

    #[test]
    fn op_0x4B() {
        let (mut gb, rom) = setup();

        // LD C, E
        gb.cpu.set_E(0xAB);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_C());
        gb.cpu.execute(0x4B, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_C());
    }

    #[test]
    fn op_0x4C() {
        let (mut gb, rom) = setup();

        // LD C, H
        gb.cpu.set_H(0xBA);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_C());
        gb.cpu.execute(0x4C, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_C());
    }

    #[test]
    fn op_0x4D() {
        let (mut gb, rom) = setup();

        // LD C, L
        gb.cpu.set_L(0xAB);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_C());
        gb.cpu.execute(0x4D, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_C());

    }

    #[test]
    fn op_0x4E() {
        let (mut gb, rom) = setup();

        // LD C, mem[HL]
        assert_ne!(gb.cpu.get_C(), 0xF);
        gb.memory.write(0xABAB, 0xF);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.execute(0x4E, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), 0xF);
    }


    #[test]
    fn op_0x4F() {
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
    fn op_0x50() {
        let (mut gb, rom) = setup();

        // LD D, B
        gb.cpu.set_B(0xFF);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_B());
        gb.cpu.execute(0x50, &rom, &mut gb.memory);               
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_B());
    }

    #[test]
    fn op_0x51() {
        let (mut gb, rom) = setup();

        // LD D, C
        gb.cpu.set_C(0xAB); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_D());
        gb.cpu.execute(0x51, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_D());
    }


    #[test]
    fn op_0x52() {
        let (mut gb, rom) = setup();

        // LD D, D
        assert_ne!(gb.cpu.get_D(), 0xBA);
        gb.cpu.set_D(0xBA);
        gb.cpu.execute(0x52, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), 0xBA);
    }

    #[test]
    fn op_0x53() {
        let (mut gb, rom) = setup();

        // LD D, E
        gb.cpu.set_E(0xAB);
        assert_ne!(gb.cpu.get_E(), gb.cpu.get_D());
        gb.cpu.execute(0x53, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), gb.cpu.get_D());
    }

    #[test]
    fn op_0x54() {
        let (mut gb, rom) = setup();

        // LD D, H
        gb.cpu.set_H(0xBA);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_D());
        gb.cpu.execute(0x54, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_D());
    }

    #[test]
    fn op_0x55() {
        let (mut gb, rom) = setup();

        // LD D, L
        gb.cpu.set_L(0xAB);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_D());
        gb.cpu.execute(0x55, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_D());

    }

    #[test]
    fn op_0x56() {
        let (mut gb, rom) = setup();

        // LD D, mem[HL]
        assert_ne!(gb.cpu.get_D(), 0x2);
        gb.memory.write(0xABAB, 0x2);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.execute(0x56, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), 0x2);
    }

    #[test]
    fn op_0x57() {
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
    fn op_0x58() {
        let (mut gb, rom) = setup();

        // LD E, B
        gb.cpu.set_B(0xFF);
        assert_ne!(gb.cpu.get_B(), gb.cpu.get_E());
        gb.cpu.execute(0x58, &rom, &mut gb.memory);               
        assert_eq!(gb.cpu.get_B(), gb.cpu.get_E());
    }

    #[test]
    fn op_0x59() {
        let (mut gb, rom) = setup();

        // LD E, C
        gb.cpu.set_C(0x31); 
        assert_ne!(gb.cpu.get_C(), gb.cpu.get_E());
        gb.cpu.execute(0x59, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_C(), gb.cpu.get_E());
    }


    #[test]
    fn op_0x5A() {
        let (mut gb, rom) = setup();

        // LD E, D
        gb.cpu.set_D(0xBA);
        assert_ne!(gb.cpu.get_D(), gb.cpu.get_E());
        gb.cpu.execute(0x5A, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_D(), gb.cpu.get_E());
    }

    #[test]
    fn op_0x5B() {
        let (mut gb, rom) = setup();

        // LD E, E
        assert_ne!(gb.cpu.get_E(), 0xAB);
        gb.cpu.set_E(0xAB);
        gb.cpu.execute(0x5B, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), 0xAB);
    }

    #[test]
    fn op_0x5C() {
        let (mut gb, rom) = setup();

        // LD E, H
        gb.cpu.set_H(0xBA);
        assert_ne!(gb.cpu.get_H(), gb.cpu.get_E());
        gb.cpu.execute(0x5C, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_H(), gb.cpu.get_E());
    }

    #[test]
    fn op_0x5D() {
        let (mut gb, rom) = setup();

        // LD E, L
        gb.cpu.set_L(0xAB);
        assert_ne!(gb.cpu.get_L(), gb.cpu.get_E());
        gb.cpu.execute(0x5D, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_L(), gb.cpu.get_E());

    }

    #[test]
    fn op_0x5E() {
        let (mut gb, rom) = setup();

        // LD E, mem[HL]
        assert_ne!(gb.cpu.get_E(), 0x3);
        gb.memory.write(0xABAB, 0x3);
        gb.cpu.set_HL(0xABAB);
        gb.cpu.execute(0x5E, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_E(), 0x3);
    }

    #[test]
    fn op_0x5F() {
        let (mut gb, rom) = setup();

        // LD E, A
        gb.cpu.set_A(0xFF);
        assert_ne!(gb.cpu.get_A(), gb.cpu.get_E());
        gb.cpu.execute(0x5F, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), gb.cpu.get_E());
    }

    // POP BC
    #[test]
    fn op_0xC1() {
        // Arrange
        let (mut gb, rom) = setup();
        gb.memory.write(0x2001, 0x12);
        gb.memory.write(0x2000, 0x34);
        gb.cpu.set_SP(0x2000);
        assert_ne!(gb.cpu.get_BC(), 0x1234);

        // Act
        gb.cpu.execute(0xC1, &rom, &mut gb.memory);

        // Assert
        assert_eq!(gb.cpu.get_BC(), 0x1234); 
        assert_eq!(gb.cpu.get_SP(), 0x2002);
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
    fn op_0xEA() {
        let (mut gb, rom) = setup();

        // LD (a16), A
        gb.cpu.set_PC(0x0);
        gb.cpu.set_A(0xDD);
        assert_ne!(gb.memory.read(0x3322), gb.cpu.get_A());
        gb.cpu.execute(0xEA, &rom, &mut gb.memory);
        assert_eq!(gb.memory.read(0x3322), gb.cpu.get_A());
    }

    #[test]
    fn op_0xF0() {
        let (mut gb, rom) = setup();
    
        // LDH A, (a8)
        gb.cpu.set_PC(0x0); 
        gb.memory.write(0xFF22, 0xFD);
        assert_ne!(gb.cpu.get_A(), 0xFD);
        gb.cpu.execute(0xF0, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), 0xFD);
    }

    #[test]
    fn op_0xF2() {
        let (mut gb, rom) = setup();
    
        // LD A, (C)
        gb.cpu.set_C(0x37);
        gb.memory.write(0xFF37, 0x99);
        assert_ne!(gb.cpu.get_A(), 0x99);
        gb.cpu.execute(0xF2, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), 0x99);
    }

    #[test]
    fn op_0xFA() {
        let (mut gb, rom) = setup();
    
        // LD A, (a16)
        gb.cpu.set_PC(0x0);
        gb.memory.write(0x3322, 0x87);
        assert_ne!(gb.cpu.get_A(), 0x87);
        gb.cpu.execute(0xFA, &rom, &mut gb.memory);
        assert_eq!(gb.cpu.get_A(), 0x87);
    }

    #[test]
    fn load_D_d8() {
    }

    #[test]
    fn load_H_d8() {
    }

    #[test]
    fn load_indirect_HL_d8() {
    }

    #[test]
    fn load_A_indirect_BC() {
    }

    #[test]
    fn load_A_indirect_DE() {
    }

    #[test]
    fn load_A_indirect_HL_inc() {
    }

    #[test]
    fn load_A_indirect_HL_dec() {
    }

    #[test]
    fn load_C_d8() {
    }

    #[test]
    fn load_E_d8() {
    }

    #[test]
    fn load_L_d8() {
    }

    #[test]
    fn load_A_d8() {
    }

    #[test]
    fn load_high_A_indirect_a8() {
    }

    #[test]
    fn load_A_indirect_C() {
    }

    #[test]
    fn load_A_indirect_a16() {
    }
}

