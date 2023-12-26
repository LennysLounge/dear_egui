use egui::{
    epaint::Shadow,
    style::{Interaction, Selection, Spacing, WidgetVisuals, Widgets},
    Color32, FontFamily, FontId, Margin, Rounding, Stroke, Style, TextStyle, Vec2, Visuals,
};

/// Converts hsv color space to rgb egui::Color32
///
/// `h` is in the range of 0 <= h < 360.
/// If h exceeds this range it is taken mod 360.
///
/// `s` and `v` are clamped to the range 0 <= x <= 1.
pub fn from_hsv(h: f32, s: f32, v: f32) -> Color32 {
    let h = h.abs() % 360.0;
    let s = s.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);
    // Formula from https://www.rapidtables.com/convert/color/hsv-to-rgb.html
    let c = v * s;
    let x = (1.0 - ((h / 60.0) % 2.0 - 1.0).abs()) * c;
    let m = v - c;
    let (r, g, b) = match h {
        h if 0.0 <= h && h < 60.0 => (c, x, 0.0),
        h if 60.0 <= h && h < 120.0 => (x, c, 0.0),
        h if 120.0 <= h && h < 180.0 => (0.0, c, x),
        h if 180.0 <= h && h < 240.0 => (0.0, x, c),
        h if 240.0 <= h && h < 300.0 => (x, 0.0, c),
        h if 300.0 <= h && h < 360.0 => (c, 0.0, x),
        _ => unreachable!(),
    };
    Color32::from_rgb(
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

pub fn get_style(hue: f32, brightness: f32) -> Style {
    let c1 = from_hsv(hue, 0.60, 0.27 * brightness);
    let c2 = from_hsv(hue, 0.67, 0.42 * brightness);
    let c3 = from_hsv(hue, 0.71, 0.67 * brightness);
    let c4 = from_hsv(hue, 0.94, 0.96 * brightness);
    let c5 = from_hsv(hue, 0.73, 0.98 * brightness);

    Style {
        // override the text styles here:
        // override_text_style: Option<TextStyle>

        // override the font id here:
        // override_font_id: Option<FontId>

        // set your text styles here:
        text_styles: {
            use FontFamily::{Monospace, Proportional};
            [
                (TextStyle::Small, FontId::new(10.0, Proportional)),
                (TextStyle::Body, FontId::new(12.0, Proportional)),
                (TextStyle::Monospace, FontId::new(12.0, Monospace)),
                (TextStyle::Button, FontId::new(12.0, Proportional)),
                (TextStyle::Heading, FontId::new(16.0, Proportional)),
            ]
            .into()
        },

        // set your drag value text style:
        // drag_value_text_style: TextStyle,
        spacing: Spacing {
            item_spacing: Vec2 { x: 4.0, y: 4.0 },
            window_margin: Margin::same(6.0),
            button_padding: Vec2 { x: 4.0, y: 1.0 },
            menu_margin: Margin::same(6.0),
            indent: 18.0,
            interact_size: Vec2 { x: 40.0, y: 18.0 },
            slider_width: 100.0,
            combo_width: 100.0,
            text_edit_width: 280.0,
            icon_width: 14.0,
            icon_width_inner: 8.0,
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
            tooltip_delay: 1.0,
        },
        visuals: Visuals {
            dark_mode: true,
            override_text_color: None,
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: Color32::RED,
                    weak_bg_fill: Color32::from_gray(27),
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(70)),
                    rounding: Rounding::same(2.0),
                    fg_stroke: Stroke::new(1.0, Color32::WHITE),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: c1,
                    weak_bg_fill: c2,
                    bg_stroke: Stroke::new(0.0, Color32::WHITE),
                    rounding: Rounding::same(2.0),
                    fg_stroke: Stroke::new(1.0, Color32::WHITE),
                    expansion: 0.0,
                },
                hovered: WidgetVisuals {
                    bg_fill: c2,
                    weak_bg_fill: c5,
                    bg_stroke: Stroke::new(0.0, Color32::WHITE),
                    rounding: Rounding::same(3.0),
                    fg_stroke: Stroke::new(1.0, Color32::WHITE),
                    expansion: 1.0,
                },
                active: WidgetVisuals {
                    bg_fill: c3,
                    weak_bg_fill: c4,
                    bg_stroke: Stroke::new(0.0, Color32::WHITE),
                    rounding: Rounding::same(2.0),
                    fg_stroke: Stroke::new(1.0, Color32::WHITE),
                    expansion: 1.0,
                },
                open: WidgetVisuals {
                    bg_fill: c2,
                    weak_bg_fill: c5,
                    bg_stroke: Stroke::new(0.0, Color32::WHITE),
                    rounding: Rounding::same(2.0),
                    fg_stroke: Stroke::new(1.0, Color32::WHITE),
                    expansion: 0.0,
                },
            },
            selection: Selection {
                bg_fill: c4,
                stroke: Stroke::new(1.0, Color32::WHITE),
            },
            hyperlink_color: Color32::from_rgba_premultiplied(90, 170, 255, 255),
            faint_bg_color: Color32::from_rgba_premultiplied(5, 5, 5, 0),
            extreme_bg_color: Color32::from_rgba_premultiplied(10, 10, 10, 255),
            code_bg_color: Color32::from_rgba_premultiplied(64, 64, 64, 255),
            warn_fg_color: Color32::from_rgba_premultiplied(255, 143, 0, 255),
            error_fg_color: Color32::from_rgba_premultiplied(255, 0, 0, 255),
            window_rounding: Rounding::same(0.0),
            window_shadow: Shadow {
                extrusion: 0.0,
                color: Color32::from_rgba_premultiplied(0, 0, 0, 0),
            },
            window_fill: Color32::from_rgba_premultiplied(15, 15, 15, 255),
            window_stroke: Stroke {
                width: 1.0,
                color: Color32::from_rgba_premultiplied(70, 70, 70, 255),
            },
            menu_rounding: Rounding::ZERO,
            panel_fill: Color32::from_rgba_premultiplied(15, 15, 15, 255),
            popup_shadow: Shadow {
                extrusion: 0.0,
                color: Color32::from_rgba_premultiplied(0, 0, 0, 0),
            },
            resize_corner_size: 12.0,
            text_cursor_preview: false,
            clip_rect_margin: 3.0,
            button_frame: true,
            collapsing_header_frame: true,
            indent_has_left_vline: true,
            striped: false,
            slider_trailing_fill: true,
            text_cursor: Stroke::new(2.0, Color32::from_rgb(192, 222, 255)),
            interact_cursor: None,
            image_loading_spinners: true,
        },
        animation_time: 0.0833333358168602,
        explanation_tooltips: false,
        ..Default::default()
    }
}
