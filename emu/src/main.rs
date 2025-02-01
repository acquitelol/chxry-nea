use std::{env, fs};
use indexmap::IndexMap;
use eframe::egui;
use q16::Register;
use q16::emu::Emulator;

fn main() {
  eframe::run_native("q16 emu", eframe::NativeOptions::default(), Box::new(|_| Ok(Box::new(App::new())))).unwrap();
}

struct App {
  emu: Emulator,
  windows: IndexMap<String, Box<dyn Window>>,
}

impl App {
  fn new() -> Self {
    let mut emu = Emulator::new();
    if let Some(p) = env::args().nth(1) {
      let bin = fs::read(p).unwrap(); // todo just ignore and log
      emu.ram.splice(0..bin.len(), bin);
    }

    let mut windows = IndexMap::new();
    windows.insert("cpu state".to_string(), Box::new(CpuStateWindow::new()) as _);
    windows.insert("memory".to_string(), Box::new(MemoryWindow::new()) as _);
    Self { emu, windows }
  }

  fn for_windows<F: FnMut(&mut Emulator, &str, &dyn Window, &mut bool)>(&mut self, ctx: &egui::Context, mut f: F) {
    for (n, w) in &self.windows {
      let id = egui::Id::new(n);
      let mut open = ctx.data_mut(|d| d.get_persisted(id).unwrap_or(true));
      f(&mut self.emu, &n, w.as_ref(), &mut open);
      ctx.data_mut(|d| d.insert_persisted(id, open));
    }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
    // todo run on seperate thread and time
    let running = self.emu.running();
    if running {
      self.emu.cycle();
    }
    // tmp!! only needed because emulation is currently running on ui thread
    ctx.request_repaint();

    egui::TopBottomPanel::top("menu").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        ui.menu_button("Windows", |ui| {
          self.for_windows(ctx, |_, n, _, open| {
            ui.toggle_value(open, n);
          });
        });
      });
    });

    self.for_windows(ctx, |emu, n, w, open| {
      egui::Window::new(n).resizable(true).open(open).show(ctx, |ui| w.show(emu, ui));
    });
  }
}

fn reg_ui(ui: &mut egui::Ui, emu: &mut Emulator, regs: &[Register]) {
  ui.horizontal(|ui| {
    for i in 0..regs.len() {
      ui.vertical(|ui| {
        ui.label(format!("{}", regs[i]));
        ui.add(egui::DragValue::new(emu.registers.get_mut(regs[i]).unwrap()).hexadecimal(4, true, false));
      });
    }
  });
}

trait Window {
  fn show(&self, emu: &mut Emulator, ui: &mut egui::Ui);
}

struct CpuStateWindow {}

impl CpuStateWindow {
  fn new() -> Self {
    Self {}
  }
}

impl Window for CpuStateWindow {
  fn show(&self, emu: &mut Emulator, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
      ui.heading("state:");
      let running = emu.running();
      if ui.button(egui::RichText::new(if running { "■" } else { "▶" }).heading()).clicked() {
        emu.set_run(!running);
      }
      if ui.button(egui::RichText::new("⏩").heading()).clicked() {
        emu.cycle();
      }
    });
    ui.separator();

    ui.heading("registers:");
    reg_ui(
      ui,
      emu,
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
    reg_ui(ui, emu, &[Register::PC, Register::SP, Register::STS]);
  }
}

struct MemoryWindow {}

impl MemoryWindow {
  fn new() -> Self {
    Self {}
  }
}

impl Window for MemoryWindow {
  fn show(&self, emu: &mut Emulator, ui: &mut egui::Ui) {
    let columns = 8;
    egui::ScrollArea::vertical().auto_shrink([false, true]).show_rows(
      ui,
      ui.text_style_height(&egui::TextStyle::Monospace),
      emu.ram.len() / columns,
      |ui, range| {
        for row in range.clone() {
          ui.horizontal(|ui| {
            ui.monospace(format!("{:04x}", row * columns));
            for i in 0..columns {
              ui.add(egui::DragValue::new(&mut emu.ram[row * columns + i]).hexadecimal(2, true, false));
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
