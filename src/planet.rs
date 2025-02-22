use macroquad::{
    color::Color,
    math::{f32, IVec2},
    shapes::{draw_circle, draw_poly},
};

use crate::{constants::*, game_state::GameState, text::draw_scaled_text};

pub struct Planet {
    pub state: PlanetState,
    pub size: f32,
    pub color: Color,

    /// Up, down, left, right
    pub gravity_field: u8,

    pub is_removable: bool,
}

impl Planet {
    pub fn new(
        gravity_field: u8,
        state: PlanetState,
        is_removable: bool,
        size: f32,
        color: Color,
    ) -> Self {
        Self {
            gravity_field,
            state,
            is_removable,

            size,
            color,
        }
    }

    pub fn place(&mut self, tile: IVec2) {
        self.state = PlanetState::Placed(tile);
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
        match self.state {
            PlanetState::Pending => {
                draw_scaled_text(
                    "Place planet",
                    8.0,
                    16.0,
                    16.0,
                    &game_state.styles.colors.white,
                );

                let x = game_state.mouse_pos.x;
                let y = game_state.mouse_pos.y;

                draw_circle(48.0, 48.0, self.size * 2.0, self.color);
                self.draw_gravity_arrows(48.0, 48.0, 2.0, game_state);
                draw_circle(x, y, self.size, self.color);
                self.draw_gravity_arrows(x, y, 1.0, game_state);
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

                let x = tile.x as f32 * TILE_SIZE_X + grid_offset.x + TILE_SIZE_X / 2.0;
                let y = tile.y as f32 * TILE_SIZE_Y + grid_offset.y + TILE_SIZE_Y / 2.0;

                draw_circle(x, y, self.size, self.color);
                self.draw_gravity_arrows(x, y, 1.0, game_state);
            }
        }
    }

    fn draw_gravity_arrows(&self, x: f32, y: f32, scale: f32, game_state: &GameState) {
        let arrow_size = 4.0 * scale;
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

#[derive(PartialEq)]
pub enum PlanetState {
    Pending,
    Placed(IVec2),
}
