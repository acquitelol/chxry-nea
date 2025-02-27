use std::{process, mem};
use owo_colors::OwoColorize;

/// prints an error message with optional context of where and what went wrong
/// exits with error code 1
pub fn err_msg(msg: &str, ctx: Option<(&str, &str)>) -> ! {
  print!("{} {}", "error:".red().bold(), msg.bold());
  match ctx {
    Some((location, content)) => {
      println!(" ({})", location);
      println!("   {} {}", "-->".blue(), content);
    }
    None => println!(),
  }
  process::exit(1)
}

pub struct CircularBuffer<T, const N: usize> {
  buf: [T; N],
  head: usize,
  len: usize,
}

impl<T, const N: usize> CircularBuffer<T, N> {
  pub fn new() -> Self {
    Self {
      // safety: we cant read this without incrementing len, which only occurs when valid data is written
      buf: unsafe { mem::zeroed() },
      head: 0,
      len: 0,
    }
  }

  pub fn clear(&mut self) {
    self.head = 0;
    self.len = 0;
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn push(&mut self, item: T) {
    self.buf[self.head] = item;
    self.head = (self.head + 1) % N;
    if self.len < N {
      self.len += 1;
    }
  }

  /// unordered
  pub fn items(&self) -> &[T] {
    &self.buf[..self.len]
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_circular_buf() {
    let mut buf = CircularBuffer::<i32, 10>::new();
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.items(), []);

    for i in 0..5 {
      buf.push(i);
    }
    assert_eq!(buf.len(), 5);
    assert_eq!(buf.items(), [0, 1, 2, 3, 4]);

    for i in 5..15 {
      buf.push(i);
    }
    assert_eq!(buf.len(), 10);
    assert_eq!(buf.items(), [10, 11, 12, 13, 14, 5, 6, 7, 8, 9]);

    buf.clear();
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.items(), []);
  }
}
