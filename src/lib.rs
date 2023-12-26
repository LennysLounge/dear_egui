pub mod colors;
pub mod imgui;

use std::collections::BTreeMap;

pub use colors::*;
use egui::{FontData, FontDefinitions, FontFamily, FontId, Style, TextStyle};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum Theme {
    Imgui,
    Cadmium,
    Acid,
    Forest,
    Sky,
    Iris,
    Violet,
    Raspberry,
    Custom { hue: f32, brightness: f32 },
}

impl Theme {
    pub fn get_style(&self) -> Style {
        match self {
            Theme::Imgui => imgui::get_style(),
            Theme::Cadmium => colors::get_style(0.0, 0.80),
            Theme::Acid => colors::get_style(70.0, 0.60),
            Theme::Forest => colors::get_style(160.0, 0.70),
            Theme::Sky => colors::get_style(212.0, 1.00),
            Theme::Iris => colors::get_style(240.0, 1.30),
            Theme::Violet => colors::get_style(290.0, 0.75),
            Theme::Raspberry => colors::get_style(310.0, 0.70),
            Theme::Custom { hue, brightness } => colors::get_style(*hue, *brightness),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Font {
    OpenSans,
    ProggyClean,
}

impl Font {
    /// Get the font definition and text styles for this font.
    pub fn get_style(&self) -> (FontDefinitions, BTreeMap<TextStyle, FontId>) {
        match self {
            Font::ProggyClean => {
                let mut fonts = FontDefinitions::default();
                //Install my own font (maybe supporting non-latin characters):
                fonts.font_data.insert(
                    "ProggyClean".to_owned(),
                    FontData::from_static(include_bytes!("../font/ProggyClean.ttf")),
                );
                // Put my font first (highest priority):
                fonts
                    .families
                    .get_mut(&FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "ProggyClean".to_owned());
                fonts
                    .families
                    .get_mut(&FontFamily::Monospace)
                    .unwrap()
                    .insert(0, "ProggyClean".to_owned());

                use FontFamily::{Monospace, Proportional};
                return (
                    fonts,
                    [
                        (TextStyle::Small, FontId::new(16.0, Proportional)),
                        (TextStyle::Body, FontId::new(16.0, Proportional)),
                        (TextStyle::Monospace, FontId::new(16.0, Monospace)),
                        (TextStyle::Button, FontId::new(16.0, Proportional)),
                        (TextStyle::Heading, FontId::new(32.0, Proportional)),
                    ]
                    .into(),
                );
            }
            Font::OpenSans => {
                let mut fonts = FontDefinitions::default();
                //Install my own font (maybe supporting non-latin characters):
                fonts.font_data.insert(
                    "OpenSans".to_owned(),
                    FontData::from_static(include_bytes!("../font/OpenSans-Regular.ttf")),
                );
                // Put my font first (highest priority):
                fonts
                    .families
                    .get_mut(&FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "OpenSans".to_owned());

                use FontFamily::{Monospace, Proportional};
                return (
                    fonts,
                    [
                        (TextStyle::Small, FontId::new(10.0, Proportional)),
                        (TextStyle::Body, FontId::new(12.0, Proportional)),
                        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
                        (TextStyle::Button, FontId::new(12.0, Proportional)),
                        (TextStyle::Heading, FontId::new(16.0, Proportional)),
                    ]
                    .into(),
                );
            }
        }
    }
}

pub fn set_theme(ctx: &egui::Context, theme: Theme, font: Font) {
    let (font, text_styles) = font.get_style();
    ctx.set_fonts(font);
    ctx.set_style(theme.get_style());
    ctx.style_mut(|style| style.text_styles = text_styles);
}
