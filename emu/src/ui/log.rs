use eframe::egui;
use crate::EmuState;
use crate::ui::Window;

pub struct LogWindow {}

impl LogWindow {
  pub fn new() -> Self {
    Self {}
  }
}

impl Window for LogWindow {
  fn show(&mut self, state: &mut EmuState, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
      for (time, msg) in &state.log {
        ui.horizontal(|ui| {
          ui.monospace(format!("{:02}:{:02}:{:02}", time.hour(), time.minute(), time.second()));
          ui.label(msg);
        });
      }
    });
  }
}
