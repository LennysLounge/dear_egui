use egui::{
    epaint::Shadow,
    style::{
        default_text_styles, Interaction, ScrollStyle, Selection, Spacing, WidgetVisuals, Widgets,
    },
    Color32, Margin, Rounding, Stroke, Style, TextStyle, Vec2, Visuals,
};

pub const COLOR_BACKGROUND: Color32 = Color32::from_rgb(15, 15, 15);
pub const COLOR_BORDER: Color32 = Color32::from_rgb(63, 63, 72);

pub const COLOR_INACTIVE: Color32 = Color32::from_rgb(29, 47, 73);
pub const COLOR_ACTIVE: Color32 = Color32::from_rgb(35, 69, 109);
pub const COLOR_HOVERED: Color32 = Color32::from_rgb(49, 106, 173);

pub const COLOR_TEXT_WHITE: Color32 = Color32::from_gray(240);

pub const STROKE_BORDER: Stroke = Stroke {
    width: 1.0,
    color: COLOR_BORDER,
};
pub const STROKE_WHITE: Stroke = Stroke {
    width: 1.0,
    color: Color32::WHITE,
};

pub fn get_style() -> Style {
    Style {
        // override the text styles here:
        // override_text_style: Option<TextStyle>

        // override the font id here:
        // override_font_id: Option<FontId>

        //Text style deliberatly left default. A font really
        // should supply its own text styles.
        text_styles: default_text_styles(),

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
            scroll: ScrollStyle {
                floating: false,
                bar_width: 8.0,
                handle_min_length: 12.0,
                bar_inner_margin: 4.0,
                bar_outer_margin: 0.0,
                floating_width: 2.0,
                floating_allocated_width: 12.0,
                foreground_color: false,
                ..Default::default()
            },
            menu_width: 150.0,
        },
        interaction: Interaction {
            resize_grab_radius_side: 5.0,
            resize_grab_radius_corner: 10.0,
            show_tooltips_only_when_still: true,
            tooltip_delay: 0.7,
            selectable_labels: true,
            multi_widget_text_select: true,
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
                    fg_stroke: Stroke::new(1.0, COLOR_TEXT_WHITE),
                    expansion: 0.0,
                },
                inactive: WidgetVisuals {
                    bg_fill: COLOR_INACTIVE,
                    weak_bg_fill: COLOR_INACTIVE,
                    bg_stroke: Stroke::NONE,
                    rounding: Rounding::ZERO,
                    fg_stroke: Stroke::new(1.0, COLOR_TEXT_WHITE),
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
                    fg_stroke: Stroke::new(1.0, COLOR_TEXT_WHITE),
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
            extreme_bg_color: Color32::from_rgba_premultiplied(10, 10, 10, 255),
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
            handle_shape: egui::style::HandleShape::Rect { aspect_ratio: 0.5 },
            window_highlight_topmost: false,
            numeric_color_space: egui::style::NumericColorSpace::GammaByte,
        },
        animation_time: 0.0833333358168602,
        explanation_tooltips: false,
        override_text_style: None,
        override_font_id: None,
        drag_value_text_style: TextStyle::Button,
        wrap: None,
        debug: Default::default(),
        always_scroll_the_only_direction: false,
    }
}
