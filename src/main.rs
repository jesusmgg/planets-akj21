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

        clear_background(game_state.styles.colors.black_1);

        render_grid(&mut game_state);

        // Planets logic and rendering
        match game_state.current_level() {
            None => {}
            Some(level) => {
                for planet in &level.planets {
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

    let grid_size_px: f32::Vec2;
    let grid_offset: f32::Vec2;
    let grid_tiles: IVec2;

    match game_state.current_level() {
        Some(level) => {
            grid_size_px = level.grid_size_px();
            grid_offset = level.grid_offset();
            grid_tiles = level.grid_tiles;
        }
        None => {
            grid_size_px = f32::Vec2::ZERO;
            grid_offset = f32::Vec2::ZERO;
            grid_tiles = IVec2::ZERO
        }
    }

    let color_lines = styles.colors.grey_dark;
    let color_dark = styles.colors.black_1;
    let color_light = styles.colors.black_2;

    // Draw alternating colored cells for a chess board effect
    for j in 0..grid_tiles.y {
        for i in 0..grid_tiles.x {
            let x = i as f32 * cell_w + grid_offset.x;
            let y = j as f32 * cell_h + grid_offset.y;

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
    for i in 0..=grid_tiles.x {
        let x = i as f32 * cell_w;
        draw_line(
            x + grid_offset.x,
            grid_offset.y,
            x + grid_offset.x,
            grid_size_px.y + grid_offset.y,
            GRID_THICKNESS,
            color_lines,
        )
    }

    // Draw horizontal grid lines
    for j in 0..=grid_tiles.y {
        let y = j as f32 * cell_h;
        draw_line(
            grid_offset.x,
            y + grid_offset.y,
            grid_size_px.x + grid_offset.x,
            y + grid_offset.y,
            GRID_THICKNESS,
            color_lines,
        );
    }
}
