use eframe::egui;
use crate::EmuState;
use crate::ui::Window;

pub struct SerialWindow {
  input_buf: String,
}

impl SerialWindow {
  pub fn new() -> Self {
    Self { input_buf: String::new() }
  }
}

impl Window for SerialWindow {
  fn name(&self) -> &'static str {
    "Serial Console"
  }

  fn show(&mut self, state: &mut EmuState, ui: &mut egui::Ui) {
    ui.monospace(String::from_utf8_lossy(&state.serial_out));

    ui.separator();
    if egui::TextEdit::singleline(&mut self.input_buf)
      .hint_text("Press enter to send")
      .desired_width(f32::INFINITY)
      .show(ui)
      .response
      .lost_focus()
      && ui.input(|i| i.key_pressed(egui::Key::Enter))
    {
      state.serial_in_queue.extend(self.input_buf.as_bytes());
      state.serial_in_queue.push_back(b'\n');
      self.input_buf.clear();
    }
    if !state.serial_in_queue.is_empty() {
      ui.label(format!("{} bytes in queue", state.serial_in_queue.len()));
    }
  }
}
