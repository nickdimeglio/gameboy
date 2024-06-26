#[derive(Clone, Debug, PartialEq)]
pub struct Registers {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: usize,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0x0100,
        }
    }

    pub fn get_af(&self) -> u16 {
        self.af
    }

    pub fn get_bc(&self) -> u16 {
        self.bc
    }

    pub fn get_de(&self) -> u16 {
        self.de
    }

    pub fn get_hl(&self) -> u16 {
        self.hl
    }

    pub fn get_inc_hl(&mut self) -> u16 {
        self.set_hl(self.get_hl() + 1);
        self.get_hl() - 1
    }

    pub fn get_dec_hl(&mut self) -> u16 {
        self.set_hl(self.get_hl() - 1);
        self.get_hl() + 1
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn get_pc(&self) -> usize {
        self.pc
    }

    pub fn set_af(&mut self, val: u16) {
        self.af = val;
    }

    pub fn set_bc(&mut self, val: u16) {
        self.bc = val;
    }

    pub fn set_de(&mut self, val: u16) {
        self.de = val;
    }

    pub fn set_hl(&mut self, val: u16) {
        self.hl = val;
    }

    pub fn set_sp(&mut self, val: u16) {
        self.sp = val;
    }

    pub fn set_pc(&mut self, address: usize) {
        self.pc = address;
    }

    pub fn inc_pc(&mut self) {
        self.pc = self.pc + 1;
    }

    pub fn get_a(&self) -> u8 {
        (self.af >> 8) as u8
    }

    pub fn set_a(&mut self, val: u8) {
        self.af = ((val as u16) << 8) | (self.af & 0x00ff);
    }

    pub fn get_f(&self) -> u8 {
        (self.af & 0x00ff) as u8
    }

    pub fn set_f(&mut self, val: u8) {
        self.af = (self.af & 0xff00) | (val as u16);
    }

    pub fn get_b(&self) -> u8 {
        (self.bc >> 8) as u8
    }

    pub fn set_b(&mut self, val: u8) {
        self.bc = ((val as u16) << 8) | (self.bc & 0x00ff);
    }

    pub fn get_c(&self) -> u8 {
        (self.bc & 0x00ff) as u8
    }

    pub fn set_c(&mut self, val: u8) {
        self.bc = (self.bc & 0xff00) | (val as u16);
    }

    pub fn get_d(&self) -> u8 {
        (self.de >> 8) as u8
    }

    pub fn set_d(&mut self, val: u8) {
        self.de = ((val as u16) << 8) | (self.de & 0x00ff);
    }

    pub fn get_e(&self) -> u8 {
        (self.de & 0x00ff) as u8
    }

    pub fn set_e(&mut self, val: u8) {
        self.de = (self.de & 0xff00) | (val as u16);
    }

    pub fn get_h(&self) -> u8 {
        (self.hl >> 8) as u8
    }

    pub fn set_h(&mut self, val: u8) {
        self.hl = ((val as u16) << 8) | (self.hl & 0x00ff);
    }

    pub fn get_l(&self) -> u8 {
        (self.hl & 0x00ff) as u8
    }

    pub fn set_l(&mut self, val: u8) {
        self.hl = (self.hl & 0xff00) | (val as u16);
    }

    pub fn get_z_flag(&mut self) -> u8 {
        self.get_f() & 0b0001
    }
    
    pub fn get_n_flag(&mut self) -> u8 {
        (self.get_f() & 0b0010) >> 1
    }

    pub fn get_h_flag(&mut self) -> u8 {
        (self.get_f() & 0b0100) >> 2
    }

    pub fn get_c_flag(&mut self) -> u8 {
        (self.get_f() & 0b1000) >> 3
    }

    pub fn set_z_flag(&mut self, val: u8) {
        assert!(val < 2, "Zero flag can only be set to 0 or 1");
        self.set_f(self.get_f() & 0b1110);
        self.set_f(self.get_f() | val)
    }

    pub fn set_n_flag(&mut self, val: u8) {
        assert!(val < 2, "Subtraction flag can only be set to 0 or 1");
        self.set_f(self.get_f() & 0b1101);
        self.set_f(self.get_f() | (val << 1));
    }

    pub fn set_h_flag(&mut self, val: u8) {
        assert!(val < 2, "Half-carry flag can only be set to 0 or 1");
        self.set_f(self.get_f() & 0b1011);
        self.set_f(self.get_f() | (val << 2))
    }

    pub fn set_c_flag(&mut self, val: u8) {
        assert!(val < 2, "Half-carry flag can only be set to 0 or 1");
        self.set_f(self.get_f() & 0b0111);
        self.set_f(self.get_f() | (val << 3));
    }

}
