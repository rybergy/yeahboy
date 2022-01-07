use egui_glium::EguiGlium;
use epi::NativeTexture;
use std::rc::Rc;

use crate::{
    cpu::{Cpu, Flags},
    debugger::{self, DebuggerWidget},
};

pub struct RegistersWidget {}

impl RegistersWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl DebuggerWidget for RegistersWidget {
    fn draw(&self, egui: &mut egui_glium::EguiGlium, cpu: &mut Cpu) {
        egui::Window::new("Registers").show(egui.ctx(), |ui| {
            let reg = cpu.reg();
            ui.columns(3, |uis| {
                let single_ui = &mut uis[0];
                single_ui.heading("Single");
                single_ui.separator();
                let single_registers = [
                    ("a", reg.a()),
                    ("f", reg.f().into()),
                    ("b", reg.b()),
                    ("c", reg.c()),
                    ("d", reg.d()),
                    ("e", reg.e()),
                    ("h", reg.h()),
                    ("l", reg.l()),
                ];

                for (name, value) in single_registers.into_iter() {
                    single_ui
                        .add(egui::Label::new(format!("{}: 0x{:02x}", name, value)).monospace());
                }

                let double_ui = &mut uis[1];
                double_ui.heading("Double");
                double_ui.separator();

                let double_registers = [
                    ("af", reg.af()),
                    ("bc", reg.bc()),
                    ("de", reg.de()),
                    ("hl", reg.hl()),
                ];

                for (name, value) in double_registers.into_iter() {
                    double_ui
                        .add(egui::Label::new(format!("{}: 0x{:04x}", name, value)).monospace());
                }

                double_ui.heading("PC/SP");
                double_ui.separator();

                let pc_sp = [("sp", cpu.sp()), ("pc", cpu.pc())];

                for (name, value) in pc_sp.into_iter() {
                    double_ui
                        .add(egui::Label::new(format!("{}: 0x{:04x}", name, value)).monospace());
                }

                let flags_ui = &mut uis[2];
                flags_ui.heading("Flags");
                flags_ui.separator();

                let flags = [
                    ("Z", Flags::ZERO),
                    ("S", Flags::SUBTRACT),
                    ("H", Flags::HALF_CARRY),
                    ("C", Flags::CARRY),
                ];

                for (name, flag) in flags.into_iter() {
                    let (color, value) = if reg.f().contains(flag) {
                        (
                            egui::Color32::from_rgba_premultiplied(0x40, 0xd0, 0x60, 0xff),
                            1,
                        )
                    } else {
                        (
                            egui::Color32::from_rgba_premultiplied(0xd0, 0x60, 0x40, 0xff),
                            0,
                        )
                    };

                    flags_ui.add(
                        egui::Label::new(format!("{}: {}", name, value))
                            .text_color(color)
                            .monospace(),
                    );
                }
            });
        });
    }
}
