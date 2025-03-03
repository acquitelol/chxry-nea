use eframe::egui;
use q16::addr;
use crate::EmuState;
use crate::ui::Window;

const DISPLAY_WIDTH: usize = 128;
const DISPLAY_HEIGHT: usize = 96;

pub struct DisplayWindow {
  texture: egui::TextureHandle,
}

impl DisplayWindow {
  pub fn new(cc: &eframe::CreationContext) -> Self {
    Self {
      texture: cc
        .egui_ctx
        .load_texture("display", egui::ColorImage::default(), egui::TextureOptions::NEAREST),
    }
  }
}

impl Window for DisplayWindow {
  fn name(&self) -> &'static str {
    "Display"
  }

  fn build<'a>(&self, window: egui::Window<'a>) -> egui::Window<'a> {
    window.default_width(896.0)
  }

  fn show(&mut self, state: &mut EmuState, ui: &mut egui::Ui) {
    let mut pixels = Vec::with_capacity(DISPLAY_WIDTH * DISPLAY_HEIGHT);
    for i in 0..DISPLAY_WIDTH * DISPLAY_HEIGHT {
      let byte = state.emu.memory[addr::VRAM as usize + i];
      // let r = byte & 0b11;
      // let g = (byte >> 2) & 0b11;
      // let b = (byte >> 4) & 0b11;
      // pixels.push(egui::Color32::from_rgb(r * 85, g * 85, b * 85));
      // todo use a real colour palette f
      pixels.push(egui::Color32::from_gray(byte));
    }

    self.texture.set(
      egui::ColorImage {
        size: [DISPLAY_WIDTH, DISPLAY_HEIGHT],
        pixels,
      },
      egui::TextureOptions::NEAREST,
    );
    let response = ui.add(egui::Image::new(egui::load::SizedTexture::from_handle(&self.texture)).shrink_to_fit());

    if response.hovered() {
      if let Some(pos) = ui.ctx().pointer_latest_pos() {
        let transformed = (pos - response.rect.min) / response.rect.size();
        state.emu.memory[addr::IO as usize] = (transformed.x * DISPLAY_WIDTH as f32) as u8;
        state.emu.memory[addr::IO as usize + 1] = (transformed.y * DISPLAY_HEIGHT as f32) as u8;
      }
    }
  }
}
