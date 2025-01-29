use crate::{Register, Opcode, Instruction};

pub const FLAG_ZERO: u16 = 0b1;
pub const FLAG_NEG: u16 = 0b01;

pub struct Emulator {
  pub ram: Vec<u8>,
  registers: Registers,
}

impl Emulator {
  pub fn new() -> Self {
    Self {
      ram: vec![],
      registers: Registers::default(),
    }
  }

  pub fn cycle(&mut self) {
    let instr = Instruction::from_u32(
      self.load_word(self.registers.pc) as u32 + 0x10000 * self.load_word(self.registers.pc + 2) as u32,
    )
    .expect("todo exceptions");
    self.registers.pc += 4;

    match instr.opc() {
      Opcode::Add => self.exec_alu(instr, u16::wrapping_add),
      Opcode::Sub => self.exec_alu(instr, u16::wrapping_sub),
      Opcode::Mul => self.exec_alu(instr, u16::wrapping_mul),
      Opcode::Div => self.exec_alu(instr, u16::wrapping_div),
      Opcode::Rem => self.exec_alu(instr, u16::wrapping_rem),
      Opcode::And => self.exec_alu(instr, |a, b| a & b),
      Opcode::Or => self.exec_alu(instr, |a, b| a | b),
      Opcode::Xor => self.exec_alu(instr, |a, b| a ^ b),
      Opcode::Lb => self
        .registers
        .write(instr.rd(), self.load_byte(self.get_i_addr(instr)) as i8 as u16),
      Opcode::Lbu => self
        .registers
        .write(instr.rd(), self.load_byte(self.get_i_addr(instr)) as u16),
      Opcode::Lw => self.registers.write(instr.rd(), self.load_word(self.get_i_addr(instr))),
      Opcode::Sb => self.store_byte(self.get_i_addr(instr), self.registers.read(instr.rd()) as u8),
      Opcode::Sw => self.store_word(self.get_i_addr(instr), self.registers.read(instr.rd())),
      Opcode::Jeq => {
        if self.registers.sts_zero() {
          self.registers.pc = self.get_i_addr(instr)
        }
      }
      Opcode::Jne => {
        if !self.registers.sts_zero() {
          self.registers.pc = self.get_i_addr(instr)
        }
      }
      Opcode::Jgt => {
        if !self.registers.sts_zero() && self.registers.sts_neg() {
          self.registers.pc = self.get_i_addr(instr)
        }
      }
      Opcode::Jlt => {
        if !self.registers.sts_zero() && !self.registers.sts_neg() {
          self.registers.pc = self.get_i_addr(instr)
        }
      }
    }
  }

  // maybe make these do smart stuff with the address
  fn load_word(&self, addr: u16) -> u16 {
    u16::from_le_bytes([self.ram[addr as usize], self.ram[addr as usize + 1]])
  }

  fn load_byte(&self, addr: u16) -> u8 {
    self.ram[addr as usize]
  }

  fn store_word(&mut self, addr: u16, x: u16) {
    self.ram.splice(addr as usize..addr as usize + 2, x.to_le_bytes());
  }

  fn store_byte(&mut self, addr: u16, x: u8) {
    self.ram[addr as usize] = x;
  }

  fn exec_alu<F: Fn(u16, u16) -> u16>(&mut self, instr: Instruction, f: F) {
    let a = self.registers.read(instr.r1());
    let b = match instr {
      Instruction::R(_, _, _, r2) => self.registers.read(r2),
      Instruction::I(_, _, _, imm) => imm,
    };
    let r = f(a, b);
    if r == 0 {
      self.registers.sts = FLAG_ZERO;
    } else if r & 0x8000 > 0 {
      self.registers.sts = FLAG_NEG;
    }
    self.registers.write(instr.rd(), r);
  }

  fn get_i_addr(&self, instr: Instruction) -> u16 {
    self.registers.read(instr.r1()) + instr.imm().unwrap()
  }
}

#[derive(Default, Debug)]
struct Registers {
  r1: u16,
  r2: u16,
  r3: u16,
  r4: u16,
  r5: u16,
  r6: u16,
  r7: u16,
  r8: u16,
  pc: u16,
  sp: u16,
  sts: u16,
}

impl Registers {
  fn read(&self, reg: Register) -> u16 {
    match reg {
      Register::R0 => 0,
      Register::R1 => self.r1,
      Register::R2 => self.r2,
      Register::R3 => self.r3,
      Register::R4 => self.r4,
      Register::R5 => self.r5,
      Register::R6 => self.r6,
      Register::R7 => self.r7,
      Register::R8 => self.r8,
      Register::PC => self.pc,
      Register::SP => self.sp,
      Register::STS => self.sts,
    }
  }

  fn write(&mut self, reg: Register, x: u16) {
    match reg {
      Register::R0 => {}
      Register::R1 => self.r1 = x,
      Register::R2 => self.r2 = x,
      Register::R3 => self.r3 = x,
      Register::R4 => self.r4 = x,
      Register::R5 => self.r5 = x,
      Register::R6 => self.r6 = x,
      Register::R7 => self.r7 = x,
      Register::R8 => self.r8 = x,
      Register::PC => self.pc = x,
      Register::SP => self.sp = x,
      Register::STS => self.sts = x,
    }
  }

  fn sts_zero(&self) -> bool {
    self.sts & FLAG_ZERO > 0
  }

  fn sts_neg(&self) -> bool {
    self.sts & FLAG_NEG > 0
  }
}
