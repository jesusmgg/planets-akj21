use macroquad::{
    color::{Color, WHITE},
    math::IVec2,
    shapes::draw_circle,
    texture::draw_texture,
};

use crate::{constants::*, game_state::GameState, text::draw_scaled_text};

pub struct Planet {
    pub state: PlanetState,
    pub size: f32,
    pub color: Color,
}

impl Planet {
    pub fn new(state: PlanetState, size: f32, color: Color) -> Self {
        Self { state, size, color }
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

                let mouse_pos = &game_state.mouse_pos;

                draw_circle(mouse_pos.x, mouse_pos.y, self.size, self.color);
            }
            PlanetState::Placed(tile) => {
                let x = tile.x as f32 * TILE_SIZE_X + GRID_OFFSET_X + TILE_SIZE_X / 2.0;
                let y = tile.y as f32 * TILE_SIZE_Y + GRID_OFFSET_Y + TILE_SIZE_Y / 2.0;

                draw_circle(x, y, self.size, self.color);
            }
        }
    }
}

pub enum PlanetState {
    Pending,
    Placed(IVec2),
}
