#![feature(map_try_insert)]
pub mod util;
pub mod obj;
pub mod asm;
pub mod emu;

use strum::{EnumString, FromRepr};

#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumString, FromRepr)]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
pub enum Register {
  R0, R1, R2, R3, R4, R5, R6, R7, R8,
  PC, SP, STS
}

#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumString, FromRepr)]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
pub enum Opcode {
  Add = 0b0000001,
  Sub = 0b0000010,
  Mul = 0b0000011,
  Div = 0b0000100,
  Rem = 0b0000101,
  And = 0b0000110,
  Or  = 0b0000111,
  Xor = 0b0001000,
  Lb  = 0b0001001,
  Lbu = 0b0001010,
  Lw  = 0b0001011,
  Sb  = 0b0001100,
  Sw  = 0b0001101,
  Jeq = 0b0001110,
  Jne = 0b0001111,
  Jgt = 0b0010000,
  Jlt = 0b0010001,
}

impl Opcode {
  fn valid_r(&self) -> bool {
    matches!(
      self,
      Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div | Opcode::Rem | Opcode::And | Opcode::Or | Opcode::Xor
    )
  }

  fn valid_i(&self) -> bool {
    *self != Opcode::Sub
  }
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
  R(Opcode, Register, Register, Register),
  I(Opcode, Register, Register, u16),
}

impl Instruction {
  pub fn as_u32(&self) -> u32 {
    let (opc, rd, rs, x) = match self {
      Self::R(opc, rd, r1, r2) if opc.valid_r() => (*opc as u32, rd, r1, *r2 as u32),
      Self::I(opc, rd, r1, imm) if opc.valid_i() => (*opc as u32 + 0x80, rd, r1, *imm as u32),
      _ => panic!("opcode/operand mismatch"),
    };
    opc + *rd as u32 * 0x100 + *rs as u32 * 0x1000 + x * 0x10000
  }

  /// verifies opcode compatability
  pub fn from_u32(x: u32) -> Option<Self> {
    let opc = Opcode::from_repr(x & 0x7F)?;
    let rd = Register::from_repr((x >> 8) & 0xF)?;
    let r1 = Register::from_repr((x >> 12) & 0xF)?;
    if x & 0x80 > 0 {
      if !opc.valid_i() {
        return None;
      }
      Some(Self::I(opc, rd, r1, (x >> 16) as u16))
    } else {
      if !opc.valid_r() {
        return None;
      }
      Some(Self::R(opc, rd, r1, Register::from_repr((x >> 16) & 0xF)?))
    }
  }

  fn shared(&self) -> (Opcode, Register, Register) {
    match self {
      Self::R(opc, rd, r1, _) => (*opc, *rd, *r1),
      Self::I(opc, rd, r1, _) => (*opc, *rd, *r1),
    }
  }

  pub fn opc(&self) -> Opcode {
    self.shared().0
  }

  pub fn rd(&self) -> Register {
    self.shared().1
  }

  pub fn r1(&self) -> Register {
    self.shared().2
  }

  fn imm(&self) -> Option<u16> {
    match self {
      Self::I(_, _, _, imm) => Some(*imm),
      Self::R(_, _, _, _) => None,
    }
  }
}
