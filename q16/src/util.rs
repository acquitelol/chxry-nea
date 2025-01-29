use owo_colors::OwoColorize;

/// prints an error message with optional context of where and what went wrong
pub fn err_msg(msg: &str, ctx: Option<(&str, &str)>) {
  print!("{} {}", "error:".red().bold(), msg.bold());
  match ctx {
    Some((location, content)) => {
      println!(" ({})", location);
      println!("   {} {}", "-->".blue(), content);
    }
    None => println!(),
  }
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => { Err(format!($($arg)*)) };
}

#[macro_export]
macro_rules! assert {
    ($cond:expr, $($arg:tt)*) => { if $cond { Ok(()) } else { err!($($arg)*) } };
}

pub use {err, assert};
