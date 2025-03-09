use std::fs;
use q16::obj::Obj;
use q16::util::{ArgParser, err_msg};

fn main() {
  let mut args = ArgParser::from_env();
  let out_path = match args.take_flag("-o") {
    Some(p) => p,
    None => return print_help(),
  };
  let paths = args.remaining();
  if paths.is_empty() {
    return print_help();
  }

  let mut out = Obj::new();
  for path in paths {
    let obj = match fs::read(&path).map_err(|e| e.to_string()).and_then(|b| Obj::load(&b)) {
      Ok(r) => r,
      Err(e) => err_msg(&format!("couldn't open '{}': {}", path, e), None),
    };
    if let Err(e) = out.extend(obj) {
      err_msg(&format!("couldn't link '{}': {}", path, e), None);
    }
  }

  let bin = match out.out_bin() {
    Ok(b) => b,
    Err(e) => err_msg(&e, None),
  };
  if fs::write(&out_path, bin).is_err() {
    err_msg(&format!("could not write to {:?}", out_path), None);
  }
}

fn print_help() {
  println!("q16-ld help:");
  println!("usage: q16-ld [input objects] -o <out binary>");
}
