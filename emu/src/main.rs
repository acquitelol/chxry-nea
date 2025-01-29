use eframe::egui;
use q16::Register;
use q16::emu::{Emulator, set_bit};

fn main() {
  eframe::run_native("q16 emu", eframe::NativeOptions::default(), Box::new(|_| Ok(Box::new(App::new())))).unwrap();
}

struct App {
  emu: Emulator,
  selected_index: Option<usize>,
  input_buffer: String,
}

impl App {
  fn new() -> Self {
    Self {
      emu: Emulator::new(),
      selected_index: None,
      input_buffer: String::new(),
    }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
    egui::Window::new("cpu state").resizable(true).show(ctx, |ui| {
      ui.heading("registers:");
      reg_ui(
        ui,
        &mut self.emu,
        &[
          Register::R1,
          Register::R2,
          Register::R3,
          Register::R4,
          Register::R5,
          Register::R6,
          Register::R7,
          Register::R8,
        ],
      );
      reg_ui(ui, &mut self.emu, &[Register::PC, Register::SP, Register::STS]);
    });
    egui::Window::new("memory").resizable(true).show(ctx, |ui| {
      let columns = 8;
      egui::ScrollArea::vertical().auto_shrink([false, true]).show_rows(
        ui,
        ui.text_style_height(&egui::TextStyle::Monospace),
        self.emu.ram.len() / columns,
        |ui, range| {
          for row in range.clone() {
            ui.horizontal(|ui| {
              ui.monospace(format!("{:04x}", row * columns));
              for i in 0..columns {
                ui.add(egui::DragValue::new(&mut self.emu.ram[row * columns + i]).hexadecimal(2, true, false));
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
    });
  }
}

fn reg_ui(ui: &mut egui::Ui, emu: &mut Emulator, regs: &[Register]) {
  ui.horizontal(|ui| {
    for i in 0..regs.len() {
      ui.vertical(|ui| {
        ui.label(format!("{:?}", regs[i]));
        ui.add(egui::DragValue::new(emu.registers.get_mut(regs[i]).unwrap()).hexadecimal(4, true, false));
      });
    }
  });
}
