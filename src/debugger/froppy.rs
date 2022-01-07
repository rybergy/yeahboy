use egui_glium::EguiGlium;
use epi::NativeTexture;
use std::rc::Rc;

use crate::{
    cpu::Cpu,
    debugger::{self, DebuggerWidget},
};

pub struct FroppyWidget {
    image_size: egui::Vec2,
    texture_id: egui::TextureId,
}

impl FroppyWidget {
    pub fn new(egui: &mut EguiGlium, display: &glium::Display) -> Self {
        let image = debugger::load_glium_image(include_bytes!("../../resources/froppy.png"));
        // Load to gpu memory
        // Allocate egui's texture id for GL texture
        let image_size = egui::Vec2::new(image.width as f32, image.height as f32);
        let texture = Rc::new(glium::texture::SrgbTexture2d::new(display, image).unwrap());
        let texture_id = egui.painter_mut().register_native_texture(texture);

        Self {
            image_size,
            texture_id,
        }
    }
}

impl DebuggerWidget for FroppyWidget {
    fn draw(&self, egui: &mut egui_glium::EguiGlium, _cpu: &mut Cpu) {
        egui::Window::new("Moral Support").show(egui.ctx(), |ui| {
            ui.image(self.texture_id, self.image_size);
        });
    }
}
