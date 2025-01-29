use std::{env, fs};
use q16::obj::Obj;
use q16::util::err_msg;

fn main() {
  let mut paths = env::args().skip(1);
  let out_path = match paths.next() {
    Some(p) => p,
    None => {
      println!("q16-ld help:");
      println!("usage: q16-ld <out> [input files]");
      return;
    }
  };
  let mut out = Obj::new();
  for path in paths {
    let obj = match fs::read(&path).map_err(|e| e.to_string()).and_then(|b| Obj::load(&b)) {
      Ok(r) => r,
      Err(e) => return err_msg(&format!("couldn't open '{}' - {}", path, e), None),
    };
    if let Err(e) = out.extend(obj) {
      return err_msg(&format!("couldn't link '{}' - {}", path, e), None);
    }
  }

  let bin = match out.out_bin() {
    Ok(b) => b,
    Err(e) => return err_msg(&e, None),
  };
  if fs::write(&out_path, bin).is_err() {
    err_msg(&format!("could not write to {:?}", out_path), None);
  }
}
