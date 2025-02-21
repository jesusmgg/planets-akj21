use macroquad::{
    color::Color,
    text::{camera_font_scale, TextParams},
};

pub fn get_text_params(font_size: f32, color: &Color) -> TextParams {
    let (font_size, font_scale, font_aspect) = camera_font_scale(font_size);
    let text_params = TextParams {
        font_size,
        font_scale,
        font_scale_aspect: font_aspect,
        color: *color,
        ..Default::default()
    };

    text_params
}
