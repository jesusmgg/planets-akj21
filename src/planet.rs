use macroquad::{
    color::{Color, WHITE},
    math::{f32, IVec2},
    shapes::{draw_circle, draw_poly, draw_poly_lines},
    texture::draw_texture,
};

use crate::{constants::*, game_state::GameState, text::draw_scaled_text};

pub struct Planet {
    pub state: PlanetState,
    pub size: f32,
    pub color: Color,

    /// Up, down, left, right
    pub gravity_field: u8,
}

impl Planet {
    pub fn new(gravity_field: u8, state: PlanetState, size: f32, color: Color) -> Self {
        Self {
            gravity_field,
            state,
            size,
            color,
        }
    }

    pub fn try_place(&mut self, tile: IVec2, game_state: &GameState) -> bool {
        let level_active = match game_state.level_active {
            Some(i) => i,
            None => return false,
        };

        for planet in &game_state.levels[level_active].planets {
            match planet.state {
                PlanetState::Pending => {
                    self.state = PlanetState::Placed(tile);
                    return true;
                }
                PlanetState::Placed(other_tile) => {
                    if other_tile == tile {
                        return false;
                    }
                    self.state = PlanetState::Placed(tile);
                    return true;
                }
            }
        }

        false
    }

    pub fn remove(&mut self) {
        self.state = PlanetState::Pending;
    }

    pub fn has_gravity_up(&self) -> bool {
        self.gravity_field & 0b1000 > 0
    }
    pub fn has_gravity_down(&self) -> bool {
        self.gravity_field & 0b0100 > 0
    }
    pub fn has_gravity_left(&self) -> bool {
        self.gravity_field & 0b0010 > 0
    }
    pub fn has_gravity_right(&self) -> bool {
        self.gravity_field & 0b0001 > 0
    }

    pub fn render(&self, game_state: &GameState) {
        let x: f32;
        let y: f32;

        match self.state {
            PlanetState::Pending => {
                draw_scaled_text(
                    "Place planet",
                    8.0,
                    16.0,
                    16.0,
                    &game_state.styles.colors.white,
                );

                x = game_state.mouse_pos.x;
                y = game_state.mouse_pos.y;
            }
            PlanetState::Placed(tile) => {
                let grid_offset: f32::Vec2;
                let grid_tiles: IVec2;

                match game_state.current_level() {
                    Some(level) => {
                        grid_offset = level.grid_offset();
                        grid_tiles = level.grid_tiles;
                    }
                    None => {
                        grid_offset = f32::Vec2::ZERO;
                        grid_tiles = IVec2::ZERO
                    }
                }

                x = tile.x as f32 * TILE_SIZE_X + grid_offset.x + TILE_SIZE_X / 2.0;
                y = tile.y as f32 * TILE_SIZE_Y + grid_offset.y + TILE_SIZE_Y / 2.0;
            }
        }
        draw_circle(x, y, self.size, self.color);

        // Draw gravity arrows
        let arrow_size = 4.0;
        let arrow_color = game_state.styles.colors.red_dark;
        if self.has_gravity_up() {
            draw_poly(x, y - arrow_size, 3, arrow_size, 90.0, arrow_color);
        }
        if self.has_gravity_down() {
            draw_poly(x, y + arrow_size, 3, arrow_size, -90.0, arrow_color);
        }
        if self.has_gravity_left() {
            draw_poly(x - arrow_size, y, 3, arrow_size, 0.0, arrow_color);
        }
        if self.has_gravity_right() {
            draw_poly(x + arrow_size, y, 3, arrow_size, -180.0, arrow_color);
        }
    }
}

pub enum PlanetState {
    Pending,
    Placed(IVec2),
}
