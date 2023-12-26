use std::collections::BTreeMap;

use egui::{
    epaint::Shadow,
    style::{Interaction, Selection, Spacing, WidgetVisuals, Widgets},
    Color32, FontData, FontDefinitions, FontFamily, FontId, Margin, Rounding, Stroke, Style,
    TextStyle, Vec2, Visuals,
};

pub const COLOR_BACKGROUND: Color32 = Color32::from_rgb(15, 15, 15);
pub const COLOR_BORDER: Color32 = Color32::from_rgb(63, 63, 72);

pub const COLOR_INACTIVE: Color32 = Color32::from_rgb(29, 47, 73);
pub const COLOR_ACTIVE: Color32 = Color32::from_rgb(35, 69, 109);
pub const COLOR_HOVERED: Color32 = Color32::from_rgb(49, 106, 173);

pub const STROKE_BORDER: Stroke = Stroke {
    width: 1.0,
    color: COLOR_BORDER,
};
pub const STROKE_WHITE: Stroke = Stroke {
    width: 1.0,
    color: Color32::WHITE,
};

/// The font to use for egui.
pub enum Font {
    /// Use the default imgui font "Proggy Clean".
    ///
    /// Sets this font for the proportional and monospaced font family.
    ProggyClean,

    /// The the font "OpenSans-Regular" as the font.
    ///
    /// Sets only the proportional font family.
    OpenSans,
}

impl Font {
    fn get_style(&self) -> (FontDefinitions, BTreeMap<TextStyle, FontId>) {
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

pub fn set_theme(ctx: &egui::Context, font: Font) {
    let (fonts, text_styles) = font.get_style();
    ctx.set_fonts(fonts);
    ctx.set_style(Style {
        // override the text styles here:
        // override_text_style: Option<TextStyle>

        // override the font id here:
        // override_font_id: Option<FontId>
        text_styles: text_styles,
        // set your drag value text style:
        // drag_value_text_style: TextStyle,
        spacing: Spacing {
            item_spacing: Vec2 { x: 3.0, y: 3.0 },
            window_margin: Margin {
                left: 6.0,
                right: 6.0,
                top: 6.0,
                bottom: 6.0,
            },
            button_padding: Vec2 { x: 4.0, y: 1.0 },
            menu_margin: Margin {
                left: 6.0,
                right: 6.0,
                top: 6.0,
                bottom: 6.0,
            },
            indent: 18.0,
            interact_size: Vec2 { x: 40.0, y: 18.0 },
            slider_width: 100.0,
            combo_width: 100.0,
            text_edit_width: 280.0,
            icon_width: 18.0,
            icon_width_inner: 10.0,
            icon_spacing: 4.0,
            tooltip_width: 600.0,
            indent_ends_with_horizontal_line: false,
            combo_height: 200.0,
            scroll_bar_width: 8.0,
            scroll_handle_min_length: 12.0,
            scroll_bar_inner_margin: 4.0,
            scroll_bar_outer_margin: 0.0,
        },
        interaction: Interaction {
            resize_grab_radius_side: 5.0,
            resize_grab_radius_corner: 10.0,
            show_tooltips_only_when_still: true,
            tooltip_delay: 0.7,
        },
        visuals: Visuals {
            dark_mode: true,
            override_text_color: None,
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: Color32::from_rgba_premultiplied(27, 27, 27, 255),
                    weak_bg_fill: Color32::from_rgba_premultiplied(27, 27, 27, 255),
                    bg_stroke: STROKE_BORDER,
                    rounding: Rounding::ZERO,
                    fg_stroke: STROKE_WHITE,
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: COLOR_INACTIVE,
                    weak_bg_fill: COLOR_INACTIVE,
                    bg_stroke: Stroke::NONE,
                    rounding: Rounding::ZERO,
                    fg_stroke: STROKE_WHITE,
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: COLOR_HOVERED,
                    weak_bg_fill: COLOR_HOVERED,
                    bg_stroke: Stroke::NONE,
                    rounding: Rounding::ZERO,
                    fg_stroke: STROKE_WHITE,
                    expansion: 0.0,
                },
                active: WidgetVisuals {
                    bg_fill: COLOR_ACTIVE,
                    weak_bg_fill: COLOR_ACTIVE,
                    bg_stroke: Stroke::NONE,
                    rounding: Rounding::ZERO,
                    fg_stroke: STROKE_WHITE,
                    expansion: 0.0,
                },
                open: WidgetVisuals {
                    bg_fill: COLOR_INACTIVE,
                    weak_bg_fill: COLOR_INACTIVE,
                    bg_stroke: Stroke::NONE,
                    rounding: Rounding::ZERO,
                    fg_stroke: STROKE_WHITE,
                    expansion: 0.0,
                },
            },
            selection: Selection {
                bg_fill: Color32::from_rgba_premultiplied(0, 92, 128, 255),
                stroke: Stroke {
                    width: 1.0,
                    color: Color32::from_rgba_premultiplied(192, 222, 255, 255),
                },
            },
            hyperlink_color: Color32::from_rgba_premultiplied(90, 170, 255, 255),
            faint_bg_color: Color32::from_rgba_premultiplied(5, 5, 5, 0),
            extreme_bg_color: Color32::from_rgba_premultiplied(29, 47, 73, 255),
            code_bg_color: Color32::from_rgba_premultiplied(10, 10, 10, 255),
            warn_fg_color: Color32::from_rgba_premultiplied(255, 143, 0, 255),
            error_fg_color: Color32::from_rgba_premultiplied(255, 0, 0, 255),
            window_rounding: Rounding::ZERO,
            window_shadow: Shadow {
                extrusion: 32.0,
                color: Color32::from_rgba_premultiplied(0, 0, 0, 96),
            },
            window_fill: Color32::from_rgba_premultiplied(15, 15, 15, 255),
            window_stroke: Stroke {
                width: 1.0,
                color: Color32::from_rgba_premultiplied(61, 61, 73, 255),
            },
            menu_rounding: Rounding::ZERO,
            panel_fill: COLOR_BACKGROUND,
            popup_shadow: Shadow {
                extrusion: 0.0,
                color: Color32::default(),
            },
            resize_corner_size: 12.0,
            text_cursor_preview: true,
            clip_rect_margin: 3.0,
            button_frame: true,
            collapsing_header_frame: true,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: false,
            text_cursor: Stroke::new(2.0, Color32::from_rgb(192, 222, 255)),
            interact_cursor: None,
            image_loading_spinners: true,
        },
        animation_time: 0.0833333358168602,
        explanation_tooltips: false,
        ..Default::default()
    });
}
