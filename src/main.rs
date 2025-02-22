mod constants;
mod game_state;
mod planet;
mod styles;
mod text;

use constants::*;
use game_state::GameState;
use macroquad::prelude::*;
use text::draw_scaled_text;

#[macroquad::main("akj-21")]
async fn main() {
    let mut game_state = GameState::new().await;

    loop {
        game_state.mouse_pos = game_state
            .camera
            .screen_to_world(f32::Vec2::from(mouse_position()));

        if is_key_pressed(macroquad::input::KeyCode::Q) {
            miniquad::window::quit();
        }

        clear_background(game_state.styles.colors.black_1);

        render_grid(&mut game_state);

        match game_state.level_active {
            None => {}
            Some(i) => {
                for planet in &game_state.levels[i].planets {
                    planet.render(&game_state);
                }
            }
        };

        next_frame().await
    }
}

fn render_grid(game_state: &mut GameState) {
    let styles = &game_state.styles;
    let mouse_pos = &game_state.mouse_pos;

    let cell_w = TILE_SIZE_X;
    let cell_h = TILE_SIZE_Y;

    let color_lines = styles.colors.grey_dark;
    let color_dark = styles.colors.black_1;
    let color_light = styles.colors.black_2;

    // Draw alternating colored cells for a chess board effect
    for j in 0..GRID_H {
        for i in 0..GRID_W {
            let x = i as f32 * cell_w + GRID_OFFSET_X;
            let y = j as f32 * cell_h + GRID_OFFSET_Y;

            let is_dark = (i + j) % 2 == 0;

            let mut color = if is_dark { color_dark } else { color_light };

            draw_rectangle(x, y, cell_w, cell_h, color);

            if mouse_pos.x >= x
                && mouse_pos.x < x + cell_w
                && mouse_pos.y >= y
                && mouse_pos.y < y + cell_h
            {
                game_state.tile_highlighted.x = i;
                game_state.tile_highlighted.y = j;

                color = styles.colors.red_dark;
                draw_rectangle(x, y, cell_w, cell_h, color);

                let font_size = 12.0;
                draw_scaled_text(
                    format!(
                        "{},{}",
                        game_state.tile_highlighted.x + 1,
                        game_state.tile_highlighted.y + 1
                    )
                    .as_str(),
                    x,
                    y + TILE_SIZE_Y - GRID_THICKNESS,
                    font_size,
                    &game_state.styles.colors.white,
                );
            }
        }
    }

    // Draw vertical grid lines
    for i in 0..=GRID_W {
        let x = i as f32 * cell_w;
        draw_line(
            x + GRID_OFFSET_X,
            GRID_OFFSET_Y,
            x + GRID_OFFSET_X,
            TILE_SIZE_Y * GRID_H as f32 + GRID_OFFSET_Y,
            GRID_THICKNESS,
            color_lines,
        )
    }

    // Draw horizontal grid lines
    for j in 0..=GRID_H {
        let y = j as f32 * cell_h;
        draw_line(
            GRID_OFFSET_X,
            y + GRID_OFFSET_Y,
            TILE_SIZE_X * GRID_W as f32 + GRID_OFFSET_X,
            y + GRID_OFFSET_Y,
            GRID_THICKNESS,
            color_lines,
        );
    }
}
