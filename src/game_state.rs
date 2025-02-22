use macroquad::{
    camera::{set_camera, Camera2D},
    math::{f32, IVec2, Rect},
    texture::{load_texture, set_default_filter_mode, FilterMode, Texture2D},
};

use crate::styles::Styles;
use crate::{constants::*, planet::Planet};

#[derive(Clone)]
pub struct GameState {
    pub styles: Styles,

    pub mouse_pos: f32::Vec2,
    pub tile_highlighted: IVec2,

    pub levels: Vec<Level>,
    pub level_active: Option<usize>,
    pub planet_current_index: usize,

    pub sim_step: usize,
    pub sim_step_computed: usize,

    pub texture_explosion_01: Texture2D,
}

impl GameState {
    pub async fn new() -> Self {
        let styles = Styles::new();

        let mouse_pos = f32::Vec2::ZERO;
        let tile_highlighted = IVec2::ZERO;

        let levels = GameState::create_levels(&styles);
        let level_active = Some(0);
        let planet_current_index = 0;

        let sim_step = 0;
        let sim_step_computed = 0;

        let texture_explosion_01 = load_texture("assets/explosion_01.png").await.unwrap();

        Self {
            styles,

            mouse_pos,
            tile_highlighted,

            level_active,
            levels,
            planet_current_index,

            sim_step,
            sim_step_computed,

            texture_explosion_01,
        }
    }

    pub fn current_level_mut(&mut self) -> Option<&mut Level> {
        match self.level_active {
            None => return None,
            Some(i) => return Some(&mut self.levels[i]),
        }
    }

    pub fn current_level(&self) -> Option<&Level> {
        match self.level_active {
            None => return None,
            Some(i) => return Some(&self.levels[i]),
        }
    }

    pub fn create_levels(styles: &Styles) -> Vec<Level> {
        use crate::planet::PlanetState::*;

        let levels = vec![
            Level::new(
                "Level 1",
                IVec2::new(7, 7),
                vec![
                    Planet::new(0b0000, Pending, true, 8.0, styles.colors.white),
                    Planet::new(0b1001, Pending, true, 8.0, styles.colors.yellow_1),
                    Planet::new(0b0011, Pending, true, 8.0, styles.colors.grey_mid),
                    Planet::new(
                        0b1111,
                        Placed(IVec2::new(5, 5)),
                        false,
                        8.0,
                        styles.colors.yellow_2,
                    ),
                    Planet::new(0b0001, Pending, true, 8.0, styles.colors.yellow_4),
                    Planet::new(0b0001, Pending, true, 8.0, styles.colors.grey_light),
                ],
            ),
            Level::new(
                "Level 2",
                IVec2::new(6, 7),
                vec![
                    Planet::new(0b1111, Pending, true, 8.0, styles.colors.white),
                    Planet::new(0b1111, Pending, true, 8.0, styles.colors.grey_light),
                    Planet::new(0b0000, Pending, true, 9.0, styles.colors.yellow_1),
                ],
            ),
        ];

        levels
    }
}

#[derive(Clone)]
pub struct Level {
    pub name: &'static str,
    pub planets: Vec<Planet>,
    pub grid_tiles: IVec2,

    pub is_failed: bool,
}

impl Level {
    pub fn new(name: &'static str, grid_tiles: IVec2, planets: Vec<Planet>) -> Self {
        let is_failed = false;

        Self {
            name,
            planets,
            grid_tiles,
            is_failed,
        }
    }

    pub fn grid_size_px(&self) -> f32::Vec2 {
        f32::Vec2::new(
            TILE_SIZE_X * self.grid_tiles.x as f32,
            TILE_SIZE_Y * self.grid_tiles.y as f32,
        )
    }

    pub fn grid_offset(&self) -> f32::Vec2 {
        let grid_size_px = self.grid_size_px();

        f32::Vec2::new(
            (SCREEN_W - grid_size_px.x) / 2.0,
            (SCREEN_H - grid_size_px.y) / 2.0,
        )
    }
}
