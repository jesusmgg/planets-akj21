use macroquad::{
    camera::{set_camera, Camera2D},
    math::{f32, IVec2, Rect},
    texture::{set_default_filter_mode, FilterMode},
};

use crate::styles::Styles;
use crate::{constants::*, planet::Planet};

pub struct GameState {
    pub styles: Styles,

    pub camera: Camera2D,
    pub mouse_pos: f32::Vec2,
    pub tile_highlighted: IVec2,

    pub levels: Vec<Level>,
    pub level_active: Option<usize>,
    pub planet_current_index: usize,
    pub sim_step: usize,
}

impl GameState {
    pub async fn new() -> Self {
        GameState::configure();

        let styles = Styles::new();

        let camera = GameState::get_camera();
        let mouse_pos = f32::Vec2::ZERO;
        let tile_highlighted = IVec2::ZERO;

        let levels = GameState::create_levels(&styles);
        let level_active = Some(0);
        let planet_current_index = 0;
        let sim_step = 0;

        Self {
            styles,

            camera,
            mouse_pos,
            tile_highlighted,

            level_active,
            levels,
            planet_current_index,
            sim_step,
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
            Level {
                name: "Level 1",
                grid_tiles: IVec2::new(7, 7),
                planets: vec![
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
            },
            Level {
                grid_tiles: IVec2::new(5, 5),
                name: "Level 2",
                planets: vec![
                    Planet::new(0b0000, Pending, true, 8.0, styles.colors.white),
                    Planet::new(0b0000, Pending, true, 8.0, styles.colors.grey_light),
                ],
            },
        ];

        levels
    }
}

pub struct Level {
    pub name: &'static str,
    pub planets: Vec<Planet>,
    pub grid_tiles: IVec2,
}

impl Level {
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
