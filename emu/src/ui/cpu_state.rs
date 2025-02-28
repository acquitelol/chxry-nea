use std::time::Duration;
use eframe::egui;
use q16::Register;
use crate::{EmuState, ONE_SEC_NANOS};
use crate::ui::Window;

pub struct CpuStateWindow {}

impl CpuStateWindow {
  pub fn new() -> Self {
    Self {}
  }
}

impl Window for CpuStateWindow {
  fn show(&mut self, state: &mut EmuState, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
      ui.heading("State:");
      let running = state.emu.running();
      if ui.button(egui::RichText::new(if running { "■" } else { "▶" }).heading()).clicked() {
        state.emu.set_run(!running);
      }
      if ui.button(egui::RichText::new("⏩").heading()).clicked() {
        state.emu.cycle();
      }
    });
    ui.horizontal(|ui| {
      ui.label("Emulation speed:");
      if ui
        .add(
          egui::DragValue::new(&mut state.target_speed)
            .suffix("Hz")
            .range(1..=50_000_000)
            .speed(10),
        )
        .changed()
      {
        state.time_history.clear();
      }
      if state.time_history.len() > 0 {
        let measured_speed =
          ONE_SEC_NANOS / (state.time_history.items().iter().sum::<Duration>().as_nanos() as u64 / state.time_history.len() as u64);
        ui.label(format!("(actual: {}Hz)", measured_speed));
        if measured_speed < state.target_speed * 4 / 5 {
          ui.colored_label(egui::Color32::RED, "can't keep up!");
        }
      }
    });
    ui.horizontal(|ui| {
      ui.label("Last instruction:");
      ui.monospace(state.emu.last_instr.map(|i| i.to_string()).unwrap_or("---".to_string()));
    });
    ui.separator();

    ui.heading("Registers:");
    reg_ui(
      ui,
      state,
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
    reg_ui(ui, state, &[Register::PC, Register::SP, Register::RA, Register::STS]);
  }
}

fn reg_ui(ui: &mut egui::Ui, state: &mut EmuState, regs: &[Register]) {
  ui.horizontal(|ui| {
    for r in regs {
      ui.vertical(|ui| {
        ui.label(r.to_string());
        let drag = egui::DragValue::new(state.emu.registers.get_mut(*r).unwrap());
        ui.add(if *r == Register::STS {
          drag.binary(16, true)
        } else {
          drag.hexadecimal(4, true, false)
        });
      });
    }
  });
}
