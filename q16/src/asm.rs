use std::iter;
use std::str::FromStr;
use crate::{Opcode, Register, Instruction, sts};
use crate::util::{err, assert};
use crate::obj::Obj;

pub struct Assembler {
  pub obj: Obj,
}

impl Assembler {
  pub fn new() -> Self {
    Self { obj: Obj::new() }
  }

  pub fn assemble(&mut self, src: &str) -> Result<(), (usize, String)> {
    for (n, line) in src.lines().enumerate() {
      self.assemble_line(line).map_err(|e| (n, e))?;
    }
    Ok(())
  }

  fn assemble_line(&mut self, line: &str) -> Result<(), String> {
    let line = line.split(';').next().unwrap().trim();
    if line.is_empty() {
      return Ok(());
    }

    if let Some(label) = line.strip_suffix(':') {
      self.obj.insert_label(label.to_string())?;
      return Ok(());
    }

    let mut split = line.splitn(2, ' ');
    let mnemonic = split.next().unwrap().to_lowercase();
    let operands = split
      .next()
      .unwrap_or_default()
      .split_terminator(',')
      .map(|s| Operand::parse(s.trim()))
      .collect::<Result<Vec<_>, _>>()?;
    self.assemble_instr(&mnemonic, operands)
  }

  fn assemble_instr(&mut self, mnemonic: &str, operands: Vec<Operand>) -> Result<(), String> {
    match Opcode::from_str(mnemonic) {
      Ok(opc @ (Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div | Opcode::Rem | Opcode::And | Opcode::Or | Opcode::Xor)) => {
        assert_len(mnemonic, &operands, 3)?;
        match operands[2] {
          Operand::Literal(l) if opc == Opcode::Sub => {
            self.assemble_3(Opcode::Add, &[operands[0], operands[1], Operand::Literal(negate(l))])
          }
          _ => self.assemble_3(opc, &operands),
        }
      }
      Ok(opc @ (Opcode::Lb | Opcode::Lbu | Opcode::Lw | Opcode::Sb | Opcode::Sw)) => {
        if operands.len() == 3 {
          self.assemble_3(opc, &operands)
        } else if operands.len() == 2 {
          self.assemble_2(opc, &operands)
        } else {
          err!("'{}' requires 2 or 3 operands, found {}", mnemonic, operands.len())
        }
      }
      Ok(opc @ (Opcode::Jeq | Opcode::Jne | Opcode::Jgt | Opcode::Jlt)) => {
        if operands.len() == 2 {
          self.assemble_2(opc, &operands)
        } else if operands.len() == 1 {
          self.assemble_1(opc, operands[0]);
          Ok(())
        } else {
          err!("'{}' requires 1 or 2 operands, found {}", mnemonic, operands.len())
        }
      }
      Err(_) => match mnemonic {
        "nop" => {
          self
            .obj
            .emit_instr(Instruction::R(Opcode::Add, Register::R0, Register::R0, Register::R0));
          Ok(())
        }
        "hlt" => {
          self
            .obj
            .emit_instr(Instruction::I(Opcode::And, Register::STS, Register::STS, !(1 << sts::RUN)));
          Ok(())
        }
        "mov" => {
          assert_len("mov", &operands, 2)?;
          self.assemble_2(Opcode::Add, &operands)
        }
        "neg" => {
          assert_len("neg", &operands, 2)?;
          self.assemble_3(Opcode::Sub, &[operands[0], Operand::Register(Register::R0), operands[1]])
        }
        "not" => {
          assert_len("not", &operands, 2)?;
          self.assemble_3(Opcode::Xor, &[operands[0], operands[1], Operand::Literal(u16::MAX)])
        }
        "cmp" => {
          assert_len("cmp", &operands, 2)?;
          match operands[1] {
            Operand::Literal(l) => self.assemble_3(
              Opcode::Add,
              &[Operand::Register(Register::R0), operands[0], Operand::Literal(negate(l))],
            ),
            _ => self.assemble_3(Opcode::Sub, &[Operand::Register(Register::R0), operands[0], operands[1]]),
          }
        }
        "jmp" => {
          if operands.len() == 2 {
            self.assemble_3(Opcode::Add, &[Operand::Register(Register::PC), operands[0], operands[1]])
          } else if operands.len() == 1 {
            self.assemble_2(Opcode::Add, &[Operand::Register(Register::PC), operands[0]])
          } else {
            err!("'jmp' requires 1 or 2 operands, found {}", operands.len())
          }
        }
        "inc" => {
          assert_len("inc", &operands, 1)?;
          self.assemble_3(Opcode::Add, &[operands[0], operands[0], Operand::Literal(1)])
        }
        ".db" => {
          assert_len(".db", &operands, 1)?;
          match operands[0] {
            Operand::Literal(x) => self.obj.data.push(x as u8),
            _ => return err!("invalid operand for '.db'."),
          }
          Ok(())
        }
        ".dw" => {
          assert_len(".db", &operands, 1)?;
          match operands[0] {
            Operand::Literal(x) => self.obj.data.extend(x.to_le_bytes()),
            Operand::Label(l) => {
              self.obj.insert_label_usage(l.to_string(), 0);
              self.obj.data.extend([0, 0]);
            }
            _ => return err!("invalid operand for '.db'."),
          }
          Ok(())
        }
        ".skip" => {
          assert_len(".skip", &operands, 1)?;
          match operands[0] {
            Operand::Literal(n) => self.obj.data.extend(iter::repeat_n(0, n as _)),
            _ => return err!("invalid operand for '.skip'."),
          }
          Ok(())
        }
        _ => err!("unknown mnemonic '{}'", mnemonic),
      },
    }
  }

  /// accepts %rd, %r2, %r1/imm/label, requires operands.len() == 3
  fn assemble_3(&mut self, opcode: Opcode, operands: &[Operand]) -> Result<(), String> {
    let instr = match operands[0..3] {
      [Operand::Register(rd), Operand::Register(r1), Operand::Register(r2)] => Instruction::R(opcode, rd, r1, r2),
      [Operand::Register(rd), Operand::Register(r1), Operand::Literal(imm)] => Instruction::I(opcode, rd, r1, imm),
      [Operand::Register(rd), Operand::Register(r1), Operand::Label(l)] => {
        self.obj.insert_label_usage(l.to_string(), 2);
        Instruction::I(opcode, rd, r1, 0)
      }
      _ => return err!("invalid operands for '{}'", opcode),
    };
    self.obj.emit_instr(instr);
    Ok(())
  }

  /// accepts %rd, %r1/imm/label, requires operands.len() == 2
  fn assemble_2(&mut self, opcode: Opcode, operands: &[Operand]) -> Result<(), String> {
    let instr = match operands[0..2] {
      [Operand::Register(rd), Operand::Register(r1)] => Instruction::I(opcode, rd, r1, 0),
      [Operand::Register(rd), Operand::Literal(imm)] => Instruction::I(opcode, rd, Register::R0, imm),
      [Operand::Register(rd), Operand::Label(l)] => {
        self.obj.insert_label_usage(l.to_string(), 2);
        Instruction::I(opcode, rd, Register::R0, 0)
      }
      _ => return err!("invalid operands for '{}'", opcode),
    };
    self.obj.emit_instr(instr);
    Ok(())
  }

  /// accepts %r1/imm/label
  fn assemble_1(&mut self, opcode: Opcode, operand: Operand) {
    let instr = match operand {
      Operand::Register(r1) => Instruction::I(opcode, Register::R0, r1, 0),
      Operand::Literal(imm) => Instruction::I(opcode, Register::R0, Register::R0, imm),
      Operand::Label(l) => {
        self.obj.insert_label_usage(l.to_string(), 2);
        Instruction::I(opcode, Register::R0, Register::R0, 0)
      }
    };
    self.obj.emit_instr(instr);
  }
}

fn assert_len(mnemonic: &str, operands: &[Operand], expect: usize) -> Result<(), String> {
  assert!(
    operands.len() == expect,
    "'{}' requires {} operands, found {}",
    mnemonic,
    expect,
    operands.len()
  )
}

fn negate(x: u16) -> u16 {
  x.wrapping_neg()
}

#[derive(Copy, Clone, Debug)]
enum Operand<'a> {
  Literal(u16),
  Register(Register),
  Label(&'a str),
}

impl<'a> Operand<'a> {
  fn parse(s: &'a str) -> Result<Self, String> {
    let mut chars = s.chars();
    match chars.next() {
      Some('0') => match chars.next() {
        Some('x') => Self::parse_literal(s, 16),
        Some('X') => Self::parse_literal(s, 16),
        Some('o') => Self::parse_literal(s, 8),
        Some('O') => Self::parse_literal(s, 8),
        Some('b') => Self::parse_literal(s, 2),
        Some('B') => Self::parse_literal(s, 2),
        Some(c) if c.is_ascii_digit() => Self::parse_literal(s, 10),
        Some(c) => err!("unknown base '{}'", c),
        None => Ok(Self::Literal(0)),
      },
      Some(c) if c.is_ascii_digit() => Self::parse_literal(s, 10),
      Some('%') => match Register::from_str(&s[1..].to_lowercase()) {
        Ok(r) => Ok(Self::Register(r)),
        Err(_) => err!("unknown register '{}'", s),
      },
      Some(_) => Ok(Self::Label(s)),
      None => err!("empty operand"),
    }
  }

  /// accounts for 0x prefix when radix != 10
  fn parse_literal(s: &str, radix: u32) -> Result<Self, String> {
    match u16::from_str_radix(if radix == 10 { s } else { &s[2..] }, radix) {
      Ok(n) => Ok(Self::Literal(n)),
      Err(_) => err!("could not parse literal '{}'", s),
    }
  }
}
