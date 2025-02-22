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
    configure();
    let camera = get_camera();

    let mut game_state = GameState::new().await;

    loop {
        game_state.mouse_pos = camera.screen_to_world(f32::Vec2::from(mouse_position()));

        // Restart level
        if is_key_pressed(KeyCode::R) {
            game_state.levels = GameState::create_levels(&game_state.styles);
            game_state.planet_current_index = 0;
        }

        update_planets(&mut game_state);

        update_sim(&mut game_state);

        clear_background(game_state.styles.colors.black_1);

        render_grid(&mut game_state);
        render_planets(&mut game_state);

        next_frame().await
    }
}

fn update_sim(game_state: &mut GameState) {
    // Simulation advances 1 step when a planet is placed or removed
    if game_state.sim_step_computed >= game_state.sim_step {
        return;
    }

    let level = match game_state.current_level_mut() {
        None => return,
        Some(level) => level,
    };

    let planets_clone = level.planets.clone();

    let mut i: usize = 0;
    'outer: for planet in &mut level.planets {
        if let PlanetState::Placed(tile) = planet.state {
            planet.sim_tile_delta.x = 0;
            planet.sim_tile_delta.y = 0;

            let mut j: usize = 0;
            for other_planet in &planets_clone {
                if let PlanetState::Placed(other_tile) = other_planet.state {
                    if i == j {
                        j += 1;
                        continue;
                    }

                    // Collision
                    if other_tile == tile {
                        planet.state = PlanetState::Colliding(tile + planet.sim_tile_delta);
                        j += 1;
                        continue 'outer;
                    }
                    // Row gravity
                    else if other_tile.y == tile.y {
                        if other_tile.x < tile.x && other_planet.has_gravity_right() {
                            planet.sim_tile_delta.x += -1;
                        } else if other_tile.x > tile.x && other_planet.has_gravity_left() {
                            planet.sim_tile_delta.x += 1;
                        }
                    }
                    // Column gravity
                    else if other_tile.x == tile.x {
                        if other_tile.y < tile.y && other_planet.has_gravity_down() {
                            planet.sim_tile_delta.y += -1;
                        } else if other_tile.y > tile.y && other_planet.has_gravity_up() {
                            planet.sim_tile_delta.y += 1;
                        }
                    }

                    j += 1;
                }
            }

            if let PlanetState::Placed(_) = planet.state {
                planet.state = PlanetState::Placed(tile + planet.sim_tile_delta);
            }
            i += 1;
        }
    }

    game_state.sim_step_computed += 1;
}

fn update_planets(game_state: &mut GameState) {
    let input_click =
        is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Right);

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
    if input_click && !has_placed_all {
        let mut is_tile_free = true;
        let grid_offset = level.grid_offset();

        for planet in &level.planets {
            match planet.state {
                PlanetState::Placed(other_tile) => {
                    if other_tile == tile {
                        is_tile_free = false;
                        break;
                    }
                }
                _ => {}
            }
        }

        if is_tile_free {
            let planet_current = &mut level.planets[planet_current_index];
            planet_current.place(tile, grid_offset);

            let mut next_index = 0;
            for planet in &level.planets {
                if let PlanetState::Pending = planet.state {
                    break;
                }
                next_index += 1;
            }

            game_state.planet_current_index = next_index;

            // Planed was placed, advance simulation
            game_state.sim_step += 1;

            return;
        }
    }

    // Remove planet
    if input_click && has_placed_all {
        let mut planet_index = 0;
        for planet in &mut level.planets {
            match planet.state {
                PlanetState::Placed(other_tile) => {
                    if other_tile == tile {
                        if planet.is_removable {
                            planet.remove();
                            game_state.planet_current_index = planet_index;

                            // Planed was removed, advance simulation
                            game_state.sim_step += 1;

                            return;
                        }
                    }
                }
                _ => {}
            }

            planet_index += 1;
        }
    }
}

fn render_planets(game_state: &mut GameState) {
    // TODO(Jesus): major data reestructuring needed to avoid these.
    let game_state_clone = game_state.clone();

    match game_state.current_level_mut() {
        None => {}
        Some(level) => {
            let mut planet_i = 0;

            for planet in &mut level.planets {
                planet.render_stack(planet_i, &game_state_clone);
                match planet.state {
                    PlanetState::Placed(_) => planet.render(&game_state_clone),
                    PlanetState::Colliding(_) => planet.render(&game_state_clone),
                    PlanetState::Pending => {
                        if planet_i == game_state_clone.planet_current_index {
                            planet.render(&game_state_clone)
                        }
                    }
                }

                planet_i += 1;
            }
        }
    };

    let planet_current_index = game_state.planet_current_index;

    let level = match game_state.current_level() {
        None => return,
        Some(level) => level,
    };

    let has_placed_all = planet_current_index >= level.planets.len();

    if has_placed_all {
        draw_scaled_text(
            "Remove a planet",
            8.0,
            16.0,
            16.0,
            &game_state.styles.colors.white,
        );
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

fn configure() {
    set_default_filter_mode(FilterMode::Nearest);
}

fn get_camera() -> Camera2D {
    let camera_rect = Rect {
        x: 0.0,
        y: 0.0,
        w: SCREEN_W,
        h: SCREEN_H,
    };

    let camera_target = f32::vec2(
        camera_rect.x + camera_rect.w / 2.,
        camera_rect.y + camera_rect.h / 2.,
    );
    let camera_zoom = f32::vec2(1. / camera_rect.w * 2., 1. / camera_rect.h * 2.);

    let camera = Camera2D {
        target: camera_target,
        zoom: camera_zoom,
        offset: f32::Vec2::ZERO,
        rotation: 0.,

        render_target: None,
        viewport: None,
    };

    set_camera(&camera);

    camera
}
