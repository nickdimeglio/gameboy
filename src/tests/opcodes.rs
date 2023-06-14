#[cfg(test)]
use std::fs::read;
use crate::gameboy::GameBoy;

#[test]
fn op_0x00() {
    // NOP

    // Arrange
    let rom: Vec<u8> = read("./roms/tests/op0x00.gbc").expect("Test ROM 0x01 not found");
    let mut gb = GameBoy::new(rom);
    gb.registers.set_pc(0x0);

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
    let rom = read("./roms/tests/op0x01.gbc").expect("Test ROM 0x01 not found");
    let mut gb = GameBoy::new(rom);
    gb.registers.set_pc(0x0000);
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
fn op_0x11() {
    // LD DE, d16
    // 3-byte, 12-cycle

    // Arrange
    let rom = read("./roms/tests/op0x11.gbc").expect("Test ROM 0x11 not found");
    let mut gb = GameBoy::new(rom);
    gb.registers.set_pc(0x0);
    assert_ne!(gb.registers.get_de(), 0xBBAA);

    // Act
    let result = gb.execute();

    // Assert
    assert!(result.is_ok(), "Op 0x11 failed: {}", result.unwrap_err());
    let opcode = result.unwrap();
            
    assert_eq!(opcode, 0x11);
    assert_eq!(gb.registers.get_de(), 0xBBAA);
    assert_eq!(gb.registers.get_pc(), 0x0003);
}