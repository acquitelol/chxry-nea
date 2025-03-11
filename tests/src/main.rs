use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Instant;
use regex::Regex;
use owo_colors::OwoColorize;
use q16::Register;
use q16::asm::Assembler;
use q16::emu::Emulator;
use q16::util::err;

const BASE_DIR: &str = "./tests/auto";

fn main() {
  let mut tests = vec![];
  discover_tests(BASE_DIR, &mut tests);
  let total = tests.len();
  println!("running {} tests", total);

  let mut fails = 0;
  let start = Instant::now();
  for path in tests {
    print!(
      "test {} ... ",
      path.strip_prefix(BASE_DIR).unwrap().display().bright_white().italic()
    );
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
    println!("{}", "all passed".green().bold());
  }
}

fn discover_tests<P: AsRef<Path>>(dir: P, files: &mut Vec<PathBuf>) {
  for entry in fs::read_dir(dir).unwrap().flatten() {
    if entry.file_type().unwrap().is_dir() {
      discover_tests(entry.path(), files);
    } else {
      files.push(entry.path());
    }
  }
}

fn run_test(path: &Path) -> Result<(), String> {
  let mut assembler = Assembler::new();
  let src = fs::read_to_string(path).unwrap();
  let obj = match assembler.assemble(&src) {
    Ok(_) => assembler.obj,
    Err(e) => return Err(e.1),
  };
  let bin = obj.out_bin()?;

  let mut emu = Emulator::new();
  emu.memory.splice(0..bin.len(), bin);

  let find_asserts = Regex::new(r";assert[^;]*").unwrap();
  let find_inner = Regex::new(r"(\w+)=(\d+)").unwrap();
  for (n, a) in find_asserts.captures_iter(&src).enumerate() {
    emu.set_run(true);
    while emu.running() {
      emu.cycle();
    }
    for r in find_inner.captures_iter(&a[0]) {
      let reg = Register::from_str(&r[1]).unwrap();
      let found = emu.registers.read(reg);
      let expect = r[2].parse().unwrap();
      if found != expect {
        return err!("assertion #{}: expected {}={}, found {}", n + 1, reg, expect, found);
      }
    }
  }

  Ok(())
}
