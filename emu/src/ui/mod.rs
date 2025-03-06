mod cpu_state;
mod memory;
mod display;
mod serial;
mod log;

use eframe::egui;
use crate::EmuState;

pub use cpu_state::CpuStateWindow;
pub use memory::MemoryWindow;
pub use display::DisplayWindow;
pub use serial::SerialWindow;
pub use log::LogWindow;

pub trait Window {
  fn build<'a>(&self, window: egui::Window<'a>) -> egui::Window<'a> {
    window
  }
  fn name(&self) -> &'static str;
  fn show(&mut self, state: &mut EmuState, ui: &mut egui::Ui);
}
