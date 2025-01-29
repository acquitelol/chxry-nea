use crate::{Register, Opcode, Instruction};

pub struct Emulator {
  pub ram: Vec<u8>,
  pub registers: Registers,
}

impl Emulator {
  pub fn new() -> Self {
    Self {
      ram: vec![0; u16::MAX as _],
      registers: Registers::default(),
    }
  }

  pub fn cycle(&mut self) {
    let instr = Instruction::from_u32(self.load_word(self.registers.pc) as u32 + 0x10000 * self.load_word(self.registers.pc + 2) as u32)
      .expect("todo exceptions");
    self.registers.pc += 4;

    match instr.opc() {
      Opcode::Add => self.exec_alu(instr, u16::wrapping_add),
      Opcode::Sub => self.exec_alu(instr, u16::wrapping_sub),
      Opcode::Mul => self.exec_alu(instr, u16::wrapping_mul),
      Opcode::Div => self.exec_alu(instr, u16::wrapping_div), // panics on b=0 !!
      Opcode::Rem => self.exec_alu(instr, u16::wrapping_rem),
      Opcode::And => self.exec_alu(instr, |a, b| a & b),
      Opcode::Or => self.exec_alu(instr, |a, b| a | b),
      Opcode::Xor => self.exec_alu(instr, |a, b| a ^ b),
      Opcode::Lb => self
        .registers
        .write(instr.rd(), self.load_byte(self.get_i_addr(instr)) as i8 as u16),
      Opcode::Lbu => self.registers.write(instr.rd(), self.load_byte(self.get_i_addr(instr)) as u16),
      Opcode::Lw => self.registers.write(instr.rd(), self.load_word(self.get_i_addr(instr))),
      Opcode::Sb => self.store_byte(self.get_i_addr(instr), self.registers.read(instr.rd()) as u8),
      Opcode::Sw => self.store_word(self.get_i_addr(instr), self.registers.read(instr.rd())),
      Opcode::Jeq => {
        if get_bit(self.registers.sts, STS_ZERO) {
          self.registers.pc = self.get_i_addr(instr)
        }
      }
      Opcode::Jne => {
        if !get_bit(self.registers.sts, STS_ZERO) {
          self.registers.pc = self.get_i_addr(instr)
        }
      }
      Opcode::Jgt => {
        if !get_bit(self.registers.sts, STS_ZERO) && get_bit(self.registers.sts, STS_NEG) {
          self.registers.pc = self.get_i_addr(instr)
        }
      }
      Opcode::Jlt => {
        if !get_bit(self.registers.sts, STS_ZERO) && !get_bit(self.registers.sts, STS_NEG) {
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
    set_bit(&mut self.registers.sts, STS_ZERO, r == 0);
    set_bit(&mut self.registers.sts, STS_NEG, get_bit(r, 15));
    self.registers.write(instr.rd(), r);
  }

  fn get_i_addr(&self, instr: Instruction) -> u16 {
    self.registers.read(instr.r1()) + instr.imm().unwrap()
  }
}

#[derive(Default, Debug)]
pub struct Registers {
  pub r1: u16,
  pub r2: u16,
  pub r3: u16,
  pub r4: u16,
  pub r5: u16,
  pub r6: u16,
  pub r7: u16,
  pub r8: u16,
  pub pc: u16,
  pub sp: u16,
  pub sts: u16,
}

impl Registers {
  pub fn read(&self, reg: Register) -> u16 {
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

  pub fn get_mut(&mut self, reg: Register) -> Option<&mut u16> {
    match reg {
      Register::R0 => None,
      Register::R1 => Some(&mut self.r1),
      Register::R2 => Some(&mut self.r2),
      Register::R3 => Some(&mut self.r3),
      Register::R4 => Some(&mut self.r4),
      Register::R5 => Some(&mut self.r5),
      Register::R6 => Some(&mut self.r6),
      Register::R7 => Some(&mut self.r7),
      Register::R8 => Some(&mut self.r8),
      Register::PC => Some(&mut self.pc),
      Register::SP => Some(&mut self.sp),
      Register::STS => Some(&mut self.sts),
    }
  }

  pub fn write(&mut self, reg: Register, x: u16) {
    self.get_mut(reg).map(|r| *r = x);
  }
}

pub const STS_ZERO: u16 = 0;
pub const STS_NEG: u16 = 1;
pub const STS_RUN: u16 = 8;

pub fn get_bit(x: u16, n: u16) -> bool {
  (x & (1 << n)) != 0
}

pub fn set_bit(x: &mut u16, n: u16, on: bool) {
  if on {
    *x |= 1 << n;
  } else {
    *x &= !(1 << n);
  }
}
