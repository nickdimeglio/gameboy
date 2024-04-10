use crate::gameboy::GameBoy;
use std::fs::read;

#[cfg(test)]
fn setup(opcode: u16) -> GameBoy {
    let mut rom_path = format!("./roms/tests/op0x{opcode:0>2}.gb");
    let rom: Vec<u8> = read(rom_path.clone()).expect(
        &format!("Test ROM {opcode:x} not found at {rom_path}")
    );
    let mut gb = GameBoy::new(rom);
    gb.registers.set_pc(0x000);

    gb
}

#[test]
fn op_0x00() {
    // NOP

    // Arrange
    let mut gb = setup(0x00);
    let mut new_gb = gb.clone();
    new_gb.registers.set_pc(0x0001);

    // Act
    let result = gb.execute();

    // Assert
    assert!(result.is_ok(), "{}", result.unwrap_err());
    let opcode = result.unwrap();
    assert_eq!(opcode, 0x00);
    assert_eq!(gb.registers, new_gb.registers);
    assert_eq!(gb.memory, new_gb.memory);
    assert_eq!(gb.registers.get_pc(), 0x0001)
}

#[test]
fn op_0x01() {
    // LD BC, d16
    // 3-byte, 12-cycle

    // Arrange
    let mut gb = setup(0x01);
    assert_ne!(gb.registers.get_bc(), 0xBBAA);

    // Act
    let result = gb.execute();

    // Assert
    assert!(result.is_ok(), "Op 0x01 failed: {}", result.unwrap_err());
    let opcode = result.unwrap();

    assert_eq!(opcode, 0x01);
    assert_eq!(gb.registers.get_bc(), 0xBBAA);
    assert_eq!(gb.registers.get_pc(), 0x0003);
}

#[test]
fn op_0x02() {
    // LD (BC), A
    // 1-byte, 8-cycle

    // Arrange
    let mut gb = setup(0x02);
    gb.registers.set_bc(0x64);
    gb.registers.set_a(0xF);

    let at_bc = gb.memory.read_8(gb.registers.get_bc() as usize);
    assert!(at_bc.is_some());
    assert_ne!(at_bc.unwrap(), gb.registers.get_a());

    // Act
    let result = gb.execute();

    // Assert
    assert!(result.is_ok(), "Op 0x02 failed: {}", result.unwrap_err());
    let opcode = result.unwrap();
    assert_eq!(opcode, 0x02);

    let at_bc = gb.memory.read_8(gb.registers.get_bc() as usize);
    assert!(at_bc.is_some());
    assert_eq!(at_bc.unwrap(), gb.registers.get_a());

    assert_eq!(gb.registers.get_pc(), 0x1);
}

#[test]
fn op_0x03() {
    /* INC BC
     * 1-byte, 8-cycle
     */

    // Arrange
    let mut gb = setup(0x03);
    assert_eq!(gb.registers.get_bc(), 0x0);

    // Act
    let result = gb.execute();

    // Assert
    assert!(result.is_ok(), "Op 0x03 failed: {}", result.unwrap_err());
    let opcode = result.unwrap();
    assert_eq!(opcode, 0x03);

    assert_eq!(gb.registers.get_bc(), 0x1);
    assert_eq!(gb.registers.get_pc(), 0x001);
}

#[test]
fn op_0x04() {
    /* INC B
     * 1-byte, 4-cycle
     * Z 0 H -
     */

    // Arrange (half-carry)
    let mut gb = setup(0x04);
    gb.registers.set_n_flag(0x1);
    gb.registers.set_z_flag(0x1);
    gb.registers.set_b(0xBF);

    // Act (half-carry)
    let mut result = gb.execute();

    // Assert (half-carry)
    assert!(result.is_ok(), "Op 0x04 failed: {}", result.unwrap_err());
    let mut opcode = result.unwrap();
    assert_eq!(opcode, 0x04);
    assert_eq!(gb.registers.get_b(), 0xC0);
    assert_eq!(gb.registers.get_z_flag(), 0x0);
    assert_eq!(gb.registers.get_n_flag(), 0x0);
    assert_eq!(gb.registers.get_h_flag(), 0x1);
    assert_eq!(gb.registers.get_pc(), 0x001);

    // Arrange (no half-carry)
    gb.registers.set_n_flag(0x1);
    gb.registers.set_z_flag(0x1);
    gb.registers.set_b(0xFB);
    
    // Act (no half-carry)
    result = gb.execute(); 
    assert!(result.is_ok(), "Op 0x04 failed: {}", result.unwrap_err());
    opcode = result.unwrap();
    assert_eq!(opcode, 0x04);
    assert_eq!(gb.registers.get_b(), 0xFC);
    assert_eq!(gb.registers.get_z_flag(), 0x0);
    assert_eq!(gb.registers.get_n_flag(), 0x0);
    assert_eq!(gb.registers.get_h_flag(), 0x0);

    // Arrange (zero)
    gb.registers.set_n_flag(0x1);
    gb.registers.set_b(0xFF);
    
    // Act (no half-carry)
    result = gb.execute(); 
    assert!(result.is_ok(), "Op 0x04 failed: {}", result.unwrap_err());
    opcode = result.unwrap();
    assert_eq!(opcode, 0x04);
    assert_eq!(gb.registers.get_b(), 0x00);
    assert_eq!(gb.registers.get_z_flag(), 0x1);
    assert_eq!(gb.registers.get_n_flag(), 0x0);
    assert_eq!(gb.registers.get_h_flag(), 0x1);
}
