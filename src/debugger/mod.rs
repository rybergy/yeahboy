//! Example how to use [epi::NativeTexture] with glium.
use egui_glium::EguiGlium;
use epi::NativeTexture;
use glium::glutin;
use std::rc::Rc;

use crate::cpu::Cpu;

use self::{
    control::ControlWidget, froppy::FroppyWidget, instructions::InstructionsWidget,
    meta::MetadataWidget, registers::RegistersWidget,
};

mod control;
mod froppy;
mod instructions;
mod meta;
mod registers;

pub trait DebuggerWidget {
    fn draw(&self, egui: &mut EguiGlium, cpu: &mut Cpu);
}

fn create_display(event_loop: &glutin::event_loop::EventLoop<()>) -> glium::Display {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 400.0,
            height: 300.0,
        })
        .with_title("yeahboy")
        .with_maximized(true);

    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(true);

    glium::Display::new(window_builder, context_builder, event_loop).unwrap()
}

fn load_glium_image(image_data: &[u8]) -> glium::texture::RawImage2d<u8> {
    // Load image using the image crate:
    let image = image::load_from_memory(image_data).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();

    // Premultiply alpha:
    let pixels: Vec<_> = image
        .into_vec()
        .chunks_exact(4)
        .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
        .flat_map(|color| color.to_array())
        .collect();

    // Convert to glium image:
    glium::texture::RawImage2d::from_raw_rgba(pixels, image_dimensions)
}

pub fn run(rom: Vec<u8>) {
    let mut cpu = Cpu::new(rom);

    let event_loop = glutin::event_loop::EventLoop::with_user_event();

    let display = create_display(&event_loop);
    let mut egui = egui_glium::EguiGlium::new(&display);

    let mut fonts = egui::FontDefinitions::default();
    fonts.family_and_size.insert(
        egui::TextStyle::Monospace,
        (egui::FontFamily::Monospace, 20.0),
    );
    // fonts
    //     .family_and_size
    //     .insert(egui::TextStyle::Heading, (egui::FontFamily::Prop, 32.0));
    egui.ctx().set_fonts(fonts);

    let widgets: Vec<Box<dyn DebuggerWidget>> = vec![
        Box::new(FroppyWidget::new(&mut egui, &display)),
        Box::new(RegistersWidget::new()),
        Box::new(InstructionsWidget::new(&cpu)),
        Box::new(MetadataWidget::new(cpu.mmu())),
        Box::new(ControlWidget::new()),
    ];

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            egui.begin_frame(&display);

            for widget in widgets.iter() {
                widget.draw(&mut egui, &mut cpu)
            }

            let (needs_repaint, shapes) = egui.end_frame(&display);

            *control_flow = if needs_repaint {
                display.gl_window().window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else {
                glutin::event_loop::ControlFlow::Wait
            };

            {
                use glium::Surface as _;
                let mut target = display.draw();

                let clear_color = egui::Rgba::from_rgb(0.05, 0.35, 0.25);
                target.clear_color(
                    clear_color[0],
                    clear_color[1],
                    clear_color[2],
                    clear_color[3],
                );

                // draw things behind egui here

                egui.paint(&display, &mut target, shapes);

                // draw things on top of egui here

                target.finish().unwrap();
            }
        };

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            glutin::event::Event::WindowEvent { event, .. } => {
                if egui.is_quit_event(&event) {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                }

                egui.on_event(&event);

                display.gl_window().window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }

            _ => (),
        }
    });
}
