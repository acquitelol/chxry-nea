use strum::IntoEnumIterator;
use crate::{Register, Opcode, Instruction, sts};

pub const MEM_LEN: usize = u16::MAX as usize + 1;

pub struct Emulator {
  pub memory: Vec<u8>,
  pub registers: Registers,
}

impl Emulator {
  pub fn new() -> Self {
    Self {
      memory: vec![0; MEM_LEN],
      registers: Registers::default(),
    }
  }

  pub fn set_run(&mut self, run: bool) {
    set_bit(&mut self.registers.sts, sts::RUN, run);
  }

  pub fn running(&mut self) -> bool {
    get_bit(self.registers.sts, sts::RUN)
  }

  pub fn cycle(&mut self) -> CycleOutput {
    let instr_raw = self.load_word(self.registers.pc) as u32 + 0x10000 * self.load_word(self.registers.pc + 2) as u32;
    let mut output = CycleOutput {
      instr: Instruction::from_u32(instr_raw),
      mem_load: None,
      mem_store: None,
    };
    let instr = match output.instr {
      Some(i) => i,
      None => {
        self.soft_reset();
        return output;
      }
    };
    self.registers.pc = self.registers.pc.wrapping_add(4);

    match instr.opc() {
      Opcode::Add => self.exec_alu(instr, u16::wrapping_add),
      Opcode::Sub => self.exec_alu(instr, u16::wrapping_sub),
      Opcode::Mul => self.exec_alu(instr, u16::wrapping_mul),
      Opcode::Div => self.exec_alu(instr, |a, b| if b == 0 { 0xffff } else { (a as i16).wrapping_div(b as i16) as u16 }),
      Opcode::Rem => self.exec_alu(instr, |a, b| if b == 0 { 0xffff } else { (a as i16 % b as i16) as u16 }),
      Opcode::And => self.exec_alu(instr, |a, b| a & b),
      Opcode::Or => self.exec_alu(instr, |a, b| a | b),
      Opcode::Xor => self.exec_alu(instr, |a, b| a ^ b),
      Opcode::Lb => {
        let addr = self.get_i_addr(instr);
        output.mem_load = Some(addr);
        self.registers.write(instr.rd(), self.load_byte(addr) as i8 as u16)
      }
      Opcode::Lbu => {
        let addr = self.get_i_addr(instr);
        output.mem_load = Some(addr);
        self.registers.write(instr.rd(), self.load_byte(addr) as u16)
      }
      Opcode::Lw => {
        let addr = self.get_i_addr(instr);
        output.mem_load = Some(addr);
        self.registers.write(instr.rd(), self.load_word(addr))
      }
      Opcode::Sb => {
        let addr = self.get_i_addr(instr);
        output.mem_store = Some(addr);
        self.store_byte(addr, self.registers.read(instr.rd()) as u8);
      }
      Opcode::Sw => {
        let addr = self.get_i_addr(instr);
        output.mem_store = Some(addr);
        self.store_word(addr, self.registers.read(instr.rd()));
      }
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
    output
  }

  /// zero registers and memory
  pub fn reset(&mut self) {
    *self = Self::new();
  }

  /// reset register only
  pub fn soft_reset(&mut self) {
    self.registers = Registers::default();
  }

  pub fn load_word(&self, addr: u16) -> u16 {
    u16::from_le_bytes([self.load_byte(addr), if addr < u16::MAX { self.load_byte(addr + 1) } else { 0 }])
  }

  pub fn load_byte(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }

  pub fn store_word(&mut self, addr: u16, x: u16) {
    let bytes = x.to_le_bytes();
    self.store_byte(addr, bytes[0]);
    if addr < u16::MAX {
      self.store_byte(addr.wrapping_add(1), bytes[1]);
    }
  }

  pub fn store_byte(&mut self, addr: u16, x: u8) {
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

  pub fn save_state(&self) -> Vec<u8> {
    let mut out = vec![];
    out.extend(&self.memory);
    for r in Register::iter() {
      out.extend(self.registers.read(r).to_le_bytes());
    }
    out
  }

  pub fn from_state(mut data: Vec<u8>) -> Option<Self> {
    if data.len() < MEM_LEN + 12 * 2 {
      return None;
    }
    let mut registers = Registers::default();
    for (i, r) in Register::iter().enumerate() {
      registers.write(r, u16::from_le_bytes([data[MEM_LEN + i * 2], data[MEM_LEN + i * 2 + 1]]));
    }
    data.truncate(MEM_LEN);
    Some(Self { memory: data, registers })
  }
}

pub struct CycleOutput {
  pub instr: Option<Instruction>,
  pub mem_load: Option<u16>,
  pub mem_store: Option<u16>,
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

  pub fn read(&self, reg: Register) -> u16 {
    // safety: self.get_mut doesnt mutate itself
    unsafe { (*(self as *const _ as *mut Self)).get_mut(reg).copied().unwrap_or(0) }
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
