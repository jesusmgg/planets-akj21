mod constants;
mod game_state;
mod styles;
mod text;

use constants::*;
use game_state::GameState;
use macroquad::prelude::*;
use text::get_text_params;

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

        update_player(&mut game_state);

        clear_background(Color::from_hex(0x151515));

        render_grid(&mut game_state);
        render_player(&mut game_state);

        let text_params = get_text_params(16.0, &game_state.styles.colors.white);
        draw_text_ex(
            format!(
                "Tile: {}, {}",
                game_state.tile_highlighted.x, game_state.tile_highlighted.y
            )
            .as_str(),
            20.0,
            20.0,
            text_params,
        );

        next_frame().await
    }
}

fn render_grid(game_state: &mut GameState) {
    let styles = &game_state.styles;
    let mouse_pos = &game_state.mouse_pos;

    let cell_w = TILE_SIZE_X;
    let cell_h = TILE_SIZE_Y;

    let color_dark = styles.colors.grey_mid;
    let color_light = styles.colors.grey_light;

    // Draw alternating colored cells for a chess board effect.
    for j in 0..GRID_H {
        for i in 0..GRID_W {
            let x = i as f32 * cell_w;
            let y = j as f32 * cell_h;

            let is_dark = (i + j) % 2 == 0;

            let mut color = if is_dark { color_dark } else { color_light };

            if mouse_pos.x >= x
                && mouse_pos.x < x + cell_w
                && mouse_pos.y >= y
                && mouse_pos.y < y + cell_h
            {
                game_state.tile_highlighted.x = i;
                game_state.tile_highlighted.y = j;

                color = styles.colors.red_dark;
            }

            draw_rectangle(x, y, cell_w, cell_h, color);
        }
    }

    let color_lines = styles.colors.black;

    // Draw vertical grid lines.
    for i in 0..=GRID_W {
        let x = i as f32 * cell_w;
        draw_line(
            x,
            0.0,
            x,
            TILE_SIZE_Y * GRID_H as f32,
            GRID_THICKNESS,
            color_lines,
        )
    }

    // Draw horizontal grid lines.
    for j in 0..=GRID_H {
        let y = j as f32 * cell_h;
        draw_line(
            0.0,
            y,
            TILE_SIZE_X * GRID_W as f32,
            y,
            GRID_THICKNESS,
            color_lines,
        );
    }
}

fn update_player(game_state: &mut GameState) {
    let mut delta_x: i32 = 0;
    let mut delta_y: i32 = 0;

    if is_key_pressed(macroquad::input::KeyCode::J)
        || is_key_pressed(macroquad::input::KeyCode::Down)
    {
        delta_y += 1;
    }
    if is_key_pressed(macroquad::input::KeyCode::K) || is_key_pressed(macroquad::input::KeyCode::Up)
    {
        delta_y -= 1;
    }
    if is_key_pressed(macroquad::input::KeyCode::L)
        || is_key_pressed(macroquad::input::KeyCode::Right)
    {
        delta_x += 1;
    }
    if is_key_pressed(macroquad::input::KeyCode::H)
        || is_key_pressed(macroquad::input::KeyCode::Left)
    {
        delta_x -= 1;
    }

    game_state.player_tile.x = clamp(game_state.player_tile.x + delta_x, 0, GRID_W - 1);
    game_state.player_tile.y = clamp(game_state.player_tile.y + delta_y, 0, GRID_H - 1);
}

fn render_player(game_state: &mut GameState) {
    let x = game_state.player_tile.x as f32 * TILE_SIZE_X;
    let y = game_state.player_tile.y as f32 * TILE_SIZE_Y;

    draw_texture(&game_state.texture_player, x, y, WHITE);
}
