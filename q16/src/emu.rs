use crate::{Register, Opcode, Instruction, sts};

pub struct Emulator {
  pub memory: Vec<u8>,
  pub registers: Registers,
}

impl Emulator {
  pub fn new() -> Self {
    Self {
      memory: vec![0; u16::MAX as _],
      registers: Registers::default(),
    }
  }

  pub fn set_run(&mut self, run: bool) {
    set_bit(&mut self.registers.sts, sts::RUN, run);
  }

  pub fn running(&mut self) -> bool {
    get_bit(self.registers.sts, sts::RUN)
  }

  pub fn cycle(&mut self) -> Option<Instruction> {
    let instr_raw = self.load_word(self.registers.pc) as u32 + 0x10000 * self.load_word(self.registers.pc + 2) as u32;
    let instr = match Instruction::from_u32(instr_raw) {
      Some(i) => i,
      None => {
        self.soft_reset();
        return None;
      }
    };
    self.registers.pc += 4;

    match instr.opc() {
      Opcode::Add => self.exec_alu(instr, u16::wrapping_add),
      Opcode::Sub => self.exec_alu(instr, u16::wrapping_sub),
      Opcode::Mul => self.exec_alu(instr, u16::wrapping_mul),
      Opcode::Div => self.exec_alu(instr, |a, b| if b == 0 { 0xffff } else { (a as i16).wrapping_div(b as i16) as u16 }),
      Opcode::Rem => self.exec_alu(instr, |a, b| if b == 0 { 0xffff } else { (a as i16 % b as i16) as u16 }),
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
        if get_bit(self.registers.sts, sts::ZERO) {
          self.registers.pc = self.get_i_addr(instr)
        }
      }
      Opcode::Jne => {
        if !get_bit(self.registers.sts, sts::ZERO) {
          self.registers.pc = self.get_i_addr(instr)
        }
      }
      Opcode::Jgt => {
        if !get_bit(self.registers.sts, sts::ZERO) && get_bit(self.registers.sts, sts::NEG) {
          self.registers.pc = self.get_i_addr(instr)
        }
      }
      Opcode::Jlt => {
        if !get_bit(self.registers.sts, sts::ZERO) && !get_bit(self.registers.sts, sts::NEG) {
          self.registers.pc = self.get_i_addr(instr)
        }
      }
    };
    Some(instr)
  }

  /// zero registers and memory
  pub fn reset(&mut self) {
    *self = Self::new();
  }

  /// reset register only
  fn soft_reset(&mut self) {
    self.registers = Registers::default();
  }

  // todo handle edge cases
  fn load_word(&self, addr: u16) -> u16 {
    u16::from_le_bytes([self.memory[addr as usize], self.memory[addr as usize + 1]])
  }

  fn load_byte(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }

  fn store_word(&mut self, addr: u16, x: u16) {
    self.memory.splice(addr as usize..addr as usize + 2, x.to_le_bytes());
  }

  fn store_byte(&mut self, addr: u16, x: u8) {
    self.memory[addr as usize] = x;
  }

  fn exec_alu<F: Fn(u16, u16) -> u16>(&mut self, instr: Instruction, f: F) {
    let a = self.registers.read(instr.r1());
    let b = match instr {
      Instruction::R(_, _, _, r2) => self.registers.read(r2),
      Instruction::I(_, _, _, imm) => imm,
    };
    let r = f(a, b);
    set_bit(&mut self.registers.sts, sts::ZERO, r == 0);
    set_bit(&mut self.registers.sts, sts::NEG, get_bit(r, 15));
    self.registers.write(instr.rd(), r);
  }

  fn get_i_addr(&self, instr: Instruction) -> u16 {
    self.registers.read(instr.r1()).wrapping_add(instr.imm().unwrap())
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
  pub ra: u16,
  pub sts: u16,
}

impl Registers {
  pub fn read(&self, reg: Register) -> u16 {
    // safety: self.get_mut doesnt mutate
    unsafe { (*(self as *const _ as *mut Self)).get_mut(reg).copied().unwrap_or(0) }
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
      Register::RA => Some(&mut self.ra),
      Register::STS => Some(&mut self.sts),
    }
  }

  pub fn write(&mut self, reg: Register, x: u16) {
    if let Some(r) = self.get_mut(reg) {
      *r = x;
    }
  }
}

fn get_bit(x: u16, n: u16) -> bool {
  (x & (1 << n)) != 0
}

fn set_bit(x: &mut u16, n: u16, on: bool) {
  if on {
    *x |= 1 << n;
  } else {
    *x &= !(1 << n);
  }
}
