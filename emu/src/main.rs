use eframe::egui;
use q16::emu::Emulator;

fn main() {
  eframe::run_native(
    "q16 emu",
    eframe::NativeOptions::default(),
    Box::new(|_| Ok(Box::new(App::new()))),
  )
  .unwrap();
}

struct App {
  emu: Emulator,
}

impl App {
  fn new() -> Self {
    Self { emu: Emulator::new() }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("...");
    });
  }
}
