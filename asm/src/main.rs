use std::{env, fs};
use std::path::PathBuf;
use q16::asm::Assembler;
use q16::util::err_msg;

fn main() {
  let src_path = match env::args().nth(1) {
    Some(p) => p,
    None => {
      println!("q16-asm help:");
      println!("usage: q16-asm <input>");
      println!("in.asm -> in.o");
      return;
    }
  };
  let src = match fs::read_to_string(&src_path) {
    Ok(s) => s,
    Err(_) => err_msg(&format!("couldn't open {:?}", src_path), None),
  };

  let mut assembler = Assembler::new();
  let obj = match assembler.assemble(&src) {
    Ok(_) => assembler.obj,
    Err((n, e)) => err_msg(&e, Some((&format!("{}:{}", src_path, n + 1), src.lines().nth(n).unwrap().trim()))),
  };

  let out_path = PathBuf::from(src_path).with_extension("o");
  if fs::write(&out_path, obj.out_obj()).is_err() {
    err_msg(&format!("could not write to {:?}", out_path), None);
  }
}
