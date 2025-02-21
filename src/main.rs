mod constants;
mod game_state;
mod styles;

use constants::*;
use game_state::GameState;
use macroquad::prelude::*;

#[macroquad::main("akj-21")]
async fn main() {
    let mut game_state = GameState::new();

    loop {
        game_state.mouse_pos = mouse_position();

        if is_key_pressed(macroquad::input::KeyCode::Q) {
            miniquad::window::quit();
        }

        clear_background(Color::from_hex(0x151515));

        render_grid(&mut game_state);
        draw_text(
            format!(
                "Tile: {}, {}",
                game_state.tile_highlighted.x, game_state.tile_highlighted.y
            )
            .as_str(),
            20.0,
            20.0,
            30.0,
            game_state.styles.colors.white,
        );

        next_frame().await
    }
}

fn render_grid(game_state: &mut GameState) {
    let styles = &game_state.styles;
    let mouse_pos = &game_state.mouse_pos;

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
                game_state.tile_highlighted.x = i;
                game_state.tile_highlighted.y = j;

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
