use eframe::egui;
use crate::EmuState;
use crate::ui::Window;

pub struct MemoryWindow {}

impl MemoryWindow {
  pub fn new() -> Self {
    Self {}
  }
}

impl Window for MemoryWindow {
  fn show(&mut self, state: &mut EmuState, ui: &mut egui::Ui) {
    let columns = 8;
    egui::ScrollArea::vertical().auto_shrink([false, true]).show_rows(
      ui,
      ui.text_style_height(&egui::TextStyle::Monospace),
      state.emu.memory.len() / columns,
      |ui, range| {
        for row in range.clone() {
          ui.horizontal(|ui| {
            ui.monospace(format!("{:04x}", row * columns));
            for i in 0..columns {
              ui.add(egui::DragValue::new(&mut state.emu.memory[row * columns + i]).hexadecimal(2, true, false));
              // todo make an actual input
              // let response = ui.add(
              //   egui::TextEdit::singleline(&mut "2f".to_string())
              //     .margin(egui::Margin::same(0.0))
              //     .desired_width(24.0)
              //     .font(egui::TextStyle::Monospace),
              // );
            }
          });
        }
      },
    );
  }
}
