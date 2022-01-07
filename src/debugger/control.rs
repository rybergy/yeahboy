use egui_glium::EguiGlium;
use epi::NativeTexture;
use std::rc::Rc;

use crate::{
    cpu::{Cpu, Flags},
    debugger::{self, DebuggerWidget},
};

pub struct ControlWidget {}

impl ControlWidget {
    pub fn new() -> Self {
        Self {}
    }
}

impl DebuggerWidget for ControlWidget {
    fn draw(&self, egui: &mut egui_glium::EguiGlium, cpu: &mut Cpu) {
        egui::Window::new("Control").show(egui.ctx(), |ui| {
            if ui.button("Step").clicked() {
                cpu.step()
            }

            if ui.button("Continue").clicked() {
                loop {
                    cpu.step()
                }
            }
        });
    }
}
