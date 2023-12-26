#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{ops::RangeInclusive, time::Instant};

use eframe::egui;
use egui::Layout;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(428.0, 445.0)),
        default_theme: eframe::Theme::Dark,
        follow_system_theme: false,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            //dear_egui::imgui::set_theme(&cc.egui_ctx, dear_egui::imgui::Font::OpenSans);
            dear_egui::set_theme(
                &cc.egui_ctx,
                dear_egui::Theme::Imgui,
                dear_egui::Font::OpenSans,
            );
            Box::new(MyApp {
                theme: dear_egui::Theme::Imgui,
                font: dear_egui::Font::OpenSans,
                some_bool: false,
                counter: 0,
                last_frame: Instant::now(),
                float: 0.3,
                r: 0,
                g: 0,
                b: 0,
                custom_hue: 0.5,
                custom_brightness: 0.5,
            })
        }),
    )
}

struct MyApp {
    theme: dear_egui::Theme,
    custom_hue: f32,
    custom_brightness: f32,
    font: dear_egui::Font,
    some_bool: bool,
    counter: i32,
    last_frame: Instant,
    float: f32,
    r: u8,
    g: u8,
    b: u8,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        dear_egui::set_theme(ctx, self.theme, self.font);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dear-Egui");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Theme:");
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    let mut bool = matches!(self.font, dear_egui::Font::OpenSans);
                    if ui.checkbox(&mut bool, "OpenSans").changed() && bool {
                        self.font = dear_egui::Font::OpenSans;
                    }
                    let mut bool = matches!(self.font, dear_egui::Font::ProggyClean);
                    if ui.checkbox(&mut bool, "Proggy Clean").changed() && bool {
                        self.font = dear_egui::Font::ProggyClean;
                    }
                });
            });
            if let dear_egui::Theme::Custom { hue, brightness } = &mut self.theme {
                ui.horizontal(|ui| {
                    ui.label("Hue:");
                    ui.add(egui::DragValue::new(hue).clamp_range(RangeInclusive::new(0.0, 360.0)));
                    ui.label("Brightness:");
                    ui.add(
                        egui::DragValue::new(brightness)
                            .clamp_range(RangeInclusive::new(0.0, 2.0))
                            .speed(0.01),
                    );
                });
            }
            ui.horizontal(|ui| {
                let mut tab = |theme: dear_egui::Theme, label: &str| {
                    let text_styles = ui.style().text_styles.clone();
                    ui.set_style(theme.get_style());
                    ui.style_mut().text_styles = text_styles;
                    ui.visuals_mut().widgets.inactive.rounding = egui::Rounding {
                        se: 0.0,
                        sw: 0.0,
                        ..ui.visuals().widgets.inactive.rounding
                    };
                    ui.visuals_mut().widgets.hovered.rounding = egui::Rounding {
                        se: 0.0,
                        sw: 0.0,
                        ..ui.visuals().widgets.hovered.rounding
                    };
                    ui.visuals_mut().widgets.active.rounding = egui::Rounding {
                        se: 0.0,
                        sw: 0.0,
                        ..ui.visuals().widgets.active.rounding
                    };
                    ui.selectable_value(&mut self.theme, theme, label);
                };
                tab(dear_egui::Theme::Imgui, "Imgui");
                tab(dear_egui::Theme::Cadmium, "Cadmium");
                tab(dear_egui::Theme::Acid, "Acid");
                tab(dear_egui::Theme::Forest, "Forest");
                tab(dear_egui::Theme::Sky, "Sky");
                tab(dear_egui::Theme::Iris, "Iris");
                tab(dear_egui::Theme::Violet, "Violet");
                tab(dear_egui::Theme::Raspberry, "Raspberry");
                tab(
                    dear_egui::Theme::Custom {
                        hue: self.custom_hue,
                        brightness: self.custom_brightness,
                    },
                    "Custom",
                );
            });
            //ui.allocate_space(egui::vec2(0.0, -8.0));
            //ui.separator();
            ui.painter().rect_filled(
                egui::Rect::from_min_size(
                    ui.cursor().min + egui::vec2(0.0, -ui.spacing().item_spacing.y),
                    egui::vec2(ui.available_width(), 2.0),
                ),
                0.0,
                ui.visuals().selection.bg_fill,
            );
            //ui.allocate_space(egui::vec2(0.0, 0.0));

            egui::Frame::none()
                .inner_margin(egui::Margin::same(3.0))
                .show(ui, |ui| {
                    ui.label("This is some useful text.");
                    ui.checkbox(&mut self.some_bool, "Demo Window");
                    ui.checkbox(&mut self.some_bool, "Another Window");

                    ui.add(
                        egui::Slider::new(&mut self.float, RangeInclusive::new(0.0, 1.0))
                            .text("float"),
                    );
                    ui.horizontal(|ui| {
                        ui.add_sized([80.0, 18.0], egui::DragValue::new(&mut self.r).prefix("R:"));
                        ui.add_sized([80.0, 18.0], egui::DragValue::new(&mut self.g).prefix("G:"));
                        ui.add_sized([80.0, 18.0], egui::DragValue::new(&mut self.b).prefix("B:"));
                        _ = ui.button("test");
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Button").clicked() {
                            self.counter += 1;
                        };
                        ui.label(format!("counter = {}", self.counter));
                    });

                    let now = Instant::now();
                    let delta = now - self.last_frame;
                    let fps = 1.0 / delta.as_secs_f32();
                    ui.label(format!(
                        "Application average {:.3}ms/frame ({:.1} FPS",
                        delta.as_secs_f32() * 1000.0,
                        fps,
                    ));
                    self.last_frame = now;

                    ui.horizontal(|ui| {
                        _ = ui.button("test");
                        ui.checkbox(&mut self.some_bool, "test");
                        _ = ui.button("test");
                        ui.radio_value(&mut self.some_bool, true, "test");
                    });

                    ui.horizontal(|ui| {
                        ui.label("pick one:");
                        egui::ComboBox::from_label("")
                            .selected_text("First one")
                            .show_ui(ui, |ui| {
                                _ = ui.selectable_label(true, "First one");
                                _ = ui.selectable_label(false, "Second one");
                                _ = ui.selectable_label(false, "Third one");
                            });
                    });

                    ui.separator();
                    ui.label("Label text");
                    ui.small("Small text");
                    ui.heading("Heading text");
                    ui.monospace("012356789  ! Monospace text.");
                    ui.monospace("Hello world! This works good.");
                    _ = ui.button("Button text");
                });
        });
        ctx.request_repaint();
    }
}
