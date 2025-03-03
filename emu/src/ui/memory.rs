use eframe::egui;
use crate::EmuState;
use crate::ui::Window;

pub struct MemoryWindow {
  scroll_target: u16,
}

impl MemoryWindow {
  pub fn new() -> Self {
    Self { scroll_target: 0 }
  }
}

impl Window for MemoryWindow {
  fn name(&self) -> &'static str {
    "Memory"
  }

  fn show(&mut self, state: &mut EmuState, ui: &mut egui::Ui) {
    let columns = 8;
    let text_height = ui.text_style_height(&egui::TextStyle::Monospace);
    let row_height = text_height + ui.spacing().item_spacing.y;
    let mut scroll = false;

    ui.horizontal(|ui| {
      ui.label("Jump to");
      scroll = ui
        .add(egui::DragValue::new(&mut self.scroll_target).hexadecimal(4, true, false))
        .changed();
    });

    ui.separator();

    let mut area = egui::ScrollArea::vertical().auto_shrink([false, true]);
    if scroll {
      area = area.vertical_scroll_offset((self.scroll_target as usize / columns) as f32 * row_height);
    }
    let offset = area
      .show_rows(ui, text_height, state.emu.memory.len() / columns, |ui, range| {
        for row in range {
          ui.horizontal(|ui| {
            ui.monospace(format!("{:04x}", row * columns));
            for i in 0..columns {
              ui.add(egui::DragValue::new(&mut state.emu.memory[row * columns + i]).hexadecimal(2, true, false));
            }
          });
        }
      })
      .state
      .offset;
    self.scroll_target = ((offset.y / row_height) as usize * columns) as _;
  }
}
