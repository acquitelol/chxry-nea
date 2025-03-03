use std::fs;
use q16::asm::Assembler;
use q16::util::{ArgParser, err_msg};

fn main() {
  let mut args = ArgParser::from_env();
  let out_path = match args.take_flag("-o") {
    Some(p) => p,
    None => return help(),
  };
  let paths = args.remaining();
  if paths.len() != 1 {
    return help();
  }
  let src_path = &paths[0];

  let src = match fs::read_to_string(src_path) {
    Ok(s) => s,
    Err(_) => err_msg(&format!("couldn't open {:?}", src_path), None),
  };

  let mut assembler = Assembler::new();
  let obj = match assembler.assemble(&src) {
    Ok(_) => assembler.obj,
    Err((n, e)) => err_msg(&e, Some((&format!("{}:{}", src_path, n + 1), src.lines().nth(n).unwrap().trim()))),
  };

  if fs::write(&out_path, obj.out_obj()).is_err() {
    err_msg(&format!("could not write to {:?}", out_path), None);
  }
}

fn help() {
  println!("q16-asm help:");
  println!("usage: q16-asm <input file> -o <out object>");
}
