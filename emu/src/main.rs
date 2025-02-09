mod ui;

use std::{env, fs, thread};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use indexmap::IndexMap;
use eframe::egui;
use q16::emu::Emulator;
use crate::ui::{Window, CpuStateWindow, MemoryWindow, DisplayWindow};

const ONE_SEC_NANOS: u64 = 1_000_000_000;

fn main() {
  eframe::run_native(
    "q16 emu",
    eframe::NativeOptions::default(),
    Box::new(|cc| Ok(Box::new(App::new(cc)))),
  )
  .unwrap();
}

struct App {
  emu_state: Arc<Mutex<EmuState>>,
  windows: IndexMap<String, Box<dyn Window>>,
}

impl App {
  fn new(cc: &eframe::CreationContext) -> Self {
    let mut emu = Emulator::new();
    // tmp
    if let Some(p) = env::args().nth(1) {
      let bin = fs::read(p).unwrap();
      emu.memory.splice(0..bin.len(), bin);
    }
    let emu_state = Arc::new(Mutex::new(EmuState::new(emu)));
    spawn_emu_thread(emu_state.clone());

    let mut windows = IndexMap::new();
    windows.insert("CPU State".to_string(), Box::new(CpuStateWindow::new()) as _);
    windows.insert("Memory".to_string(), Box::new(MemoryWindow::new()) as _);
    windows.insert("Display".to_string(), Box::new(DisplayWindow::new(cc)) as _);
    Self { emu_state, windows }
  }

  fn for_windows<F: FnMut(Arc<Mutex<EmuState>>, &str, &mut dyn Window, &mut bool)>(&mut self, ctx: &egui::Context, mut f: F) {
    for (n, w) in &mut self.windows {
      let id = egui::Id::new(n);
      let mut open = ctx.data_mut(|d| d.get_persisted(id).unwrap_or(true));
      f(self.emu_state.clone(), n, w.as_mut(), &mut open);
      ctx.data_mut(|d| d.insert_persisted(id, open));
    }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
    ctx.request_repaint();

    egui::TopBottomPanel::top("menu").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |_| {});
        ui.menu_button("Windows", |ui| {
          self.for_windows(ctx, |_, n, _, open| {
            ui.toggle_value(open, n);
          });
        });
      });
    });

    self.for_windows(ctx, |emu, n, w, open| {
      w.build(egui::Window::new(n).open(open))
        .show(ctx, |ui| w.show(&mut emu.lock().unwrap(), ui));
    });
  }
}

struct EmuState {
  emu: Emulator,
  freq_hz: u64,
  freq_warning: bool,
}

impl EmuState {
  fn new(emu: Emulator) -> Self {
    Self {
      emu,
      freq_hz: 1_000_000,
      freq_warning: false,
    }
  }
}

fn spawn_emu_thread(state: Arc<Mutex<EmuState>>) {
  thread::spawn(move || loop {
    let start = Instant::now();
    let mut state = state.lock().unwrap();
    if state.emu.running() {
      state.emu.cycle();
    }

    let interval = Duration::from_nanos(ONE_SEC_NANOS / state.freq_hz).checked_sub(start.elapsed());
    state.freq_warning = interval.is_none();
    drop(state);
    if let Some(d) = interval {
      thread::sleep(d);
    }
  });
}
