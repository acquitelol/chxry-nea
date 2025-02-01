use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;
use owo_colors::OwoColorize;
use q16::Register;
use q16::asm::Assembler;
use q16::emu::Emulator;
use q16::util::err;

fn main() {
  let tests: Vec<_> = fs::read_dir("./tests/tests").unwrap().collect();
  let total = tests.len();
  let mut fails = 0;

  println!("running {} tests", total);
  let start = Instant::now();
  for entry in tests {
    let path = entry.unwrap().path();
    print!("test {} ... ", path.file_name().unwrap().to_string_lossy().bright_white().italic());
    match run_test(&path) {
      Ok(_) => println!("{}", "pass".green().bold()),
      Err(e) => {
        println!("{}\n> {}", "fail".red().bold(), e);
        fails += 1;
      }
    }
  }

  println!();
  print!("ran {} tests in {}ms, ", total, start.elapsed().as_millis());
  if fails > 0 {
    println!("{} passed, {}", total - fails, format!("{} failed", fails).red().bold());
  } else {
    println!("{}", format!("all passed").green().bold());
  }
}

fn run_test(path: &Path) -> Result<(), String> {
  let mut assembler = Assembler::new();
  let src = fs::read_to_string(path).unwrap();
  let obj = match assembler.assemble(&src) {
    Ok(_) => assembler.obj,
    Err(e) => return Err(e.1),
  };
  let bin = match obj.out_bin() {
    Ok(b) => b,
    Err(e) => return Err(e),
  };

  let mut emu = Emulator::new();
  emu.ram.splice(0..bin.len(), bin);

  let find_asserts = Regex::new(r";assert.*").unwrap();
  let find_inner = Regex::new(r"(\w+)=(\d+)").unwrap();
  for a in find_asserts.captures_iter(&src) {
    emu.set_run(true);
    while emu.running() {
      emu.cycle();
    }
    for r in find_inner.captures_iter(&a[0]) {
      // error handle
      let reg = Register::from_str(&r[1]).unwrap();
      let found = emu.registers.read(reg);
      let expect = u16::from_str_radix(&r[2], 10).unwrap();
      if found != expect {
        return err!("expected {}={}, found {}", reg, expect, found);
      }
    }
  }

  Ok(())
}
