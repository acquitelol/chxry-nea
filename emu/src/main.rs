mod ui;

use std::{fs, thread};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use std::path::Path;
use eframe::egui;
use time::OffsetDateTime;
use q16::Instruction;
use q16::emu::Emulator;
use q16::util::{CircularBuffer, ArgParser};
use crate::ui::{Window, CpuStateWindow, MemoryWindow, DisplayWindow, LogWindow};

pub const ONE_SEC_NANOS: u64 = 1_000_000_000;

fn main() {
  eframe::run_native(
    "q16 Emulator",
    eframe::NativeOptions {
      viewport: egui::ViewportBuilder::default().with_icon(eframe::icon_data::from_png_bytes(include_bytes!("icon.png")).unwrap()),
      ..Default::default()
    },
    Box::new(|cc| Ok(Box::new(App::new(cc)))),
  )
  .unwrap();
}

struct App {
  emu_state: Arc<Mutex<EmuState>>,
  windows: Vec<Box<dyn Window>>,
}

impl App {
  fn new(cc: &eframe::CreationContext) -> Self {
    let mut emu_state = EmuState::new();
    let mut args = ArgParser::from_env();
    if let Some(p) = args.take_flag("-b") {
      emu_state.load_binary(p);
    }

    let emu_state = Arc::new(Mutex::new(emu_state));
    spawn_emu_thread(emu_state.clone());

    let windows = vec![
      Box::new(CpuStateWindow::new()) as _,
      Box::new(MemoryWindow::new()) as _,
      Box::new(DisplayWindow::new(cc)) as _,
      Box::new(LogWindow::new()) as _,
    ];
    Self { emu_state, windows }
  }

  fn for_windows<F: FnMut(Arc<Mutex<EmuState>>, &mut dyn Window, &mut bool)>(&mut self, ctx: &egui::Context, mut f: F) {
    for w in &mut self.windows {
      let id = egui::Id::new(w.name());
      let mut open = ctx.data_mut(|d| d.get_persisted(id).unwrap_or(true));
      f(self.emu_state.clone(), w.as_mut(), &mut open);
      ctx.data_mut(|d| d.insert_persisted(id, open));
    }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
    ctx.request_repaint();

    egui::TopBottomPanel::top("menu").show(ctx, |ui| {
      egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
          if ui.button("Load Binary").clicked() {
            let state = self.emu_state.clone();
            thread::spawn(move || {
              if let Some(path) = rfd::FileDialog::new().pick_file() {
                state.lock().unwrap().load_binary(&path);
              }
            });
          }
        });
        ui.menu_button("Windows", |ui| {
          self.for_windows(ctx, |_, w, open| {
            ui.toggle_value(open, w.name());
          });
        });
      });
    });

    self.for_windows(ctx, |emu, w, open| {
      w.build(egui::Window::new(w.name()).open(open))
        .show(ctx, |ui| w.show(&mut emu.lock().unwrap(), ui));
    });
  }
}

struct EmuState {
  emu: Emulator,
  last_instr: Option<Instruction>,
  target_speed: u64,
  time_history: CircularBuffer<Duration, 100_000>,
  log: Vec<(OffsetDateTime, String)>,
}

impl EmuState {
  fn new() -> Self {
    Self {
      emu: Emulator::new(),
      last_instr: None,
      target_speed: 25_000_000,
      time_history: CircularBuffer::new(),
      log: vec![],
    }
  }

  pub fn load_binary<P: AsRef<Path>>(&mut self, path: P) {
    match fs::read(&path) {
      Ok(bin) => {
        self.emu.reset();
        self.emu.memory.splice(..bin.len(), bin);
        self.last_instr = None;
        self.log(format!("Loaded binary from '{}'.", path.as_ref().display()));
      }
      Err(_) => self.log(format!("Couldn't load '{}'.", path.as_ref().display())),
    };
  }

  pub fn log(&mut self, msg: String) {
    self.log.push((OffsetDateTime::now_utc(), msg));
  }
}

fn spawn_emu_thread(state: Arc<Mutex<EmuState>>) {
  let mut carry_forward = Duration::ZERO;
  thread::spawn(move || loop {
    let start = Instant::now();
    let mut state = state.lock().unwrap();
    if state.emu.running() {
      match state.emu.cycle() {
        Some(i) => state.last_instr = Some(i),
        None => state.log("Resetting".to_string()),
      };

      let target_time = Duration::from_nanos(ONE_SEC_NANOS / state.target_speed);
      let elapsed = start.elapsed();
      if elapsed > target_time {
        carry_forward += elapsed - target_time;
        state.time_history.push(elapsed);
      } else {
        let mut interval = target_time - elapsed;
        if carry_forward >= interval {
          carry_forward -= interval;
          state.time_history.push(elapsed);
        } else {
          interval -= carry_forward;
          state.time_history.push(elapsed + interval);
          carry_forward = Duration::ZERO;
          drop(state);
          thread::sleep(interval);
        }
      }
    } else {
      drop(state);
      thread::sleep(Duration::from_millis(100));
    }
  });
}
