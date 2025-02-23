mod constants;
mod game_state;
mod planet;
mod styles;
mod text;

use constants::*;
use game_state::GameState;
use macroquad::{
    audio::{play_sound, play_sound_once, stop_sound, PlaySoundParams},
    prelude::*,
};
use planet::PlanetState;
use text::draw_scaled_text;

#[macroquad::main("akj-21")]
async fn main() {
    configure();
    let camera = get_camera();

    let mut game_state = GameState::new().await;

    loop {
        game_state.mouse_pos = camera.screen_to_world(f32::Vec2::from(mouse_position()));

        update_next_level(&mut game_state);
        if setup_level(&mut game_state) {
            next_frame().await;
            continue;
        }

        update_planets(&mut game_state);
        update_sim(&mut game_state);
        update_score(&mut game_state);

        clear_background(game_state.styles.colors.black_1);
        render_grid(&mut game_state);
        render_level_name(&game_state);
        render_planets(&mut game_state);
        render_level_failed(&game_state);
        render_help(&game_state);
        render_score(&game_state);

        update_win_condition(&mut game_state);

        next_frame().await
    }
}

fn render_level_name(game_state: &GameState) {
    let level = match game_state.current_level() {
        None => return,
        Some(level) => level,
    };

    let font_size = 16.0;
    let message_size = 148.0;
    let pos_message_x = SCREEN_W / 2.0 - message_size / 2.0;
    let pos_message_y = 4.0;
    draw_rectangle(
        pos_message_x - 2.0,
        pos_message_y - 2.0,
        message_size + 4.0,
        16.0 + 4.0,
        game_state.styles.colors.yellow_4,
    );
    draw_rectangle(
        pos_message_x,
        pos_message_y,
        message_size,
        16.0,
        game_state.styles.colors.yellow_1,
    );
    draw_scaled_text(
        format!("{}", level.name).as_str(),
        pos_message_x,
        pos_message_y + font_size / 1.333,
        font_size,
        &game_state.styles.colors.black_1,
    );
}

fn render_score(game_state: &GameState) {
    let font_size = 12.0;
    let pos_message_x = 8.0;
    let pos_message_y = SCREEN_H - font_size * 1.666;
    draw_scaled_text(
        format!("Score: {}", game_state.score).as_str(),
        pos_message_x,
        pos_message_y,
        font_size,
        &game_state.styles.colors.white,
    );
}

fn render_help(game_state: &GameState) {
    let font_size = 12.0;
    let pos_message_x = 8.0;
    let pos_message_y = SCREEN_H - font_size * 0.666;
    draw_scaled_text(
        "<R> to retry level",
        pos_message_x,
        pos_message_y,
        font_size,
        &game_state.styles.colors.grey_mid,
    );
}

/// Returns `true` if level was setup this frame
fn setup_level(game_state: &mut GameState) -> bool {
    let level = match game_state.current_level_mut() {
        None => return false,
        Some(level) => level,
    };

    if level.is_setup {
        return false;
    }

    level.is_setup = true;

    game_state.planet_current_index = 0;
    game_state.sim_step = 0;
    game_state.sim_step_computed = 0;

    play_sound_once(&game_state.sfx_level_start_01);

    true
}

fn update_next_level(game_state: &mut GameState) {
    let level_count = game_state.levels.len();
    let mut level_index: isize = match game_state.level_active {
        Some(i) => i as isize,
        None => -1,
    };

    // Restart level
    if is_key_pressed(KeyCode::R) {
        match game_state.current_level_mut() {
            None => {}
            Some(level) => level.reset(),
        }
    }
    // Change level
    else if is_key_pressed(KeyCode::F1) {
        match game_state.current_level_mut() {
            None => {}
            Some(level) => level.reset(),
        }
        level_index -= 1;
        level_index = clamp(level_index, 0, level_count as isize - 1);
        game_state.level_active = Some(level_index as usize);
        match game_state.current_level_mut() {
            None => {}
            Some(level) => level.reset(),
        }
    } else if is_key_pressed(KeyCode::F2) {
        match game_state.current_level_mut() {
            None => {}
            Some(level) => level.reset(),
        }
        level_index += 1;
        level_index = clamp(level_index, 0, level_count as isize - 1);
        game_state.level_active = Some(level_index as usize);
        match game_state.current_level_mut() {
            None => {}
            Some(level) => level.reset(),
        }
    } else if is_key_pressed(KeyCode::F3) {
        // TODO(Jesus): Remove before release.
        match game_state.current_level_mut() {
            None => {}
            Some(level) => level.reset(),
        }
        game_state.level_active = Some(0);
        match game_state.current_level_mut() {
            None => {}
            Some(level) => level.reset(),
        }
    } else if is_key_pressed(KeyCode::F4) {
        // TODO(Jesus): Remove before release.
        match game_state.current_level_mut() {
            None => {}
            Some(level) => level.reset(),
        }
        game_state.level_active = Some(level_count - 1);
        match game_state.current_level_mut() {
            None => {}
            Some(level) => level.reset(),
        }
    }

    let level = match game_state.current_level_mut() {
        None => return,
        Some(level) => level,
    };

    if level.is_stable {
        if is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
            let current_level_i = match game_state.level_active {
                Some(i) => i,
                None => return,
            };

            if current_level_i + 1 >= game_state.levels.len() {
            } else {
                // Load next level
                game_state.level_active = Some(current_level_i + 1);
            }
        }
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

    // Moves computation
    let planets_clone = level.planets.clone();
    let mut i: usize = 0;
    for planet in &mut level.planets {
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
                    // Row gravity
                    else if other_tile.y == tile.y {
                        if other_tile.x < tile.x && other_planet.has_gravity_right() {
                            planet.sim_tile_delta.x += -1;
                            planet.sim_tile_delta.x = clamp(planet.sim_tile_delta.x, -1, 1);
                        } else if other_tile.x > tile.x && other_planet.has_gravity_left() {
                            planet.sim_tile_delta.x += 1;
                            planet.sim_tile_delta.x = clamp(planet.sim_tile_delta.x, -1, 1);
                        }
                    }
                    // Column gravity
                    else if other_tile.x == tile.x {
                        if other_tile.y < tile.y && other_planet.has_gravity_down() {
                            planet.sim_tile_delta.y += -1;
                            planet.sim_tile_delta.y = clamp(planet.sim_tile_delta.y, -1, 1);
                        } else if other_tile.y > tile.y && other_planet.has_gravity_up() {
                            planet.sim_tile_delta.y += 1;
                            planet.sim_tile_delta.y = clamp(planet.sim_tile_delta.y, -1, 1);
                        }
                    }

                    j += 1;
                }
            }

            if let PlanetState::Placed(_) = planet.state {
                planet.sim_tile_delta.x = clamp(planet.sim_tile_delta.x, -1, 1);
                planet.sim_tile_delta.y = clamp(planet.sim_tile_delta.y, -1, 1);
                planet.state = PlanetState::Placed(tile + planet.sim_tile_delta);
            }
            i += 1;
        }
    }

    // Collisions computation
    let planets_clone = level.planets.clone();
    let mut i: usize = 0;
    for planet in &mut level.planets {
        if let PlanetState::Placed(tile) = planet.state {
            let mut j: usize = 0;
            for other_planet in &planets_clone {
                if let PlanetState::Placed(other_tile) = other_planet.state {
                    if i == j {
                        j += 1;
                        continue;
                    }

                    if tile == other_tile {
                        planet.state = PlanetState::Colliding(tile);
                        level.is_failed = true;
                        continue;
                    }

                    j += 1;
                }
            }

            i += 1;
        }
    }

    game_state.sim_step_computed += 1;
}

fn update_win_condition(game_state: &mut GameState) {
    let colors = game_state.styles.colors.clone();

    let level_count = game_state.levels.len();
    let is_last_level = match game_state.level_active {
        None => false,
        Some(i) => i >= level_count - 1,
    };

    let level = match game_state.current_level_mut() {
        None => return,
        Some(level) => level,
    };

    // Check for stable system
    for planet in &level.planets {
        if let PlanetState::Placed(_) = planet.state {
            level.is_stable = planet.sim_tile_delta.x == 0 && planet.sim_tile_delta.y == 0;
        } else {
            level.is_stable = false;
        }

        if !level.is_stable {
            break;
        }
    }

    if level.is_stable {
        let font_size = 16.0;
        let message_size = 132.0;
        let pos_message_x = SCREEN_W / 2.0 - message_size / 2.0;
        let mut pos_message_y = (SCREEN_H * 0.333) - font_size;
        draw_rectangle(
            pos_message_x - 2.0,
            pos_message_y - 2.0,
            message_size + 4.0,
            32.0 + 4.0,
            colors.yellow_4,
        );
        draw_rectangle(
            pos_message_x,
            pos_message_y,
            message_size,
            32.0,
            colors.yellow_2,
        );
        draw_scaled_text(
            "Stable system!",
            pos_message_x,
            pos_message_y + font_size / 1.333,
            font_size,
            &colors.black_1,
        );

        pos_message_y += font_size;
        let message = if is_last_level {
            "Thanks for playing!"
        } else {
            "Click to continue"
        };
        draw_scaled_text(
            message,
            pos_message_x,
            pos_message_y + font_size / 1.333,
            font_size,
            &colors.black_1,
        );
    }

    let mut play_sound_stable = false;
    if level.is_stable && !level.was_stable {
        level.was_stable = true;
        play_sound_stable = true;
    }

    let mut play_sound_failed = false;
    if level.is_failed && !level.was_failed {
        level.was_failed = true;
        play_sound_failed = true;
    }

    if play_sound_stable {
        stop_sound(&game_state.music_level_end_01);
        play_sound(
            &game_state.music_level_end_01,
            PlaySoundParams {
                looped: false,
                volume: 0.8,
            },
        );
    }

    if play_sound_failed {
        play_sound_once(&game_state.sfx_explosion_01);
    }
}

fn update_score(game_state: &mut GameState) {
    // Total score
    let mut score = 0;
    for level in &game_state.levels {
        if level.is_stable {
            score += 100 + level.score;
        }
    }

    game_state.score = score;
}

fn update_planets(game_state: &mut GameState) {
    let mut play_sound_place = false;
    let mut play_sound_place_deny = false;
    let mut play_sound_remove = false;
    let mut play_sound_remove_deny = false;

    let mut score_delta = 0;

    let input_click =
        is_mouse_button_pressed(MouseButton::Left) || is_mouse_button_pressed(MouseButton::Right);
    let is_mouse_in_grid = game_state.is_mouse_in_grid;

    let tile = game_state.tile_highlighted;
    let planet_current_index = game_state.planet_current_index;

    let level = match game_state.current_level_mut() {
        None => return,
        Some(level) => level,
    };

    if level.is_failed || level.is_stable {
        return;
    }

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

        if is_tile_free && is_mouse_in_grid {
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
            play_sound_place = true;
            score_delta = -1;
            game_state.sim_step += 1;
        } else if is_mouse_in_grid {
            play_sound_place_deny = true;
        }
    }
    // Remove planet
    else if input_click && has_placed_all && is_mouse_in_grid {
        let mut planet_index = 0;
        for planet in &mut level.planets {
            match planet.state {
                PlanetState::Placed(other_tile) => {
                    if other_tile == tile {
                        if planet.is_removable {
                            planet.remove();
                            game_state.planet_current_index = planet_index;

                            // Planed was removed, advance simulation
                            play_sound_remove = true;
                            score_delta = -1;
                            game_state.sim_step += 1;

                            break;
                        } else if is_mouse_in_grid {
                            play_sound_remove_deny = true;
                        }
                    }
                }
                _ => {}
            }

            planet_index += 1;
        }
    }

    if play_sound_place {
        play_sound_once(&game_state.sfx_planet_place_01);
    } else if play_sound_place_deny {
        play_sound_once(&game_state.sfx_planet_place_deny_01);
    } else if play_sound_remove {
        play_sound_once(&game_state.sfx_planet_remove_01);
    } else if play_sound_remove_deny {
        play_sound_once(&game_state.sfx_planet_remove_deny_01);
    }

    // New borrow for score
    let level = match game_state.current_level_mut() {
        None => return,
        Some(level) => level,
    };

    level.score += score_delta;
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

    if has_placed_all && !level.is_stable && !level.is_failed {
        draw_scaled_text(
            "Remove a planet",
            8.0,
            16.0,
            16.0,
            &game_state.styles.colors.white,
        );
    }
}

fn render_level_failed(game_state: &GameState) {
    let level = match game_state.current_level() {
        None => return,
        Some(level) => level,
    };

    if level.is_failed {
        let font_size = 16.0;
        let message_size = 162.0;
        let pos_message_x = SCREEN_W / 2.0 - message_size / 2.0;
        let pos_message_y = (SCREEN_H * 0.333) - font_size;
        draw_rectangle(
            pos_message_x - 2.0,
            pos_message_y - 2.0,
            message_size + 4.0,
            16.0 + 4.0,
            game_state.styles.colors.red_light,
        );
        draw_rectangle(
            pos_message_x,
            pos_message_y,
            message_size,
            16.0,
            game_state.styles.colors.red_dark,
        );
        draw_scaled_text(
            "Collision! <R> to retry",
            pos_message_x,
            pos_message_y + font_size / 1.333,
            font_size,
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

    let is_stable: bool;
    let is_failed: bool;

    match game_state.current_level() {
        Some(level) => {
            grid_size_px = level.grid_size_px();
            grid_offset = level.grid_offset();
            grid_tiles = level.grid_tiles;
            is_stable = level.is_stable;
            is_failed = level.is_failed;
        }
        None => {
            grid_size_px = f32::Vec2::ZERO;
            grid_offset = f32::Vec2::ZERO;
            grid_tiles = IVec2::ZERO;
            is_stable = false;
            is_failed = false;
        }
    }

    let color_lines = styles.colors.grey_dark;
    let color_dark = styles.colors.black_1;
    let color_light = styles.colors.black_2;

    // Draw alternating colored cells for a chess board effect
    game_state.is_mouse_in_grid = false;
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
                game_state.is_mouse_in_grid = true;

                if game_state.tile_highlighted_prev != game_state.tile_highlighted {
                    game_state.tile_highlighted_prev = game_state.tile_highlighted;
                    if !is_stable && !is_failed {
                        play_sound(
                            &game_state.sfx_hover_01,
                            PlaySoundParams {
                                looped: false,
                                volume: 0.1,
                            },
                        );
                    }
                }

                color = styles.colors.grey_light;
                color.a = 0.5;
                draw_rectangle(x, y, cell_w, cell_h, color);
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
