use egui_glium::EguiGlium;
use epi::NativeTexture;
use std::rc::Rc;

use crate::{
    bits,
    cpu::Cpu,
    debugger::{self, DebuggerWidget},
    mmu::Mmu,
};

pub struct MetadataWidget {
    title: String,
    licensee: u16,
    cart_type: u8,
    rom_size: u8,
    ram_size: u8,
}

impl MetadataWidget {
    pub fn new(mmu: &Mmu) -> Self {
        let mut title = String::new();
        for addr in (0x0134..=0x0142) {
            let byte = mmu.rb(addr);
            if byte == 0 {
                break;
            }
            title.push(byte as char);
        }
        let licensee = bits::pack_u16(mmu.rb(0x144), mmu.rb(0x145));
        let cart_type = mmu.rb(0x147);
        let rom_size = mmu.rb(0x148);
        let ram_size = mmu.rb(0x149);

        Self {
            title,
            licensee,
            cart_type,
            rom_size,
            ram_size,
        }
    }
}

impl DebuggerWidget for MetadataWidget {
    fn draw(&self, egui: &mut egui_glium::EguiGlium, _cpu: &mut Cpu) {
        egui::Window::new("Metadata").show(egui.ctx(), |ui| {
            let cart_type_str = match self.cart_type {
                0 => "ROM",
                1..=3 => "ROM+MBC1",
                5..=6 => "ROM+MBC2",
                _ => todo!(),
            };
            let rom_size_str = format!("{} Banks", 2_u8.pow(self.rom_size as u32 + 1));
            let ram_size_str = match self.ram_size {
                0 => "None",
                1 | 2 => "1 bank",
                3 => "4 banks",
                4 => "16 banks",
                _ => "<INVALID>",
            };

            let rows = [
                ("Title", self.title.clone()),
                ("Licensee", format!("{:02x}", self.licensee)),
                (
                    "Cart Type",
                    format!("{:02x} ({})", self.cart_type, cart_type_str),
                ),
                (
                    "ROM Size",
                    format!("{:02x} ({})", self.rom_size, rom_size_str),
                ),
                (
                    "RAM Size",
                    format!("{:02x} ({})", self.ram_size, ram_size_str),
                ),
            ];

            for (name, value) in rows.iter() {
                ui.add(egui::Label::new(format!("{}: {}", name, value)).monospace());
            }
        });
    }
}
