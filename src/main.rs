mod constants;
mod game_state;
mod planet;
mod styles;
mod text;

use constants::*;
use game_state::GameState;
use macroquad::prelude::*;
use planet::PlanetState;
use text::draw_scaled_text;

#[macroquad::main("akj-21")]
async fn main() {
    let mut game_state = GameState::new().await;

    loop {
        game_state.mouse_pos = game_state
            .camera
            .screen_to_world(f32::Vec2::from(mouse_position()));

        // Restart level
        if is_key_pressed(KeyCode::R) {
            game_state.levels = GameState::create_levels(&game_state.styles);
            game_state.planet_current_index = 0;
        }

        update_planets(&mut game_state);

        clear_background(game_state.styles.colors.black_1);

        render_grid(&mut game_state);
        render_planets(&game_state);

        next_frame().await
    }
}

fn update_planets(game_state: &mut GameState) {
    let input_place = is_mouse_button_pressed(MouseButton::Left);
    let input_remove = is_mouse_button_pressed(MouseButton::Right);

    let tile = game_state.tile_highlighted;
    let planet_current_index = game_state.planet_current_index;

    let level = match game_state.current_level_mut() {
        None => return,
        Some(level) => level,
    };

    let has_placed_all = planet_current_index >= level.planets.len();

    // Skip indices from placed planets
    if !has_placed_all {
        if let PlanetState::Placed(_) = &level.planets[planet_current_index].state {
            game_state.planet_current_index += 1;
            return;
        }
    }

    // Place planet
    if input_place && !has_placed_all {
        let mut is_tile_free = true;
        for planet in &level.planets {
            match planet.state {
                PlanetState::Pending => {}
                PlanetState::Placed(other_tile) => {
                    if other_tile == tile {
                        is_tile_free = false;
                        break;
                    }
                }
            }
        }

        if is_tile_free {
            let planet_current = &mut level.planets[planet_current_index];
            planet_current.place(tile);
            game_state.planet_current_index += 1;
            return;
        }
    }

    // Remove planet
    if input_remove {
        let mut planet_index = 0;
        for planet in &mut level.planets {
            match planet.state {
                PlanetState::Pending => {}
                PlanetState::Placed(other_tile) => {
                    if other_tile == tile {
                        if planet.is_removable {
                            planet.remove();
                            game_state.planet_current_index = planet_index;
                            return;
                        }
                    }
                }
            }

            planet_index += 1;
        }
    }
}

fn render_planets(game_state: &GameState) {
    match game_state.current_level() {
        None => {}
        Some(level) => {
            let mut planet_i = 0;

            for planet in &level.planets {
                match planet.state {
                    PlanetState::Placed(_) => planet.render(&game_state),
                    PlanetState::Pending => {
                        if planet_i == game_state.planet_current_index {
                            planet.render(&game_state)
                        }
                    }
                }

                planet_i += 1;
            }
        }
    };
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
