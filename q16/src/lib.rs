#![feature(map_try_insert)]
pub mod util;
pub mod obj;
pub mod asm;
pub mod emu;

use std::fmt;
use strum::{EnumString, EnumIter, FromRepr, Display};

pub mod sts {
  // the indices of bits in the sts register
  pub const ZERO: u16 = 0;
  pub const NEG: u16 = 1;
  pub const RUN: u16 = 8;
}
pub mod addr {
  pub const VRAM: u16 = 0xc000;
  pub const SERIAL_IO: u16 = 0xf000;
}

#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq, Eq, Display, Debug, EnumString, EnumIter, FromRepr)]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
pub enum Register {
  R0, R1, R2, R3, R4, R5, R6, R7, R8,
  PC, SP, RA, STS
}

#[derive(Copy, Clone, PartialEq, Eq, Display, Debug, EnumString, FromRepr)]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
pub enum Opcode {
  Add = 1,
  Sub,
  Mul,
  Div,
  Rem,
  And,
  Or,
  Xor,
  Lb,
  Lbu,
  Lw,
  Sb,
  Sw,
  Jeq,
  Jne,
  Jgt,
  Jlt,
  Jge,
  Jle,
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

  pub fn imm(&self) -> Option<u16> {
    match self {
      Self::I(_, _, _, imm) => Some(*imm),
      Self::R(_, _, _, _) => None,
    }
  }
}

impl fmt::Display for Instruction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::I(opc, rd, r1, imm) => write!(f, "{} %{}, %{}, 0x{:x}", opc, rd, r1, imm),
      Self::R(opc, rd, r1, r2) => write!(f, "{} %{}, %{}, %{}", opc, rd, r1, r2),
    }
  }
}
