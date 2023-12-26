#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::ViewportBuilder;
use egui_demo_lib::DemoWindows;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([960.0, 720.0]),
        default_theme: eframe::Theme::Dark,
        follow_system_theme: false,
        ..Default::default()
    };
    eframe::run_native(
        "Egui demo",
        options,
        Box::new(|cc| {
            dear_egui::set_theme(
                &cc.egui_ctx,
                dear_egui::Theme::Imgui,
                dear_egui::Font::OpenSans,
            );
            Box::new(MyApp {
                demo: DemoWindows::default(),
            })
        }),
    )
}

struct MyApp {
    demo: DemoWindows,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo.ui(ctx);
    }
}
