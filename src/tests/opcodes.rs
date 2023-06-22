use crate::gameboy::GameBoy;
use std::fs::read;

#[cfg(test)]

fn setup(opcode: u16) -> GameBoy {
    let mut rom_path = String::from("./roms/tests/op0x");
    rom_path.push_str(&format!("{:0>2}", opcode));
    rom_path.push_str(".gbc");

    let rom: Vec<u8> = read(rom_path.clone()).expect(&format!("Test ROM {:x} not found at {}", opcode, rom_path));
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
    // INC BC
    // 1-byte, 8-cycle

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
