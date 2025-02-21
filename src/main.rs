mod constants;
mod styles;

use constants::*;
use macroquad::prelude::*;
use styles::Styles;

#[macroquad::main("akj-21")]
async fn main() {
    let styles = Styles::new();

    loop {
        let mouse_pos = mouse_position();

        if is_key_pressed(macroquad::input::KeyCode::Q) {
            miniquad::window::quit();
        }

        clear_background(Color::from_hex(0x151515));

        render_grid(&styles, &mouse_pos);
        draw_text(
            "HELLO, WORLD! This is a test",
            20.0,
            20.0,
            30.0,
            styles.colors.white,
        );

        next_frame().await
    }
}

fn render_grid(styles: &Styles, mouse_pos: &(f32, f32)) {
    let cell_w = SCREEN_W / GRID_W as f32;
    let cell_h = SCREEN_H / GRID_H as f32;

    let color_dark = styles.colors.bg_dark;
    let color_light = styles.colors.bg_light;

    // Draw alternating colored cells for a chess board effect.
    for j in 0..GRID_H {
        for i in 0..GRID_W {
            let x = i as f32 * cell_w;
            let y = j as f32 * cell_h;

            let is_dark = (i + j) % 2 == 0;

            let mut color = if is_dark { color_dark } else { color_light };

            if mouse_pos.0 >= x
                && mouse_pos.0 < x + cell_w
                && mouse_pos.1 >= y
                && mouse_pos.1 < y + cell_h
            {
                // let multiplier = 2.0;
                // color.r *= multiplier;
                // color.g *= multiplier;
                // color.b *= multiplier;

                color = styles.colors.red_dark;
            }

            draw_rectangle(x, y, cell_w, cell_h, color);
        }
    }

    let color_lines = styles.colors.black;

    // Draw vertical grid lines.
    for i in 0..=GRID_W {
        let x = i as f32 * cell_w;
        draw_line(x, 0.0, x, SCREEN_H, GRID_THICKNESS, color_lines)
    }

    // Draw horizontal grid lines.
    for j in 0..=GRID_H {
        let y = j as f32 * cell_h;
        draw_line(0.0, y, SCREEN_W, y, GRID_THICKNESS, color_lines);
    }
}
