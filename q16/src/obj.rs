use std::collections::hash_map::{HashMap, Entry};
use crate::{Instruction, err, assert};

/// magic bytes for object files
const MAGIC: &[u8] = b"Q16";

pub struct Obj {
  pub labels: HashMap<String, u16>,
  pub label_uses: Vec<(String, u16)>,
  pub data: Vec<u8>,
}

impl Obj {
  pub fn new() -> Self {
    Self {
      labels: HashMap::new(),
      label_uses: vec![],
      data: vec![],
    }
  }

  pub fn load(data: &[u8]) -> Result<Self, String> {
    assert!(&data[..MAGIC.len()] == MAGIC, "invalid magic bytes")?;
    let (labels, pos) = parse_table(&data[MAGIC.len()..]);
    let (label_uses, pos2) = parse_table(&data[MAGIC.len() + pos..]);
    let data = data[MAGIC.len() + pos + pos2..].to_vec();
    Ok(Self {
      labels: HashMap::from_iter(labels),
      label_uses,
      data,
    })
  }

  pub fn insert_label(&mut self, label: String) -> Result<(), String> {
    match self.labels.try_insert(label, self.data.len() as _) {
      Ok(_) => Ok(()),
      Err(e) => err!("label '{}' already declared", e.entry.key()),
    }
  }

  /// offset by 2 from current position
  pub fn insert_label_usage(&mut self, label: String) {
    self.label_uses.push((label, (self.data.len() + 2) as _));
  }

  pub fn emit_instr(&mut self, instr: Instruction) {
    self.data.extend(instr.as_u32().to_le_bytes());
  }

  pub fn extend(&mut self, other: Self) -> Result<(), String> {
    for (label, addr) in other.labels {
      match self.labels.entry(label.clone()) {
        Entry::Occupied(_) => return Err(format!("duplicate label '{}'", label)),
        Entry::Vacant(e) => {
          e.insert(self.data.len() as u16 + addr);
        }
      }
    }
    for (label, addr) in other.label_uses {
      self.label_uses.push((label, self.data.len() as u16 + addr));
    }
    self.data.extend(other.data);
    Ok(())
  }

  pub fn out_obj(self) -> Vec<u8> {
    let mut out = Vec::from(MAGIC);
    out_table(&mut out, self.labels.into_iter());
    out_table(&mut out, self.label_uses.into_iter());
    out.extend(self.data);
    out
  }

  pub fn out_bin(mut self) -> Result<Vec<u8>, String> {
    for (label, replace) in &self.label_uses {
      match self.labels.get(label) {
        Some(addr) => {
          self.data.splice(*replace as usize..*replace as usize + 2, addr.to_le_bytes());
        }
        None => return err!("undefined label '{}'", label),
      }
    }
    Ok(self.data)
  }
}

fn out_table<I: ExactSizeIterator<Item = (String, u16)>>(out: &mut Vec<u8>, iter: I) {
  out.extend((iter.len() as u16).to_le_bytes());
  for (k, v) in iter {
    out.extend(k.as_bytes());
    out.push(0);
    out.extend(v.to_le_bytes());
  }
}

fn parse_table(bin: &[u8]) -> (Vec<(String, u16)>, usize) {
  let len = u16::from_le_bytes([bin[0], bin[1]]);
  let mut pos = 2;
  let mut out = Vec::with_capacity(len as _);
  for _ in 0..len {
    let strlen = &bin[pos..].iter().position(|b| *b == 0).unwrap();
    out.push((
      String::from_utf8(bin[pos..pos + strlen].to_vec()).unwrap(),
      u16::from_le_bytes([bin[pos + strlen + 1], bin[pos + strlen + 2]]),
    ));
    pos += strlen + 3;
  }
  (out, pos)
}
