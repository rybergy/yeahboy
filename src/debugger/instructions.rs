use egui_glium::EguiGlium;
use epi::NativeTexture;
use std::{collections::BTreeMap, rc::Rc};

use crate::{
    cpu::{Cpu, Flags},
    debugger::{self, DebuggerWidget},
    instructions::DecodedInstruction,
};

struct InstructionRow {
    pub address: u16,
    pub instruction: DecodedInstruction,
}

pub struct InstructionsWidget {
    rows: Vec<InstructionRow>,
    addr_to_index: BTreeMap<u16, usize>,
}

impl InstructionsWidget {
    pub fn new(cpu: &Cpu) -> Self {
        let mut rows = Vec::new();
        let mut addr_to_index = BTreeMap::new();

        let mut pc = cpu.pc();
        let mut index = 0;

        while pc < 0x8000 {
            let inst = InstructionRow {
                address: pc,
                instruction: DecodedInstruction::decode(cpu.mmu(), pc as usize),
            };

            addr_to_index.insert(pc, index);

            pc += inst.instruction.len() as u16;

            rows.push(inst);

            index += 1;
        }

        Self {
            rows,
            addr_to_index,
        }
    }
}

impl DebuggerWidget for InstructionsWidget {
    fn draw(&self, egui: &mut egui_glium::EguiGlium, cpu: &mut Cpu) {
        egui::Window::new("Instructions").show(egui.ctx(), |ui| {
            let mut inst_index = self.addr_to_index[&cpu.pc()];

            let disp_rows = &self.rows[inst_index..inst_index + 20];
            // let disp_rows = &self.rows;

            // Columns: addr, bytes, action

            egui::ScrollArea::new([false, true]).show(ui, |ui| {
                egui::Grid::new("HELP")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .show(ui, |ui| {
                        for row in disp_rows.iter() {
                            let mut bytestr = String::new();
                            for byte in row.instruction.raw_bytes().iter() {
                                bytestr.push_str(&*format!("{:02x} ", byte))
                            }

                            let mut addr = egui::Label::new(format!("{:04x}", row.address))
                                .monospace()
                                .text_color(egui::Color32::LIGHT_BLUE);

                            if row.address == cpu.pc() {
                                addr = addr.background_color(egui::Color32::DARK_RED);
                            }

                            ui.add(addr);

                            ui.add(
                                egui::Label::new(format!("{}", bytestr))
                                    .monospace()
                                    .text_color(egui::Color32::LIGHT_GRAY),
                            );

                            ui.add(
                                egui::Label::new(format!("{}", row.instruction.action()))
                                    .monospace()
                                    .text_color(egui::Color32::from_rgb(0xCF, 0x9F, 0xFF)),
                            );

                            ui.end_row();
                        }
                    });
            });
        });
    }
}
