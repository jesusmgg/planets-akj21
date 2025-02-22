use macroquad::{
    camera::{set_camera, Camera2D},
    math::{f32, IVec2, Rect},
    texture::{load_texture, set_default_filter_mode, FilterMode, Texture2D},
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
    pub planet_current: usize,
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
        let planet_current = 0;

        Self {
            styles,

            camera,
            mouse_pos,
            tile_highlighted,

            level_active,
            levels,
            planet_current,
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

    fn create_levels(styles: &Styles) -> Vec<Level> {
        use crate::planet::PlanetState::*;

        let levels = vec![
            Level {
                name: "Level 1",
                planets: vec![
                    Planet::new(Pending, 8.0, styles.colors.white),
                    Planet::new(Placed(IVec2::new(5, 5)), 8.0, styles.colors.yellow_2),
                ],
            },
            Level {
                name: "Level 2",
                planets: vec![
                    Planet::new(Pending, 8.0, styles.colors.white),
                    Planet::new(Pending, 8.0, styles.colors.grey_light),
                ],
            },
        ];

        levels
    }
}

pub struct Level {
    pub name: &'static str,
    pub planets: Vec<Planet>,
}
