use eframe::egui;
use q16::addr;
use crate::EmuState;
use crate::ui::Window;

const DISPLAY_WIDTH: usize = 128;
const DISPLAY_HEIGHT: usize = 96;

pub struct DisplayWindow {
  texture: egui::TextureHandle,
  hover_pos: (usize, usize),
}

impl DisplayWindow {
  pub fn new(cc: &eframe::CreationContext) -> Self {
    Self {
      texture: cc
        .egui_ctx
        .load_texture("display", egui::ColorImage::default(), egui::TextureOptions::NEAREST),
      hover_pos: (0, 0),
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
      let (r, g, b) = parse_r3g3b2(state.emu.load_byte(addr::VRAM + i as u16));
      pixels.push(egui::Color32::from_rgb(
        (r as f32 / 7.0 * 255.0) as _,
        (g as f32 / 7.0 * 255.0) as _,
        (b as f32 / 3.0 * 255.0) as _,
      ));
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
        let local_pos = (pos - response.rect.min) / response.rect.size();
        self.hover_pos = (
          (local_pos.x * DISPLAY_WIDTH as f32) as usize,
          (local_pos.y * DISPLAY_HEIGHT as f32) as usize,
        );
      }
    }

    let addr = self.hover_pos.1 * DISPLAY_WIDTH + self.hover_pos.0;
    let color = state.emu.memory[addr::VRAM as usize + addr];
    let (r, g, b) = parse_r3g3b2(color);
    ui.horizontal(|ui| {
      ui.label("Hovered pixel:");
      ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
      ui.monospace(format!(
        "{}, {} (0x{:04x}) - 0x{:02x} ",
        self.hover_pos.0 + 1,
        self.hover_pos.1 + 1,
        addr,
        color
      ));
      ui.monospace(egui::RichText::new(format!("{:03b}", r)).color(egui::Color32::RED));
      ui.monospace(egui::RichText::new(format!("{:03b}", g)).color(egui::Color32::GREEN));
      ui.monospace(egui::RichText::new(format!("{:02b}", b)).color(egui::Color32::BLUE));
    });
  }
}

fn parse_r3g3b2(color: u8) -> (u8, u8, u8) {
  (color & 0b111, (color >> 3) & 0b111, (color >> 6) & 0b11)
}
